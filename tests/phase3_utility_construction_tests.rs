use cogniverse_rpg::agent::needs::NpcNeeds;
use cogniverse_rpg::agent::utility_ai::{UtilityAi, UtilityGoal};
use cogniverse_rpg::agent::{Agent, CivilizationRace};
use cogniverse_rpg::brain::desire::AmbitionArchetype;
use cogniverse_rpg::world::construction::{ConstructionStage, JobBoard};
use cogniverse_rpg::world::items::ItemId;
use cogniverse_rpg::world::tile::StructureKind;
use cogniverse_rpg::world::WorldGrid;

#[test]
fn test_utility_ai_hysteresis_and_preemption() {
    let mut ai = UtilityAi::new();
    let mut needs = NpcNeeds {
        hunger: 0.55,
        rest: 0.53,
        safety: 0.0,
        shelter: 0.1,
        social: 0.1,
    };
    let job_board = JobBoard::new();

    // 1. Initial goal selection: hunger (0.55^2 = 0.3025) > rest (0.53^2 = 0.2809)
    let goal = ai.evaluate_best_goal(&needs, true, true, &job_board);
    assert_eq!(
        goal,
        UtilityGoal::EatFood,
        "Initial highest utility should be EatFood"
    );

    // 2. Slight shift in fatigue: rest rises to 0.58 (0.58^2 = 0.3364 > 0.3025).
    // In naive utility systems this causes rapid thrashing/jitter.
    // With hysteresis stickiness (0.15), EatFood utility = 0.3025 + 0.15 = 0.4525 > 0.3364.
    needs.rest = 0.58;
    let goal2 = ai.evaluate_best_goal(&needs, true, true, &job_board);
    assert_eq!(
        goal2,
        UtilityGoal::EatFood,
        "Hysteresis stickiness (0.15) must prevent jitter and keep active EatFood goal"
    );

    // 3. Significant fatigue increase: rest rises to 0.85 (0.85^2 = 0.7225 > 0.4525).
    // The need genuinely outweighs the hysteresis stickiness.
    needs.rest = 0.85;
    let goal3 = ai.evaluate_best_goal(&needs, true, true, &job_board);
    assert_eq!(
        goal3,
        UtilityGoal::Rest,
        "Substantial need must break through hysteresis threshold"
    );

    // 4. Mortal threat preemption: danger appears (safety = 0.90, U = (0.9^2 * 1.6) = 1.296 -> 1.0).
    // FleeDanger must immediately preempt Rest.
    needs.safety = 0.90;
    let goal4 = ai.evaluate_best_goal(&needs, true, true, &job_board);
    assert_eq!(
        goal4,
        UtilityGoal::FleeDanger,
        "High danger must immediately preempt current task"
    );
}

#[test]
fn test_unbiased_isotropic_exploration_no_north_bias() {
    let mut ai = UtilityAi::new();
    let width = 128;
    let height = 128;
    let center_x = width / 2;
    let center_y = height / 2;

    let mut current_x = center_x;
    let mut current_y = center_y;

    let mut north_steps = 0usize;
    let mut south_steps = 0usize;
    let mut east_steps = 0usize;
    let mut west_steps = 0usize;

    for _ in 0..1000 {
        let (next_x, next_y) = ai.unbiased_wander_step(current_x, current_y, width, height);
        if next_y < current_y {
            north_steps += 1;
        } else if next_y > current_y {
            south_steps += 1;
        }
        if next_x > current_x {
            east_steps += 1;
        } else if next_x < current_x {
            west_steps += 1;
        }

        // Clamp away from boundary for wander test
        current_x = next_x.clamp(20, width - 21);
        current_y = next_y.clamp(20, height - 21);
    }

    // Over 1000 steps, neither north nor south should dominate exclusively (ratio must be reasonable, not > 4:1)
    let total_vertical = north_steps + south_steps;
    assert!(
        total_vertical > 200,
        "Agent should have vertical motion: total = {}",
        total_vertical
    );
    let north_ratio = north_steps as f32 / total_vertical as f32;
    assert!(
        (0.20..=0.80).contains(&north_ratio),
        "Exploration should be isotropic and balanced between North and South: north_ratio = {:.3}",
        north_ratio
    );

    let total_horizontal = east_steps + west_steps;
    assert!(
        total_horizontal > 200,
        "Agent should have horizontal motion: total = {}",
        total_horizontal
    );
    let east_ratio = east_steps as f32 / total_horizontal as f32;
    assert!(
        (0.20..=0.80).contains(&east_ratio),
        "Exploration should be isotropic and balanced between East and West: east_ratio = {:.3}",
        east_ratio
    );

    // Verify NPC does not collapse onto y = 0
    assert!(
        current_y > 5,
        "Agent must not migrate and permanently stay stuck at top edge y = 0 (current_y = {})",
        current_y
    );
}

