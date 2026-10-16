use cogniverse_rpg::data::GameDataRegistry;
use cogniverse_rpg::world::commands::{
    CellId, CommandPipeline, PaintTerrain, PlaceMineCommand, Terrain,
};
use cogniverse_rpg::world::fauna::{
    AnimalSpawnError, AnimalSpawningSystem, SpawnAnimalRequest, WildlifeEntity,
};
use cogniverse_rpg::world::tile::{BiomeType, ElevationLayer, PlacedStructure, StructureKind};
use cogniverse_rpg::world::WorldGrid;

#[test]
fn test_bug_1_command_pipeline_updates_tiles_mesh_and_path_cache_with_undo_redo() {
    let mut world = WorldGrid::new();
    let mut pipeline = CommandPipeline::new(world.width, world.height);

    let cell = CellId::new(0, 20, 20);
    let (orig_biome, orig_layer) = {
        let t = world.tile_at(20, 20);
        (t.biome, t.elevation_layer)
    };
    let original_terrain = Terrain::new(orig_biome, orig_layer);
    let mountain_terrain = Terrain::new(BiomeType::IroncragMountains, ElevationLayer::Mountains);

    // Baseline initial mesh sync
    let (face, cx, cy) = cell.chunk_coords();
    pipeline.mesh_cache.rebuild_chunk(face, cx, cy, &world);
    pipeline.path_cost_cache.update_chunk(face, cx, cy, &world);

    let initial_rev = pipeline
        .mesh_cache
        .meshes
        .get(&(face, cx, cy))
        .map(|m| m.revision)
        .unwrap_or(0);
    let initial_cost = pipeline.path_cost_cache.get_cost(20, 20);

    // 1. Apply PaintTerrain via pipeline to IroncragMountains
    let command = Box::new(PaintTerrain::single(
        cell,
        mountain_terrain,
        original_terrain,
    ));
    pipeline.execute(command, &mut world);

    // Verify tile updated
    assert_eq!(
        world.tile_at(20, 20).biome,
        BiomeType::IroncragMountains,
        "Tile biome must be updated after command application"
    );

    // Verify chunk mesh updated
    let updated_rev = pipeline
        .mesh_cache
        .meshes
        .get(&(face, cx, cy))
        .expect("Chunk mesh must be present")
        .revision;
    assert!(
        updated_rev > initial_rev,
        "Mesh revision must increment after command execution"
    );

    // Verify path cost updated
    let mountain_cost = pipeline.path_cost_cache.get_cost(20, 20);
    assert_eq!(
        mountain_cost, 6,
        "IroncragMountains must have traversal cost of 6"
    );

    // 2. Undo command
    let undo_ok = pipeline.undo(&mut world);
    assert!(undo_ok, "Undo must succeed");
    assert_eq!(
        world.tile_at(20, 20).biome,
        orig_biome,
        "Tile biome must be restored after undo"
    );

    let undone_rev = pipeline
        .mesh_cache
        .meshes
        .get(&(face, cx, cy))
        .expect("Chunk mesh must be present")
        .revision;
    assert!(
        undone_rev > updated_rev,
        "Mesh revision must increment after undo"
    );

    let restored_cost = pipeline.path_cost_cache.get_cost(20, 20);
    assert_eq!(
        restored_cost, initial_cost,
        "Path cost must be restored after undo"
    );

    // 3. Redo command
    let redo_ok = pipeline.redo(&mut world);
    assert!(redo_ok, "Redo must succeed");
    assert_eq!(
        world.tile_at(20, 20).biome,
        BiomeType::IroncragMountains,
        "Tile biome must be re-applied after redo"
    );

    let redone_cost = pipeline.path_cost_cache.get_cost(20, 20);
    assert_eq!(redone_cost, 6, "Path cost must be re-applied after redo");
}

