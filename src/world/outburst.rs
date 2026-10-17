use crate::agent::progression::Rank;
use crate::world::danger::DangerField;
use crate::world::dungeon::{Dungeon, DungeonState};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Roaming surface monster spawned during a catastrophic dungeon outburst
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutbreakMob {
    pub id: usize,
    pub dungeon_id: usize,
    pub name: String,
    pub rank: Rank,
    pub x: usize,
    pub y: usize,
    pub hp: f32,
    pub max_hp: f32,
    pub attack_power: f32,
    pub defense: f32,
    pub speed: f32,
    pub is_rage_mode: bool,
    pub kills: u32,
}

impl OutbreakMob {
    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.hp = (self.hp - amount).max(0.0);
    }

    /// Moves one step toward the designated target coordinates across the toroidal or bounded grid
    pub fn step_toward(&mut self, target_x: usize, target_y: usize, width: usize, height: usize) {
        let mut new_x = self.x as isize;
        let mut new_y = self.y as isize;

        if (target_x as isize) > new_x {
            new_x += 1;
        } else if (target_x as isize) < new_x {
            new_x -= 1;
        }

        if (target_y as isize) > new_y {
            new_y += 1;
        } else if (target_y as isize) < new_y {
            new_y -= 1;
        }

        self.x = (new_x.max(0) as usize).min(width.saturating_sub(1));
        self.y = (new_y.max(0) as usize).min(height.saturating_sub(1));
    }
}

/// A coordinated surface invasion horde erupting from an uncontained dungeon
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutburstHorde {
    pub dungeon_id: usize,
    pub rank: Rank,
    pub origin_x: usize,
    pub origin_y: usize,
    pub mobs: Vec<OutbreakMob>,
    pub ticks_active: u32,
    pub is_rage_mode: bool,
    pub is_contained: bool,
    pub casualties_inflicted: u32,
    pub food_pillaged: u32,
}

impl OutburstHorde {
    pub fn new(dungeon: &Dungeon, next_mob_id: &mut usize, rng: &mut impl Rng) -> Self {
        let rank_mult = dungeon.rank.stat_multiplier();
        let mob_count = (3 + (rank_mult * 0.75) as usize).clamp(3, 16);

        let mut mobs = Vec::new();
        for i in 0..mob_count {
            *next_mob_id += 1;
            let offset_x = rng.gen_range(-2..=2);
            let offset_y = rng.gen_range(-2..=2);
            let mob_x = (dungeon.x as isize + offset_x).max(0) as usize;
            let mob_y = (dungeon.y as isize + offset_y).max(0) as usize;

            let is_alpha = i == 0;
            let name = if is_alpha {
                format!("{} Alpha Behemoth", dungeon.rank.display_name())
            } else {
                format!("{} Rupture Fiend #{}", dungeon.rank.display_name(), i)
            };

            let hp = if is_alpha {
                250.0 * rank_mult
            } else {
                80.0 * rank_mult
            };
            let atk = if is_alpha {
                35.0 * rank_mult
            } else {
                16.0 * rank_mult
            };
            let def = if is_alpha {
                14.0 * rank_mult
            } else {
                6.0 * rank_mult
            };

            mobs.push(OutbreakMob {
                id: *next_mob_id,
                dungeon_id: dungeon.id,
                name,
                rank: dungeon.rank,
                x: mob_x,
                y: mob_y,
                hp,
                max_hp: hp,
                attack_power: atk,
                defense: def,
                speed: 10.0 + rank_mult * 1.5,
                is_rage_mode: true, // Always spawns in rage mode
                kills: 0,
            });
        }

        Self {
            dungeon_id: dungeon.id,
            rank: dungeon.rank,
            origin_x: dungeon.x,
            origin_y: dungeon.y,
            mobs,
            ticks_active: 0,
            is_rage_mode: true,
            is_contained: false,
            casualties_inflicted: 0,
            food_pillaged: 0,
        }
    }

    pub fn is_fully_defeated(&self) -> bool {
        self.mobs.iter().all(|m| !m.is_alive())
    }

    pub fn alive_mob_count(&self) -> usize {
        self.mobs.iter().filter(|m| m.is_alive()).count()
    }

