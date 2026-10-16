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
            && self.is_domesticated
            && self.health > self.max_health * 0.7
            && self.hunger < 0.4
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
            baby.is_domesticated = true;
            baby.pen_id = self.pen_id;
            Some(baby)
        } else {
            None
        }
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

    pub fn tick(&mut self, map_width: usize, map_height: usize) {
        if !self.is_alive {
            return;
        }

        self.age_ticks += 1;
        self.hunger = (self.hunger + 0.002).min(1.0);

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

    /// Ticks regional ecology: herbivores graze plants, carnivores hunt herbivores
    pub fn tick_ecology(world: &mut WorldGrid, registry: &GameDataRegistry) {
        let mut prey_eaten = Vec::new();

        for i in 0..world.wildlife.len() {
            let animal = &world.wildlife[i];
            if !animal.is_alive {
                continue;
            }

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

            let ax = animal.x;
            let ay = animal.y;

            match diet {
                DietType::Herbivore => {
                    // Grazing: eats grass or berries on the tile to reduce hunger
                    let tile = world.tile_at_mut(ax, ay);
                    if tile.berry_amount > 0 {
                        tile.berry_amount -= 1;
                        world.wildlife[i].hunger = (world.wildlife[i].hunger - 0.25).max(0.0);
                    } else if tile.grain_amount > 0 {
                        tile.grain_amount -= 1;
                        world.wildlife[i].hunger = (world.wildlife[i].hunger - 0.20).max(0.0);
                    }
                }
                DietType::Carnivore | DietType::Omnivore => {
                    // Hunting: hunts nearby herbivores within 2 tiles
                    for (j, prey) in world.wildlife.iter().enumerate() {
                        if i == j || !prey.is_alive || prey_eaten.contains(&j) {
                            continue;
                        }
                        let prey_is_predator = if let Some(ref sp) = prey.custom_species {
                            registry
                                .get_animal(sp)
                                .map(|d| d.diet == DietType::Carnivore)
                                .unwrap_or(false)
                        } else {
                            prey.species.is_predator()
                        };

                        if !prey_is_predator
                            && (prey.x as isize - ax as isize).abs() <= 2
                            && (prey.y as isize - ay as isize).abs() <= 2
                        {
                            prey_eaten.push(j);
                            world.wildlife[i].hunger = (world.wildlife[i].hunger - 0.40).max(0.0);
                            break;
                        }
                    }
                }
            }
        }

        for idx in prey_eaten {
            world.wildlife[idx].is_alive = false;
        }
    }
}
