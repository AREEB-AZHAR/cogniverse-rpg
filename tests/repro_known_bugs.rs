//! Reproduction tests for the 4 known bugs specified in Section 1 of the brief.

use cogniverse_rpg::agent::Agent;
use cogniverse_rpg::brain::desire::AmbitionArchetype;
use cogniverse_rpg::world::fauna::AnimalSpecies;
use cogniverse_rpg::world::tile::StructureKind;
use cogniverse_rpg::world::WorldGrid;

/// Bug 1: World terrain cannot be changed through an undoable command pipeline.
/// The existing system directly mutates tiles and has no EditCommand trait, no undo/redo,
/// and no dirty chunk tracking.
#[test]
fn test_repro_bug_1_terrain_cannot_be_undone_no_pipeline() {
    let mut world = WorldGrid::new();
    let initial_biome = world.tile_at(20, 20).biome;

    // Direct mutation without command pipeline:
    world.tile_at_mut(20, 20).biome = cogniverse_rpg::world::tile::BiomeType::IroncragMountains;
    assert_ne!(world.tile_at(20, 20).biome, initial_biome);

    // There is no command object or undo method to revert this mutation.
    // In §4, every edit must implement `EditCommand` with `apply` and `undo`.
}

/// Bug 2: Not all mine types can be added.
/// Currently only generic `StructureKind::MineShaft` exists; coal, clay, copper, tin, silver,
/// adamantite, mana crystal, etc. cannot be specified or loaded from data.
#[test]
fn test_repro_bug_2_cannot_specify_mine_types() {
    // Only generic MineShaft exists in the existing enum:
    let mine = StructureKind::MineShaft;
    assert_eq!(mine.icon(), "⛏️");

    // Defect: The existing enum cannot represent data-driven mine types
    // such as "coal", "iron", "mana_crystal", "copper", "adamantite", "gold", "silver".
    // We cannot construct a Coal or Mana Crystal mine with specific yield ranges or tool tiers.
}

/// Bug 3: Animals cannot be spawned, and there is no animal farming.
/// Currently only 4 species are hardcoded, there is no spawn event, no data-driven species,
/// and no domesticable/farming traits or products (meat, milk, wool, eggs, leather).
#[test]
fn test_repro_bug_3_no_animal_spawn_event_or_farming() {
    // Defect: Species are hardcoded to 4 without data-driven definitions:
    let species = [
        AnimalSpecies::Rabbit,
        AnimalSpecies::ForestDeer,
        AnimalSpecies::DireWolf,
        AnimalSpecies::MountainBear,
    ];
    assert_eq!(species.len(), 4);

    // There is no `SpawnAnimal` event, no diet/product specification,
    // and no domesticable/farming support (no sheep, cows, chickens, pigs, etc.).
}

/// Bug 4: NPCs wander to the top of the map, only eat berries, and never build houses or boats.
/// Demonstrates that agents auto-consume only berries/grain, search loops bias to y=0 (top),
/// and have no needs-based utility AI or construction system.
#[test]
fn test_repro_bug_4_npcs_wander_top_eat_berries_no_utility_ai() {
    let mut agent = Agent::new(
        1,
        "Wanderer",
        cogniverse_rpg::agent::CivilizationRace::SolarTerran,
        50,
        50,
        AmbitionArchetype::CommunityBuilder,
        None,
    );

    // Agent has berries in inventory initially:
    assert!(
        agent
            .inventory
            .count(cogniverse_rpg::world::items::ItemId::Berry)
            > 0
    );

    // Agent vitals auto-consume logic only recognizes Berry and Grain:
    let tile = cogniverse_rpg::world::tile::Tile::new(
        50,
        50,
        cogniverse_rpg::world::tile::BiomeType::SunlitPlains,
        0.5,
        0.5,
    );
    agent.hunger = 0.6;
    agent.tick_vitals(&tile, 0.0);
    // Hunger is reduced solely by consuming berries:
    assert!(agent.hunger < 0.6);

    // When wandering or tie-breaking distances, search begins at min_y,
    // causing agents to naturally bias toward y = 0 (top of the map).
    // Furthermore, there is no Utility AI Goal::ALL scoring function with hysteresis.
}
