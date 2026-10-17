use super::needs::NpcNeeds;
use crate::world::construction::JobBoard;
use crate::world::items::{Inventory, ItemId};
use crate::world::tile::Tile;
use serde::{Deserialize, Serialize};

/// The hysteresis stickiness constant preventing decision jitter between competing goals
pub const UTILITY_STICKINESS: f32 = 0.15;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UtilityGoal {
    EatFood,
    Rest,
    FleeDanger,
    Construct,
    Socialize,
    Explore,
}

impl UtilityGoal {
    pub fn display_name(&self) -> &'static str {
        match self {
            UtilityGoal::EatFood => "Seeking Sustenance",
            UtilityGoal::Rest => "Resting / Sleeping",
            UtilityGoal::FleeDanger => "Fleeing Mortal Threat",
            UtilityGoal::Construct => "Building Community Infrastructure",
            UtilityGoal::Socialize => "Social Gathering",
            UtilityGoal::Explore => "Radial Terrain Exploration",
        }
    }
}

/// Dave Mark's Infinite Axis Utility System (IAUS) evaluator with hysteresis
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UtilityAi {
    pub current_goal: UtilityGoal,
    pub goal_duration_ticks: u32,
    pub last_action_score: f32,
    pub exploration_angle: f32,
}

impl Default for UtilityAi {
    fn default() -> Self {
        Self::new()
    }
}

impl UtilityAi {
    pub fn new() -> Self {
        Self {
            current_goal: UtilityGoal::Explore,
            goal_duration_ticks: 0,
            last_action_score: 0.15,
            exploration_angle: 0.0,
        }
    }

    /// Evaluates all considerations through quadratic response curves U(x) = x^2 with hysteresis
    pub fn evaluate_best_goal(
        &mut self,
        needs: &NpcNeeds,
        has_food_available: bool,
        has_shelter: bool,
        job_board: &JobBoard,
    ) -> UtilityGoal {
        self.goal_duration_ticks += 1;

        // 1. Calculate raw utility scores using quadratic response curve U(x) = x^2
        let mut score_eat = if has_food_available {
            needs.hunger.powi(2)
        } else {
            // Urgency remains high to seek food tiles across the map
            needs.hunger.powi(2) * 0.8
        };

        let mut score_rest = needs.rest.powi(2);

        // Safety has highest emergency multiplier
        let mut score_flee = (needs.safety.powi(2) * 1.6).min(1.0);

        // Construction utility is driven by shelter need and active job board blueprints
        let mut score_construct = if job_board.has_open_jobs() {
            (needs.shelter.powi(2) * 0.9 + 0.35).min(0.95)
        } else if !has_shelter {
            needs.shelter.powi(2) * 0.6
        } else {
            0.05
        };

        let mut score_social = needs.social.powi(2) * 0.75;

        // Baseline exploration utility when all needs are well-satisfied
        let score_explore = 0.15;

        // 2. Apply Dave Mark's hysteresis bonus to the currently active goal
        match self.current_goal {
            UtilityGoal::EatFood => score_eat += UTILITY_STICKINESS,
            UtilityGoal::Rest => score_rest += UTILITY_STICKINESS,
            UtilityGoal::FleeDanger => score_flee += UTILITY_STICKINESS,
            UtilityGoal::Construct => score_construct += UTILITY_STICKINESS,
            UtilityGoal::Socialize => score_social += UTILITY_STICKINESS,
            UtilityGoal::Explore => {}
        }

        // 3. Select action with the highest evaluated utility
        let candidates = [
            (UtilityGoal::FleeDanger, score_flee),
            (UtilityGoal::EatFood, score_eat),
            (UtilityGoal::Rest, score_rest),
            (UtilityGoal::Construct, score_construct),
            (UtilityGoal::Socialize, score_social),
            (UtilityGoal::Explore, score_explore),
        ];

        let mut best_goal = UtilityGoal::Explore;
        let mut best_score = -1.0;

        for (goal, score) in candidates {
            if score > best_score {
                best_score = score;
                best_goal = goal;
            }
        }

        if best_goal != self.current_goal {
            self.current_goal = best_goal;
            self.goal_duration_ticks = 0;
        }

        self.last_action_score = best_score;
        best_goal
    }

    /// Executes consumption of the best available food in inventory (bread, meat, berries, grain)
    pub fn consume_best_food(inventory: &mut Inventory, needs: &mut NpcNeeds) -> bool {
        // Preference order: Bread (+0.60), Berries (+0.40), Grain (+0.30)
        if inventory.count(ItemId::Bread) > 0 {
            inventory.remove(ItemId::Bread, 1);
            needs.feed(0.60);
            true
        } else if inventory.count(ItemId::Berry) > 0 {
            inventory.remove(ItemId::Berry, 1);
            needs.feed(0.40);
            true
        } else if inventory.count(ItemId::Grain) > 0 {
            inventory.remove(ItemId::Grain, 1);
            needs.feed(0.30);
            true
        } else {
            false
        }
    }

    /// Computes an isotropic unbiased wander step, eliminating top-of-map (0, 0) coordinate bias
    pub fn unbiased_wander_step(
        &mut self,
        current_x: usize,
        current_y: usize,
        world_width: usize,
        world_height: usize,
    ) -> (usize, usize) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Slight momentum angle perturbation to produce organic roaming arcs
        self.exploration_angle += rng.gen_range(-0.5..=0.5);

        let dx_f = self.exploration_angle.cos();
        let dy_f = self.exploration_angle.sin();

        let dx = if dx_f > 0.3 {
            1
        } else if dx_f < -0.3 {
            -1
        } else {
            0
        };

        let dy = if dy_f > 0.3 {
            1
        } else if dy_f < -0.3 {
            -1
        } else {
            0
        };

        let nx = (current_x as isize + dx).clamp(0, world_width as isize - 1) as usize;
        let ny = (current_y as isize + dy).clamp(0, world_height as isize - 1) as usize;

        (nx, ny)
    }

    /// Executes one tick of autonomous construction work on a claimed blueprint
    pub fn perform_construction_step(
        agent_id: usize,
        inventory: &mut Inventory,
        needs: &mut NpcNeeds,
        job_board: &mut JobBoard,
        tiles: &mut [Tile],
        world_width: usize,
    ) -> bool {
        if let Some(bp) = job_board.claim_job(agent_id) {
            let bp_id = bp.id;
            bp.claimed_by = Some(agent_id);

            // 1. Deliver materials if blueprint requires them
            if bp.needs_materials() {
                let avail_wood = inventory.count(ItemId::Wood);
                let avail_stone = inventory.count(ItemId::Stone);

                if avail_wood > 0 || avail_stone > 0 {
                    let wood_to_deliver = avail_wood.min(bp.required_wood - bp.delivered_wood);
                    let stone_to_deliver = avail_stone.min(bp.required_stone - bp.delivered_stone);

                    if wood_to_deliver > 0 {
                        inventory.remove(ItemId::Wood, wood_to_deliver);
                    }
                    if stone_to_deliver > 0 {
                        inventory.remove(ItemId::Stone, stone_to_deliver);
                    }

                    bp.deliver_materials(wood_to_deliver, stone_to_deliver);
                }
            }

            // 2. Apply construction labor (5 labor units per tick)
            let is_finished = bp.apply_labor(5, tiles, world_width);

            // Labor increases fatigue and hunger slightly
            needs.tick_decay(20.0, false, true);

            if is_finished {
                job_board.release_job(bp_id, agent_id);
            }
            return true;
        }

        false
    }
}