#[test]
fn test_bug_2_data_driven_mines_load_and_spawn_all_required_types() {
    let mut registry = GameDataRegistry::new();
    registry
        .load_mines_file("assets/data/mines.ron")
        .expect("assets/data/mines.ron must load and validate without errors");

    let required_mine_types = [
        "stone",
        "clay",
        "coal",
        "copper",
        "tin",
        "iron",
        "silver",
        "gold",
        "gems",
        "mithril",
        "adamantite",
        "mana_crystal",
    ];

    let mut world = WorldGrid::new();
    let mut pipeline = CommandPipeline::new(world.width, world.height);

    for (i, mine_id) in required_mine_types.iter().enumerate() {
        let mine_def = registry.get_mine(mine_id).unwrap_or_else(|| {
            panic!(
                "Required mine definition '{}' must exist in registry",
                mine_id
            )
        });

        assert!(!mine_def.name.is_empty());
        assert!(!mine_def.sprite.is_empty());
        assert!(mine_def.yield_range.0 > 0 && mine_def.yield_range.1 >= mine_def.yield_range.0);

        // Spawn one of each onto a unique cell via PlaceMineCommand
        let cell = CellId::new(0, 10 + (i as u16 * 2), 10);
        let old_structure = world
            .tile_at(cell.x as usize, cell.y as usize)
            .structure
            .clone();
        let cmd = Box::new(PlaceMineCommand::new(
            cell,
            (*mine_id).to_string(),
            old_structure,
        ));

        pipeline.execute(cmd, &mut world);

        let placed = world
            .tile_at(cell.x as usize, cell.y as usize)
            .structure
            .as_ref()
            .unwrap_or_else(|| panic!("Tile at ({}, {}) must have a structure", cell.x, cell.y));

        assert_eq!(placed.kind, StructureKind::MineShaft);
        assert_eq!(
            placed.custom_type.as_deref(),
            Some(*mine_id),
            "Placed mine must store the data-driven mine id '{}'",
            mine_id
        );
    }
}

