use serde::{Deserialize, Serialize};

/// Physiological and social needs for autonomous NPCs based on Dave Mark's IAUS
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct NpcNeeds {
    pub hunger: f32,  // 0.0 = sated, 1.0 = starving
    pub rest: f32,    // 0.0 = fully energized, 1.0 = exhausted
    pub safety: f32,  // 0.0 = completely secure, 1.0 = mortal danger
    pub shelter: f32, // 0.0 = secure warm shelter, 1.0 = exposed to elements
    pub social: f32,  // 0.0 = content, 1.0 = socially isolated
}

impl Default for NpcNeeds {
    fn default() -> Self {
        Self {
            hunger: 0.20,
            rest: 0.15,
            safety: 0.0,
            shelter: 0.10,
            social: 0.25,
        }
    }
}

impl NpcNeeds {
    pub fn new() -> Self {
        Self::default()
    }

    /// Advances needs over a simulation tick with environmental modifiers
    pub fn tick_decay(&mut self, ambient_temperature: f32, has_shelter: bool, is_working: bool) {
        // Heavy labor accelerates hunger and fatigue
        let work_multiplier = if is_working { 1.8 } else { 1.0 };

        self.hunger = (self.hunger + 0.0012 * work_multiplier).min(1.0);
        self.rest = (self.rest + 0.0010 * work_multiplier).min(1.0);
        self.social = (self.social + 0.0006).min(1.0);

        // Environmental shelter decay: cold/rain increases shelter need if unhoused
        if has_shelter {
            self.shelter = (self.shelter - 0.01).max(0.0);
        } else if ambient_temperature < 10.0 {
            self.shelter = (self.shelter + 0.003).min(1.0);
        } else {
            self.shelter = (self.shelter + 0.0008).min(1.0);
        }

        // Safety decays back to safe when threats depart
        self.safety = (self.safety - 0.02).max(0.0);
    }

    /// Feeds the NPC by consuming food, reducing hunger proportionally to food nutrition
    pub fn feed(&mut self, nutrition_value: f32) {
        self.hunger = (self.hunger - nutrition_value).max(0.0);
    }

    /// Rests the NPC, recovering energy and reducing rest need
    pub fn sleep(&mut self, recovery_amount: f32) {
        self.rest = (self.rest - recovery_amount).max(0.0);
    }

    /// Sets danger level based on distance to nearest threat
    pub fn update_threat_distance(&mut self, nearest_threat_distance: f32) {
        self.safety = if nearest_threat_distance <= 2.0 {
            1.0 // Imminent attack
        } else if nearest_threat_distance <= 6.0 {
            ((6.0 - nearest_threat_distance) / 4.0).clamp(0.0, 1.0)
        } else {
            0.0
        };
    }
}
