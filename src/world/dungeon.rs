use crate::agent::progression::Rank;
use crate::economy::loot::ManaStoneGrade;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

/// Current operational state of a procedural dungeon
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DungeonState {
    Undiscovered,
    Active,
    Cleared,
    Collapsed,
    Outburst,
}

/// Specialized chamber archetype within a dungeon floor
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomKind {
    EntryHall,
    MobChamber,
    TreasureVault,
    Shrine,
    BossLair,
}

impl RoomKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            RoomKind::EntryHall => "Entry Hall",
            RoomKind::MobChamber => "Monster Chamber",
            RoomKind::TreasureVault => "Treasure Vault",
            RoomKind::Shrine => "Ancient Restorative Shrine",
            RoomKind::BossLair => "Boss Lair",
        }
    }
}

/// A monster inhabiting a dungeon chamber
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DungeonMonster {
    pub name: String,
    pub rank: Rank,
    pub hp: f32,
    pub max_hp: f32,
    pub attack_power: f32,
    pub defense: f32,
    pub speed: f32,
    pub is_boss: bool,
    pub stone_grade: ManaStoneGrade,
}

impl DungeonMonster {
    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }

    pub fn take_damage(&mut self, dmg: f32) {
        self.hp = (self.hp - dmg).max(0.0);
    }
}

/// A single chamber within a dungeon floor
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DungeonRoom {
    pub room_index: usize,
    pub kind: RoomKind,
    pub monsters: Vec<DungeonMonster>,
    pub cleared: bool,
    pub treasure_claimed: bool,
}

impl DungeonRoom {
    pub fn is_combat_resolved(&self) -> bool {
        self.monsters.iter().all(|m| !m.is_alive())
    }
}

/// A full depth level consisting of sequentially linked chambers
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DungeonFloor {
    pub floor_number: usize,
    pub rooms: Vec<DungeonRoom>,
    pub cleared: bool,
}

impl DungeonFloor {
    pub fn is_fully_cleared(&self) -> bool {
        self.rooms.iter().all(|r| r.cleared)
    }
}

/// Procedural dungeon instance rooted on the world map
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dungeon {
    pub id: usize,
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub rank: Rank,
    pub floors: Vec<DungeonFloor>,
    pub state: DungeonState,
    pub mana_density: f32,
    pub respawn_timer: u32,
    pub discovery_tick: u64,
    pub mana_swell: f32,
    pub outburst_threshold: f32,
    pub outburst_count: u32,
}

impl Dungeon {
    /// Checks whether all floors including the apex boss have been cleared
    pub fn is_dungeon_cleared(&self) -> bool {
        self.floors.iter().all(|f| f.is_fully_cleared())
    }

    /// Resets monsters and treasure after respawn cooldown expires
    pub fn respawn(&mut self, seed: u64) {
        let fresh = generate_dungeon(self.id, &self.name, self.x, self.y, self.rank, seed);
        self.floors = fresh.floors;
        self.state = DungeonState::Active;
        self.respawn_timer = 0;
        self.mana_swell = 0.0;
    }

    /// Ticks the subterranean mana swell accumulator.
    /// Triggers `DungeonState::Outburst` when critical threshold is breached.
    pub fn tick_mana_swell(&mut self, rate: f32) -> bool {
        if self.state == DungeonState::Active {
            self.mana_swell += rate * self.rank.stat_multiplier();
            if self.mana_swell >= self.outburst_threshold {
                self.state = DungeonState::Outburst;
                self.outburst_count += 1;
                return true;
            }
        }
        false
    }

    /// Concludes an outburst event, returning the dungeon to Cleared status and resetting mana swell
    pub fn resolve_outburst(&mut self) {
        self.state = DungeonState::Cleared;
        self.mana_swell = 0.0;
        self.respawn_timer = 0;
    }
}

