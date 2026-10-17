use super::cube_sphere::{cell_dir, CellId, CUBE_FACES};
use super::tile::{BiomeType, ElevationLayer, PlacedStructure, StructureKind, Tile};
use glam::Vec3;
use noise::{NoiseFn, OpenSimplex};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoiKind {
    AncientRuins,
    AncientWonder,
    DungeonEntrance,
    ResourceRichVein,
}

impl PoiKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            PoiKind::AncientRuins => "Ancient Leyline Ruins",
            PoiKind::AncientWonder => "Wonder of the World",
            PoiKind::DungeonEntrance => "Subterranean Dungeon Entrance",
            PoiKind::ResourceRichVein => "Abyssal Mineral Pocket",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PointOfInterest {
    pub id: usize,
    pub name: String,
    pub kind: PoiKind,
    pub cell: CellId,
}

/// A 3D planet represented as a cube-sphere grid of 6 faces of N x N cells
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CubeSphereWorld {
    pub face_size: u16,
    pub faces: Vec<Vec<Tile>>, // Exactly 6 faces, each of length face_size * face_size
    pub points_of_interest: Vec<PointOfInterest>,
    pub seed: u64,
    pub world_tick: u64,
}

impl CubeSphereWorld {
    pub fn new(face_size: u16, seed: u64) -> Self {
        let (faces, pois) = Self::generate_terrain(face_size, seed);
        Self {
            face_size,
            faces,
            points_of_interest: pois,
            seed,
            world_tick: 0,
        }
    }

    pub fn get_tile(&self, cell: CellId) -> &Tile {
        let idx = (cell.y as usize) * (self.face_size as usize) + (cell.x as usize);
        &self.faces[cell.face as usize][idx]
    }

    pub fn get_tile_mut(&mut self, cell: CellId) -> &mut Tile {
        let idx = (cell.y as usize) * (self.face_size as usize) + (cell.x as usize);
        &mut self.faces[cell.face as usize][idx]
    }

    pub fn neighbors(&self, cell: CellId) -> [CellId; 4] {
        cell.neighbors(self.face_size)
    }

    pub fn cell_direction(&self, cell: CellId) -> Vec3 {
        cell.direction(self.face_size)
    }

