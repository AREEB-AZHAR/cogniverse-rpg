use cogniverse_rpg::brain::ast::{ActionChoice, AstVmContext, BehavioralGenome, EntityFilter};
use cogniverse_rpg::brain::neuromod::NeuromodulationState;
use cogniverse_rpg::server::covenants::{CovenantRewardType, DivineCovenant, QuestObjectiveType};
use cogniverse_rpg::server::state::SimulationEngine;
use cogniverse_rpg::world::altar::{Altar, DeityAlignment};
use cogniverse_rpg::world::items::ItemId;

#[test]
fn test_ast_evaluation_and_gas_metering() {
    let genome = BehavioralGenome::new_default();
    let mut distances = std::collections::HashMap::new();
    distances.insert(EntityFilter::FoodSource, 1.2);
    distances.insert(EntityFilter::WaterSource, 5.0);

    let has_fn = |_item: &str| false;

    // Test with high hunger
    let ctx_hungry = AstVmContext {
        hunger: 0.85,
        thirst: 0.2,
        energy: 0.9,
        warmth: 0.8,
        social_need: 0.1,
        piety: 0.5,
        health: 100.0,
        dopamine: 0.5,
        serotonin: 0.5,
        noradrenaline: 0.2,
        nearest_distances: &distances,
        inventory_has: &has_fn,
    };

    let (action, gas_used) = genome.evaluate(&ctx_hungry);
    assert!(
        gas_used > 0 && gas_used <= 500,
        "Gas consumed must be within (0, 500]"
    );
    assert_eq!(
        action,
        ActionChoice::ForageFood,
        "Hungry agent near food must forage food"
    );
}

#[test]
fn test_ast_mutation_and_crossover() {
    let parent_a = BehavioralGenome::new_default();
    let parent_b = BehavioralGenome::new_default();

    let mut child = BehavioralGenome::crossover(&parent_a, &parent_b);
    assert_eq!(child.generation, 2, "Child of gen 1 parents must be gen 2");

    let prev_mutations = child.mutations_count;
    child.mutate(0.5);
    assert_eq!(
        child.mutations_count,
        prev_mutations + 1,
        "Mutation counter must increment"
    );
}

#[test]
fn test_neuromodulation_dynamics() {
    let mut nm = NeuromodulationState::new();
    let initial_dopamine = nm.dopamine;

    // Large reward prediction error (positive reward 1.0)
    nm.update(1.0, 0.9, true, false, 0.0);
    assert!(
        nm.dopamine > initial_dopamine,
        "Dopamine must increase when receiving a high positive reward"
    );

    // Threat encounter
    nm.update(0.0, 0.5, false, true, 0.8);
    assert!(
        nm.noradrenaline > 0.3,
        "Noradrenaline must spike during threat and unexpected surprise"
    );
}

#[test]
fn test_altar_offering_and_sincerity() {
    let mut altar = Altar::new(1, 24, 36, DeityAlignment::SolarPlayerGod, "Test Sun Altar");

    // Starving peasant offers iron ore (wealth = 10, item value = 25)
    let sincerity = Altar::calculate_sincerity(ItemId::IronOre, 1, 10.0, 0.8);
    assert!(
        sincerity > 0.5,
        "Generous sacrifice relative to wealth must yield high sincerity"
    );

    let idx = altar.receive_offering(1, 42, "Kael".into(), ItemId::IronOre, 1, 10.0, 0.8, 100);
    assert_eq!(idx, 0);
    assert_eq!(altar.recent_offerings.len(), 1);
}

#[test]
fn test_covenant_progression() {
    let mut covenant = DivineCovenant::new(
        1,
        QuestObjectiveType::StockpileGrain,
        10,
        CovenantRewardType::RainOfManna,
        "Blight on crops",
        50,
    );

    assert!(!covenant.is_completed);
    let completed_step1 = covenant.advance(4);
    assert!(!completed_step1);
    assert_eq!(covenant.current_progress, 4);

    let completed_step2 = covenant.advance(6);
    assert!(completed_step2);
    assert!(
        covenant.is_completed,
        "Covenant must complete when target is reached"
    );
}

