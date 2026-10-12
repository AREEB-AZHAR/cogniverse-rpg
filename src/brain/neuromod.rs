use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuromodulationState {
    pub dopamine: f32,       // 0.0 to 1.0 (Reward Prediction Error & drive)
    pub serotonin: f32,      // 0.0 to 1.0 (Patience, long-term horizon & inhibition)
    pub noradrenaline: f32,  // 0.0 to 1.0 (Surprise, vigilance & plasticity)
    
    pub rolling_prediction_error: f32,
    pub previous_value_estimate: f32,
}

impl NeuromodulationState {
    pub fn new() -> Self {
        Self {
            dopamine: 0.5,
            serotonin: 0.6,
            noradrenaline: 0.2,
            rolling_prediction_error: 0.0,
            previous_value_estimate: 0.5,
        }
    }

    /// Updates neuromodulators based on Kenji Doya's computational neuroscience model
    pub fn update(
        &mut self,
        current_reward: f32,
        current_satiation: f32, // Based on hunger/thirst/warmth
        has_shelter: bool,
        is_threatened: bool,
        environmental_surprise: f32,
    ) {
        // 1. Dopamine: Temporal Difference Error (delta)
        // delta = reward - expected_reward
        let td_error = current_reward - self.previous_value_estimate;
        self.dopamine = (self.dopamine * 0.7 + (0.5 + td_error * 0.5) * 0.3).clamp(0.05, 0.95);
        self.previous_value_estimate = self.previous_value_estimate * 0.8 + current_reward * 0.2;

        // 2. Serotonin: Satiation + Shelter security - Threat
        let shelter_bonus = if has_shelter { 0.2 } else { 0.0 };
        let threat_penalty = if is_threatened { 0.4 } else { 0.0 };
        let raw_serotonin = current_satiation * 0.6 + shelter_bonus - threat_penalty;
        self.serotonin = (self.serotonin * 0.8 + raw_serotonin.clamp(0.1, 0.95) * 0.2).clamp(0.05, 0.95);

        // 3. Noradrenaline: Unexpected uncertainty & variance of TD error
        let error_magnitude = td_error.abs();
        self.rolling_prediction_error = self.rolling_prediction_error * 0.85 + error_magnitude * 0.15;
        let surprise = (self.rolling_prediction_error + environmental_surprise + if is_threatened { 0.5 } else { 0.0 }).clamp(0.0, 1.0);
        self.noradrenaline = (self.noradrenaline * 0.75 + surprise * 0.25).clamp(0.05, 0.95);
    }
}
