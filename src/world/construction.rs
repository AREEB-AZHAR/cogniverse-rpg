use super::tile::{PlacedStructure, StructureKind, Tile};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConstructionStage {
    SiteClearing,
    MaterialHauling,
    Framing,
    Completed,
}

impl ConstructionStage {
    pub fn display_name(&self) -> &'static str {
        match self {
            ConstructionStage::SiteClearing => "Site Clearing & Groundwork",
            ConstructionStage::MaterialHauling => "Material Hauling",
            ConstructionStage::Framing => "Structural Framing",
            ConstructionStage::Completed => "Completed Structure",
        }
    }
}

/// A physical construction site blueprint waiting for materials and labor
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Blueprint {
    pub id: usize,
    pub kind: StructureKind,
    pub x: usize,
    pub y: usize,
    pub stage: ConstructionStage,
    pub required_wood: u32,
    pub required_stone: u32,
    pub delivered_wood: u32,
    pub delivered_stone: u32,
    pub progress_ticks: u32,
    pub total_labor_ticks: u32,
    pub is_complete: bool,
    pub claimed_by: Option<usize>,
}

impl Blueprint {
    pub fn new(id: usize, kind: StructureKind, x: usize, y: usize) -> Self {
        let (req_wood, req_stone, labor) = match kind {
            StructureKind::WoodCottage => (10, 0, 30),
            StructureKind::StoneCabin => (6, 12, 50),
            StructureKind::Granary => (15, 5, 40),
            StructureKind::Dock => (12, 4, 35),
            StructureKind::BoatYard => (20, 8, 60),
            StructureKind::MineShaft => (10, 15, 60),
            StructureKind::TownHall => (30, 40, 120),
            StructureKind::Barracks => (20, 20, 80),
            StructureKind::Watchtower => (12, 10, 45),
            StructureKind::StoneWall => (0, 8, 20),
            StructureKind::Fence => (4, 0, 10),
            StructureKind::AnimalPen => (8, 4, 25),
            StructureKind::Barn => (18, 12, 55),
            _ => (5, 5, 20),
        };

        Self {
            id,
            kind,
            x,
            y,
            stage: ConstructionStage::SiteClearing,
            required_wood: req_wood,
            required_stone: req_stone,
            delivered_wood: 0,
            delivered_stone: 0,
            progress_ticks: 0,
            total_labor_ticks: labor,
            is_complete: false,
            claimed_by: None,
        }
    }

    pub fn needs_materials(&self) -> bool {
        self.delivered_wood < self.required_wood || self.delivered_stone < self.required_stone
    }

    pub fn deliver_materials(&mut self, wood: u32, stone: u32) {
        self.delivered_wood = (self.delivered_wood + wood).min(self.required_wood);
        self.delivered_stone = (self.delivered_stone + stone).min(self.required_stone);

        if !self.needs_materials() && self.stage == ConstructionStage::MaterialHauling {
            self.stage = ConstructionStage::Framing;
        }
    }

    pub fn apply_labor(&mut self, labor: u32, tiles: &mut [Tile], width: usize) -> bool {
        if self.is_complete {
            return true;
        }

        self.progress_ticks += labor;

        // Advance stages based on progress
        if self.stage == ConstructionStage::SiteClearing && self.progress_ticks >= 10 {
            self.stage = if self.needs_materials() {
                ConstructionStage::MaterialHauling
            } else {
                ConstructionStage::Framing
            };
        }

        if self.stage == ConstructionStage::Framing
            && self.progress_ticks >= self.total_labor_ticks
            && !self.needs_materials()
        {
            self.stage = ConstructionStage::Completed;
            self.is_complete = true;

            // Spawn finished structure onto the world grid
            let idx = self.y * width + self.x;
            if idx < tiles.len() {
                tiles[idx].structure = Some(PlacedStructure::new(self.kind));
            }
            return true;
        }

        false
    }
}

/// Decentralized Construction Job Board managing community building projects
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct JobBoard {
    pub blueprints: Vec<Blueprint>,
    pub next_blueprint_id: usize,
}

impl JobBoard {
    pub fn new() -> Self {
        Self {
            blueprints: Vec::new(),
            next_blueprint_id: 1,
        }
    }

    pub fn post_blueprint(&mut self, kind: StructureKind, x: usize, y: usize) -> usize {
        let id = self.next_blueprint_id;
        self.next_blueprint_id += 1;
        let bp = Blueprint::new(id, kind, x, y);
        self.blueprints.push(bp);
        id
    }

    pub fn claim_job(&mut self, agent_id: usize) -> Option<&mut Blueprint> {
        self.blueprints.iter_mut().find(|bp| {
            !bp.is_complete && (bp.claimed_by.is_none() || bp.claimed_by == Some(agent_id))
        })
    }

    pub fn release_job(&mut self, blueprint_id: usize, agent_id: usize) {
        if let Some(bp) = self.blueprints.iter_mut().find(|b| b.id == blueprint_id) {
            if bp.claimed_by == Some(agent_id) {
                bp.claimed_by = None;
            }
        }
    }

    pub fn has_open_jobs(&self) -> bool {
        self.blueprints.iter().any(|b| !b.is_complete)
    }

    pub fn get_blueprint(&self, id: usize) -> Option<&Blueprint> {
        self.blueprints.iter().find(|b| b.id == id)
    }

    pub fn get_blueprint_mut(&mut self, id: usize) -> Option<&mut Blueprint> {
        self.blueprints.iter_mut().find(|b| b.id == id)
    }

    pub fn cleanup_completed(&mut self) {
        self.blueprints.retain(|b| !b.is_complete);
    }
}
