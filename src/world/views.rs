use super::cube_sphere::CellId;
use super::planet::CubeSphereWorld;
use super::tile::Tile;
use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobePixel {
    pub cell: Option<CellId>,
    pub biome_id: u8,
    pub render_char: char,
}

pub struct GlobeView;

impl GlobeView {
    /// Renders a pixel-art 3D rotating globe view over the CubeSphereWorld using nearest-neighbor sampling
    pub fn render_orthographic(
        world: &CubeSphereWorld,
        viewport_radius: usize,
        yaw: f32,   // Rotation around vertical Y axis in radians
        pitch: f32, // Tilt angle in radians
    ) -> Vec<Vec<GlobePixel>> {
        let size = viewport_radius * 2 + 1;
        let r = viewport_radius as f32;
        let cos_yaw = yaw.cos();
        let sin_yaw = yaw.sin();
        let cos_pitch = pitch.cos();
        let sin_pitch = pitch.sin();

        let mut grid = Vec::with_capacity(size);

        for py in 0..size {
            let mut row = Vec::with_capacity(size);
            let sy = (py as f32 - r) / r; // [-1.0, 1.0]

            for px in 0..size {
                let sx = (px as f32 - r) / r; // [-1.0, 1.0]
                let dist_sq = sx * sx + sy * sy;

                if dist_sq > 1.0 {
                    // Outside sphere: space background
                    row.push(GlobePixel {
                        cell: None,
                        biome_id: 255,
                        render_char: ' ',
                    });
                    continue;
                }

                let sz = (1.0 - dist_sq).sqrt(); // Facing front (+Z in view space)

                // 1. Rotate view ray by pitch (around X) then yaw (around Y)
                let y1 = sy * cos_pitch - sz * sin_pitch;
                let z1 = sy * sin_pitch + sz * cos_pitch;

                let x2 = sx * cos_yaw + z1 * sin_yaw;
                let y2 = y1;
                let z2 = -sx * sin_yaw + z1 * cos_yaw;

                let p = Vec3::new(x2, y2, z2);

                // 2. Identify owning cube face by maximal axis component
                let abs_x = p.x.abs();
                let abs_y = p.y.abs();
                let abs_z = p.z.abs();

                let (face, u, v) = if abs_x >= abs_y && abs_x >= abs_z {
                    if p.x > 0.0 {
                        (0, -p.z / p.x, p.y / p.x)
                    } else {
                        (1, -p.z / p.x, -p.y / p.x)
                    }
                } else if abs_y >= abs_x && abs_y >= abs_z {
                    if p.y > 0.0 {
                        (2, p.x / p.y, -p.z / p.y)
                    } else {
                        (3, -p.x / p.y, -p.z / p.y)
                    }
                } else if p.z > 0.0 {
                    (4, p.x / p.z, p.y / p.z)
                } else {
                    (5, p.x / p.z, -p.y / p.z)
                };

                // 3. Nearest-neighbor sampling to integer cell coordinates
                let n = world.face_size as f32;
                let cx = (((u + 1.0) * 0.5 * n).floor() as i32).clamp(0, world.face_size as i32 - 1)
                    as u16;
                let cy = (((v + 1.0) * 0.5 * n).floor() as i32).clamp(0, world.face_size as i32 - 1)
                    as u16;

                let cell = CellId::new(face, cx, cy);
                let tile = world.get_tile(cell);

                let render_char = match tile.biome {
                    super::tile::BiomeType::DeepOcean => '~',
                    super::tile::BiomeType::CloseOcean => '=',
                    super::tile::BiomeType::CrystalRiverlands => '≈',
                    super::tile::BiomeType::IroncragMountains => '^',
                    super::tile::BiomeType::WhisperingForest => '♣',
                    super::tile::BiomeType::Jungle => '♠',
                    super::tile::BiomeType::Savanna => '░',
                    super::tile::BiomeType::SunlitPlains => '.',
                    super::tile::BiomeType::PermafrostTundra => '*',
                    super::tile::BiomeType::AncientLeylineRuins => 'Ω',
                    super::tile::BiomeType::CorruptedInfernal => '!',
                    super::tile::BiomeType::Swamp => '%',
                };

                row.push(GlobePixel {
                    cell: Some(cell),
                    biome_id: tile.biome.to_id(),
                    render_char,
                });
            }

            grid.push(row);
        }

        grid
    }
}

/// Region view: 2D pixel-art window focused on a specific area of the planet
pub struct RegionView;

impl RegionView {
    /// Extracts a top-down 2D tile slice around a center cell of size (width, height)
    pub fn extract_window(
        world: &CubeSphereWorld,
        center: CellId,
        radius_x: usize,
        radius_y: usize,
    ) -> Vec<Vec<&Tile>> {
        let width = radius_x * 2 + 1;
        let height = radius_y * 2 + 1;
        let mut rows = Vec::with_capacity(height);

        let n = world.face_size as i32;
        let cx = center.x as i32;
        let cy = center.y as i32;

        for dy in -(radius_y as i32)..=(radius_y as i32) {
            let mut row = Vec::with_capacity(width);
            for dx in -(radius_x as i32)..=(radius_x as i32) {
                let tx = cx + dx;
                let ty = cy + dy;

                let cell = if tx >= 0 && tx < n && ty >= 0 && ty < n {
                    CellId::new(center.face, tx as u16, ty as u16)
                } else {
                    // Navigate seamless cube neighbors if window extends beyond face boundary
                    let mut current = center;
                    if dx > 0 {
                        for _ in 0..dx {
                            current = current.neighbors(world.face_size)[0]; // Right
                        }
                    } else if dx < 0 {
                        for _ in 0..(-dx) {
                            current = current.neighbors(world.face_size)[2]; // Left
                        }
                    }
                    if dy > 0 {
                        for _ in 0..dy {
                            current = current.neighbors(world.face_size)[1]; // Up
                        }
                    } else if dy < 0 {
                        for _ in 0..(-dy) {
                            current = current.neighbors(world.face_size)[3]; // Down
                        }
                    }
                    current
                };

                row.push(world.get_tile(cell));
            }
            rows.push(row);
        }

        rows
    }
}
