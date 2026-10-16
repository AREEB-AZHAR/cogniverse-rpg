use super::brain::ast::EntityFilter;
use super::brain::desire::AmbitionArchetype;
use super::brain::memory::AncestralEcho;
use super::brain::CognitiveBrain;
use super::world::altar::DeityAlignment;
use super::world::items::{Inventory, ItemId};
use super::world::tile::Tile;
use super::world::WorldGrid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CivilizationRace {
    SolarTerran,
    ChthonicLunarian,
}

impl CivilizationRace {
    pub fn display_name(&self) -> &'static str {
        match self {
            CivilizationRace::SolarTerran => "Solar Terran",
            CivilizationRace::ChthonicLunarian => "Chthonic Lunarian",
        }
    }

    pub fn badge_color(&self) -> &'static str {
        match self {
            CivilizationRace::SolarTerran => "#eab308", // Radiant gold
            CivilizationRace::ChthonicLunarian => "#a855f7", // Mystical violet
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: usize,
    pub name: String,
    pub race: CivilizationRace,
    pub x: usize,
    pub y: usize,

    // Physiological Vitals
    pub health: f32,
    pub max_health: f32,
    pub hunger: f32,      // 0.0 (satiated) to 1.0 (starving)
    pub thirst: f32,      // 0.0 (hydrated) to 1.0 (dehydrated)
    pub energy: f32,      // 0.0 (exhausted) to 1.0 (energized)
    pub warmth: f32,      // 0.0 (freezing) to 1.0 (comfortably warm)
    pub social_need: f32, // 0.0 (content) to 1.0 (lonely)

    // Inventory & Tools
    pub inventory: Inventory,

    // The Cognitive Brain (AST, Memory, Neuromodulators, Desires)
    pub brain: CognitiveBrain,

    // Lineage & Social Ties
    pub partner_id: Option<usize>,
    pub children_count: u32,
    pub ancestral_echo: Option<AncestralEcho>,
    pub traits: Vec<String>,

    pub is_alive: bool,
    pub age_ticks: u64,
}

impl Agent {
    pub fn new(
        id: usize,
        name: impl Into<String>,
        race: CivilizationRace,
        x: usize,
        y: usize,
        archetype: AmbitionArchetype,
        ancestral_echo: Option<AncestralEcho>,
    ) -> Self {
        let name_str = name.into();
        let piety = match race {
            CivilizationRace::SolarTerran => 0.65,
            CivilizationRace::ChthonicLunarian => 0.55,
        };
        let mut brain = CognitiveBrain::new(archetype, 0.7, piety);

        // Apply ancestral echo bonuses if awakened
        if let Some(ref echo) = ancestral_echo {
            brain.piety += echo.innate_piety_bonus;
            brain.memory.log_episodic_event(
                0,
                format!(
                    "Awakened with Ancestral Echo of {}: {}",
                    echo.hero_name, echo.gift_description
                ),
                vec!["Akashic".into(), "Destiny".into()],
                10.0,
            );
        }

        let mut inventory = Inventory::new();
        // Starting items
        inventory.add(ItemId::Berry, 3);
        inventory.add(ItemId::Wood, 2);

        let mut traits = Vec::new();
        match archetype {
            AmbitionArchetype::SlothHedonist => {
                traits.push("Peaceful".into());
                traits.push("Content".into());
            }
            AmbitionArchetype::PowerTyrant => {
                traits.push("Ambitious".into());
                traits.push("Strong".into());
            }
            AmbitionArchetype::WealthyMerchant => {
                traits.push("Greedy".into());
                traits.push("Wise".into());
            }
            AmbitionArchetype::RomanticPursuer => {
                traits.push("Charming".into());
                traits.push("Attractive".into());
            }
            AmbitionArchetype::AsceticHermit => {
                traits.push("Wise".into());
                traits.push("Immune".into());
            }
            AmbitionArchetype::ZealotPriest => {
                traits.push("Blessed".into());
                traits.push("Zealot".into());
            }
            AmbitionArchetype::HereticRebel => {
                traits.push("Deceitful".into());
                traits.push("Swift".into());
            }
            AmbitionArchetype::CommunityBuilder => {
                traits.push("Leader".into());
                traits.push("Genius".into());
            }
        }

        Self {
            id,
            name: name_str,
            race,
            x,
            y,
            health: 100.0,
            max_health: 100.0,
            hunger: 0.25,
            thirst: 0.25,
            energy: 0.90,
            warmth: 0.80,
            social_need: 0.30,
            inventory,
            brain,
            partner_id: None,
            children_count: 0,
            ancestral_echo,
            traits,
            is_alive: true,
            age_ticks: 0,
        }
    }