    /// Generates seamless terrain across the 6 cube-sphere faces by sampling 3D noise at unit sphere directions
    fn generate_terrain(n: u16, seed: u64) -> (Vec<Vec<Tile>>, Vec<PointOfInterest>) {
        let simplex_elevation = OpenSimplex::new(seed as u32);
        let simplex_moisture = OpenSimplex::new((seed.wrapping_add(1337)) as u32);
        let simplex_roughness = OpenSimplex::new((seed.wrapping_add(4242)) as u32);

        let face_cells = (n as usize) * (n as usize);
        let mut faces = Vec::with_capacity(CUBE_FACES);
        let mut pois = Vec::new();
        let mut next_poi_id = 1;

        for face in 0..6u8 {
            let mut face_tiles = Vec::with_capacity(face_cells);

            for y in 0..n {
                for x in 0..n {
                    let dir = cell_dir(face, x, y, n);

                    // 1. Multi-octave 3D seamless elevation noise on unit sphere
                    let p_elev = [dir.x as f64 * 1.5, dir.y as f64 * 1.5, dir.z as f64 * 1.5];
                    let p_detail = [dir.x as f64 * 4.5, dir.y as f64 * 4.5, dir.z as f64 * 4.5];
                    let p_micro = [
                        dir.x as f64 * 12.0,
                        dir.y as f64 * 12.0,
                        dir.z as f64 * 12.0,
                    ];

                    let raw_elev = simplex_elevation.get(p_elev) * 0.60
                        + simplex_elevation.get(p_detail) * 0.30
                        + simplex_roughness.get(p_micro) * 0.10;
                    let stretched_elev = (raw_elev * 1.4).clamp(-1.0, 1.0);
                    let elevation = ((stretched_elev + 1.0) * 0.5) as f32; // [0.0, 1.0]

                    // 2. Latitude-based temperature: poles (dir.y ~ ±1) are freezing, equator (dir.y ~ 0) is tropical
                    let latitude = dir.y.abs(); // 0.0 at equator, 1.0 at poles
                    let base_temp = 32.0 - latitude * 46.0; // 32°C at equator, -14°C at poles
                    let temp_noise = simplex_roughness.get([
                        dir.x as f64 * 3.0,
                        dir.y as f64 * 3.0,
                        dir.z as f64 * 3.0,
                    ]) as f32
                        * 4.0;
                    let temperature = base_temp + temp_noise;

                    // 3. 3D moisture field
                    let raw_moist = simplex_moisture.get([
                        dir.x as f64 * 2.2,
                        dir.y as f64 * 2.2,
                        dir.z as f64 * 2.2,
                    ]);
                    let moisture = ((raw_moist + 1.0) * 0.5) as f32;

                    // 4. Biome and elevation layer classification
                    let elevation_layer = ElevationLayer::from_elevation(elevation);
                    let biome = if elevation < 0.28 {
                        BiomeType::DeepOcean
                    } else if elevation < 0.38 {
                        BiomeType::CloseOcean
                    } else if elevation > 0.70 {
                        BiomeType::IroncragMountains
                    } else if temperature < -2.0 {
                        BiomeType::PermafrostTundra
                    } else if elevation < 0.44 {
                        BiomeType::CrystalRiverlands
                    } else if moisture > 0.65 && temperature > 22.0 {
                        BiomeType::Jungle
                    } else if moisture > 0.45 {
                        BiomeType::WhisperingForest
                    } else if moisture < 0.30 && temperature > 18.0 {
                        BiomeType::Savanna
                    } else {
                        BiomeType::SunlitPlains
                    };

                    let mut tile = Tile::new(x as usize, y as usize, biome, elevation, moisture);
                    tile.elevation_layer = elevation_layer;
                    tile.temperature = temperature;

                    // Natural resources initialization
                    match biome {
                        BiomeType::IroncragMountains => {
                            tile.stone_amount = 40;
                            tile.iron_ore_amount = 25;
                            tile.gold_ore_amount = 10;
                            tile.mithril_amount = 5;
                        }
                        BiomeType::WhisperingForest | BiomeType::Jungle => {
                            tile.wood_amount = 35;
                            tile.berry_amount = 15;
                        }
                        BiomeType::SunlitPlains | BiomeType::Savanna => {
                            tile.grain_amount = 20;
                            tile.berry_amount = 10;
                        }
                        _ => {}
                    }

                    // 5. Points of Interest placement based on noise anomalies
                    let poi_chance =
                        (x as u32 * 17 + y as u32 * 31 + face as u32 * 53).is_multiple_of(997);
                    if poi_chance && !elevation_layer.is_water() {
                        let cell = CellId::new(face, x, y);
                        let (kind, name) = match (face as u32 + x as u32 + y as u32) % 4 {
                            0 => (
                                PoiKind::AncientRuins,
                                format!("Sunken Temple of Face {}", face),
                            ),
                            1 => (
                                PoiKind::AncientWonder,
                                format!("Colossus of the Spheres #{}", next_poi_id),
                            ),
                            2 => (
                                PoiKind::DungeonEntrance,
                                format!("Labyrinth of the Core Floor {}", face + 1),
                            ),
                            _ => (
                                PoiKind::ResourceRichVein,
                                format!("Deep Mithril Seam #{}", next_poi_id),
                            ),
                        };

                        if kind == PoiKind::AncientRuins {
                            tile.structure = Some(PlacedStructure::new(StructureKind::StoneWall));
                        }

                        pois.push(PointOfInterest {
                            id: next_poi_id,
                            name,
                            kind,
                            cell,
                        });
                        next_poi_id += 1;
                    }

                    face_tiles.push(tile);
                }
            }

            faces.push(face_tiles);
        }

        (faces, pois)
    }

    /// Advance world simulation tick
    pub fn tick(&mut self) {
        self.world_tick += 1;
    }
}
