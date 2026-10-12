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

pub const WORLD_WIDTH: usize = 64;
pub const WORLD_HEIGHT: usize = 64;

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

                // Procedural biome layout:
                // Crystal River: winding path x ~ 28 + 6*sin(y/8)
                let river_center = 28.0 + (y as f32 * 0.15).sin() * 7.0;
                let dist_to_river = (x as f32 - river_center).abs();

                let biome = if dist_to_river <= 2.2 {
                    BiomeType::CrystalRiverlands
                } else if fx > 0.65 && fy < 0.55 {
                    BiomeType::IroncragMountains
                } else if (fx - 0.5).hypot(fy - 0.45) < 0.12 {
                    BiomeType::AncientLeylineRuins
                } else if fx < 0.45 && fy < 0.70 {
                    BiomeType::WhisperingForest
                } else {
                    BiomeType::SunlitPlains
                };

                let elevation = match biome {
                    BiomeType::IroncragMountains => 0.85,
                    BiomeType::AncientLeylineRuins => 0.65,
                    BiomeType::WhisperingForest => 0.50,
                    BiomeType::SunlitPlains => 0.40,
                    BiomeType::CrystalRiverlands => 0.20,
                };

                let moisture = match biome {
                    BiomeType::CrystalRiverlands => 1.0,
                    BiomeType::WhisperingForest => 0.75,
                    BiomeType::SunlitPlains => 0.60,
                    BiomeType::AncientLeylineRuins => 0.35,
                    BiomeType::IroncragMountains => 0.25,
                };

                tiles.push(Tile::new(x, y, biome, elevation, moisture));
            }
        }

        // Initialize Altars
        let mut altars = Vec::new();
        let solar_x = 24;
        let solar_y = 36;
        let mut solar_altar = Altar::new(0, solar_x, solar_y, DeityAlignment::SolarPlayerGod, "The Golden Sun Altar");
        solar_altar.consecrated_level = 2;
        
        let lunar_x = 48;
        let lunar_y = 16;
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

        // Populate initial wildlife
        let mut wildlife = Vec::new();
        let mut next_id = 1;

        // Spawn deer in forest
        for i in 0..12 {
            let wx = 8 + (i * 3) % 20;
            let wy = 10 + (i * 4) % 30;
            wildlife.push(WildlifeEntity::new(next_id, AnimalSpecies::ForestDeer, wx, wy));
            next_id += 1;
        }

        // Spawn wolves in mountains/forest borders
        for i in 0..6 {
            let wx = 35 + (i * 4) % 20;
            let wy = 8 + (i * 3) % 20;
            wildlife.push(WildlifeEntity::new(next_id, AnimalSpecies::DireWolf, wx, wy));
            next_id += 1;
        }

        // Spawn rabbits in plains
        for i in 0..16 {
            let wx = 15 + (i * 3) % 40;
            let wy = 38 + (i * 2) % 22;
            wildlife.push(WildlifeEntity::new(next_id, AnimalSpecies::Rabbit, wx, wy));
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
        if self.wildlife.len() < 25 && self.gaia.world_tick % 100 == 0 {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let rx = rng.gen_range(5..w - 5);
            let ry = rng.gen_range(5..h - 5);
            let id = self.next_entity_id;
            self.next_entity_id += 1;
            self.wildlife.push(WildlifeEntity::new(id, AnimalSpecies::Rabbit, rx, ry));
        }
    }
}