    /// Computes distances to nearest interesting entities on the 64x64 grid
    pub fn compute_sensory_distances(
        &self,
        world: &WorldGrid,
    ) -> std::collections::HashMap<EntityFilter, f32> {
        let mut map = std::collections::HashMap::new();
        let ax = self.x as f32;
        let ay = self.y as f32;

        let mut min_altar = 999.0;

        // Search within a radius of 28 tiles for grand 128x128 continent
        let min_x = self.x.saturating_sub(28);
        let max_x = (self.x + 28).min(world.width - 1);
        let min_y = self.y.saturating_sub(28);
        let max_y = (self.y + 28).min(world.height - 1);

        let mut min_food_sq = 999999.0;
        let mut min_water_sq = 999999.0;
        let mut min_tree_sq = 999999.0;
        let mut min_stone_sq = 999999.0;
        let mut min_iron_sq = 999999.0;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let tile = world.tile_at(x, y);
                let dx = x as f32 - ax;
                let dy = y as f32 - ay;
                let d_sq = dx * dx + dy * dy;

                if (tile.berry_amount > 0 || tile.grain_amount > 0) && d_sq < min_food_sq {
                    min_food_sq = d_sq;
                }
                if tile.has_fresh_water && d_sq < min_water_sq {
                    min_water_sq = d_sq;
                }
                if tile.wood_amount > 0 && d_sq < min_tree_sq {
                    min_tree_sq = d_sq;
                }
                if (tile.stone_amount > 0 || tile.flint_amount > 0) && d_sq < min_stone_sq {
                    min_stone_sq = d_sq;
                }
                if tile.iron_ore_amount > 0 && d_sq < min_iron_sq {
                    min_iron_sq = d_sq;
                }
            }
        }

        let min_food = min_food_sq.sqrt();
        let min_water = min_water_sq.sqrt();
        let min_tree = min_tree_sq.sqrt();
        let min_stone = min_stone_sq.sqrt();
        let min_iron = min_iron_sq.sqrt();

        // Check nearest altar (preferring faction altar)
        for altar in &world.altars {
            let matches_faction = matches!(
                (self.race, altar.deity),
                (
                    CivilizationRace::SolarTerran,
                    DeityAlignment::SolarPlayerGod
                ) | (
                    CivilizationRace::ChthonicLunarian,
                    DeityAlignment::LunarShadowWeaver
                )
            );
            let penalty = if matches_faction { 0.0 } else { 200.0 };
            let d = (altar.x as f32 - ax).hypot(altar.y as f32 - ay) + penalty;
            if d < min_altar {
                min_altar = d;
            }
        }

        // Check nearest predator
        let mut min_predator = 999.0;
        for w in &world.wildlife {
            if w.is_alive && w.species.is_predator() {
                let d = (w.x as f32 - ax).hypot(w.y as f32 - ay);
                if d < min_predator {
                    min_predator = d;
                }
            }
        }

        map.insert(EntityFilter::FoodSource, min_food);
        map.insert(EntityFilter::WaterSource, min_water);
        map.insert(EntityFilter::Tree, min_tree);
        map.insert(EntityFilter::StoneRock, min_stone);
        map.insert(EntityFilter::IronVein, min_iron);
        map.insert(EntityFilter::Altar, min_altar);
        map.insert(EntityFilter::HostilePredator, min_predator);

        map
    }

    /// Advances physiological state by 1 world tick
    pub fn tick_vitals(&mut self, current_tile: &Tile, current_weather_temp_delta: f32) {
        if !self.is_alive {
            return;
        }

        self.age_ticks += 1;
        self.hunger = (self.hunger + 0.0008).min(1.0);
        self.thirst = (self.thirst + 0.0010).min(1.0);
        self.social_need = (self.social_need + 0.0006).min(1.0);

        // Auto-consume food from inventory if hungry
        if self.hunger > 0.45 {
            if self.inventory.count(ItemId::Berry) > 0 {
                self.inventory.remove(ItemId::Berry, 1);
                self.hunger = (self.hunger - 0.40).max(0.0);
                self.brain
                    .neuromod
                    .update(0.3, 1.0 - self.hunger, false, false, 0.0);
            } else if self.inventory.count(ItemId::Grain) > 0 {
                self.inventory.remove(ItemId::Grain, 1);
                self.hunger = (self.hunger - 0.30).max(0.0);
                self.brain
                    .neuromod
                    .update(0.2, 1.0 - self.hunger, false, false, 0.0);
            }
        }

        // Environmental thermal balance
        let ambient_temp = current_tile.temperature + current_weather_temp_delta;
        if ambient_temp < 8.0 {
            self.warmth = (self.warmth - 0.0015).max(0.0);
        } else if ambient_temp > 28.0 {
            self.warmth = (self.warmth + 0.002).min(1.0);
        } else {
            self.warmth = (self.warmth + 0.001).min(1.0);
        }

        // Damage only from critical deprivation (gradual, not instant death)
        let mut damage = 0.0;
        if self.hunger > 0.96 {
            damage += 0.25;
        }
        if self.thirst > 0.96 {
            damage += 0.35;
        }
        if self.warmth < 0.05 {
            damage += 0.25; // Hypothermia
        }

        if damage > 0.0 {
            self.health -= damage;
            if self.health <= 0.0 {
                self.health = 0.0;
                self.is_alive = false;
            }
        } else if self.hunger < 0.6 && self.thirst < 0.6 {
            // Natural regeneration when satisfied
            self.health = (self.health + 0.5).min(self.max_health);
        }
    }

    /// Moves one step toward target coordinates
    pub fn step_towards(&mut self, target_x: usize, target_y: usize, max_w: usize, max_h: usize) {
        let dx = (target_x as isize - self.x as isize).signum();
        let dy = (target_y as isize - self.y as isize).signum();

        let nx = (self.x as isize + dx).clamp(0, max_w as isize - 1) as usize;
        let ny = (self.y as isize + dy).clamp(0, max_h as isize - 1) as usize;

        self.x = nx;
        self.y = ny;
        self.energy = (self.energy - 0.003).max(0.0);
    }
}