#[test]
fn test_food_consumption_diversity_hierarchy() {
    let mut agent = Agent::new(
        1,
        "Alden",
        CivilizationRace::SolarTerran,
        32,
        32,
        AmbitionArchetype::CommunityBuilder,
        None,
    );

    // Clear starting inventory
    agent.inventory.remove(ItemId::Berry, 3);
    agent.inventory.remove(ItemId::Wood, 2);

    // 1. Test Bread consumption (+0.60 nutrition)
    agent.inventory.add(ItemId::Bread, 2);
    agent.inventory.add(ItemId::Berry, 3);
    agent.inventory.add(ItemId::Grain, 5);
    agent.needs.hunger = 0.80;

    let consumed = UtilityAi::consume_best_food(&mut agent.inventory, &mut agent.needs);
    assert!(consumed);
    assert_eq!(
        agent.inventory.count(ItemId::Bread),
        1,
        "Bread should be consumed first when available"
    );
    assert_eq!(agent.inventory.count(ItemId::Berry), 3);
    assert_eq!(agent.inventory.count(ItemId::Grain), 5);
    assert!(
        (agent.needs.hunger - 0.20).abs() < 1e-4,
        "Hunger should decrease by 0.60 from bread"
    );

    // 2. Consume remaining bread
    let consumed2 = UtilityAi::consume_best_food(&mut agent.inventory, &mut agent.needs);
    assert!(consumed2);
    assert_eq!(agent.inventory.count(ItemId::Bread), 0);

    // 3. Next best food: Berry (+0.40 nutrition)
    agent.needs.hunger = 0.70;
    let consumed3 = UtilityAi::consume_best_food(&mut agent.inventory, &mut agent.needs);
    assert!(consumed3);
    assert_eq!(
        agent.inventory.count(ItemId::Berry),
        2,
        "Berry should be consumed when no bread remains"
    );
    assert!(
        (agent.needs.hunger - 0.30).abs() < 1e-4,
        "Hunger should decrease by 0.40 from berry"
    );

    // 4. Clear berries, test Grain (+0.30 nutrition)
    agent.inventory.remove(ItemId::Berry, 2);
    agent.needs.hunger = 0.50;
    let consumed4 = UtilityAi::consume_best_food(&mut agent.inventory, &mut agent.needs);
    assert!(consumed4);
    assert_eq!(
        agent.inventory.count(ItemId::Grain),
        4,
        "Grain should be consumed when no berries or bread remain"
    );
    assert!(
        (agent.needs.hunger - 0.20).abs() < 1e-4,
        "Hunger should decrease by 0.30 from grain"
    );
}

#[test]
fn test_construction_pipeline_job_board_lifecycle() {
    let mut world = WorldGrid::new();
    let agent_id = 42;
    let mut agent = Agent::new(
        agent_id,
        "Mason",
        CivilizationRace::SolarTerran,
        40,
        40,
        AmbitionArchetype::CommunityBuilder,
        None,
    );

    // Clear starting wood from inventory
    agent.inventory.remove(ItemId::Wood, 2);

    let site_x = 45;
    let site_y = 45;

    // Verify initial tile has no structure
    assert!(world.tile_at(site_x, site_y).structure.is_none());

    // 1. Post Blueprint for WoodCottage (req: 10 wood, 0 stone, 30 labor ticks)
    let bp_id = world
        .job_board
        .post_blueprint(StructureKind::WoodCottage, site_x, site_y);
    assert!(world.job_board.has_open_jobs());

    let bp = world.job_board.get_blueprint(bp_id).unwrap();
    assert_eq!(bp.stage, ConstructionStage::SiteClearing);
    assert_eq!(bp.required_wood, 10);
    assert_eq!(bp.required_stone, 0);

    // 2. Step 1: Site Clearing without materials in inventory (progress 0 -> 10)
    for _ in 0..2 {
        let worked =
            world.perform_construction_step(agent_id, &mut agent.inventory, &mut agent.needs);
        assert!(worked);
    }

    let bp = world.job_board.get_blueprint(bp_id).unwrap();
    assert_eq!(
        bp.stage,
        ConstructionStage::MaterialHauling,
        "After 10 labor ticks without materials, stage should advance to MaterialHauling"
    );

    // Provide agent with required materials for hauling
    agent.inventory.add(ItemId::Wood, 15);

    // 3. Step 2: Material Hauling & Delivery
    let worked_haul =
        world.perform_construction_step(agent_id, &mut agent.inventory, &mut agent.needs);
    assert!(worked_haul);

    let bp = world.job_board.get_blueprint(bp_id).unwrap();
    assert_eq!(
        bp.delivered_wood, 10,
        "Agent should have delivered all 10 required wood"
    );
    assert_eq!(
        agent.inventory.count(ItemId::Wood),
        5,
        "Agent should retain remaining wood (15 - 10 = 5)"
    );
    assert_eq!(
        bp.stage,
        ConstructionStage::Framing,
        "Once materials delivered, stage advances to Framing"
    );

    // 4. Step 3: Framing labor ticks until completion
    let mut finished = false;
    for _ in 0..10 {
        world.perform_construction_step(agent_id, &mut agent.inventory, &mut agent.needs);
        if let Some(bp_check) = world.job_board.get_blueprint(bp_id) {
            if bp_check.is_complete {
                finished = true;
                break;
            }
        }
    }
    assert!(finished, "Building should complete within labor budget");

    // 5. Structure placement verification on WorldGrid
    let placed_structure = world
        .tile_at(site_x, site_y)
        .structure
        .as_ref()
        .expect("Structure must be placed on world tile upon completion");
    assert_eq!(placed_structure.kind, StructureKind::WoodCottage);

    // 6. Test Dock & BoatYard Blueprint Creation
    let dock_id = world
        .job_board
        .post_blueprint(StructureKind::Dock, site_x + 1, site_y);
    let dock_bp = world.job_board.get_blueprint(dock_id).unwrap();
    assert_eq!(dock_bp.kind, StructureKind::Dock);
    assert_eq!(dock_bp.required_wood, 12);
    assert_eq!(dock_bp.required_stone, 4);

    let boatyard_id = world
        .job_board
        .post_blueprint(StructureKind::BoatYard, site_x + 2, site_y);
    let boatyard_bp = world.job_board.get_blueprint(boatyard_id).unwrap();
    assert_eq!(boatyard_bp.kind, StructureKind::BoatYard);
    assert_eq!(boatyard_bp.required_wood, 20);
    assert_eq!(boatyard_bp.required_stone, 8);
}
