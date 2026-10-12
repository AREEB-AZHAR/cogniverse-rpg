use serde::{Deserialize, Serialize};
use super::ast::ActionChoice;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MentalDreamOutcome {
    pub projected_energy_delta: f32,
    pub projected_hunger_delta: f32,
    pub projected_risk_score: f32, // 0.0 to 1.0
    pub recommendation: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WorldModel {
    pub simulated_dreams_count: u64,
}

impl WorldModel {
    pub fn new() -> Self {
        Self {
            simulated_dreams_count: 0,
        }
    }

    /// Mental forward rollout: evaluates hypothetical action outcome
    pub fn simulate_mental_rollout(
        &mut self,
        action: ActionChoice,
        current_energy: f32,
        current_health: f32,
        is_near_predator: bool,
        is_night: bool,
    ) -> MentalDreamOutcome {
        self.simulated_dreams_count += 1;

        match action {
            ActionChoice::IdleRest => MentalDreamOutcome {
                projected_energy_delta: 0.15,
                projected_hunger_delta: 0.01,
                projected_risk_score: if is_near_predator { 0.8 } else { 0.05 },
                recommendation: !is_near_predator,
            },
            ActionChoice::ForageFood => MentalDreamOutcome {
                projected_energy_delta: -0.05,
                projected_hunger_delta: -0.3,
                projected_risk_score: if is_near_predator { 0.6 } else { 0.1 },
                recommendation: true,
            },
            ActionChoice::DrinkWater => MentalDreamOutcome {
                projected_energy_delta: -0.02,
                projected_hunger_delta: 0.0,
                projected_risk_score: 0.05,
                recommendation: true,
            },
            ActionChoice::FleeThreat => MentalDreamOutcome {
                projected_energy_delta: -0.12,
                projected_hunger_delta: 0.05,
                projected_risk_score: 0.2,
                recommendation: is_near_predator,
            },
            ActionChoice::MineIron | ActionChoice::MineStone => MentalDreamOutcome {
                projected_energy_delta: -0.15,
                projected_hunger_delta: 0.08,
                projected_risk_score: if is_night { 0.5 } else { 0.2 },
                recommendation: current_energy > 0.3 && current_health > 30.0,
            },
            ActionChoice::CastMagicRune => MentalDreamOutcome {
                projected_energy_delta: -0.1,
                projected_hunger_delta: 0.02,
                projected_risk_score: 0.1,
                recommendation: true,
            },
            _ => MentalDreamOutcome {
                projected_energy_delta: -0.05,
                projected_hunger_delta: 0.02,
                projected_risk_score: 0.15,
                recommendation: true,
            },
        }
    }
}