/// Procedural dungeon generator scaling depth, rooms, monster packs, and bosses by rank
pub fn generate_dungeon(
    id: usize,
    name: &str,
    x: usize,
    y: usize,
    rank: Rank,
    seed: u64,
) -> Dungeon {
    let mut rng = StdRng::seed_from_u64(seed.wrapping_add(id as u64 * 31337));

    let floor_count = match rank {
        Rank::F => 1,
        Rank::E => 2,
        Rank::D => 2,
        Rank::C => 3,
        Rank::B => 4,
        Rank::A => 5,
        Rank::S => 6,
        Rank::SPlus => 7,
        Rank::SSS => 8,
    };

    let mana_density = (rank.stat_multiplier() * 25.0).clamp(10.0, 150.0);
    let mut floors = Vec::with_capacity(floor_count);

    for f_idx in 0..floor_count {
        let floor_num = f_idx + 1;
        let is_final_floor = floor_num == floor_count;

        let room_count = match rank {
            Rank::F => 3,
            Rank::E | Rank::D => 3,
            Rank::C | Rank::B => 4,
            Rank::A => 4,
            Rank::S => 5,
            Rank::SPlus => 5,
            Rank::SSS => 6,
        };

        let mut rooms = Vec::with_capacity(room_count);

        for r_idx in 0..room_count {
            let is_first_room = r_idx == 0;
            let is_last_room = r_idx == room_count - 1;

            let kind = if is_first_room {
                RoomKind::EntryHall
            } else if is_last_room && is_final_floor {
                RoomKind::BossLair
            } else if is_last_room {
                if rng.gen_bool(0.5) {
                    RoomKind::TreasureVault
                } else {
                    RoomKind::MobChamber
                }
            } else if r_idx == 1 && room_count >= 4 && rng.gen_bool(0.4) {
                RoomKind::Shrine
            } else if rng.gen_bool(0.2) {
                RoomKind::TreasureVault
            } else {
                RoomKind::MobChamber
            };

            let monsters = match kind {
                RoomKind::EntryHall | RoomKind::Shrine => Vec::new(),
                RoomKind::MobChamber => {
                    let pack_size = rng.gen_range(2..=4);
                    (0..pack_size)
                        .map(|_| generate_mob(rank, floor_num, false, &mut rng))
                        .collect()
                }
                RoomKind::TreasureVault => {
                    let guard_count = rng.gen_range(1..=2);
                    (0..guard_count)
                        .map(|_| generate_mob(rank, floor_num, false, &mut rng))
                        .collect()
                }
                RoomKind::BossLair => {
                    let mut pack = Vec::new();
                    // Boss monster
                    pack.push(generate_mob(rank, floor_num, true, &mut rng));
                    // Optional elite minions
                    if rank >= Rank::B {
                        pack.push(generate_mob(rank, floor_num, false, &mut rng));
                    }
                    pack
                }
            };

            let cleared = monsters.is_empty();

            rooms.push(DungeonRoom {
                room_index: r_idx,
                kind,
                monsters,
                cleared,
                treasure_claimed: false,
            });
        }

        let floor_cleared = rooms.iter().all(|r| r.cleared);
        floors.push(DungeonFloor {
            floor_number: floor_num,
            rooms,
            cleared: floor_cleared,
        });
    }

    Dungeon {
        id,
        name: name.to_string(),
        x,
        y,
        rank,
        floors,
        state: DungeonState::Active,
        mana_density,
        respawn_timer: 0,
        discovery_tick: 0,
        mana_swell: 0.0,
        outburst_threshold: 100.0,
        outburst_count: 0,
    }
}

/// Generates a single dungeon mob or climax boss scaled by dungeon rank and floor depth
fn generate_mob(rank: Rank, floor: usize, is_boss: bool, rng: &mut impl Rng) -> DungeonMonster {
    let rank_mult = rank.stat_multiplier();
    let floor_mult = 1.0 + 0.12 * (floor as f32);

    let (base_hp, base_atk, base_def, base_spd) = if is_boss {
        (
            150.0 * rank_mult * floor_mult,
            24.0 * rank_mult * floor_mult,
            12.0 * rank_mult,
            10.0 + (rank as u32 as f32) * 1.2,
        )
    } else {
        (
            45.0 * rank_mult * floor_mult,
            10.0 * rank_mult * floor_mult,
            4.0 * rank_mult,
            7.0 + (rank as u32 as f32) * 0.8,
        )
    };

    let name = if is_boss {
        boss_name_for_rank(rank)
    } else {
        mob_name_for_rank(rank, rng)
    };

    let stone_grade = match rank {
        Rank::F => ManaStoneGrade::Flawed,
        Rank::E | Rank::D => ManaStoneGrade::Regular,
        Rank::C | Rank::B => ManaStoneGrade::Flawless,
        Rank::A | Rank::S => ManaStoneGrade::Radiant,
        Rank::SPlus | Rank::SSS => ManaStoneGrade::Astral,
    };

    DungeonMonster {
        name,
        rank,
        hp: base_hp,
        max_hp: base_hp,
        attack_power: base_atk,
        defense: base_def,
        speed: base_spd,
        is_boss,
        stone_grade,
    }
}

fn mob_name_for_rank(rank: Rank, rng: &mut impl Rng) -> String {
    let names: &[&str] = match rank {
        Rank::F => &["Cave Goblin", "Giant Sewer Rat", "Skeletal Scout"],
        Rank::E => &["Shadow Wolf", "Kobold Skirmisher", "Tomb Zombie"],
        Rank::D => &["Hobgoblin Brute", "Cave Troll", "Blight Spider"],
        Rank::C => &["Minotaur Warrior", "Gargoyle Sentry", "Corrupted Treant"],
        Rank::B => &["Frost Drake Whelp", "Abyssal Lurker", "Iron Automaton"],
        Rank::A => &["Infernal Chimera", "Lich Acolyte", "Wyvern Stalker"],
        Rank::S => &["Void Fiend", "Eldritch Behemoth", "Dragonkin Berserker"],
        Rank::SPlus => &[
            "Titan Fragment",
            "Nether Sovereign Minion",
            "Worldbreaker Wurm",
        ],
        Rank::SSS => &[
            "Chaos God Aspect",
            "Transcendent Leviathan",
            "Primordial Remnant",
        ],
    };
    names[rng.gen_range(0..names.len())].to_string()
}

fn boss_name_for_rank(rank: Rank) -> String {
    match rank {
        Rank::F => "Goblin Chieftain Grashnak".to_string(),
        Rank::E => "Grave Lord Malvok".to_string(),
        Rank::D => "Broodmother Arachna".to_string(),
        Rank::C => "Minotaur Bloodrager Taurus".to_string(),
        Rank::B => "Forgemaster Automaton Aegis-9".to_string(),
        Rank::A => "Elder Lich Malakor the Undying".to_string(),
        Rank::S => "Ancient Storm Dragon Valstrath".to_string(),
        Rank::SPlus => "Astral Dreadnought Xyphos".to_string(),
        Rank::SSS => "Primordial Void Sovereign Azathoth".to_string(),
    }
}
