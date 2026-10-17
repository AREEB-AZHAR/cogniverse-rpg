use super::commands::CellId;
use super::WorldGrid;
use crate::data::{DietType, GameDataRegistry};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimalSpecies {
    Rabbit,
    ForestDeer,
    DireWolf,
    MountainBear,
}

impl AnimalSpecies {
    pub fn is_predator(&self) -> bool {
        matches!(self, AnimalSpecies::DireWolf | AnimalSpecies::MountainBear)
    }

    pub fn max_health(&self) -> f32 {
        match self {
            AnimalSpecies::Rabbit => 20.0,
            AnimalSpecies::ForestDeer => 60.0,
            AnimalSpecies::DireWolf => 80.0,
            AnimalSpecies::MountainBear => 160.0,
        }
    }

    pub fn attack_power(&self) -> f32 {
        match self {
            AnimalSpecies::Rabbit => 0.0,
            AnimalSpecies::ForestDeer => 2.0,
            AnimalSpecies::DireWolf => 15.0,
            AnimalSpecies::MountainBear => 30.0,
        }
    }
}

/// Dynamic behavioral state of an animal in the living ecosystem
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimalBehaviorState {
    Idle,
    Grazing,
    Hunting { target_id: usize },
    Fleeing { threat_x: usize, threat_y: usize },
    Mating { partner_id: usize },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WildlifeEntity {
    pub id: usize,
    pub species: AnimalSpecies,
    pub custom_species: Option<String>,
    pub x: usize,
    pub y: usize,
    pub health: f32,
    pub max_health: f32,
    pub hunger: f32, // 0.0 (full) to 1.0 (starving)
    pub age_ticks: u64,
    pub is_alive: bool,
    pub is_domesticated: bool,
    pub pen_id: Option<usize>,
    pub behavior_state: AnimalBehaviorState,
    pub product_cooldown: u32,
    pub breeding_cooldown: u32,
}

impl WildlifeEntity {
    pub fn new(id: usize, species: AnimalSpecies, x: usize, y: usize) -> Self {
        let max_hp = species.max_health();
        Self {
            id,
            species,
            custom_species: None,
            x,
            y,
            health: max_hp,
            max_health: max_hp,
            hunger: 0.2,
            age_ticks: 0,
            is_alive: true,
            is_domesticated: false,
            pen_id: None,
            behavior_state: AnimalBehaviorState::Idle,
            product_cooldown: 50,
            breeding_cooldown: 0,
        }
    }

    pub fn new_custom(id: usize, species_id: String, max_hp: f32, x: usize, y: usize) -> Self {
        Self {
            id,
            species: AnimalSpecies::Rabbit, // Fallback base enum
            custom_species: Some(species_id),
            x,
            y,
            health: max_hp,
            max_health: max_hp,
            hunger: 0.2,
            age_ticks: 0,
            is_alive: true,
            is_domesticated: false,
            pen_id: None,
            behavior_state: AnimalBehaviorState::Idle,
            product_cooldown: 50,
            breeding_cooldown: 0,
        }
    }

    pub fn species_name<'a>(&'a self, registry: &'a GameDataRegistry) -> &'a str {
        if let Some(ref custom_id) = self.custom_species {
            if let Some(def) = registry.get_animal(custom_id) {
                return &def.name;
            }
            return custom_id.as_str();
        }
        match self.species {
            AnimalSpecies::Rabbit => "Wild Rabbit",
            AnimalSpecies::ForestDeer => "Forest Deer",
            AnimalSpecies::DireWolf => "Dire Wolf",
            AnimalSpecies::MountainBear => "Mountain Bear",
        }
    }

    pub fn is_domesticable(&self, registry: &GameDataRegistry) -> bool {
        if let Some(ref custom_id) = self.custom_species {
            if let Some(def) = registry.get_animal(custom_id) {
                return def.domesticable;
            }
        }
        matches!(self.species, AnimalSpecies::Rabbit)
    }

    pub fn is_carnivore(&self, registry: &GameDataRegistry) -> bool {
        if let Some(ref sp) = self.custom_species {
            registry
                .get_animal(sp)
                .map(|d| d.diet == DietType::Carnivore)
                .unwrap_or(false)
        } else {
            self.species.is_predator()
        }
    }

    pub fn is_omnivore(&self, registry: &GameDataRegistry) -> bool {
        if let Some(ref sp) = self.custom_species {
            registry
                .get_animal(sp)
                .map(|d| d.diet == DietType::Omnivore)
                .unwrap_or(false)
        } else {
            false
        }
    }

    pub fn is_herbivore(&self, registry: &GameDataRegistry) -> bool {
        !self.is_carnivore(registry) && !self.is_omnivore(registry)
    }

    pub fn tame(&mut self, registry: &GameDataRegistry) -> bool {
        if self.is_domesticable(registry) {
            self.is_domesticated = true;
            true
        } else {
            false
        }
    }

    pub fn assign_pen(&mut self, pen_id: usize) {
        if self.is_domesticated {
            self.pen_id = Some(pen_id);
        }
    }

    pub fn can_breed(&self) -> bool {
        self.is_alive
            && self.health > self.max_health * 0.6
            && self.hunger < 0.45
            && self.breeding_cooldown == 0
    }

    pub fn breed(&self, partner: &WildlifeEntity, next_id: usize) -> Option<WildlifeEntity> {
        if self.can_breed()
            && partner.can_breed()
            && self.custom_species == partner.custom_species
            && self.species == partner.species
        {
            let mut baby = if let Some(ref sp) = self.custom_species {
                WildlifeEntity::new_custom(next_id, sp.clone(), self.max_health, self.x, self.y)
            } else {
                WildlifeEntity::new(next_id, self.species, self.x, self.y)
            };
            baby.is_domesticated = self.is_domesticated;
            baby.pen_id = self.pen_id;
            baby.breeding_cooldown = 100;
            Some(baby)
        } else {
            None
        }
    }

    pub fn can_harvest(&self) -> bool {
        self.is_domesticated && self.product_cooldown == 0
    }

    pub fn harvest_products(&self, registry: &GameDataRegistry) -> Vec<String> {
        if !self.is_domesticated {
            return Vec::new();
        }
        if let Some(ref custom_id) = self.custom_species {
            if let Some(def) = registry.get_animal(custom_id) {
                return def.products.clone();
            }
        }
        vec!["meat".into(), "leather".into()]
    }

    pub fn tick_domestic_products(&mut self, registry: &GameDataRegistry) -> Option<Vec<String>> {
        if !self.is_domesticated || !self.is_alive {
            return None;
        }
        if self.product_cooldown > 0 {
            self.product_cooldown -= 1;
            None
        } else {
            self.product_cooldown = 60;
            Some(self.harvest_products(registry))
        }
    }

    pub fn flee_from(&mut self, threat_x: usize, threat_y: usize, map_w: usize, map_h: usize) {
        if self.pen_id.is_some() {
            return;
        }
        let dx = (self.x as isize - threat_x as isize).signum();
        let dy = (self.y as isize - threat_y as isize).signum();

        let (step_x, step_y) = if dx == 0 && dy == 0 { (1, 0) } else { (dx, dy) };

        let new_x = (self.x as isize + step_x).clamp(0, map_w as isize - 1) as usize;
        let new_y = (self.y as isize + step_y).clamp(0, map_h as isize - 1) as usize;
        self.x = new_x;
        self.y = new_y;
        self.behavior_state = AnimalBehaviorState::Fleeing { threat_x, threat_y };
    }

    pub fn chase_towards(&mut self, target_x: usize, target_y: usize, map_w: usize, map_h: usize) {
        let dx = (target_x as isize - self.x as isize).signum();
        let dy = (target_y as isize - self.y as isize).signum();

        let new_x = (self.x as isize + dx).clamp(0, map_w as isize - 1) as usize;
        let new_y = (self.y as isize + dy).clamp(0, map_h as isize - 1) as usize;
        self.x = new_x;
        self.y = new_y;
    }

    pub fn tick(&mut self, map_width: usize, map_height: usize) {
        if !self.is_alive {
            return;
        }

        self.age_ticks += 1;
        self.hunger = (self.hunger + 0.002).min(1.0);

        if self.breeding_cooldown > 0 {
            self.breeding_cooldown -= 1;
        }
        if self.product_cooldown > 0 {
            self.product_cooldown -= 1;
        }

        // Starvation damage
        if self.hunger > 0.9 {
            self.health -= 0.5;
            if self.health <= 0.0 {
                self.is_alive = false;
                return;
            }
        }

        // Domesticated penned animals stay near their pen
        if self.pen_id.is_some() {
            return;
        }

        // Animals currently fleeing or hunting perform purposeful movement instead of random wander
        if matches!(
            self.behavior_state,
            AnimalBehaviorState::Fleeing { .. } | AnimalBehaviorState::Hunting { .. }
        ) {
            return;
        }

        // Random roaming or foraging step
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.4) {
            let dx = rng.gen_range(-1..=1);
            let dy = rng.gen_range(-1..=1);

            let new_x = (self.x as isize + dx).clamp(0, map_width as isize - 1) as usize;
            let new_y = (self.y as isize + dy).clamp(0, map_height as isize - 1) as usize;
            self.x = new_x;
            self.y = new_y;
        }
    }
}

