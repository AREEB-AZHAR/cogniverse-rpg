use super::tile::{BiomeType, ElevationLayer, Tile, PlacedStructure, StructureKind};
use super::altar::{Altar, DeityAlignment};

pub struct MapGenerator;

impl MapGenerator {
    /// Generates a WorldBox-style 128x128 Grand Continental Archipelago with coherent fractal noise
    pub fn generate_world(width: usize, height: usize) -> (Vec<Tile>, Vec<Altar>) {
        let mut tiles = Vec::with_capacity(width * height);
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;

        for y in 0..height {
            for x in 0..width {
                let fx = x as f32 / width as f32;
                let fy = y as f32 / height as f32;

                // 1. Continental Elliptical Distance Falloff (Guarantees deep ocean encircling continent)
                let dx_norm = (x as f32 - center_x) / (center_x * 0.94);
                let dy_norm = (y as f32 - center_y) / (center_y * 0.94);
                let dist_from_center = (dx_norm * dx_norm + dy_norm * dy_norm).sqrt();
                let land_mask = (1.05 - dist_from_center.powf(2.2)).clamp(0.0, 1.0);

                // 2. Multi-Octave Coherent Fractal Noise (Simulated with harmonics)
                let n1 = ((fx * 6.28 * 2.0).sin() * (fy * 6.28 * 2.0).cos()) * 0.5 + 0.5;
                let n2 = ((fx * 6.28 * 4.5 + 1.2).cos() * (fy * 6.28 * 4.5 + 0.8).sin()) * 0.5 + 0.5;
                let n3 = ((fx * 6.28 * 9.0 + 2.5).sin() * (fy * 6.28 * 9.0 + 3.1).cos()) * 0.5 + 0.5;
                let raw_noise = n1 * 0.55 + n2 * 0.30 + n3 * 0.15;

                // Combined elevation
                let mut elevation = raw_noise * land_mask;

                // Force outer ring into deep ocean
                let is_edge_ocean = x < 9 || x >= width - 9 || y < 9 || y >= height - 9;
                if is_edge_ocean {
                    elevation = 0.05;
                }

                // 3. Great Continental River Systems
                let main_river_center = 56.0 + (y as f32 * 0.08).sin() * 14.0 + (y as f32 * 0.22).cos() * 5.0;
                let dist_to_main_river = (x as f32 - main_river_center).abs();
                let is_main_river = dist_to_main_river <= 2.2 && elevation > 0.20 && !is_edge_ocean;

                let tributary_center = 86.0 - ((y as f32 - 15.0) * 0.12).sin() * 16.0;
                let is_tributary = y > 15 && y < 65 && (x as f32 - tributary_center).abs() <= 1.8 && elevation > 0.20 && !is_edge_ocean;

                let is_fresh_river = is_main_river || is_tributary;
                if is_fresh_river {
                    elevation = 0.30; // Shallow waters depth
                }

                // Boost elevation in northeastern mountains
                if fx > 0.58 && fy < 0.52 && !is_edge_ocean && !is_fresh_river {
                    elevation = (elevation + 0.32).min(0.98);
                }

                // 4. Biome Selection based on geographic zones & elevation
                let elevation_layer = ElevationLayer::from_elevation(elevation);

                let biome = if is_fresh_river {
                    BiomeType::CrystalRiverlands
                } else if is_edge_ocean || elevation_layer == ElevationLayer::DeepOcean || elevation_layer == ElevationLayer::CloseOcean {
                    // Oceans are not riverlands!
                    BiomeType::CrystalRiverlands 
                } else if elevation >= 0.82 {
                    BiomeType::IroncragMountains
                } else if (fx - 0.48).hypot(fy - 0.50) < 0.10 {
                    BiomeType::AncientLeylineRuins
                } else if fx > 0.70 && fy > 0.65 {
                    BiomeType::Savanna
                } else if fx < 0.45 && fy < 0.40 {
                    BiomeType::Jungle
                } else if (fx < 0.45 && fy < 0.70) || (fx < 0.38 && fy > 0.65) {
                    BiomeType::WhisperingForest
                } else {
                    BiomeType::SunlitPlains
                };

                let moisture = if is_fresh_river {
                    1.0
                } else if elevation_layer.is_water() {
                    0.9
                } else {
                    match biome {
                        BiomeType::Jungle => 0.95,
                        BiomeType::WhisperingForest => 0.78,
                        BiomeType::SunlitPlains => 0.60,
                        BiomeType::Savanna => 0.35,
                        BiomeType::AncientLeylineRuins => 0.40,
                        BiomeType::IroncragMountains => 0.30,
                        _ => 0.50,
                    }
                };

                let mut tile = Tile::new(x, y, biome, elevation, moisture);
                
                // Specific freshness rules: Only inland freshwater rivers and mountain springs have fresh drinking water
                if is_edge_ocean || elevation_layer == ElevationLayer::DeepOcean || elevation_layer == ElevationLayer::CloseOcean {
                    tile.has_fresh_water = false;
                    tile.berry_amount = 0;
                    tile.grain_amount = 0;
                    tile.wood_amount = 0;
                } else if is_fresh_river {
                    tile.has_fresh_water = true;
                }

                tiles.push(tile);
            }
        }

        // Initialize Altars
        let mut altars = Vec::new();
        let solar_x = 38;
        let solar_y = 86;
        let mut solar_altar = Altar::new(0, solar_x, solar_y, DeityAlignment::SolarPlayerGod, "The Golden Sun Altar");
        solar_altar.consecrated_level = 2;

        let lunar_x = 92;
        let lunar_y = 34;
        let mut lunar_altar = Altar::new(1, lunar_x, lunar_y, DeityAlignment::LunarShadowWeaver, "The Obsidian Moon Altar");
        lunar_altar.consecrated_level = 2;

        // Place Altar structures on tiles
        let solar_idx = solar_y * width + solar_x;
        tiles[solar_idx].structure = Some(PlacedStructure::new(StructureKind::SolarAltar));

        let lunar_idx = lunar_y * width + lunar_x;
        tiles[lunar_idx].structure = Some(PlacedStructure::new(StructureKind::LunarAltar));

        // Vital Fix for Lunarians: Provision their mountain stronghold around (92, 34)
        // with pure underground mountain spring water and cave food deposits!
        for dy in -3..=3 {
            for dx in -3..=3 {
                let qx = (lunar_x as isize + dx).clamp(0, width as isize - 1) as usize;
                let qy = (lunar_y as isize + dy).clamp(0, height as isize - 1) as usize;
                let idx = qy * width + qx;
                tiles[idx].berry_amount = 15; // Mountain cave berries & lichen
                if (dx.abs() + dy.abs()) <= 2 {
                    tiles[idx].has_fresh_water = true; // Mountain cavern fresh water spring
                }
            }
        }

        // Seed initial village buildings for Solar Terrans (Village of Solaria)
        let solar_buildings = [
            (36, 84, StructureKind::Campfire),
            (35, 86, StructureKind::StoneCabin),
            (39, 83, StructureKind::WoodCottage),
            (41, 87, StructureKind::Granary),
            (37, 88, StructureKind::TownHall),
            (40, 85, StructureKind::Barracks),
        ];
        for (bx, by, kind) in solar_buildings {
            let idx = by * width + bx;
            tiles[idx].structure = Some(PlacedStructure::new(kind));
        }

        // Seed initial village buildings for Chthonic Lunarians (Shadow Hold)
        let lunar_buildings = [
            (90, 32, StructureKind::Campfire),
            (91, 35, StructureKind::StoneCabin),
            (94, 33, StructureKind::MineShaft),
            (89, 36, StructureKind::Watchtower),
            (93, 36, StructureKind::TownHall),
            (95, 34, StructureKind::Barracks),
        ];
        for (bx, by, kind) in lunar_buildings {
            let idx = by * width + bx;
            tiles[idx].structure = Some(PlacedStructure::new(kind));
        }

        altars.push(solar_altar);
        altars.push(lunar_altar);

        (tiles, altars)
    }
}
