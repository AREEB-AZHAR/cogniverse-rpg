use crate::economy::Commodity;
use crate::world::tile::{BiomeType, Tile};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VesselKind {
    FishingBoat,
    CargoCog,
    Galleon,
}

impl VesselKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            VesselKind::FishingBoat => "Coastal Fishing Boat",
            VesselKind::CargoCog => "Merchant Cargo Cog",
            VesselKind::Galleon => "Heavy Trade Galleon",
        }
    }

    pub fn cargo_capacity(&self) -> u32 {
        match self {
            VesselKind::FishingBoat => 40,
            VesselKind::CargoCog => 150,
            VesselKind::Galleon => 400,
        }
    }

    pub fn max_durability(&self) -> f32 {
        match self {
            VesselKind::FishingBoat => 60.0,
            VesselKind::CargoCog => 150.0,
            VesselKind::Galleon => 300.0,
        }
    }

    pub fn speed_tiles_per_tick(&self) -> f32 {
        match self {
            VesselKind::FishingBoat => 1.2,
            VesselKind::CargoCog => 0.9,
            VesselKind::Galleon => 0.7,
        }
    }

    pub fn required_materials(&self) -> (u32, u32, u32) {
        // (Wood, WoolCloth, IronIngot)
        match self {
            VesselKind::FishingBoat => (8, 2, 0),
            VesselKind::CargoCog => (20, 6, 4),
            VesselKind::Galleon => (45, 15, 12),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipStatus {
    AtPort {
        dock_id: usize,
    },
    Loading {
        dock_id: usize,
        progress: u32,
        target: u32,
    },
    Unloading {
        dock_id: usize,
        progress: u32,
        target: u32,
    },
    EnRoute {
        destination_dock: usize,
        target_x: usize,
        target_y: usize,
    },
    Fishing {
        origin_dock: usize,
        ticks_remaining: u32,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShipEntity {
    pub id: usize,
    pub name: String,
    pub kind: VesselKind,
    pub x: usize,
    pub y: usize,
    pub home_dock: usize,
    pub status: ShipStatus,
    pub cargo: HashMap<Commodity, u32>,
    pub durability: f32,
    pub max_durability: f32,
    pub path: Vec<(usize, usize)>,
}

impl ShipEntity {
    pub fn new(
        id: usize,
        name: impl Into<String>,
        kind: VesselKind,
        x: usize,
        y: usize,
        home_dock: usize,
    ) -> Self {
        let max_hp = kind.max_durability();
        Self {
            id,
            name: name.into(),
            kind,
            x,
            y,
            home_dock,
            status: ShipStatus::AtPort { dock_id: home_dock },
            cargo: HashMap::new(),
            durability: max_hp,
            max_durability: max_hp,
            path: Vec::new(),
        }
    }

    pub fn cargo_count(&self) -> u32 {
        self.cargo.values().sum()
    }

    pub fn available_capacity(&self) -> u32 {
        self.kind
            .cargo_capacity()
            .saturating_sub(self.cargo_count())
    }

    pub fn load_cargo(&mut self, commodity: Commodity, amount: u32) -> u32 {
        let transferable = amount.min(self.available_capacity());
        if transferable > 0 {
            *self.cargo.entry(commodity).or_insert(0) += transferable;
        }
        transferable
    }

    pub fn unload_cargo(&mut self, commodity: Commodity, amount: u32) -> u32 {
        if let Some(current) = self.cargo.get_mut(&commodity) {
            let unloaded = amount.min(*current);
            *current -= unloaded;
            if *current == 0 {
                self.cargo.remove(&commodity);
            }
            unloaded
        } else {
            0
        }
    }

    pub fn step_along_path(&mut self) -> bool {
        if let Some((next_x, next_y)) = self.path.pop() {
            self.x = next_x;
            self.y = next_y;
            true
        } else {
            false
        }
    }
}

/// Checks if a tile is navigable water for maritime vessels
pub fn is_navigable_water(tile: &Tile) -> bool {
    tile.elevation_layer.is_water()
        || matches!(
            tile.biome,
            BiomeType::DeepOcean
                | BiomeType::CloseOcean
                | BiomeType::CrystalRiverlands
                | BiomeType::Swamp
        )
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PathNode {
    cost: usize,
    pos: (usize, usize),
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A* maritime pathfinding strictly traversing navigable water tiles
pub fn find_water_path(
    start: (usize, usize),
    goal: (usize, usize),
    tiles: &[Tile],
    width: usize,
    height: usize,
) -> Option<Vec<(usize, usize)>> {
    if start == goal {
        return Some(Vec::new());
    }

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();
    let mut closed_set: HashSet<(usize, usize)> = HashSet::new();

    g_score.insert(start, 0);
    let h_start = (start.0 as isize - goal.0 as isize).unsigned_abs()
        + (start.1 as isize - goal.1 as isize).unsigned_abs();
    open_set.push(PathNode {
        cost: h_start,
        pos: start,
    });

    while let Some(PathNode { pos, .. }) = open_set.pop() {
        if pos == goal {
            // Reconstruct path in reverse (so pop() steps forward)
            let mut path = Vec::new();
            let mut curr = goal;
            while let Some(&prev) = came_from.get(&curr) {
                path.push(curr);
                curr = prev;
            }
            return Some(path);
        }

        if !closed_set.insert(pos) {
            continue;
        }

        let curr_g = *g_score.get(&pos).unwrap_or(&usize::MAX);

        // 4-cardinal neighbors
        let neighbors = [
            (pos.0.wrapping_sub(1), pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1.wrapping_sub(1)),
            (pos.0, pos.1 + 1),
        ];

        for (nx, ny) in neighbors {
            if nx >= width || ny >= height {
                continue;
            }

            let next_pos = (nx, ny);
            if closed_set.contains(&next_pos) {
                continue;
            }

            let idx = ny * width + nx;
            let tile = &tiles[idx];

            // Target tile is valid if it is navigable water, or if it is the destination goal
            let is_passable = is_navigable_water(tile) || next_pos == goal;
            if !is_passable {
                continue;
            }

            let tentative_g = curr_g + 1;
            let best_g = *g_score.get(&next_pos).unwrap_or(&usize::MAX);

            if tentative_g < best_g {
                came_from.insert(next_pos, pos);
                g_score.insert(next_pos, tentative_g);
                let h = (nx as isize - goal.0 as isize).unsigned_abs()
                    + (ny as isize - goal.1 as isize).unsigned_abs();
                open_set.push(PathNode {
                    cost: tentative_g + h,
                    pos: next_pos,
                });
            }
        }
    }

    None
}

/// Constructs and launches a vessel from a BoatYard into the nearest adjacent navigable water tile
#[allow(clippy::too_many_arguments)]
pub fn launch_ship_from_boat_yard(
    boat_yard_x: usize,
    boat_yard_y: usize,
    kind: VesselKind,
    name: impl Into<String>,
    tiles: &[Tile],
    width: usize,
    height: usize,
    home_dock: usize,
    next_ship_id: usize,
) -> Result<ShipEntity, &'static str> {
    // Search adjacent tiles (3x3 ring) for navigable water
    let min_x = boat_yard_x.saturating_sub(1);
    let max_x = (boat_yard_x + 1).min(width - 1);
    let min_y = boat_yard_y.saturating_sub(1);
    let max_y = (boat_yard_y + 1).min(height - 1);

    let mut spawn_pos = None;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if x == boat_yard_x && y == boat_yard_y {
                continue;
            }
            let idx = y * width + x;
            if is_navigable_water(&tiles[idx]) {
                spawn_pos = Some((x, y));
                break;
            }
        }
        if spawn_pos.is_some() {
            break;
        }
    }

    let (sx, sy) = spawn_pos
        .ok_or("No adjacent navigable water tile found around boat yard to launch vessel")?;
    Ok(ShipEntity::new(next_ship_id, name, kind, sx, sy, home_dock))
}
