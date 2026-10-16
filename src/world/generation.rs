use super::altar::{Altar, DeityAlignment};
use super::tile::{BiomeType, ElevationLayer, PlacedStructure, StructureKind, Tile};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MapArchetype {
    GrandArchipelago,
    Pangaea,
    ScatteredIslands,
    RingAtoll,
    TwinContinents,
    GlacialWastes,
    VolcanicCaldera,
}

impl MapArchetype {
    pub fn display_name(&self) -> &'static str {
        match self {
            MapArchetype::GrandArchipelago => "Grand Archipelago",
            MapArchetype::Pangaea => "Pangaea Supercontinent",
            MapArchetype::ScatteredIslands => "Scattered Islands",
            MapArchetype::RingAtoll => "Sacred Ring Atoll",
            MapArchetype::TwinContinents => "Twin Continents",
            MapArchetype::GlacialWastes => "Glacial Wastes",
            MapArchetype::VolcanicCaldera => "Volcanic Caldera",
        }
    }

    pub fn default_title(&self) -> &'static str {
        match self {
            MapArchetype::GrandArchipelago => "World of Genesis",
            MapArchetype::Pangaea => "The Primeval Supercontinent",
            MapArchetype::ScatteredIslands => "The Sapphire Archipelago",
            MapArchetype::RingAtoll => "Sacred Ring of the Ancients",
            MapArchetype::TwinContinents => "The Divided Twin Realms",
            MapArchetype::GlacialWastes => "The Frozen Tundra of Boreas",
            MapArchetype::VolcanicCaldera => "The Obsidian Caldera",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            MapArchetype::GrandArchipelago => "Signature continent encircled by deep ocean, split by the Great River and mountain spine.",
            MapArchetype::Pangaea => "A massive monolithic supercontinent with inland freshwater seas and dense ancient forests.",
            MapArchetype::ScatteredIslands => "Numerous tropical and rocky islands separated by turquoise ocean channels.",
            MapArchetype::RingAtoll => "A grand volcanic atoll with a sheltered turquoise lagoon and central sanctuary.",
            MapArchetype::TwinContinents => "Two colossal rival continental plates separated by a Great Ocean Rift.",
            MapArchetype::GlacialWastes => "Subzero permafrost continent with frozen waterways, blizzards, and pine taiga.",
            MapArchetype::VolcanicCaldera => "A smoldering volcanic crater with basalt ridges, magma crusts, and obsidian spires.",
        }
    }
}

pub struct MapGenerator;

impl MapGenerator {
    /// Generates a default Grand Archipelago world (seed 42)
    pub fn generate_world(width: usize, height: usize) -> (Vec<Tile>, Vec<Altar>) {
        let (tiles, altars, _) = Self::generate_world_with_params(
            width,
            height,
            MapArchetype::GrandArchipelago,
            42,
            0.5,
            0.5,
        );
        (tiles, altars)
    }

    /// Generates a procedural map based on archetype, seed, water level, and mountain density
    pub fn generate_world_with_params(
        width: usize,
        height: usize,
        archetype: MapArchetype,
        seed: u64,
        water_level: f32,      // 0.0 = Dry, 0.5 = Standard, 1.0 = Flooded
        mountain_density: f32, // 0.0 = Flat, 0.5 = Moderate, 1.0 = Rugged
    ) -> (Vec<Tile>, Vec<Altar>, String) {
        let mut tiles = Vec::with_capacity(width * height);
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;

        // Deterministic pseudo-random seed offsets
        let s = (seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407)
            >> 16) as f32
            * 0.0001;
        let p1 = (s * 1.6180339).sin() * 40.0;
        let p2 = (s * std::f32::consts::E).cos() * 40.0;
        let p3 = (s * std::f32::consts::PI).sin() * 40.0;

        // Sea level adjustment offset: positive water_level pushes elevations down
        let water_offset = (0.5 - water_level.clamp(0.0, 1.0)) * 0.22;
        let mountain_scale = 0.70 + mountain_density.clamp(0.0, 1.0) * 0.60;