/// Spawn animal event emitted by god-tools and procedural generators
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpawnAnimalRequest {
    pub species: String,
    pub cell: CellId,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AnimalSpawnError {
    #[error("Unknown animal species: '{0}'")]
    UnknownSpecies(String),

    #[error("Incompatible biome: '{species}' cannot spawn in biome '{biome}'")]
    IncompatibleBiome { species: String, biome: String },

    #[error("Regional capacity reached: region ({}, {}) has reached maximum animal count of {}", region.0, region.1, max)]
    RegionalCapacityReached {
        region: (usize, usize),
        current: usize,
        max: usize,
    },

    #[error("Target coordinates ({0}, {1}) are out of map bounds")]
    OutOfBounds(usize, usize),
}

/// Central system that validates (biome & capacity) and spawns animals
pub struct AnimalSpawningSystem;

impl AnimalSpawningSystem {
    pub fn validate_and_spawn(
        req: &SpawnAnimalRequest,
        world: &mut WorldGrid,
        registry: &GameDataRegistry,
        region_capacity_limit: usize,
    ) -> Result<usize, AnimalSpawnError> {
        let x = req.cell.x as usize;
        let y = req.cell.y as usize;

        if x >= world.width || y >= world.height {
            return Err(AnimalSpawnError::OutOfBounds(x, y));
        }

        let animal_def = registry
            .get_animal(&req.species)
            .ok_or_else(|| AnimalSpawnError::UnknownSpecies(req.species.clone()))?;

        // 1. Biome compatibility validation
        let tile = world.tile_at(x, y);
        let biome_name = format!("{:?}", tile.biome).to_lowercase();
        let is_biome_compatible = animal_def.biomes.iter().any(|b| {
            let b_lower = b.to_lowercase();
            biome_name.contains(&b_lower)
                || (b_lower == "forest" && biome_name.contains("forest"))
                || (b_lower == "mountain" && biome_name.contains("mountain"))
                || (b_lower == "plains" && biome_name.contains("plains"))
                || (b_lower == "hills" && biome_name.contains("mountain"))
                || (b_lower == "riverlands" && biome_name.contains("river"))
                || (b_lower == "swamp" && biome_name.contains("swamp"))
        });

        if !is_biome_compatible {
            return Err(AnimalSpawnError::IncompatibleBiome {
                species: req.species.clone(),
                biome: biome_name,
            });
        }

        // 2. Regional carrying capacity validation (16x16 chunk radius)
        let rx = x / 16;
        let ry = y / 16;
        let regional_count = world
            .wildlife
            .iter()
            .filter(|a| a.is_alive && (a.x / 16 == rx) && (a.y / 16 == ry))
            .count();

        if regional_count >= region_capacity_limit {
            return Err(AnimalSpawnError::RegionalCapacityReached {
                region: (rx, ry),
                current: regional_count,
                max: region_capacity_limit,
            });
        }

        // 3. Spawn the animal
        let id = world.next_entity_id;
        world.next_entity_id += 1;

        let entity =
            WildlifeEntity::new_custom(id, req.species.clone(), animal_def.max_health, x, y);
        world.wildlife.push(entity);

        Ok(id)
    }

    /// Natural reproduction based on nourishment, proximity, and regional carrying capacity
    pub fn tick_natural_reproduction(
        world: &mut WorldGrid,
        _registry: &GameDataRegistry,
        carrying_capacity: usize,
    ) -> Vec<usize> {
        let mut new_babies = Vec::new();
        let len = world.wildlife.len();

        for i in 0..len {
            if !world.wildlife[i].is_alive
                || world.wildlife[i].hunger > 0.40
                || world.wildlife[i].age_ticks < 20
                || world.wildlife[i].breeding_cooldown > 0
            {
                continue;
            }

            let sp_i = world.wildlife[i].custom_species.clone();
            let rx = world.wildlife[i].x / 16;
            let ry = world.wildlife[i].y / 16;

            let regional_count = world
                .wildlife
                .iter()
                .filter(|a| a.is_alive && (a.x / 16 == rx) && (a.y / 16 == ry))
                .count();

            if regional_count >= carrying_capacity {
                continue;
            }

            for j in (i + 1)..len {
                if !world.wildlife[j].is_alive
                    || world.wildlife[j].hunger > 0.40
                    || world.wildlife[j].age_ticks < 20
                    || world.wildlife[j].breeding_cooldown > 0
                {
                    continue;
                }

                if world.wildlife[j].custom_species == sp_i
                    && (world.wildlife[j].x as isize - world.wildlife[i].x as isize).abs() <= 2
                    && (world.wildlife[j].y as isize - world.wildlife[i].y as isize).abs() <= 2
                {
                    let next_id = world.next_entity_id;
                    world.next_entity_id += 1;

                    if let Some(baby) = world.wildlife[i].breed(&world.wildlife[j], next_id) {
                        world.wildlife[i].breeding_cooldown = 100;
                        world.wildlife[j].breeding_cooldown = 100;
                        new_babies.push(baby);
                        break;
                    }
                }
            }
        }

        let spawned_ids = new_babies.iter().map(|b| b.id).collect();
        world.wildlife.extend(new_babies);
        spawned_ids
    }

    /// Ticks regional ecology: carnivores hunting, herbivores grazing, and prey fleeing
    pub fn tick_ecology(world: &mut WorldGrid, registry: &GameDataRegistry) {
        let mut prey_eaten = Vec::new();
        let map_w = world.width;
        let map_h = world.height;
        let num_animals = world.wildlife.len();

        // 1. Hunting for carnivores and hungry omnivores
        for i in 0..num_animals {
            let animal = &world.wildlife[i];
            if !animal.is_alive {
                continue;
            }
            let is_hunter = animal.is_carnivore(registry)
                || (animal.is_omnivore(registry) && animal.hunger > 0.40);
            if !is_hunter {
                continue;
            }

            let ax = animal.x;
            let ay = animal.y;

            let mut target_prey: Option<(usize, usize, usize, usize)> = None;
            for (j, prey) in world.wildlife.iter().enumerate() {
                if i == j || !prey.is_alive || prey_eaten.contains(&j) {
                    continue;
                }
                if prey.is_carnivore(registry) {
                    continue;
                }
                let dist =
                    (prey.x as isize - ax as isize).abs() + (prey.y as isize - ay as isize).abs();
                if dist <= 5 {
                    if let Some((best_dist, _, _, _)) = target_prey {
                        if (dist as usize) < best_dist {
                            target_prey = Some((dist as usize, j, prey.x, prey.y));
                        }
                    } else {
                        target_prey = Some((dist as usize, j, prey.x, prey.y));
                    }
                }
            }

            if let Some((dist, prey_idx, px, py)) = target_prey {
                if dist <= 2 {
                    prey_eaten.push(prey_idx);
                    world.wildlife[i].hunger = (world.wildlife[i].hunger - 0.40).max(0.0);
                    world.wildlife[i].behavior_state = AnimalBehaviorState::Idle;
                } else {
                    world.wildlife[i].behavior_state = AnimalBehaviorState::Hunting {
                        target_id: world.wildlife[prey_idx].id,
                    };
                    world.wildlife[i].chase_towards(px, py, map_w, map_h);
                }
            }
        }

        for &idx in &prey_eaten {
            world.wildlife[idx].is_alive = false;
        }

        // 2. Grazing for surviving herbivores and omnivores
        for i in 0..num_animals {
            let (is_alive, diet, ax, ay) = {
                let animal = &world.wildlife[i];
                if !animal.is_alive || prey_eaten.contains(&i) {
                    (false, DietType::Herbivore, 0, 0)
                } else {
                    let diet = if let Some(ref sp) = animal.custom_species {
                        registry
                            .get_animal(sp)
                            .map(|d| d.diet)
                            .unwrap_or(DietType::Herbivore)
                    } else if animal.species.is_predator() {
                        DietType::Carnivore
                    } else {
                        DietType::Herbivore
                    };
                    (true, diet, animal.x, animal.y)
                }
            };

            if is_alive && (diet == DietType::Herbivore || diet == DietType::Omnivore) {
                let tile = world.tile_at_mut(ax, ay);
                let eaten_type = if tile.berry_amount > 0 {
                    tile.berry_amount -= 1;
                    Some(0.25)
                } else if tile.grain_amount > 0 {
                    tile.grain_amount -= 1;
                    Some(0.20)
                } else {
                    None
                };

                if let Some(hunger_reduction) = eaten_type {
                    let animal = &mut world.wildlife[i];
                    animal.hunger = (animal.hunger - hunger_reduction).max(0.0);
                    animal.behavior_state = AnimalBehaviorState::Grazing;
                }
            }
        }

        // 3. Prey threat assessment and fleeing from distant predators
        for i in 0..num_animals {
            let animal = &world.wildlife[i];
            if !animal.is_alive || animal.pen_id.is_some() || prey_eaten.contains(&i) {
                continue;
            }
            let is_prey = animal.is_herbivore(registry) || animal.is_omnivore(registry);
            if !is_prey {
                continue;
            }

            let ax = animal.x;
            let ay = animal.y;

            let mut nearest_threat: Option<(usize, usize, usize)> = None;
            for (j, threat) in world.wildlife.iter().enumerate() {
                if i == j || !threat.is_alive || !threat.is_carnivore(registry) {
                    continue;
                }
                let dist = (threat.x as isize - ax as isize).abs()
                    + (threat.y as isize - ay as isize).abs();
                if (1..=5).contains(&dist) {
                    if let Some((best_dist, _, _)) = nearest_threat {
                        if (dist as usize) < best_dist {
                            nearest_threat = Some((dist as usize, threat.x, threat.y));
                        }
                    } else {
                        nearest_threat = Some((dist as usize, threat.x, threat.y));
                    }
                }
            }

            if let Some((_, tx, ty)) = nearest_threat {
                world.wildlife[i].flee_from(tx, ty, map_w, map_h);
            }
        }
    }
}

/// Tracks extinction events and repopulation dynamics across regional biomes
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtinctionRecord {
    pub species: String,
    pub region: (usize, usize),
    pub extinct_at_tick: u64,
}

/// Emitted when an ecological repopulation or migration occurs
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepopulationEvent {
    pub species: String,
    pub region: (usize, usize),
    pub count: usize,
    pub tick: u64,
}

/// Central ecology and biodiversity manager
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcologyManager {
    pub carrying_capacity_per_region: usize,
    pub repopulation_cooldown_ticks: u64,
    pub active_extinctions: Vec<ExtinctionRecord>,
    pub recent_repopulations: Vec<RepopulationEvent>,
}

impl Default for EcologyManager {
    fn default() -> Self {
        Self {
            carrying_capacity_per_region: 12,
            repopulation_cooldown_ticks: 120,
            active_extinctions: Vec::new(),
            recent_repopulations: Vec::new(),
        }
    }
}

impl EcologyManager {
    pub fn new(carrying_capacity: usize, repopulation_cooldown: u64) -> Self {
        Self {
            carrying_capacity_per_region: carrying_capacity,
            repopulation_cooldown_ticks: repopulation_cooldown,
            active_extinctions: Vec::new(),
            recent_repopulations: Vec::new(),
        }
    }

    /// Evaluates regional biodiversity and triggers repopulation migration events when flora recovers
    pub fn tick_extinctions_and_repopulation(
        &mut self,
        world: &mut WorldGrid,
        registry: &GameDataRegistry,
        current_tick: u64,
    ) -> Vec<RepopulationEvent> {
        let mut new_repopulations = Vec::new();
        let regions_x = world.width.div_ceil(16);
        let regions_y = world.height.div_ceil(16);

        for ry in 0..regions_y {
            for rx in 0..regions_x {
                for (sp_id, animal_def) in &registry.animals {
                    let count = world
                        .wildlife
                        .iter()
                        .filter(|a| {
                            a.is_alive
                                && a.custom_species.as_deref() == Some(sp_id.as_str())
                                && a.x / 16 == rx
                                && a.y / 16 == ry
                        })
                        .count();

                    if count == 0 {
                        // Check if this region contains a biome compatible with this species
                        let region_has_compatible_biome = {
                            let mut compatible = false;
                            for dy in 0..16 {
                                for dx in 0..16 {
                                    let tx = rx * 16 + dx;
                                    let ty = ry * 16 + dy;
                                    if tx < world.width && ty < world.height {
                                        let tile = world.tile_at(tx, ty);
                                        let biome_name = format!("{:?}", tile.biome).to_lowercase();
                                        if animal_def.biomes.iter().any(|b| {
                                            let b_lower = b.to_lowercase();
                                            biome_name.contains(&b_lower)
                                                || (b_lower == "forest"
                                                    && biome_name.contains("forest"))
                                                || (b_lower == "mountain"
                                                    && biome_name.contains("mountain"))
                                                || (b_lower == "plains"
                                                    && biome_name.contains("plains"))
                                                || (b_lower == "hills"
                                                    && biome_name.contains("mountain"))
                                                || (b_lower == "riverlands"
                                                    && biome_name.contains("river"))
                                                || (b_lower == "swamp"
                                                    && biome_name.contains("swamp"))
                                        }) {
                                            compatible = true;
                                            break;
                                        }
                                    }
                                }
                                if compatible {
                                    break;
                                }
                            }
                            compatible
                        };

                        if !region_has_compatible_biome {
                            continue;
                        }

                        let existing_record = self
                            .active_extinctions
                            .iter_mut()
                            .find(|r| r.species == *sp_id && r.region == (rx, ry));

                        match existing_record {
                            None => {
                                self.active_extinctions.push(ExtinctionRecord {
                                    species: sp_id.clone(),
                                    region: (rx, ry),
                                    extinct_at_tick: current_tick,
                                });
                            }
                            Some(rec) => {
                                if current_tick
                                    >= rec.extinct_at_tick + self.repopulation_cooldown_ticks
                                {
                                    let mut spawn_cell: Option<(usize, usize)> = None;
                                    for dy in 0..16 {
                                        for dx in 0..16 {
                                            let tx = rx * 16 + dx;
                                            let ty = ry * 16 + dy;
                                            if tx < world.width && ty < world.height {
                                                let t = world.tile_at(tx, ty);
                                                if !t.elevation_layer.is_water() {
                                                    spawn_cell = Some((tx, ty));
                                                    break;
                                                }
                                            }
                                        }
                                        if spawn_cell.is_some() {
                                            break;
                                        }
                                    }

                                    if let Some((sx, sy)) = spawn_cell {
                                        let herd_count = 2.min(self.carrying_capacity_per_region);
                                        for _ in 0..herd_count {
                                            let id = world.next_entity_id;
                                            world.next_entity_id += 1;
                                            let mut animal = WildlifeEntity::new_custom(
                                                id,
                                                sp_id.clone(),
                                                animal_def.max_health,
                                                sx,
                                                sy,
                                            );
                                            animal.hunger = 0.25;
                                            world.wildlife.push(animal);
                                        }

                                        let event = RepopulationEvent {
                                            species: sp_id.clone(),
                                            region: (rx, ry),
                                            count: herd_count,
                                            tick: current_tick,
                                        };
                                        new_repopulations.push(event);
                                    }
                                }
                            }
                        }
                    } else {
                        self.active_extinctions
                            .retain(|r| !(r.species == *sp_id && r.region == (rx, ry)));
                    }
                }
            }
        }

        for repop in &new_repopulations {
            self.active_extinctions
                .retain(|r| !(r.species == repop.species && r.region == repop.region));
        }

        self.recent_repopulations.extend(new_repopulations.clone());
        new_repopulations
    }
}