#[test]
fn test_simulation_engine_ticks() {
    let mut engine = SimulationEngine::new();
    let initial_tick = engine.world.gaia.world_tick;
    assert_eq!(initial_tick, 0);
    assert_eq!(
        engine.agents.len(),
        16,
        "Must start with 8 Terrans and 8 Lunarians"
    );

    for _ in 0..10 {
        engine.tick();
    }

    assert_eq!(engine.world.gaia.world_tick, 10);
    assert!(!engine.event_log.is_empty(), "Events must be logged");
}

#[test]
fn test_settlement_dispersion_and_movement() {
    let mut engine = SimulationEngine::new();
    let initial_count = engine.agents.len();
    assert_eq!(initial_count, 16);

    // Simulate 60 ticks
    for _ in 0..60 {
        engine.tick();
    }

    // Agents must remain alive and thrive
    let alive_count = engine.agents.iter().filter(|a| a.is_alive).count();
    assert!(
        alive_count >= 16,
        "Population must survive and not collapse"
    );

    // Coordinates should not all be identical (no single-tile black hole)
    let first_pos = (engine.agents[0].x, engine.agents[0].y);
    let all_same = engine.agents.iter().all(|a| (a.x, a.y) == first_pos);
    assert!(
        !all_same,
        "Agents must disperse across the continent, not stack on a single tile"
    );
}

#[test]
fn test_architecture_placement_and_ore_enrichment() {
    use cogniverse_rpg::server::protocol::TerraformBrush;
    use cogniverse_rpg::world::tile::StructureKind;

    let mut engine = SimulationEngine::new();

    // Erect a stone cabin via God brush
    engine.terraform_area(50, 50, TerraformBrush::BuildCabin, 1);
    let tile = engine.world.tile_at(50, 50);
    assert!(tile.structure.is_some());
    assert_eq!(
        tile.structure.as_ref().unwrap().kind,
        StructureKind::StoneCabin
    );

    // Enrich gold vein
    let initial_gold = engine.world.tile_at(60, 60).gold_ore_amount;
    engine.terraform_area(60, 60, TerraformBrush::EnrichGoldVein, 1);
    assert!(engine.world.tile_at(60, 60).gold_ore_amount > initial_gold);

    // Verify structures in telemetry
    let telemetry = engine.generate_telemetry();
    assert!(telemetry
        .structures
        .iter()
        .any(|s| s.x == 50 && s.y == 50 && s.kind == StructureKind::StoneCabin));
}

#[test]
fn test_cultural_technology_progression() {
    let mut engine = SimulationEngine::new();
    assert_eq!(engine.civilizations.terran_culture.knowledge_points, 0);

    // Add 120 knowledge points (should unlock Primitive Toolcraft, Fire Mastery, Stone Masonry)
    let unlocks = engine.civilizations.terran_culture.add_knowledge(120);
    assert!(unlocks.len() >= 3);
    assert!(engine.civilizations.terran_culture.technologies[0].is_unlocked); // Primitive Toolcraft (25 KP)
    assert!(engine.civilizations.terran_culture.technologies[1].is_unlocked); // Fire Mastery (50 KP)
    assert!(engine.civilizations.terran_culture.technologies[2].is_unlocked); // Stone Masonry (100 KP)

    let telemetry = engine.generate_telemetry();
    assert_eq!(telemetry.terran_culture.knowledge_points, 120);
    assert!(
        !telemetry.activities.is_empty(),
        "Activities telemetry must be populated"
    );
}

