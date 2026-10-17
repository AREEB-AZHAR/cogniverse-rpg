use cogniverse_rpg::data::GameDataRegistry;
use cogniverse_rpg::world::fauna::{
    AnimalBehaviorState, AnimalSpawningSystem, EcologyManager, WildlifeEntity,
};
use cogniverse_rpg::world::tile::{PlacedStructure, StructureKind};
use cogniverse_rpg::world::WorldGrid;

fn setup_test_registry() -> GameDataRegistry {
    let mut registry = GameDataRegistry::new();
    registry
        .load_animals_file("assets/data/animals.ron")
        .expect("assets/data/animals.ron must load and validate without errors");
    registry
}

#[test]
fn test_ecology_food_chain_grazing_hunting_and_fleeing() {
    let registry = setup_test_registry();
    let mut world = WorldGrid::new();
    world.wildlife.clear();

    // 1. Verify herbivore grazing reduces hunger and consumes vegetation
    world.tile_at_mut(20, 20).berry_amount = 6;
    let mut rabbit = WildlifeEntity::new_custom(1, "rabbit".into(), 20.0, 20, 20);
    rabbit.hunger = 0.75;
    world.wildlife.push(rabbit);

    AnimalSpawningSystem::tick_ecology(&mut world, &registry);

    assert!(
        world.wildlife[0].hunger < 0.75,
        "Herbivore grazing must reduce hunger"
    );
    assert_eq!(
        world.tile_at(20, 20).berry_amount,
        5,
        "Grazing must consume 1 vegetation unit"
    );
    assert_eq!(
        world.wildlife[0].behavior_state,
        AnimalBehaviorState::Grazing,
        "Herbivore should be in Grazing state"
    );

    // 2. Verify prey detects distant predator and flees
    let wolf = WildlifeEntity::new_custom(2, "dire_wolf".into(), 80.0, 23, 20);
    world.wildlife.push(wolf);

    let rabbit_pos_before = (world.wildlife[0].x, world.wildlife[0].y);
    AnimalSpawningSystem::tick_ecology(&mut world, &registry);

    let rabbit_pos_after = (world.wildlife[0].x, world.wildlife[0].y);
    assert_ne!(
        rabbit_pos_before, rabbit_pos_after,
        "Prey must flee from approaching predator"
    );
    assert!(
        matches!(
            world.wildlife[0].behavior_state,
            AnimalBehaviorState::Fleeing { .. }
        ),
        "Prey should enter Fleeing state"
    );

    // 3. Verify carnivore hunting strike when in ambush range (dist <= 2)
    world.wildlife.clear();
    let mut deer = WildlifeEntity::new_custom(3, "forest_deer".into(), 60.0, 30, 30);
    deer.hunger = 0.2;
    world.wildlife.push(deer);

    let mut hungry_wolf = WildlifeEntity::new_custom(4, "dire_wolf".into(), 80.0, 30, 31);
    hungry_wolf.hunger = 0.85;
    world.wildlife.push(hungry_wolf);

    AnimalSpawningSystem::tick_ecology(&mut world, &registry);

    assert!(
        !world.wildlife[0].is_alive,
        "Prey deer must be killed by hunting predator"
    );
    assert!(
        world.wildlife[1].hunger < 0.85,
        "Hunting carnivore must satisfy hunger upon feasting"
    );

    // 4. Verify omnivore behavior (Wild Boar grazes or hunts small prey)
    world.wildlife.clear();
    world.tile_at_mut(40, 40).berry_amount = 0;
    world.tile_at_mut(40, 40).grain_amount = 8;
    let mut boar = WildlifeEntity::new_custom(5, "wild_boar".into(), 75.0, 40, 40);
    boar.hunger = 0.60;
    world.wildlife.push(boar);

    AnimalSpawningSystem::tick_ecology(&mut world, &registry);

    assert!(
        world.wildlife[0].hunger < 0.60,
        "Omnivore boar should graze crops when food is present"
    );
    assert_eq!(world.tile_at(40, 40).grain_amount, 7);
}

#[test]
fn test_natural_reproduction_and_carrying_capacity() {
    let registry = setup_test_registry();
    let mut world = WorldGrid::new();
    world.wildlife.clear();

    // Place 2 healthy, mature rabbits close together
    let mut r1 = WildlifeEntity::new_custom(10, "rabbit".into(), 20.0, 15, 15);
    r1.hunger = 0.1;
    r1.age_ticks = 100;
    let mut r2 = WildlifeEntity::new_custom(11, "rabbit".into(), 20.0, 16, 15);
    r2.hunger = 0.1;
    r2.age_ticks = 100;

    world.wildlife.push(r1);
    world.wildlife.push(r2);

    // Natural reproduction with carrying capacity limit of 4
    let carrying_capacity = 4;
    let new_ids =
        AnimalSpawningSystem::tick_natural_reproduction(&mut world, &registry, carrying_capacity);

    assert_eq!(
        new_ids.len(),
        1,
        "Healthy breeding pair should produce 1 offspring"
    );
    assert_eq!(
        world.wildlife.len(),
        3,
        "Total regional count should now be 3"
    );

    // If carrying capacity is set to current count (3), no new births should be allowed
    let no_births = AnimalSpawningSystem::tick_natural_reproduction(&mut world, &registry, 3);
    assert!(
        no_births.is_empty(),
        "Carrying capacity must cap population growth"
    );
}

