use cogniverse_rpg::brain::ast::{
    ActionChoice, AstVmContext, BehavioralGenome, EntityFilter, Expr, Op, Stmt, VitalType,
};
use cogniverse_rpg::brain::neuromod::NeuromodulationState;
use cogniverse_rpg::world::altar::{Altar, DeityAlignment};
use cogniverse_rpg::world::items::ItemId;
use cogniverse_rpg::world::WorldGrid;
use cogniverse_rpg::server::covenants::{DivineCovenant, QuestObjectiveType, CovenantRewardType};
use cogniverse_rpg::server::state::SimulationEngine;

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
    assert!(gas_used > 0 && gas_used <= 500, "Gas consumed must be within (0, 500]");
    assert_eq!(action, ActionChoice::ForageFood, "Hungry agent near food must forage food");
}

#[test]
fn test_ast_mutation_and_crossover() {
    let parent_a = BehavioralGenome::new_default();
    let parent_b = BehavioralGenome::new_default();

    let mut child = BehavioralGenome::crossover(&parent_a, &parent_b);
    assert_eq!(child.generation, 2, "Child of gen 1 parents must be gen 2");

    let prev_mutations = child.mutations_count;
    child.mutate(0.5);
    assert_eq!(child.mutations_count, prev_mutations + 1, "Mutation counter must increment");
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
    assert!(sincerity > 0.5, "Generous sacrifice relative to wealth must yield high sincerity");

    let idx = altar.receive_offering(
        1,
        42,
        "Kael".into(),
        ItemId::IronOre,
        1,
        10.0,
        0.8,
        100,
    );
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
    assert!(covenant.is_completed, "Covenant must complete when target is reached");
}

#[test]
fn test_simulation_engine_ticks() {
    let mut engine = SimulationEngine::new();
    let initial_tick = engine.world.gaia.world_tick;
    assert_eq!(initial_tick, 0);
    assert_eq!(engine.agents.len(), 16, "Must start with 8 Terrans and 8 Lunarians");

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
    assert!(alive_count >= 16, "Population must survive and not collapse");

    // Coordinates should not all be identical (no single-tile black hole)
    let first_pos = (engine.agents[0].x, engine.agents[0].y);
    let all_same = engine.agents.iter().all(|a| (a.x, a.y) == first_pos);
    assert!(!all_same, "Agents must disperse across the continent, not stack on a single tile");
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
    assert_eq!(tile.structure.as_ref().unwrap().kind, StructureKind::StoneCabin);

    // Enrich gold vein
    let initial_gold = engine.world.tile_at(60, 60).gold_ore_amount;
    engine.terraform_area(60, 60, TerraformBrush::EnrichGoldVein, 1);
    assert!(engine.world.tile_at(60, 60).gold_ore_amount > initial_gold);

    // Verify structures in telemetry
    let telemetry = engine.generate_telemetry();
    assert!(telemetry.structures.iter().any(|s| s.x == 50 && s.y == 50 && s.kind == StructureKind::StoneCabin));
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
    assert!(!telemetry.activities.is_empty(), "Activities telemetry must be populated");
}