#[test]
fn test_worldbox_traits_inheritance_and_rare_mutations() {
    use cogniverse_rpg::brain::reproduction::ReproductionManager;

    let parent_a_traits = vec!["Strong".to_string(), "Sunblessed".to_string()];
    let parent_b_traits = vec!["WiseScholar".to_string(), "Swift".to_string()];

    let child_traits = ReproductionManager::inherit_traits(&parent_a_traits, &parent_b_traits);
    assert!(
        !child_traits.is_empty(),
        "Child must have at least one inherited or mutated trait"
    );
}

#[test]
fn test_worldbox_plots_and_diplomacy_resolution() {
    use cogniverse_rpg::agent::CivilizationRace;
    use cogniverse_rpg::factions::plots::PlotType;
    use cogniverse_rpg::factions::DiplomaticStatus;

    let mut engine = SimulationEngine::new();
    assert_eq!(
        engine.civilizations.diplomacy,
        DiplomaticStatus::PeacefulCoexistence
    );

    // Hatch a Holy War plot
    engine.civilizations.hatch_plot(
        1,
        "Orion",
        CivilizationRace::SolarTerran,
        PlotType::HolyWar,
        "Crusade of the Golden Sun",
        "Eradicate heresy across the mountain passes",
    );

    let plot_idx = engine.civilizations.active_plots.len() - 1;
    // Fast forward plot to 100%
    engine.civilizations.active_plots[plot_idx].progress = 99.5;
    let events = engine.civilizations.tick_plots();

    assert!(
        !events.is_empty(),
        "Resolved plot must produce announcement event"
    );
    assert_eq!(
        engine.civilizations.diplomacy,
        DiplomaticStatus::OpenWarfare,
        "Holy war plot must escalate diplomacy to Open Warfare"
    );

    // Telemetry check
    let telemetry = engine.generate_telemetry();
    assert_eq!(telemetry.diplomacy_status, "Open Warfare");
    assert!(!telemetry.active_plots.is_empty());
}

#[test]
fn test_worldbox_architecture_and_divine_powers() {
    use cogniverse_rpg::server::god_powers::DivineInterventionKind;
    use cogniverse_rpg::server::protocol::TerraformBrush;
    use cogniverse_rpg::world::tile::StructureKind;

    let mut engine = SimulationEngine::new();

    // Place Town Hall and Barracks
    engine.terraform_area(40, 80, TerraformBrush::BuildTownHall, 1);
    engine.terraform_area(42, 80, TerraformBrush::BuildBarracks, 1);

    assert_eq!(
        engine
            .world
            .tile_at(40, 80)
            .structure
            .as_ref()
            .unwrap()
            .kind,
        StructureKind::TownHall
    );
    assert_eq!(
        engine
            .world
            .tile_at(42, 80)
            .structure
            .as_ref()
            .unwrap()
            .kind,
        StructureKind::Barracks
    );
    assert_eq!(
        engine
            .world
            .tile_at(40, 80)
            .structure
            .as_ref()
            .unwrap()
            .max_health,
        350.0
    );

    // Trigger Divine Inspiration
    let prev_knowledge = engine.civilizations.terran_culture.knowledge_points;
    engine.execute_intervention(DivineInterventionKind::DivineInspiration { x: 38, y: 86 });
    assert_eq!(
        engine.civilizations.terran_culture.knowledge_points,
        prev_knowledge + 60
    );

    // Trigger Golden Age Peace
    engine.execute_intervention(DivineInterventionKind::GoldenAgePeace);
    assert_eq!(
        engine.civilizations.diplomacy,
        cogniverse_rpg::factions::DiplomaticStatus::CommercialAlliance
    );
}