#[test]
fn test_extinction_detection_and_repopulation_events() {
    let registry = setup_test_registry();
    let mut world = WorldGrid::new();
    world.wildlife.clear();

    let mut ecology = EcologyManager::new(8, 50);

    // Ensure region (2, 2) has viable land vegetation and Plains biome
    world.tile_at_mut(35, 35).biome = cogniverse_rpg::world::tile::BiomeType::SunlitPlains;
    world.tile_at_mut(35, 35).elevation_layer =
        cogniverse_rpg::world::tile::ElevationLayer::SoilLow;
    world.tile_at_mut(35, 35).berry_amount = 5;

    // Tick 1: Detects that "rabbit" is extinct in region (2, 2)
    let repops = ecology.tick_extinctions_and_repopulation(&mut world, &registry, 1);
    assert!(
        repops.is_empty(),
        "Repopulation should not trigger before cooldown"
    );
    assert!(
        ecology
            .active_extinctions
            .iter()
            .any(|e| e.species == "rabbit" && e.region == (2, 2)),
        "Active extinction record must be logged for region (2, 2)"
    );

    // Tick 60: Cooldown (50 ticks) expired, vegetation is present -> repopulation event fires!
    let repops_after = ecology.tick_extinctions_and_repopulation(&mut world, &registry, 60);

    assert!(
        !repops_after.is_empty(),
        "Repopulation event must trigger after cooldown in viable biomes"
    );
    let rabbit_repop = repops_after
        .iter()
        .find(|r| r.species == "rabbit" && r.region == (2, 2));
    assert!(
        rabbit_repop.is_some(),
        "Rabbit herd should migrate and repopulate region"
    );

    let rabbits_in_region = world
        .wildlife
        .iter()
        .filter(|a| {
            a.is_alive
                && a.custom_species.as_deref() == Some("rabbit")
                && a.x / 16 == 2
                && a.y / 16 == 2
        })
        .count();
    assert!(
        rabbits_in_region >= 2,
        "A small repopulation herd must be spawned"
    );
}

#[test]
fn test_animal_farming_husbandry_pens_and_products() {
    let registry = setup_test_registry();
    let mut world = WorldGrid::new();
    world.wildlife.clear();

    // 1. Taming domesticable species
    let mut sheep = WildlifeEntity::new_custom(101, "sheep".into(), 50.0, 50, 50);
    assert!(sheep.is_domesticable(&registry));
    assert!(sheep.tame(&registry));
    assert!(sheep.is_domesticated);

    let mut cow = WildlifeEntity::new_custom(102, "cow".into(), 100.0, 51, 50);
    assert!(cow.tame(&registry));

    let mut chicken = WildlifeEntity::new_custom(103, "chicken".into(), 15.0, 52, 50);
    assert!(chicken.tame(&registry));

    // 2. Pen containment in AnimalPen / Barn
    let pen_id = 42;
    world.tile_at_mut(50, 50).structure = Some(PlacedStructure::new(StructureKind::AnimalPen));
    world.tile_at_mut(51, 50).structure = Some(PlacedStructure::new(StructureKind::Barn));
    world.tile_at_mut(52, 50).structure = Some(PlacedStructure::new(StructureKind::Fence));

    sheep.assign_pen(pen_id);
    cow.assign_pen(pen_id);
    chicken.assign_pen(pen_id);

    // Ticking penned animals preserves their position (no wandering outside pen)
    sheep.tick(world.width, world.height);
    assert_eq!(
        (sheep.x, sheep.y),
        (50, 50),
        "Penned sheep must stay in pen"
    );

    // 3. Domestic product production cycles
    sheep.product_cooldown = 0;
    cow.product_cooldown = 0;
    chicken.product_cooldown = 0;

    let sheep_products = sheep.tick_domestic_products(&registry).unwrap();
    assert!(sheep_products.contains(&"wool".to_string()));
    assert!(sheep_products.contains(&"milk".to_string()));
    assert!(sheep_products.contains(&"meat".to_string()));

    let cow_products = cow.tick_domestic_products(&registry).unwrap();
    assert!(cow_products.contains(&"milk".to_string()));
    assert!(cow_products.contains(&"leather".to_string()));

    let chicken_products = chicken.tick_domestic_products(&registry).unwrap();
    assert!(chicken_products.contains(&"eggs".to_string()));

    // After harvest, cooldown resets
    assert!(
        sheep.product_cooldown > 0,
        "Product cooldown must reset after harvest"
    );
    assert!(
        sheep.tick_domestic_products(&registry).is_none(),
        "Cannot harvest again while cooldown is active"
    );
}