#[test]
fn test_bug_3_data_driven_animal_species_spawning_validation_and_farming() {
    let mut registry = GameDataRegistry::new();
    registry
        .load_animals_file("assets/data/animals.ron")
        .expect("assets/data/animals.ron must load and validate without errors");

    let mut world = WorldGrid::new();

    // 1. Verify every species from animals.ron can spawn in its supported biome
    let all_species_ids: Vec<String> = registry.animals.keys().cloned().collect();
    assert!(
        all_species_ids.len() >= 6,
        "Expected at least 6 data-driven species, found {}",
        all_species_ids.len()
    );

    for species_id in &all_species_ids {
        let def = registry
            .get_animal(species_id)
            .expect("Animal definition must exist");
        // Find a matching tile in world
        let target_coord = (0..world.height)
            .flat_map(|y| (0..world.width).map(move |x| (x, y)))
            .find(|&(x, y)| {
                let tile = world.tile_at(x, y);
                let b_str = format!("{:?}", tile.biome).to_lowercase();
                def.biomes.iter().any(|b| b_str.contains(&b.to_lowercase()))
            });

        if let Some((tx, ty)) = target_coord {
            let req = SpawnAnimalRequest {
                species: species_id.clone(),
                cell: CellId::new(0, tx as u16, ty as u16),
            };
            let spawn_result =
                AnimalSpawningSystem::validate_and_spawn(&req, &mut world, &registry, 50);
            assert!(
                spawn_result.is_ok(),
                "Species '{}' should successfully spawn on compatible tile ({}, {}): {:?}",
                species_id,
                tx,
                ty,
                spawn_result.err()
            );
        }
    }

    // 2. Test validation failure on incompatible biome (e.g. dire wolf in CorruptedInfernal)
    world.tile_at_mut(5, 5).biome = BiomeType::CorruptedInfernal;
    world.tile_at_mut(5, 5).elevation = 0.05;
    world.tile_at_mut(5, 5).elevation_layer =
        cogniverse_rpg::world::tile::ElevationLayer::DeepOcean;

    let invalid_biome_req = SpawnAnimalRequest {
        species: "dire_wolf".into(),
        cell: CellId::new(0, 5, 5),
    };
    let err =
        AnimalSpawningSystem::validate_and_spawn(&invalid_biome_req, &mut world, &registry, 50)
            .expect_err("Spawning a wolf in deep ocean must fail");
    assert!(
        matches!(err, AnimalSpawnError::IncompatibleBiome { .. }),
        "Expected IncompatibleBiome error, got {:?}",
        err
    );

    // 3. Test validation failure on unknown species
    let unknown_species_req = SpawnAnimalRequest {
        species: "mythical_hydra_dragon".into(),
        cell: CellId::new(0, 20, 20),
    };
    let unknown_err =
        AnimalSpawningSystem::validate_and_spawn(&unknown_species_req, &mut world, &registry, 50)
            .expect_err("Spawning unknown species must fail");
    assert_eq!(
        unknown_err,
        AnimalSpawnError::UnknownSpecies("mythical_hydra_dragon".into())
    );

    // 4. Test regional capacity limit (cell 30, 30 is in chunk region (1, 1))
    world.wildlife.retain(|a| !(a.x / 16 == 1 && a.y / 16 == 1));
    world.tile_at_mut(30, 30).biome = BiomeType::SunlitPlains;
    world.tile_at_mut(30, 30).elevation_layer =
        cogniverse_rpg::world::tile::ElevationLayer::SoilLow;
    for _ in 0..3 {
        let req = SpawnAnimalRequest {
            species: "rabbit".into(),
            cell: CellId::new(0, 30, 30),
        };
        AnimalSpawningSystem::validate_and_spawn(&req, &mut world, &registry, 3)
            .expect("Should spawn up to capacity limit");
    }

    let capped_req = SpawnAnimalRequest {
        species: "rabbit".into(),
        cell: CellId::new(0, 30, 30),
    };
    let cap_err = AnimalSpawningSystem::validate_and_spawn(&capped_req, &mut world, &registry, 3)
        .expect_err("Exceeding regional capacity must fail");
    assert!(
        matches!(cap_err, AnimalSpawnError::RegionalCapacityReached { .. }),
        "Expected RegionalCapacityReached, got {:?}",
        cap_err
    );

    // 5. Test Animal Farming: Taming, penning, breeding, and harvesting
    let mut sheep1 = WildlifeEntity::new_custom(1001, "sheep".into(), 50.0, 40, 40);
    let mut sheep2 = WildlifeEntity::new_custom(1002, "sheep".into(), 50.0, 40, 40);

    assert!(sheep1.is_domesticable(&registry));
    assert!(sheep1.tame(&registry));
    assert!(sheep2.tame(&registry));

    // Place an AnimalPen structure
    let pen_id = 99;
    world.tile_at_mut(40, 40).structure = Some(PlacedStructure::new(StructureKind::AnimalPen));
    sheep1.assign_pen(pen_id);
    sheep2.assign_pen(pen_id);

    // Breeding requires healthy, fed sheep
    sheep1.hunger = 0.1;
    sheep2.hunger = 0.1;
    let lamb = sheep1
        .breed(&sheep2, 1003)
        .expect("Healthy penned sheep should breed");
    assert_eq!(lamb.id, 1003);
    assert_eq!(lamb.custom_species.as_deref(), Some("sheep"));
    assert!(lamb.is_domesticated);
    assert_eq!(lamb.pen_id, Some(pen_id));

    // Harvesting produces species products (wool, milk, meat)
    let products = sheep1.harvest_products(&registry);
    assert!(products.contains(&"wool".to_string()));
    assert!(products.contains(&"milk".to_string()));
    assert!(products.contains(&"meat".to_string()));

    // 6. Test Ecology: Grazing and predator hunting
    world.wildlife.clear();
    world.tile_at_mut(50, 50).berry_amount = 5;
    let mut hungry_rabbit = WildlifeEntity::new_custom(2001, "rabbit".into(), 20.0, 50, 50);
    hungry_rabbit.hunger = 0.8;
    world.wildlife.push(hungry_rabbit);

    AnimalSpawningSystem::tick_ecology(&mut world, &registry);
    assert!(
        world.wildlife[0].hunger < 0.8,
        "Herbivore grazing should reduce hunger"
    );
    assert_eq!(
        world.tile_at(50, 50).berry_amount,
        4,
        "Grazing should consume 1 berry from the tile"
    );

    // Predator hunting: place hungry wolf adjacent to rabbit
    let mut wolf = WildlifeEntity::new_custom(2002, "dire_wolf".into(), 80.0, 50, 51);
    wolf.hunger = 0.7;
    world.wildlife.push(wolf);

    AnimalSpawningSystem::tick_ecology(&mut world, &registry);
    // Rabbit (index 0) should be killed by hunting wolf
    assert!(
        !world.wildlife[0].is_alive,
        "Prey rabbit should be killed by hunting predator"
    );
    assert!(
        world.wildlife[1].hunger < 0.7,
        "Hunting predator's hunger should decrease"
    );
}