#[test]
fn test_lunar_people_do_not_wander_to_origin() {
    use cogniverse_rpg::agent::CivilizationRace;

    let mut engine = SimulationEngine::new();

    // Verify initial positions of Lunarians are near mountain stronghold (92, 34)
    let initial_lunarians: Vec<_> = engine
        .agents
        .iter()
        .filter(|a| a.race == CivilizationRace::ChthonicLunarian)
        .map(|a| (a.id, a.x, a.y))
        .collect();
    assert!(
        !initial_lunarians.is_empty(),
        "Must have initial Lunarian agents"
    );

    for (_id, x, y) in &initial_lunarians {
        assert!(
            *x >= 80 && *x <= 105,
            "Initial Lunarians must be in northeastern mountains, got ({}, {})",
            x,
            y
        );
        assert!(
            *y >= 20 && *y <= 50,
            "Initial Lunarians must be in northeastern mountains, got ({}, {})",
            x,
            y
        );
    }

    // Run 150 simulation ticks
    for _ in 0..150 {
        engine.tick();
    }

    // Check all Lunarians: NONE should be at (0, 0) or stuck in the deep northwest ocean
    let mut lunarian_count = 0;
    for a in &engine.agents {
        if a.race == CivilizationRace::ChthonicLunarian {
            lunarian_count += 1;
            assert!(
                !(a.x == 0 && a.y == 0),
                "Lunarian {} must NOT be at (0, 0)! Position: ({}, {})",
                a.name,
                a.x,
                a.y
            );
            assert!(
                !(a.x < 8 && a.y < 8),
                "Lunarian {} must NOT wander to deep northwest ocean! Position: ({}, {})",
                a.name,
                a.x,
                a.y
            );
            // Verify they maintain territorial proximity to food/water in eastern continent
            assert!(
                a.x >= 50,
                "Lunarians should stay in the eastern mountain/foothills realm, got ({}, {})",
                a.x,
                a.y
            );
        }
    }
    assert!(lunarian_count > 0, "Lunarians must remain alive and active");
}

#[test]
fn test_worldbox_cellular_fire_spread_and_ash() {
    use cogniverse_rpg::world::tile::{BiomeType, ElevationLayer};

    let mut engine = SimulationEngine::new();

    // Set up flammable forest on dry soil at (50, 50) and (51, 50)
    engine.world.tile_at_mut(50, 50).elevation_layer = ElevationLayer::SoilLow;
    engine.world.tile_at_mut(50, 50).biome = BiomeType::WhisperingForest;
    engine.world.tile_at_mut(50, 50).wood_amount = 8;
    engine.world.tile_at_mut(50, 50).moisture = 0.1;

    engine.world.tile_at_mut(51, 50).elevation_layer = ElevationLayer::SoilLow;
    engine.world.tile_at_mut(51, 50).biome = BiomeType::WhisperingForest;
    engine.world.tile_at_mut(51, 50).wood_amount = 8;
    engine.world.tile_at_mut(51, 50).moisture = 0.1;

    // Ignite wildfire on tile (50, 50)
    engine.world.tile_at_mut(50, 50).is_on_fire = true;
    engine.world.tile_at_mut(50, 50).fire_ticks = 4;

    // Tick cellular dynamics several times
    for _ in 0..12 {
        engine.world.tick_cellular();
    }

    // Fire should have spread to neighbor or original tile decayed to ash
    let t0 = engine.world.tile_at(50, 50);
    let t1 = engine.world.tile_at(51, 50);
    assert!(
        t0.burned_stages > 0 || t1.is_on_fire || t1.burned_stages > 0,
        "Wildfire cellular engine must spread or burn wood to ash"
    );
}