        for y in 0..height {
            for x in 0..width {
                let fx = x as f32 / width as f32;
                let fy = y as f32 / height as f32;

                use std::f32::consts::TAU;
                // Multi-octave coherent harmonics
                let n1 = ((fx * TAU * 2.0 + p1).sin() * (fy * TAU * 2.0 + p2).cos()) * 0.5 + 0.5;
                let n2 = ((fx * TAU * 4.5 + p2 + 1.2).cos() * (fy * TAU * 4.5 + p3 + 0.8).sin())
                    * 0.5
                    + 0.5;
                let n3 = ((fx * TAU * 9.0 + p3 + 2.5).sin() * (fy * TAU * 9.0 + p1 + 3.1).cos())
                    * 0.5
                    + 0.5;
                let raw_noise = n1 * 0.55 + n2 * 0.30 + n3 * 0.15;

                let dx_norm = (x as f32 - center_x) / (center_x * 0.94);
                let dy_norm = (y as f32 - center_y) / (center_y * 0.94);
                let dist_from_center = (dx_norm * dx_norm + dy_norm * dy_norm).sqrt();

                let mut elevation: f32;
                let mut is_fresh_river = false;
                let mut force_biome: Option<BiomeType> = None;

                match archetype {
                    MapArchetype::GrandArchipelago => {
                        let land_mask = (1.05 - dist_from_center.powf(2.2)).clamp(0.0, 1.0);
                        elevation = raw_noise * land_mask + water_offset;

                        // River system
                        let main_river_center = 56.0
                            + (y as f32 * 0.08 + p1).sin() * 14.0
                            + (y as f32 * 0.22).cos() * 5.0;
                        let dist_to_main = (x as f32 - main_river_center).abs();
                        let is_edge = x < 9 || x >= width - 9 || y < 9 || y >= height - 9;
                        if dist_to_main <= 2.2 && elevation > 0.20 && !is_edge {
                            is_fresh_river = true;
                            elevation = 0.30;
                        }

                        // Northeast mountains
                        if fx > 0.58 && fy < 0.52 && !is_edge && !is_fresh_river {
                            elevation = (elevation + 0.28 * mountain_scale).min(0.98);
                        }
                    }

                    MapArchetype::Pangaea => {
                        // Monolithic supercontinent covering most of the map
                        let edge_dist = ((fx - 0.5).powi(2) + (fy - 0.5).powi(2)).sqrt() * 2.0;
                        let land_mask = (1.35 - edge_dist.powf(2.8)).clamp(0.0, 1.0);
                        elevation = (raw_noise * 0.75 + 0.25) * land_mask + water_offset;

                        // Central massive mountain backbone
                        let spine_dist = ((fx - 0.50).abs() * 2.0 + (fy - 0.45).abs()).hypot(0.0);
                        if spine_dist < 0.28 {
                            elevation = (elevation + 0.25 * mountain_scale).min(0.98);
                        }

                        // Great inland freshwater lake
                        let lake_dist = (fx - 0.38).hypot(fy - 0.58);
                        if lake_dist < 0.10 && elevation > 0.20 {
                            is_fresh_river = true;
                            elevation = 0.30;
                        }
                    }

                    MapArchetype::ScatteredIslands => {
                        // High-frequency cellular island clusters
                        let island_harmonics = ((fx * 14.0 + p1).sin() * (fy * 14.0 + p2).cos()
                            + (fx * 10.0 - p2).cos() * (fy * 10.0 + p3).sin())
                            * 0.5
                            + 0.5;
                        let combined = raw_noise * 0.35 + island_harmonics * 0.65;
                        let mask = (1.0 - dist_from_center.powf(2.6)).clamp(0.0, 1.0);
                        elevation = (combined * 1.5 - 0.50 + water_offset) * mask;
                    }

                    MapArchetype::RingAtoll => {
                        // Circular caldera ring with central crystal lagoon and inner sanctuary
                        let dist = ((fx - 0.5).powi(2) + (fy - 0.5).powi(2)).sqrt() * 2.0;
                        let ring_delta = (dist - 0.58).abs();
                        let ring_height = (1.0 - (ring_delta / 0.24).powi(2)).max(0.0);
                        let center_sanctuary = (1.0 - (dist / 0.16).powi(2)).max(0.0) * 0.85;

                        let base = ring_height * 0.70 + center_sanctuary + raw_noise * 0.25;
                        elevation = base * (1.1 - (0.5 - water_offset)) + water_offset;

                        // Central lagoon gets crystal fresh/shallow waters
                        if dist < 0.42 && dist > 0.18 {
                            is_fresh_river = true;
                            elevation = 0.30;
                        }
                    }

                    MapArchetype::TwinContinents => {
                        // Two large continental plates separated by a great ocean rift
                        let rift_center = 0.50 + (fy * 5.0 + p1).sin() * 0.08;
                        let dist_to_rift = (fx - rift_center).abs();
                        let rift_carve = (dist_to_rift / 0.15).min(1.0).powf(1.8);

                        let margin_mask = (1.05 - ((fx - 0.5).abs() * 1.9).powf(3.0))
                            .clamp(0.0, 1.0)
                            * (1.05 - ((fy - 0.5).abs() * 1.9).powf(3.0)).clamp(0.0, 1.0);

                        elevation =
                            (raw_noise * 0.90 + 0.10) * margin_mask * rift_carve + water_offset;

                        if fx > 0.62 && elevation > 0.45 {
                            // Eastern continent has towering jagged mountain stronghold
                            elevation = (elevation + 0.22 * mountain_scale).min(0.98);
                        }
                    }

                    MapArchetype::GlacialWastes => {
                        let land_mask = (1.18 - dist_from_center.powf(2.0)).clamp(0.0, 1.0);
                        elevation = raw_noise * land_mask + water_offset;
                        if elevation >= 0.35 {
                            force_biome = if elevation > 0.75 {
                                Some(BiomeType::IroncragMountains)
                            } else if (fx * 5.0 + fy * 7.0 + p1).sin() > 0.4 {
                                Some(BiomeType::WhisperingForest) // Snow pine taiga
                            } else {
                                Some(BiomeType::PermafrostTundra)
                            };
                        }
                    }

                    MapArchetype::VolcanicCaldera => {
                        let land_mask = (1.12 - dist_from_center.powf(2.4)).clamp(0.0, 1.0);
                        elevation = raw_noise * land_mask + water_offset;

                        // Central volcanic cauldron
                        let crater_dist = (fx - 0.5).hypot(fy - 0.5);
                        if crater_dist < 0.14 {
                            elevation = 0.88; // Volcanic caldera rim
                            force_biome = Some(BiomeType::CorruptedInfernal);
                        } else if elevation >= 0.40 {
                            force_biome = if elevation > 0.78 {
                                Some(BiomeType::IroncragMountains)
                            } else if (fx * 4.0 + fy * 4.0 + p2).cos() > 0.2 {
                                Some(BiomeType::CorruptedInfernal)
                            } else if (fx * 6.0 + fy * 6.0 + p1).sin() > 0.5 {
                                Some(BiomeType::AncientLeylineRuins)
                            } else {
                                Some(BiomeType::Savanna)
                            };
                        }
                    }
                }

                // Force outer ring into deep ocean
                let is_outer_edge = x < 8 || x >= width - 8 || y < 8 || y >= height - 8;
                if is_outer_edge {
                    elevation = 0.05;
                    is_fresh_river = false;
                }

                let elevation_layer = ElevationLayer::from_elevation(elevation);

                // Biome Selection
                let biome = if let Some(fb) = force_biome {
                    if elevation_layer.is_water() {
                        BiomeType::CrystalRiverlands
                    } else {
                        fb
                    }
                } else if is_fresh_river
                    || is_outer_edge
                    || elevation_layer == ElevationLayer::DeepOcean
                    || elevation_layer == ElevationLayer::CloseOcean
                {
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
                        BiomeType::PermafrostTundra => 0.25,
                        BiomeType::CorruptedInfernal => 0.15,
                        BiomeType::Swamp => 0.95,
                        _ => 0.50,
                    }
                };

                let mut tile = Tile::new(x, y, biome, elevation, moisture);

                if is_outer_edge
                    || elevation_layer == ElevationLayer::DeepOcean
                    || elevation_layer == ElevationLayer::CloseOcean
                {
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

        // Adaptive Settlement Placement: Find optimal locations for Solar and Lunar altars
        let (solar_x, solar_y) = Self::find_optimal_terran_site(&tiles, width, height);
        let (lunar_x, lunar_y) =
            Self::find_optimal_lunar_site(&tiles, width, height, solar_x, solar_y);

        let mut altars = Vec::new();
        let mut solar_altar = Altar::new(
            0,
            solar_x,
            solar_y,
            DeityAlignment::SolarPlayerGod,
            "The Golden Sun Altar",
        );
        solar_altar.consecrated_level = 2;

        let mut lunar_altar = Altar::new(
            1,
            lunar_x,
            lunar_y,
            DeityAlignment::LunarShadowWeaver,
            "The Obsidian Moon Altar",
        );
        lunar_altar.consecrated_level = 2;

        // Place Altar structures
        let solar_idx = solar_y * width + solar_x;
        tiles[solar_idx].structure = Some(PlacedStructure::new(StructureKind::SolarAltar));
        tiles[solar_idx].has_fresh_water = true;
        tiles[solar_idx].berry_amount = 25;

        let lunar_idx = lunar_y * width + lunar_x;
        tiles[lunar_idx].structure = Some(PlacedStructure::new(StructureKind::LunarAltar));
        tiles[lunar_idx].has_fresh_water = true;
        tiles[lunar_idx].berry_amount = 25;

        // Provision resource bases around both settlements
        for dy in -3..=3 {
            for dx in -3..=3 {
                let sx = (solar_x as isize + dx).clamp(0, width as isize - 1) as usize;
                let sy = (solar_y as isize + dy).clamp(0, height as isize - 1) as usize;
                let s_idx = sy * width + sx;
                if !tiles[s_idx].elevation_layer.is_water() {
                    tiles[s_idx].berry_amount = tiles[s_idx].berry_amount.max(12);
                    tiles[s_idx].grain_amount = tiles[s_idx].grain_amount.max(15);
                    if (dx.abs() + dy.abs()) <= 2 {
                        tiles[s_idx].has_fresh_water = true;
                    }
                }

                let lx = (lunar_x as isize + dx).clamp(0, width as isize - 1) as usize;
                let ly = (lunar_y as isize + dy).clamp(0, height as isize - 1) as usize;
                let l_idx = ly * width + lx;
                if !tiles[l_idx].elevation_layer.is_water() {
                    tiles[l_idx].berry_amount = tiles[l_idx].berry_amount.max(15);
                    tiles[l_idx].iron_ore_amount = tiles[l_idx].iron_ore_amount.max(30);
                    if (dx.abs() + dy.abs()) <= 2 {
                        tiles[l_idx].has_fresh_water = true;
                    }
                }
            }
        }

        // Seed initial village buildings for Solar Terrans
        let solar_buildings = [
            (-2, -2, StructureKind::Campfire),
            (-3, 0, StructureKind::StoneCabin),
            (1, -3, StructureKind::WoodCottage),
            (3, 1, StructureKind::Granary),
            (-1, 2, StructureKind::TownHall),
            (2, -1, StructureKind::Barracks),
        ];
        for (off_x, off_y, kind) in solar_buildings {
            let bx = (solar_x as isize + off_x).clamp(0, width as isize - 1) as usize;
            let by = (solar_y as isize + off_y).clamp(0, height as isize - 1) as usize;
            let idx = by * width + bx;
            if !tiles[idx].elevation_layer.is_water() && tiles[idx].structure.is_none() {
                tiles[idx].structure = Some(PlacedStructure::new(kind));
            }
        }

        // Seed initial village buildings for Lunarians
        let lunar_buildings = [
            (-2, -2, StructureKind::Campfire),
            (-1, 1, StructureKind::StoneCabin),
            (2, -1, StructureKind::MineShaft),
            (-3, 2, StructureKind::Watchtower),
            (1, 2, StructureKind::TownHall),
            (3, 0, StructureKind::Barracks),
        ];
        for (off_x, off_y, kind) in lunar_buildings {
            let bx = (lunar_x as isize + off_x).clamp(0, width as isize - 1) as usize;
            let by = (lunar_y as isize + off_y).clamp(0, height as isize - 1) as usize;
            let idx = by * width + bx;
            if !tiles[idx].elevation_layer.is_water() && tiles[idx].structure.is_none() {
                tiles[idx].structure = Some(PlacedStructure::new(kind));
            }
        }

        altars.push(solar_altar);
        altars.push(lunar_altar);

        let map_title = format!("{} (Seed {})", archetype.default_title(), seed);
        (tiles, altars, map_title)
    }

    fn find_optimal_terran_site(tiles: &[Tile], width: usize, height: usize) -> (usize, usize) {
        let mut best_score = -999.0;
        let mut best_coord = (width / 3, (height * 2) / 3);

        for y in 12..(height - 12) {
            for x in 12..(width - 12) {
                let idx = y * width + x;
                let t = &tiles[idx];
                if t.elevation_layer.is_water() {
                    continue;
                }

                // Prefer fertile lowlands, plains, forest, near freshwater
                let mut score = match t.biome {
                    BiomeType::SunlitPlains => 40.0,
                    BiomeType::WhisperingForest => 35.0,
                    BiomeType::CrystalRiverlands => 45.0,
                    BiomeType::Savanna => 25.0,
                    BiomeType::Jungle => 20.0,
                    _ => 5.0,
                };

                if t.elevation >= 0.44 && t.elevation <= 0.70 {
                    score += 20.0;
                }
                if t.has_fresh_water {
                    score += 30.0;
                }

                // Slight preference for southwestern region
                let dist_from_preferred =
                    ((x as f32 - 40.0).powi(2) + (y as f32 - 80.0).powi(2)).sqrt();
                score -= dist_from_preferred * 0.15;

                if score > best_score {
                    best_score = score;
                    best_coord = (x, y);
                }
            }
        }

        best_coord
    }

    fn find_optimal_lunar_site(
        tiles: &[Tile],
        width: usize,
        height: usize,
        solar_x: usize,
        solar_y: usize,
    ) -> (usize, usize) {
        let mut best_score = -999.0;
        let mut best_coord = ((width * 3) / 4, height / 3);

        for y in 12..(height - 12) {
            for x in 12..(width - 12) {
                let idx = y * width + x;
                let t = &tiles[idx];
                if t.elevation_layer.is_water() {
                    continue;
                }

                let dist_from_solar = ((x as f32 - solar_x as f32).powi(2)
                    + (y as f32 - solar_y as f32).powi(2))
                .sqrt();
                if dist_from_solar < 28.0 {
                    continue; // Must have sufficient geopolitical distance
                }

                // Prefer rugged mountains, hills, leyline ruins, high ground
                let mut score = match t.biome {
                    BiomeType::IroncragMountains => 45.0,
                    BiomeType::AncientLeylineRuins => 40.0,
                    BiomeType::CorruptedInfernal => 35.0,
                    BiomeType::PermafrostTundra => 30.0,
                    _ => 10.0,
                };

                if t.elevation >= 0.65 {
                    score += 25.0;
                }

                // Preference for northeastern region
                let dist_from_preferred =
                    ((x as f32 - 90.0).powi(2) + (y as f32 - 35.0).powi(2)).sqrt();
                score -= dist_from_preferred * 0.15;

                if score > best_score {
                    best_score = score;
                    best_coord = (x, y);
                }
            }
        }

        best_coord
    }
}
