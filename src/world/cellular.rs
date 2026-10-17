use super::tile::{ElevationLayer, FrozenState, StructureKind, Tile};
use rand::Rng;

pub struct CellularAutomataEngine;

impl CellularAutomataEngine {
    /// Executes a single cellular automata simulation step across the tile matrix
    pub fn tick(
        tiles: &mut [Tile],
        width: usize,
        height: usize,
        global_temp_offset: f32,
        current_tick: u64,
    ) -> Vec<(usize, usize)> {
        let mut modified_coords = Vec::new();

        // 1. Fire Propagation & Burn Decay (every 2 ticks)
        if current_tick.is_multiple_of(2) {
            Self::tick_fire(tiles, width, height, &mut modified_coords);
        }

        // 2. Lava Flow & Water Solidification into Basalt (every 4 ticks)
        if current_tick.is_multiple_of(4) {
            Self::tick_lava(tiles, width, height, &mut modified_coords);
        }

        // 3. Temperature Diffusion, Freezing to Ice & Thawing (every 8 ticks)
        if current_tick.is_multiple_of(8) {
            Self::tick_freeze_thaw(
                tiles,
                width,
                height,
                global_temp_offset,
                &mut modified_coords,
            );
        }

        // 4. Biome Creep & Competitive Overgrowth (every 10 ticks)
        if current_tick.is_multiple_of(10) {
            Self::tick_biome_creep(tiles, width, height, &mut modified_coords);
        }

        modified_coords
    }

