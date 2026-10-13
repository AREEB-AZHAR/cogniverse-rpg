pub mod tile;
pub mod items;
pub mod altar;
pub mod ecology;
pub mod fauna;

use serde::{Deserialize, Serialize};
use tile::{BiomeType, Tile, PlacedStructure, StructureKind};
use altar::{Altar, DeityAlignment};
use ecology::GaiaEngine;
use fauna::{AnimalSpecies, WildlifeEntity};

pub const WORLD_WIDTH: usize = 128;
pub const WORLD_HEIGHT: usize = 128;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldGrid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
    pub gaia: GaiaEngine,
    pub altars: Vec<Altar>,
    pub wildlife: Vec<WildlifeEntity>,
    pub next_entity_id: usize,
}

impl WorldGrid {
    pub fn new() -> Self {
        let width = WORLD_WIDTH;
        let height = WORLD_HEIGHT;
        let mut tiles = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let fx = x as f32 / width as f32;
                let fy = y as f32 / height as f32;

                // Great Continental River System:
                // Main River flowing from north to south
                let river_center = 56.0 + (y as f32 * 0.08).sin() * 14.0 + (y as f32 * 0.22).cos() * 5.0;
                let dist_to_river = (x as f32 - river_center).abs();

                // Tributary river flowing out of the northeastern mountains into the main river
                let tributary_center = 86.0 - ((y as f32 - 15.0) * 0.12).sin() * 16.0;
                let is_tributary = y > 15 && y < 65 && (x as f32 - tributary_center).abs() <= 2.2;

                // Deep Oceanic Margins encircling the continent
                let is_ocean = x < 8 || x >= width - 8 || y < 8 || y >= height - 8;

                let biome = if is_ocean || dist_to_river <= 2.8 || is_tributary {
                    BiomeType::CrystalRiverlands
                } else if fx > 0.60 && fy < 0.50 {
                    BiomeType::IroncragMountains
                } else if (fx - 0.48).hypot(fy - 0.50) < 0.10 {
                    BiomeType::AncientLeylineRuins
                } else if (fx < 0.45 && fy < 0.68) || (fx < 0.38 && fy > 0.65) {
                    BiomeType::WhisperingForest
                } else {
                    BiomeType::SunlitPlains
                };

                let elevation = match biome {
                    BiomeType::IroncragMountains => 0.88,
                    BiomeType::AncientLeylineRuins => 0.65,
                    BiomeType::WhisperingForest => 0.50,
                    BiomeType::SunlitPlains => 0.40,
                    BiomeType::CrystalRiverlands => if is_ocean { 0.05 } else { 0.20 },
                };

                let moisture = match biome {
                    BiomeType::CrystalRiverlands => 1.0,
                    BiomeType::WhisperingForest => 0.75,
                    BiomeType::SunlitPlains => 0.60,
                    BiomeType::AncientLeylineRuins => 0.35,
                    BiomeType::IroncragMountains => 0.22,
                };

                tiles.push(Tile::new(x, y, biome, elevation, moisture));
            }
        }

        // Initialize Altars in grand continent
        let mut altars = Vec::new();
        let solar_x = 38;
        let solar_y = 86;
        let mut solar_altar = Altar::new(0, solar_x, solar_y, DeityAlignment::SolarPlayerGod, "The Golden Sun Altar");
        solar_altar.consecrated_level = 2;
        
        let lunar_x = 92;
        let lunar_y = 34;
        let mut lunar_altar = Altar::new(1, lunar_x, lunar_y, DeityAlignment::LunarShadowWeaver, "The Obsidian Moon Altar");
        lunar_altar.consecrated_level = 2;

        // Place altar structures on tiles
        let solar_idx = solar_y * width + solar_x;
        tiles[solar_idx].structure = Some(PlacedStructure {
            kind: StructureKind::SolarAltar,
            owner_id: None,
            health: 500.0,
            max_health: 500.0,
        });

        let lunar_idx = lunar_y * width + lunar_x;
        tiles[lunar_idx].structure = Some(PlacedStructure {
            kind: StructureKind::LunarAltar,
            owner_id: None,
            health: 500.0,
            max_health: 500.0,
        });

        altars.push(solar_altar);
        altars.push(lunar_altar);

        // Populate initial wildlife across the 128x128 expanse
        let mut wildlife = Vec::new();
        let mut next_id = 1;

        // Spawn deer herds in the Whispering Forest
        for i in 0..28 {
            let wx = 14 + (i * 5) % 40;
            let wy = 16 + (i * 7) % 65;
            wildlife.push(WildlifeEntity::new(next_id, AnimalSpecies::ForestDeer, wx, wy));
            next_id += 1;
        }

        // Spawn dire wolves in mountain foothills
        for i in 0..14 {
            let wx = 75 + (i * 4) % 42;
            let wy = 14 + (i * 6) % 40;
            wildlife.push(WildlifeEntity::new(next_id, AnimalSpecies::DireWolf, wx, wy));
            next_id += 1;
        }

        // Spawn rabbits in plains
        for i in 0..36 {
            let wx = 22 + (i * 7) % 80;
            let wy = 60 + (i * 5) % 52;
            wildlife.push(WildlifeEntity::new(next_id, AnimalSpecies::Rabbit, wx, wy));
            next_id += 1;
        }

        // Spawn mountain bears
        for i in 0..12 {
            let wx = 70 + (i * 5) % 45;
            let wy = 20 + (i * 6) % 35;
            wildlife.push(WildlifeEntity::new(next_id, AnimalSpecies::MountainBear, wx, wy));
            next_id += 1;
        }

        Self {
            width,
            height,
            tiles,
            gaia: GaiaEngine::new(),
            altars,
            wildlife,
            next_entity_id: next_id,
        }
    }

    pub fn tile_at(&self, x: usize, y: usize) -> &Tile {
        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);
        &self.tiles[y * self.width + x]
    }

    pub fn tile_at_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);
        let idx = y * self.width + x;
        &mut self.tiles[idx]
    }

    pub fn tick(&mut self) {
        // Tick Gaia climate and flora
        self.gaia.tick(&mut self.tiles);

        // Tick wildlife
        let w = self.width;
        let h = self.height;
        for animal in self.wildlife.iter_mut() {
            animal.tick(w, h);
        }

        // Prune dead wildlife
        self.wildlife.retain(|a| a.is_alive);

        // Wildlife reproduction if population drops below carrying capacity
        if self.wildlife.len() < 60 && self.gaia.world_tick % 80 == 0 {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let rx = rng.gen_range(12..w - 12);
            let ry = rng.gen_range(12..h - 12);
            let id = self.next_entity_id;
            self.next_entity_id += 1;
            let species = match rng.gen_range(0..4) {
                0 => AnimalSpecies::Rabbit,
                1 => AnimalSpecies::ForestDeer,
                2 => AnimalSpecies::MountainBear,
                _ => AnimalSpecies::DireWolf,
            };
            self.wildlife.push(WildlifeEntity::new(id, species, rx, ry));
        }
    }
}
