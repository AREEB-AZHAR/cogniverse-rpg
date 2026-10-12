use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RivalGodState {
    pub name: String,
    pub title: String,
    pub shadow_favor_meter: f32, // 0.0 to 100.0
    pub lunarian_population: usize,
    pub active_directive: String,
    pub wrath_level: f32,
    pub ticks_since_last_intervention: u64,
}

impl RivalGodState {
    pub fn new() -> Self {
        Self {
            name: "The Shadow Weaver".into(),
            title: "Architect of the Lunar Abyss".into(),
            shadow_favor_meter: 65.0,
            lunarian_population: 12,
            active_directive: "Harvest flint and iron from Ironcrag crags to forge shadow blades.".into(),
            wrath_level: 0.1,
            ticks_since_last_intervention: 0,
        }
    }

    /// Autonomous meta-learning heuristic for the Rival AI God
    pub fn evaluate_and_intervene(
        &mut self,
        _current_tick: u64,
        terran_count: usize,
        lunarian_count: usize,
    ) -> Option<String> {
        self.lunarian_population = lunarian_count;
        self.ticks_since_last_intervention += 1;

        if self.ticks_since_last_intervention < 200 {
            return None;
        }

        self.ticks_since_last_intervention = 0;

        // If Terrans outnumber Lunarians significantly, grant dark blessings
        if terran_count > lunarian_count + 3 {
            self.shadow_favor_meter = (self.shadow_favor_meter + 15.0).min(100.0);
            self.active_directive = "Terran light encroaches! Rally at the Obsidian Altar for a boon of vitality.".into();
            Some(format!(
                "{} whispered across the shadows: 'Children of Dusk, grow strong against the dawn!' (Lunarians blessed with endurance)",
                self.name
            ))
        } else if lunarian_count >= 15 {
            self.active_directive = "Our strength grows. Inscribe Shadow Veils and expand borders toward the Riverlands.".into();
            Some(format!(
                "{} proclaimed: 'The night expands! Inscribe ancient runes and claim the leylines!'",
                self.name
            ))
        } else {
            self.active_directive = "Prowl the deep forests and gather mana crystals from the ruins.".into();
            None
        }
    }
}