#[test]
fn test_worldbox_cellular_lava_and_basalt_solidification() {
    use cogniverse_rpg::world::tile::{BiomeType, ElevationLayer};

    let mut engine = SimulationEngine::new();

    // Place water at (60, 61) and lava at (60, 60)
    engine.world.tile_at_mut(60, 61).elevation = 0.2;
    engine.world.tile_at_mut(60, 61).elevation_layer = ElevationLayer::CloseOcean;
    engine.world.tile_at_mut(60, 61).biome = BiomeType::CrystalRiverlands;
    engine.world.tile_at_mut(60, 60).elevation = 0.85;
    engine.world.tile_at_mut(60, 60).elevation_layer = ElevationLayer::Hills;
    engine.world.tile_at_mut(60, 60).lava_level = 3;

    // Tick cellular dynamics
    for _ in 0..6 {
        engine.world.tick_cellular();
    }

    // Lava flowing into water must solidify into rock/hills (Basalt formation)
    let t_water = engine.world.tile_at(60, 61);
    assert!(
        t_water.elevation_layer >= ElevationLayer::Hills
            || t_water.biome == BiomeType::IroncragMountains,
        "Lava meeting water must solidify into basalt rock/hills"
    );
}

#[test]
fn test_worldbox_cellular_freeze_and_ice_walking() {
    use cogniverse_rpg::world::tile::{ElevationLayer, FrozenState};

    let mut engine = SimulationEngine::new();

    // Set shallow water at (70, 70) with subzero temperature
    engine.world.tile_at_mut(70, 70).elevation = 0.35;
    engine.world.tile_at_mut(70, 70).elevation_layer = ElevationLayer::ShallowWaters;
    engine.world.tile_at_mut(70, 70).temperature = -15.0;

    // Tick cellular dynamics
    for _ in 0..5 {
        engine.world.tick_cellular();
    }

    // Shallow water at freezing temperature freezes into solid walkable ice
    let t = engine.world.tile_at(70, 70);
    assert_eq!(
        t.frozen_state,
        FrozenState::Ice,
        "Freezing shallow water must become walkable Ice"
    );
    assert!(t.is_walkable(), "Ice formed on water must be walkable");
}

#[test]
fn test_worldbox_biome_creep_competition() {
    use cogniverse_rpg::world::tile::{BiomeType, ElevationLayer};

    let mut engine = SimulationEngine::new();

    // Seed corrupted biome on soil surrounded by plains
    for dx in -2isize..=2isize {
        for dy in -2isize..=2isize {
            let nx = (45isize + dx) as usize;
            let ny = (45isize + dy) as usize;
            engine.world.tile_at_mut(nx, ny).elevation_layer = ElevationLayer::SoilLow;
            engine.world.tile_at_mut(nx, ny).biome = BiomeType::SunlitPlains;
            engine.world.tile_at_mut(nx, ny).burned_stages = 0;
        }
    }
    engine.world.tile_at_mut(45, 45).biome = BiomeType::CorruptedInfernal;

    // Run cellular updates: strong biomes (grow_strength 5) can creep into low strength neighbors (grow_strength 1)
    let mut crept = false;
    for _ in 0..300 {
        engine.world.tick_cellular();
        for dx in -1isize..=1isize {
            for dy in -1isize..=1isize {
                if dx != 0 || dy != 0 {
                    let nx = (45isize + dx) as usize;
                    let ny = (45isize + dy) as usize;
                    if engine.world.tile_at(nx, ny).biome == BiomeType::CorruptedInfernal {
                        crept = true;
                        break;
                    }
                }
            }
            if crept {
                break;
            }
        }
        if crept {
            break;
        }
    }

    assert!(
        crept,
        "Corrupted biome with high grow_strength must creep into weaker neighbor tiles"
    );
}

#[test]
fn test_procedural_map_archetypes_generation() {
    use cogniverse_rpg::world::generation::{MapArchetype, MapGenerator};

    let archetypes = [
        MapArchetype::GrandArchipelago,
        MapArchetype::Pangaea,
        MapArchetype::ScatteredIslands,
        MapArchetype::RingAtoll,
        MapArchetype::TwinContinents,
        MapArchetype::GlacialWastes,
        MapArchetype::VolcanicCaldera,
    ];

    for arch in archetypes {
        let (tiles, altars, title) =
            MapGenerator::generate_world_with_params(128, 128, arch, 9999, 0.5, 0.5);

        assert_eq!(
            tiles.len(),
            128 * 128,
            "Every archetype must produce 16384 tiles"
        );
        assert_eq!(
            altars.len(),
            2,
            "Every archetype must place exactly 2 altars"
        );
        assert!(!title.is_empty(), "World title must be populated");

        // Altars must be placed on dry land
        for altar in &altars {
            let idx = altar.y * 128 + altar.x;
            assert!(
                !tiles[idx].elevation_layer.is_water(),
                "Altar for {:?} at ({}, {}) must be situated on dry land!",
                arch,
                altar.x,
                altar.y
            );
        }
    }
}