    /// Advances the horde toward the designated high-population civilian settlement.
    /// In rage mode, monsters relentlessly advance and attack infrastructure.
    pub fn advance_swarm(
        &mut self,
        target_settlement: Option<(usize, usize)>,
        width: usize,
        height: usize,
    ) {
        self.ticks_active += 1;
        if let Some((tgt_x, tgt_y)) = target_settlement {
            for mob in &mut self.mobs {
                if mob.is_alive() {
                    mob.step_toward(tgt_x, tgt_y, width, height);
                }
            }
        }
    }
}

/// Central manager overseeing subterranean mana swells and surface outburst incursions
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DungeonOutburstSystem {
    pub active_hordes: Vec<OutburstHorde>,
    pub next_mob_id: usize,
}

impl DungeonOutburstSystem {
    pub fn new() -> Self {
        Self {
            active_hordes: Vec::new(),
            next_mob_id: 1,
        }
    }

    /// Ticks dungeon mana swell and creates an Outburst horde when threshold is breached
    pub fn tick(
        &mut self,
        dungeons: &mut [Dungeon],
        danger_field: &mut DangerField,
        settlements: &[(usize, usize, usize)], // (x, y, population)
        width: usize,
        height: usize,
        rng: &mut impl Rng,
    ) -> Vec<usize> {
        let mut newly_outburst_dungeon_ids = Vec::new();

        // 1. Tick dungeon mana swells
        for dungeon in dungeons.iter_mut() {
            if dungeon.state == DungeonState::Active {
                let broke = dungeon.tick_mana_swell(0.10);
                if broke {
                    newly_outburst_dungeon_ids.push(dungeon.id);
                }
            } else if dungeon.state == DungeonState::Outburst
                && !self
                    .active_hordes
                    .iter()
                    .any(|h| h.dungeon_id == dungeon.id && !h.is_contained)
            {
                newly_outburst_dungeon_ids.push(dungeon.id);
            }
        }

        // 2. Spawn hordes for any newly outburst dungeons
        for &d_id in &newly_outburst_dungeon_ids {
            if let Some(dungeon) = dungeons.iter().find(|d| d.id == d_id) {
                let horde = OutburstHorde::new(dungeon, &mut self.next_mob_id, rng);
                // Inject catastrophic hazard in a radius of 2 around dungeon entrance
                for dx in -2..=2 {
                    for dy in -2..=2 {
                        let hx = (dungeon.x as isize + dx).max(0) as usize;
                        let hy = (dungeon.y as isize + dy).max(0) as usize;
                        if hx < width && hy < height {
                            danger_field.inject_hazard(hx, hy, 90.0);
                        }
                    }
                }
                self.active_hordes.push(horde);
            }
        }

        // 3. Advance active hordes toward the nearest densely populated settlement
        for horde in &mut self.active_hordes {
            if horde.is_contained || horde.is_fully_defeated() {
                continue;
            }

            // Find closest settlement by Manhattan distance, weighted by population
            let target_town = settlements
                .iter()
                .min_by_key(|(sx, sy, pop)| {
                    let dist = (horde.origin_x as isize - *sx as isize).abs()
                        + (horde.origin_y as isize - *sy as isize).abs();
                    // Prefer higher population settlements as prime biological targets
                    dist * 100 / (*pop).max(1) as isize
                })
                .map(|(sx, sy, _)| (*sx, *sy));

            horde.advance_swarm(target_town, width, height);

            // Corrupt tiles under living horde mobs
            for mob in &horde.mobs {
                if mob.is_alive() && mob.x < width && mob.y < height {
                    danger_field.inject_hazard(mob.x, mob.y, 45.0);
                }
            }
        }

        // 4. Resolve defeated hordes
        for horde in &mut self.active_hordes {
            if !horde.is_contained && horde.is_fully_defeated() {
                horde.is_contained = true;
                if let Some(dungeon) = dungeons.iter_mut().find(|d| d.id == horde.dungeon_id) {
                    dungeon.resolve_outburst();
                }
            }
        }

        newly_outburst_dungeon_ids
    }
}
