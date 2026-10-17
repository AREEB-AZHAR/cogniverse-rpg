use glam::Vec3;
use serde::{Deserialize, Serialize};

pub const CUBE_FACES: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct CellId {
    pub face: u8,
    pub x: u16,
    pub y: u16,
}

impl CellId {
    pub fn new(face: u8, x: u16, y: u16) -> Self {
        Self { face, x, y }
    }

    /// Unit direction for a cell centre; sample 3D noise here for seamless terrain.
    pub fn direction(&self, n: u16) -> Vec3 {
        cell_dir(self.face, self.x, self.y, n)
    }

    /// Returns the 4 orthogonal neighbors [Right, Up, Left, Down] on the cube-sphere surface.
    /// Every cell on the sphere has exactly 4 distinct neighbors, and the neighbor relation is symmetric.
    pub fn neighbors(&self, n: u16) -> [CellId; 4] {
        debug_assert!(n > 0);
        let max = n - 1;
        let f = self.face;
        let x = self.x;
        let y = self.y;

        // Right (+x), Up (+y), Left (-x), Down (-y)
        let right = if x < max {
            CellId::new(f, x + 1, y)
        } else {
            step_cube_edge(f, 0, y, n)
        };

        let up = if y < max {
            CellId::new(f, x, y + 1)
        } else {
            step_cube_edge(f, 1, x, n)
        };

        let left = if x > 0 {
            CellId::new(f, x - 1, y)
        } else {
            step_cube_edge(f, 2, y, n)
        };

        let down = if y > 0 {
            CellId::new(f, x, y - 1)
        } else {
            step_cube_edge(f, 3, x, n)
        };

        [right, up, left, down]
    }
}

/// Unit direction for a cell centre on face [0..6] with grid resolution n.
pub fn cell_dir(face: u8, x: u16, y: u16, n: u16) -> Vec3 {
    let u = (x as f32 + 0.5) / n as f32 * 2.0 - 1.0;
    let v = (y as f32 + 0.5) / n as f32 * 2.0 - 1.0;
    let p = match face {
        0 => Vec3::new(1.0, v, -u),
        1 => Vec3::new(-1.0, v, u),
        2 => Vec3::new(u, 1.0, -v),
        3 => Vec3::new(u, -1.0, v),
        4 => Vec3::new(u, v, 1.0),
        _ => Vec3::new(-u, v, -1.0),
    };
    p.normalize()
}

/// Cube edge transition stepping:
/// edge 0 = Right (+x at x=n-1), coord is y
/// edge 1 = Up (+y at y=n-1), coord is x
/// edge 2 = Left (-x at x=0), coord is y
/// edge 3 = Down (-y at y=0), coord is x
fn step_cube_edge(face: u8, edge: u8, coord: u16, n: u16) -> CellId {
    let max = n - 1;
    let rev = max - coord;

    match (face, edge) {
        // Face 0 (+X): right -> Face 5 (left, -Z), up -> Face 2 (+Y), left -> Face 4 (+Z), down -> Face 3 (-Y)
        (0, 0) => CellId::new(5, 0, coord),
        (0, 1) => CellId::new(2, max, rev),
        (0, 2) => CellId::new(4, max, coord),
        (0, 3) => CellId::new(3, max, coord),

        // Face 1 (-X): right -> Face 4 (+Z), up -> Face 2 (+Y), left -> Face 5 (-Z), down -> Face 3 (-Y)
        (1, 0) => CellId::new(4, 0, coord),
        (1, 1) => CellId::new(2, 0, coord),
        (1, 2) => CellId::new(5, max, coord),
        (1, 3) => CellId::new(3, 0, rev),

        // Face 2 (+Y): right -> Face 0, up -> Face 5, left -> Face 1, down -> Face 4
        (2, 0) => CellId::new(0, rev, max),
        (2, 1) => CellId::new(5, rev, max),
        (2, 2) => CellId::new(1, coord, max),
        (2, 3) => CellId::new(4, coord, max),

        // Face 3 (-Y): right -> Face 0, up -> Face 4, left -> Face 1, down -> Face 5
        (3, 0) => CellId::new(0, coord, 0),
        (3, 1) => CellId::new(4, coord, 0),
        (3, 2) => CellId::new(1, rev, 0),
        (3, 3) => CellId::new(5, rev, 0),

        // Face 4 (+Z): right -> Face 0, up -> Face 2, left -> Face 1, down -> Face 3
        (4, 0) => CellId::new(0, 0, coord),
        (4, 1) => CellId::new(2, coord, 0),
        (4, 2) => CellId::new(1, max, coord),
        (4, 3) => CellId::new(3, coord, max),

        // Face 5 (-Z): right -> Face 1, up -> Face 2, left -> Face 0, down -> Face 3
        (5, 0) => CellId::new(1, 0, coord),
        (5, 1) => CellId::new(2, rev, max),
        (5, 2) => CellId::new(0, max, coord),
        (5, 3) => CellId::new(3, rev, 0),

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_cube_sphere_neighbors_distinct_and_symmetric() {
        for n in [4u16, 8, 16] {
            for face in 0..6u8 {
                for y in 0..n {
                    for x in 0..n {
                        let cell = CellId::new(face, x, y);
                        let neighbors = cell.neighbors(n);

                        // 1. Every cell must have exactly 4 distinct neighbors
                        let set: HashSet<_> = neighbors.iter().copied().collect();
                        assert_eq!(
                            set.len(),
                            4,
                            "Cell {:?} at n={} has non-distinct neighbors: {:?}",
                            cell,
                            n,
                            neighbors
                        );

                        // 2. Neighbor relation must be symmetric: if B is neighbor of A, A must be neighbor of B
                        for &nb in &neighbors {
                            let back_neighbors = nb.neighbors(n);
                            assert!(
                                back_neighbors.contains(&cell),
                                "Symmetry broken at n={}: cell {:?} has neighbor {:?}, but {:?}'s neighbors are {:?}",
                                n,
                                cell,
                                nb,
                                nb,
                                back_neighbors
                            );
                        }
                    }
                }
            }
        }
    }

    proptest::proptest! {
        #[test]
        fn test_cube_sphere_neighbors_proptest_invariants(
            face in 0u8..6u8,
            n in 4u16..64u16,
            x_ratio in 0.0f32..1.0f32,
            y_ratio in 0.0f32..1.0f32,
        ) {
            let x = ((n as f32 - 1.0) * x_ratio) as u16;
            let y = ((n as f32 - 1.0) * y_ratio) as u16;
            let cell = CellId::new(face, x, y);
            let neighbors = cell.neighbors(n);

            let set: HashSet<_> = neighbors.iter().copied().collect();
            proptest::prop_assert_eq!(set.len(), 4);

            for &nb in &neighbors {
                let back_neighbors = nb.neighbors(n);
                proptest::prop_assert!(back_neighbors.contains(&cell));
            }
        }
    }
}