#[test]
fn test_world_reset_and_map_change() {
    use cogniverse_rpg::world::generation::MapArchetype;

    let mut engine = SimulationEngine::new();
    assert_eq!(engine.map_id, 1);
    assert_eq!(engine.map_archetype, MapArchetype::GrandArchipelago);

    // Run some ticks
    for _ in 0..15 {
        engine.tick();
    }

    // Reset world
    engine.reset_world();
    assert_eq!(engine.map_id, 2, "map_id must increment upon reset");
    assert_eq!(
        engine.agents.len(),
        16,
        "Fresh 16 pioneers must spawn on reset"
    );
    assert_eq!(engine.births_total, 16);
    assert_eq!(engine.deaths_total, 0);

    // Change map to Ring Atoll
    engine.change_map(MapArchetype::RingAtoll, Some(777), Some(0.4), Some(0.6));
    assert_eq!(engine.map_id, 3, "map_id must increment upon map change");
    assert_eq!(engine.map_archetype, MapArchetype::RingAtoll);
    assert!(
        engine.map_name.contains("Ring"),
        "Map name must reflect Ring Atoll archetype"
    );

    // Test map base64 encoding
    let encoded = engine.encode_map_base64();
    assert_eq!(
        encoded.len(),
        (128 * 128usize).div_ceil(3) * 4,
        "Base64 encoded map must match 16384 bytes encoded length"
    );

    // Telemetry generation must include full map sync when pending
    let telemetry = engine.generate_telemetry();
    assert!(
        telemetry.full_map_sync.is_some(),
        "Pending map sync must produce full_map_sync payload"
    );
    assert_eq!(telemetry.map_id, 3);
    assert_eq!(telemetry.map_archetype, MapArchetype::RingAtoll);

    // Subsequent telemetry call should clear pending flag
    let telemetry2 = engine.generate_telemetry();
    assert!(
        telemetry2.full_map_sync.is_none(),
        "Subsequent telemetry must not resend full map sync"
    );
}

#[test]
fn test_export_and_import_run_state() {
    let mut engine = SimulationEngine::new();

    // Advance simulation to accumulate achievements, memories, and learning
    for _ in 0..20 {
        engine.tick();
    }

    // Export run snapshot to JSON
    let exported_json = engine.export_run_snapshot_json();
    assert!(
        !exported_json.is_empty(),
        "Exported snapshot JSON must not be empty"
    );
    assert!(
        exported_json.contains("World of Genesis"),
        "Exported JSON must contain world title or map info"
    );
    assert!(
        exported_json.contains("milestones"),
        "Exported JSON must contain learning milestones"
    );
    assert!(
        exported_json.contains("civilizations"),
        "Exported JSON must contain civilizations"
    );

    // Modify engine state (e.g. reset world)
    engine.reset_world();
    assert_eq!(
        engine.world.gaia.world_tick, 0,
        "Reset engine should start at tick 0"
    );

    // Import exported snapshot back
    let import_res = engine.import_run_snapshot_json(&exported_json);
    assert!(
        import_res.is_ok(),
        "Importing valid snapshot JSON must succeed"
    );
    assert_eq!(
        engine.world.gaia.world_tick, 20,
        "Imported state must restore tick count 20"
    );
    assert_eq!(
        engine.agents.len(),
        16,
        "Imported state must restore agents"
    );
}
