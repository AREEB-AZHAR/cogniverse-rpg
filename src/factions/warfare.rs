use crate::agent::CivilizationRace;
use crate::world::tile::{PlacedStructure, Tile};
use serde::{Deserialize, Serialize};

/// Specialized artillery machine used to breach enemy fortifications
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SiegeEngineKind {
    Catapult,
    HeavyBallista,
    BatteringRam,
}

impl SiegeEngineKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            SiegeEngineKind::Catapult => "☄️ Heavy Timber Catapult",
            SiegeEngineKind::HeavyBallista => "🏹 Iron-Bolt Ballista",
            SiegeEngineKind::BatteringRam => "🪵 Iron-Headed Battering Ram",
        }
    }

    pub fn base_structural_damage(&self) -> f32 {
        match self {
            SiegeEngineKind::Catapult => 85.0,
            SiegeEngineKind::HeavyBallista => 55.0,
            SiegeEngineKind::BatteringRam => 140.0,
        }
    }

    pub fn range(&self) -> usize {
        match self {
            SiegeEngineKind::Catapult => 6,
            SiegeEngineKind::HeavyBallista => 8,
            SiegeEngineKind::BatteringRam => 1,
        }
    }
}

/// An active siege artillery engine deployed in the theater of war
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiegeEngineEntity {
    pub id: usize,
    pub kind: SiegeEngineKind,
    pub faction: CivilizationRace,
    pub x: usize,
    pub y: usize,
    pub target_x: usize,
    pub target_y: usize,
    pub health: f32,
    pub max_health: f32,
    pub is_operational: bool,
}

impl SiegeEngineEntity {
    pub fn step_toward(&mut self, width: usize, height: usize) {
        if !self.is_operational {
            return;
        }

        let mut new_x = self.x as isize;
        let mut new_y = self.y as isize;

        if (self.target_x as isize) > new_x {
            new_x += 1;
        } else if (self.target_x as isize) < new_x {
            new_x -= 1;
        }

        if (self.target_y as isize) > new_y {
            new_y += 1;
        } else if (self.target_y as isize) < new_y {
            new_y -= 1;
        }

        self.x = (new_x.max(0) as usize).min(width.saturating_sub(1));
        self.y = (new_y.max(0) as usize).min(height.saturating_sub(1));
    }

    /// Bombards an enemy fortification within siege range
    pub fn bombard_structure(&self, structure: &mut PlacedStructure) -> (f32, bool) {
        if !self.is_operational {
            return (0.0, false);
        }

        let dist = (self.x as isize - self.target_x as isize).abs()
            + (self.y as isize - self.target_y as isize).abs();
        if dist as usize <= self.kind.range() {
            let dmg = self.kind.base_structural_damage();
            structure.health = (structure.health - dmg).max(0.0);
            let destroyed = structure.health <= 0.0;
            return (dmg, destroyed);
        }

        (0.0, false)
    }
}

/// Dynamic territorial sovereignty influence matrix across the realm
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerritoryGrid {
    pub width: usize,
    pub height: usize,
    pub terran_influence: Vec<f32>, // 0.0 (Pure Lunarian) to 1.0 (Pure Terran)
}

impl TerritoryGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut terran_influence = vec![0.5; size];

        // Seed initial western/eastern territorial alignment
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                if x < width / 2 {
                    terran_influence[idx] = 0.85; // Solaria western home provinces
                } else {
                    terran_influence[idx] = 0.15; // Lunar eastern home provinces
                }
            }
        }

        Self {
            width,
            height,
            terran_influence,
        }
    }

    pub fn get_owner(&self, x: usize, y: usize) -> Option<CivilizationRace> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let val = self.terran_influence[y * self.width + x];
        if val > 0.60 {
            Some(CivilizationRace::SolarTerran)
        } else if val < 0.40 {
            Some(CivilizationRace::ChthonicLunarian)
        } else {
            None // Contested no-man's land
        }
    }

    pub fn shift_influence(&mut self, x: usize, y: usize, faction: CivilizationRace, amount: f32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = y * self.width + x;
        match faction {
            CivilizationRace::SolarTerran => {
                self.terran_influence[idx] = (self.terran_influence[idx] + amount).min(1.0);
            }
            CivilizationRace::ChthonicLunarian => {
                self.terran_influence[idx] = (self.terran_influence[idx] - amount).max(0.0);
            }
        }
    }
}

