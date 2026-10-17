use cogniverse_rpg::agent::needs::NpcNeeds;
use cogniverse_rpg::agent::progression::{roll_rank, CharacterClass, Rank, Subclass};
use cogniverse_rpg::agent::utility_ai::{UtilityAi, UtilityGoal};
use cogniverse_rpg::agent::{Agent, CivilizationRace};
use cogniverse_rpg::brain::desire::AmbitionArchetype;
use cogniverse_rpg::world::construction::JobBoard;
use cogniverse_rpg::world::danger::DangerField;
use cogniverse_rpg::world::disaster::DisasterKind;
use cogniverse_rpg::world::tile::{BiomeType, PlacedStructure, StructureKind};
use cogniverse_rpg::world::WorldGrid;

#[test]
fn test_danger_field_diffusion_and_temporal_decay() {
    let mut field = DangerField::new(32, 32);

    // 1. Initial field is completely calm
    assert_eq!(field.sample_danger(16, 16), 0.0);

    // 2. Inject intense hazard into center tile with radius 3
    field.inject_danger(16, 16, 3, 80.0);
    let center_danger = field.sample_danger(16, 16);
    assert!(
        center_danger >= 70.0,
        "Center danger should be high: {}",
        center_danger
    );

    // 3. Test safety gradient calculation:
    // East of center (18, 16) should point east (gx > 0) away from center towards safety
    let (gx, gy) = field.sample_danger_gradient(18, 16);
    assert!(
        gx > 0.0,
        "Gradient gx should point away from high danger towards safety: gx={}",
        gx
    );
    assert!(
        gy.abs() < 0.2,
        "Gradient gy should be near zero on horizontal axis: gy={}",
        gy
    );

    // 4. Tick simulation: apply diffusion and temporal exponential decay
    let mut prev_center = center_danger;
    for _ in 0..10 {
        field.tick();
        let current = field.sample_danger(16, 16);
        assert!(
            current <= prev_center,
            "Center danger must decay over time: current={}, prev={}",
            current,
            prev_center
        );
        prev_center = current;
    }

    // After 10 ticks, danger should have decayed significantly
    assert!(
        prev_center < center_danger * 0.75,
        "Danger should have dissipated by >25% after 10 ticks: was {}, now {}",
        center_danger,
        prev_center
    );

    // Neighboring cell (16, 19) outside original injection should have received diffused hazard
    let diffused = field.sample_danger(16, 19);
    assert!(
        diffused > 0.0,
        "Neighboring cell should receive diffused danger: {}",
        diffused
    );
}

#[test]
fn test_disaster_wildfire_spread_and_hazard_generation() {
    let mut world = WorldGrid::new();

    // Setup a wooden cottage in a flammable forest tile
    world.tile_at_mut(25, 25).biome = BiomeType::WhisperingForest;
    world.tile_at_mut(25, 25).structure = Some(PlacedStructure::new(StructureKind::WoodCottage));
    let initial_health = world.tile_at(25, 25).structure.as_ref().unwrap().health;

    // Trigger wildfire disaster centered at (25, 25)
    world.disasters.trigger_disaster(
        DisasterKind::Wildfire,
        25,
        25,
        2,   // radius
        6,   // duration ticks
        1.5, // intensity
    );

    assert_eq!(world.disasters.active_disasters.len(), 1);

    // Tick the world simulation
    world.tick();

    // 1. Verify tile is engulfed in flames
    assert!(
        world.tile_at(25, 25).is_on_fire,
        "Forest tile in wildfire epicenter must ignite"
    );

    // 2. Verify danger field around wildfire is heavily elevated
    let danger_level = world.danger_field.sample_danger(25, 25);
    assert!(
        danger_level >= 25.0,
        "Danger field must spike in disaster zone: danger={}",
        danger_level
    );

    // 3. Verify wooden building took damage
    let damaged_health = world.tile_at(25, 25).structure.as_ref().unwrap().health;
    assert!(
        damaged_health < initial_health,
        "Wooden structure must suffer fire damage: initial={}, current={}",
        initial_health,
        damaged_health
    );

    // 4. Verify adaptation tracker recorded wildfire damage
    assert!(
        world.disasters.adaptation.wildfire_damage_suffered > 0,
        "Adaptation tracker must record disaster exposure"
    );
}