    /// WorldBox Fire Mechanics: Ignites flammable neighbors, extinguishes on water, leaves ash
    fn tick_fire(
        tiles: &mut [Tile],
        width: usize,
        height: usize,
        modified: &mut Vec<(usize, usize)>,
    ) {
        let mut rng = rand::thread_rng();
        let mut fire_spread_targets = Vec::new();
        let mut fires_to_extinguish = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let tile = &tiles[idx];

                // Ash recovery: burned ground slowly regenerates fertility
                if tile.burned_stages > 0 && !tile.is_on_fire && rng.gen_bool(0.15) {
                    modified.push((x, y));
                }

                if !tile.is_on_fire {
                    continue;
                }

                // Water quenches fire immediately
                if tile.elevation_layer.is_water() {
                    fires_to_extinguish.push(idx);
                    continue;
                }

                // Check 4 adjacent neighbors for fire spread
                let neighbors = [
                    (x.wrapping_sub(1), y, x > 0),
                    (x + 1, y, x + 1 < width),
                    (x, y.wrapping_sub(1), y > 0),
                    (x, y + 1, y + 1 < height),
                ];

                for (nx, ny, valid) in neighbors {
                    if valid {
                        let n_idx = ny * width + nx;
                        let n_tile = &tiles[n_idx];
                        if n_tile.can_catch_fire() && !n_tile.is_on_fire {
                            // Spread chance based on moisture and biome flammability
                            let spread_chance = (0.28 * (1.1 - n_tile.moisture)).clamp(0.05, 0.45);
                            if rng.gen_bool(spread_chance as f64) {
                                fire_spread_targets.push((n_idx, nx, ny));
                            }
                        }
                    }
                }
            }
        }

        // Apply fire spreads
        for (idx, x, y) in fire_spread_targets {
            let t = &mut tiles[idx];
            if !t.is_on_fire {
                t.is_on_fire = true;
                t.fire_ticks = rng.gen_range(20..45);
                // Consume flammable surface resources
                if t.wood_amount > 0 {
                    t.wood_amount = t.wood_amount.saturating_sub(2);
                }
                if t.grain_amount > 0 {
                    t.grain_amount = 0;
                }
                modified.push((x, y));
            }
        }

        // Tick down existing fires & extinguish burned out tiles
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let t = &mut tiles[idx];

                // Ash gradual recovery
                if t.burned_stages > 0 && !t.is_on_fire && rng.gen_bool(0.10) {
                    t.burned_stages = t.burned_stages.saturating_sub(1);
                    t.soil_fertility = (t.soil_fertility + 0.15).min(0.9);
                }

                if t.is_on_fire {
                    if t.fire_ticks > 0 {
                        t.fire_ticks -= 1;
                        // Fire damages wood buildings on tile
                        if let Some(ref mut s) = t.structure {
                            if matches!(
                                s.kind,
                                StructureKind::WoodCottage
                                    | StructureKind::Granary
                                    | StructureKind::Campfire
                            ) {
                                s.health = (s.health - 4.0).max(0.0);
                            }
                        }
                    } else {
                        t.is_on_fire = false;
                        t.burned_stages = 4; // Charred ash ground
                        t.soil_fertility = 0.2;
                        modified.push((x, y));
                    }
                }
            }
        }

        for idx in fires_to_extinguish {
            let t = &mut tiles[idx];
            t.is_on_fire = false;
            t.fire_ticks = 0;
            modified.push((t.x, t.y));
        }
    }

    /// WorldBox Lava Mechanics: Flows downhill, ignites neighbors, turns into basalt rock when meeting water
    fn tick_lava(
        tiles: &mut [Tile],
        width: usize,
        height: usize,
        modified: &mut Vec<(usize, usize)>,
    ) {
        let mut rng = rand::thread_rng();
        let mut lava_creations = Vec::new();
        let mut water_solidifications = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let lava_level = tiles[idx].lava_level;
                if lava_level == 0 {
                    continue;
                }

                let current_elev = tiles[idx].elevation;

                let neighbors = [
                    (x.wrapping_sub(1), y, x > 0),
                    (x + 1, y, x + 1 < width),
                    (x, y.wrapping_sub(1), y > 0),
                    (x, y + 1, y + 1 < height),
                ];

                for (nx, ny, valid) in neighbors {
                    if !valid {
                        continue;
                    }
                    let n_idx = ny * width + nx;
                    let n_tile = &tiles[n_idx];

                    // 1. WATER REACTION: When lava touches water, it creates solid basalt rock / hills!
                    if n_tile.elevation_layer.is_water() {
                        water_solidifications.push((n_idx, nx, ny));
                        continue;
                    }

                    // 2. FLOW DOWNHILL: Lava flows downhill into lower adjacent ground
                    if n_tile.elevation <= current_elev
                        && n_tile.lava_level < lava_level
                        && lava_level > 1
                        && rng.gen_bool(0.40)
                    {
                        lava_creations.push((n_idx, nx, ny, lava_level - 1));
                    }

                    // Lava melts ice
                    if n_tile.frozen_state != FrozenState::None {
                        modified.push((nx, ny));
                    }
                }
            }
        }

        // Apply water solidifications into basalt rock (WorldBox signature reaction)
        for (idx, x, y) in water_solidifications {
            let t = &mut tiles[idx];
            t.elevation_layer = ElevationLayer::Hills;
            t.elevation = 0.82;
            t.has_fresh_water = false;
            t.stone_amount += 35;
            t.flint_amount += 15;
            t.lava_level = 0;
            t.is_on_fire = false;
            modified.push((x, y));
        }

        // Apply lava flow downhill
        for (idx, x, y, level) in lava_creations {
            let t = &mut tiles[idx];
            t.lava_level = level;
            t.temperature = 95.0;
            t.frozen_state = FrozenState::None;
            // Lava destroys any existing building
            t.structure = None;
            t.wood_amount = 0;
            t.grain_amount = 0;
            modified.push((x, y));
        }

        // Lava edge cooling: lower-level lava on outskirts gradually cools into rock
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let t = &mut tiles[idx];
                if t.lava_level == 1 && rng.gen_bool(0.12) {
                    t.lava_level = 0;
                    t.elevation_layer = ElevationLayer::Hills;
                    t.stone_amount += 25;
                    t.temperature = 35.0;
                    modified.push((x, y));
                }
            }
        }
    }

    /// WorldBox Freeze/Thaw: Shallow water freezes to walkable ice, snow caps summits, heat melts ice
    fn tick_freeze_thaw(
        tiles: &mut [Tile],
        width: usize,
        height: usize,
        global_temp_offset: f32,
        modified: &mut Vec<(usize, usize)>,
    ) {
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let t = &mut tiles[idx];

                // Summit is perpetually frozen
                if t.elevation_layer == ElevationLayer::Summit {
                    if t.frozen_state != FrozenState::Snow {
                        t.frozen_state = FrozenState::Snow;
                        modified.push((x, y));
                    }
                    continue;
                }

                let effective_temp = t.temperature + global_temp_offset;

                if effective_temp < -2.0 {
                    // Freezing conditions
                    if t.elevation_layer == ElevationLayer::ShallowWaters
                        && t.frozen_state != FrozenState::Ice
                    {
                        t.frozen_state = FrozenState::Ice; // Solid walkable ice!
                        modified.push((x, y));
                    } else if (t.elevation_layer == ElevationLayer::Sand
                        || t.elevation_layer == ElevationLayer::SoilLow)
                        && t.frozen_state != FrozenState::Snow
                    {
                        t.frozen_state = FrozenState::Snow;
                        modified.push((x, y));
                    }
                } else if effective_temp > 4.0 {
                    // Thawing conditions
                    if t.frozen_state == FrozenState::Ice
                        || (t.frozen_state == FrozenState::Snow
                            && t.elevation_layer != ElevationLayer::Summit)
                    {
                        t.frozen_state = FrozenState::None;
                        modified.push((x, y));
                    }
                }

                // Smoothly diffuse back toward ambient biome base temperature over time
                let target_temp = t.biome.base_temperature() + global_temp_offset;
                t.temperature += (target_temp - t.temperature) * 0.05;
            }
        }
    }

    /// WorldBox Biome Creep (WorldBehaviourActionBiomes): Biomes creep to neighboring soil tiles
    fn tick_biome_creep(
        tiles: &mut [Tile],
        width: usize,
        height: usize,
        modified: &mut Vec<(usize, usize)>,
    ) {
        let mut rng = rand::thread_rng();
        let mut conversions = Vec::new();

        // Sample a representative subset of tiles across the map
        for _ in 0..1200 {
            let rx = rng.gen_range(0..width);
            let ry = rng.gen_range(0..height);
            let idx = ry * width + rx;
            let source_tile = &tiles[idx];

            let str_source = source_tile.biome.grow_strength();
            if str_source == 0
                || source_tile.burned_stages > 0
                || source_tile.is_on_fire
                || source_tile.lava_level > 0
            {
                continue;
            }

            // Check neighbor to creep into
            let neighbors = [
                (rx.wrapping_sub(1), ry, rx > 0),
                (rx + 1, ry, rx + 1 < width),
                (rx, ry.wrapping_sub(1), ry > 0),
                (rx, ry + 1, ry + 1 < height),
            ];

            let (nx, ny, valid) = neighbors[rng.gen_range(0..4)];
            if !valid {
                continue;
            }

            let n_idx = ny * width + nx;
            let target_tile = &tiles[n_idx];

            // Only creep onto soil or plateau land
            if matches!(
                target_tile.elevation_layer,
                ElevationLayer::SoilLow | ElevationLayer::SoilHigh
            ) && target_tile.burned_stages == 0
                && target_tile.biome != source_tile.biome
            {
                let str_target = target_tile.biome.grow_strength();
                let can_overgrow = if str_target == 0 {
                    true
                } else if str_source > str_target {
                    rng.gen_bool(0.70)
                } else if str_source == str_target {
                    rng.gen_bool(0.20)
                } else {
                    false
                };

                if can_overgrow {
                    conversions.push((n_idx, nx, ny, source_tile.biome));
                }
            }
        }

        for (idx, x, y, new_biome) in conversions {
            let t = &mut tiles[idx];
            t.biome = new_biome;
            modified.push((x, y));
        }
    }
}
