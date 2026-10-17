use serde::{Deserialize, Serialize};

/// High-performance 2D dynamic decaying hazard and danger field
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DangerField {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<f32>,
    #[serde(skip)]
    pub buffer: Vec<f32>,
    /// Temporal decay rate per simulation tick (e.g. 0.05 = 5% decay per tick)
    pub decay_rate: f32,
    /// Spatial neighborhood diffusion bleed factor (e.g. 0.08)
    pub diffusion_rate: f32,
}

impl Default for DangerField {
    fn default() -> Self {
        Self::new(128, 128)
    }
}

impl DangerField {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            cells: vec![0.0; size],
            buffer: vec![0.0; size],
            decay_rate: 0.05,
            diffusion_rate: 0.08,
        }
    }

    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        (y % self.height) * self.width + (x % self.width)
    }

    /// Sample danger intensity at coordinates (x, y) clamped to [0.0, 100.0]
    pub fn sample_danger(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.cells[self.index(x, y)]
    }

    /// Injects hazard/danger intensity into an epicenter with quadratic distance falloff
    pub fn inject_danger(&mut self, center_x: usize, center_y: usize, radius: usize, amount: f32) {
        let min_x = center_x.saturating_sub(radius);
        let max_x = (center_x + radius).min(self.width.saturating_sub(1));
        let min_y = center_y.saturating_sub(radius);
        let max_y = (center_y + radius).min(self.height.saturating_sub(1));

        let r_f32 = radius.max(1) as f32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dx = (x as isize - center_x as isize).unsigned_abs() as f32;
                let dy = (y as isize - center_y as isize).unsigned_abs() as f32;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist <= r_f32 {
                    let falloff = (1.0 - dist / r_f32).powi(2);
                    let idx = self.index(x, y);
                    self.cells[idx] = (self.cells[idx] + amount * falloff).min(100.0);
                }
            }
        }
    }

    /// Computes the normalized gradient direction vector (gx, gy) pointing AWAY from danger towards safety.
    /// If there is no danger gradient, returns (0.0, 0.0).
    pub fn sample_danger_gradient(&self, x: usize, y: usize) -> (f32, f32) {
        if x >= self.width || y >= self.height {
            return (0.0, 0.0);
        }

        let left = if x > 0 {
            self.sample_danger(x - 1, y)
        } else {
            self.sample_danger(x, y)
        };
        let right = if x + 1 < self.width {
            self.sample_danger(x + 1, y)
        } else {
            self.sample_danger(x, y)
        };
        let up = if y > 0 {
            self.sample_danger(x, y - 1)
        } else {
            self.sample_danger(x, y)
        };
        let down = if y + 1 < self.height {
            self.sample_danger(x, y + 1)
        } else {
            self.sample_danger(x, y)
        };

        // Negative gradient points downhill towards safety
        let gx = -(right - left);
        let gy = -(down - up);

        let len = (gx * gx + gy * gy).sqrt();
        if len > 0.001 {
            (gx / len, gy / len)
        } else {
            (0.0, 0.0)
        }
    }

    /// Ticks the danger field: applies spatial 4-neighbor diffusion and temporal exponential decay
    pub fn tick(&mut self) {
        if self.buffer.len() != self.cells.len() {
            self.buffer = vec![0.0; self.cells.len()];
        }

        let w = self.width;
        let h = self.height;
        let diff = self.diffusion_rate;
        let retain = (1.0 - self.decay_rate).max(0.0);

        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                let current = self.cells[idx];

                // Sample orthogonal neighbors for diffusion
                let mut neighbor_sum = 0.0;
                let mut count = 0.0;

                if x > 0 {
                    neighbor_sum += self.cells[y * w + (x - 1)];
                    count += 1.0;
                }
                if x + 1 < w {
                    neighbor_sum += self.cells[y * w + (x + 1)];
                    count += 1.0;
                }
                if y > 0 {
                    neighbor_sum += self.cells[(y - 1) * w + x];
                    count += 1.0;
                }
                if y + 1 < h {
                    neighbor_sum += self.cells[(y + 1) * w + x];
                    count += 1.0;
                }

                if current <= 0.0001 && neighbor_sum <= 0.0001 {
                    self.buffer[idx] = 0.0;
                    continue;
                }

                let diffused = if count > 0.0 {
                    (1.0 - diff) * current + diff * (neighbor_sum / count)
                } else {
                    current
                };

                let decayed = diffused * retain;
                self.buffer[idx] = if decayed < 0.01 {
                    0.0
                } else {
                    decayed.min(100.0)
                };
            }
        }

        // Swap buffer with cells to finalize update
        std::mem::swap(&mut self.cells, &mut self.buffer);
    }

    /// Clear all danger across the entire field
    pub fn clear(&mut self) {
        self.cells.fill(0.0);
        self.buffer.fill(0.0);
    }
}