#[test]
fn test_temple_blessing_rank_roll_distribution_proptest() {
    use std::collections::HashMap;
    let mut rng = rand::thread_rng();
    let mut counts = HashMap::new();
    let total_rolls = 10_000;

    for _ in 0..total_rolls {
        let rank = roll_rank(&mut rng, &Rank::CANONICAL_WEIGHTS);
        *counts.entry(rank).or_insert(0) += 1;
    }

    // Ensure all 9 ranks were rolled
    for rank in Rank::ALL.iter() {
        assert!(
            counts.contains_key(rank),
            "Rank {:?} should have been rolled in {} iterations",
            rank,
            total_rolls
        );
    }

    let count_f = *counts.get(&Rank::F).unwrap_or(&0);
    let count_e = *counts.get(&Rank::E).unwrap_or(&0);
    let count_d = *counts.get(&Rank::D).unwrap_or(&0);
    let count_c = *counts.get(&Rank::C).unwrap_or(&0);
    let count_b = *counts.get(&Rank::B).unwrap_or(&0);
    let count_a = *counts.get(&Rank::A).unwrap_or(&0);
    let count_s = *counts.get(&Rank::S).unwrap_or(&0);
    let count_sss = *counts.get(&Rank::SSS).unwrap_or(&0);

    // Verify statistical bounds:
    // F: ~30.0% (expected ~3000, bounds: 2600..3400)
    assert!(
        (2600..=3400).contains(&count_f),
        "F rank frequency {} outside expected range",
        count_f
    );

    // E: ~25.0% (expected ~2500, bounds: 2150..2850)
    assert!(
        (2150..=2850).contains(&count_e),
        "E rank frequency {} outside expected range",
        count_e
    );

    // D: ~18.0% (expected ~1800, bounds: 1500..2100)
    assert!(
        (1500..=2100).contains(&count_d),
        "D rank frequency {} outside expected range",
        count_d
    );

    // SSS: ~0.1% (expected ~10, bounds: 1..35)
    assert!(
        (1..=35).contains(&count_sss),
        "SSS rank frequency {} outside expected range",
        count_sss
    );

    // Strict monotonic ordering across the common ranks
    assert!(count_f > count_e);
    assert!(count_e > count_d);
    assert!(count_d > count_c);
    assert!(count_c > count_b);
    assert!(count_b > count_a);
    assert!(count_a > count_s);
}

#[test]
fn test_npc_coming_of_age_blessing_and_class_progression() {
    let mut agent = Agent::new(
        1,
        "Aethelred",
        CivilizationRace::SolarTerran,
        30,
        30,
        AmbitionArchetype::CommunityBuilder,
        None,
    );

    // 1. Initial state: Level 1, unranked, unblessed
    assert_eq!(agent.progression.level, 1);
    assert!(agent.progression.rank.is_none());
    assert!(agent.progression.class.is_none());
    assert!(agent.progression.subclass.is_none());
    assert_eq!(agent.progression.attributes.strength, 10.0);

    // 2. Utility AI: when reaching age 16 (1600 ticks) and unblessed, scores Goal::SeekBlessing
    agent.age_ticks = 1600;
    let mut ai = UtilityAi::new();
    let needs = NpcNeeds::new();
    let job_board = JobBoard::new();

    let goal = ai.evaluate_best_goal_with_context(
        &needs, true, true, &job_board, 0.0, true, // needs_blessing = true
    );

    assert_eq!(
        goal,
        UtilityGoal::SeekBlessing,
        "16-year-old unblessed citizen must prioritize temple pilgrimage"
    );

    // 3. Perform Temple Blessing ceremony at altar: choose Warrior class
    let mut rng = rand::thread_rng();
    let rolled_rank = agent.progression.bless_at_temple(
        &mut rng,
        &Rank::CANONICAL_WEIGHTS,
        CharacterClass::Warrior,
        1600,
    );

    assert_eq!(agent.progression.rank, Some(rolled_rank));
    assert_eq!(agent.progression.class, Some(CharacterClass::Warrior));
    assert_eq!(agent.progression.blessed_at_tick, Some(1600));

    // Attributes should be multiplied by rolled rank stat multiplier
    let mult = rolled_rank.stat_multiplier();
    let expected_str = 10.0 * mult;
    assert!(
        (agent.progression.attributes.strength - expected_str).abs() < 0.01,
        "Strength should be multiplied by rank multiplier: expected={}, actual={}",
        expected_str,
        agent.progression.attributes.strength
    );

    // 4. Earn experience points to advance levels
    // Test subclass unlock by ensuring hero has rank cap >= 25 (e.g. Rank C with cap 50)
    agent.progression.rank = Some(Rank::C);
    for _ in 0..100 {
        agent.progression.gain_xp(5000);
    }

    assert!(
        agent.progression.level >= 25,
        "Agent should have leveled to at least 25: level={}",
        agent.progression.level
    );

    // Subclass must be unlocked for Warrior (Paladin)
    assert_eq!(
        agent.progression.subclass,
        Some(Subclass::Paladin),
        "Level 25+ Warrior must unlock Paladin subclass"
    );

    // 5. Verify level cap enforcement on F-rank (strictly capped at 20)
    let mut f_agent = Agent::new(
        2,
        "Peasant",
        CivilizationRace::SolarTerran,
        10,
        10,
        AmbitionArchetype::SlothHedonist,
        None,
    );
    f_agent.progression.rank = Some(Rank::F);
    f_agent.progression.class = Some(CharacterClass::Farmer);
    for _ in 0..100 {
        f_agent.progression.gain_xp(10_000);
    }
    assert_eq!(
        f_agent.progression.level, 20,
        "F-rank commoners cannot exceed level 20 cap"
    );
    assert!(
        f_agent.progression.subclass.is_none(),
        "F-rank commoners cannot unlock heroic subclasses"
    );
}