/// The state of an active inter-realm military campaign
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WarCampaign {
    pub is_active: bool,
    pub aggressor: CivilizationRace,
    pub defender: CivilizationRace,
    pub sieges_won: u32,
    pub gold_plundered: u32,
    pub engines: Vec<SiegeEngineEntity>,
    pub next_engine_id: usize,
    pub territory: TerritoryGrid,
}

impl WarCampaign {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            is_active: false,
            aggressor: CivilizationRace::SolarTerran,
            defender: CivilizationRace::ChthonicLunarian,
            sieges_won: 0,
            gold_plundered: 0,
            engines: Vec::new(),
            next_engine_id: 1,
            territory: TerritoryGrid::new(width, height),
        }
    }

    pub fn declare_war(&mut self, aggressor: CivilizationRace, defender: CivilizationRace) {
        self.is_active = true;
        self.aggressor = aggressor;
        self.defender = defender;
    }

    pub fn build_siege_engine(
        &mut self,
        kind: SiegeEngineKind,
        faction: CivilizationRace,
        x: usize,
        y: usize,
        target_x: usize,
        target_y: usize,
    ) -> usize {
        let id = self.next_engine_id;
        self.next_engine_id += 1;

        let max_hp = match kind {
            SiegeEngineKind::Catapult => 120.0,
            SiegeEngineKind::HeavyBallista => 100.0,
            SiegeEngineKind::BatteringRam => 200.0,
        };

        self.engines.push(SiegeEngineEntity {
            id,
            kind,
            faction,
            x,
            y,
            target_x,
            target_y,
            health: max_hp,
            max_health: max_hp,
            is_operational: true,
        });

        id
    }

    /// Ticks siege artillery, bombardment, and fortress collapse
    pub fn tick(&mut self, tiles: &mut [Tile], width: usize, height: usize) -> Vec<String> {
        if !self.is_active {
            return Vec::new();
        }

        let mut events = Vec::new();

        for engine in &mut self.engines {
            if !engine.is_operational {
                continue;
            }

            let dist = (engine.x as isize - engine.target_x as isize).abs()
                + (engine.y as isize - engine.target_y as isize).abs();

            if dist as usize <= engine.kind.range() {
                // In range: bombard the target structure!
                let tgt_idx = engine.target_y * width + engine.target_x;
                if tgt_idx < tiles.len() {
                    if let Some(ref mut structure) = tiles[tgt_idx].structure {
                        let (dmg, destroyed) = engine.bombard_structure(structure);
                        if destroyed {
                            events.push(format!(
                                "💥 [FORTRESS COLLAPSED] {} shattered {} at ({}, {})!",
                                engine.kind.display_name(),
                                structure.kind.display_name(),
                                engine.target_x,
                                engine.target_y
                            ));
                            tiles[tgt_idx].structure = None;
                            self.sieges_won += 1;
                            self.gold_plundered += 250;
                            // Shift territorial influence toward the conquering faction
                            self.territory.shift_influence(
                                engine.target_x,
                                engine.target_y,
                                engine.faction,
                                0.45,
                            );
                        } else if dmg > 0.0 {
                            events.push(format!(
                                "🎯 [SIEGE BOMBARDMENT] {} struck fortification at ({}, {}) for {:.0} structural damage!",
                                engine.kind.display_name(),
                                engine.target_x,
                                engine.target_y,
                                dmg
                            ));
                        }
                    }
                }
            } else {
                // Move toward target
                engine.step_toward(width, height);
            }
        }

        events
    }
}
