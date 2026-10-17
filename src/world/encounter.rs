use crate::agent::progression::{Attributes, Rank};
use crate::economy::loot::{DungeonLootDrop, EquipmentItem, LootTable};
use crate::world::dungeon::{Dungeon, DungeonState, RoomKind};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Combatant party member participating in an off-screen dungeon expedition
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpeditionMember {
    pub agent_id: usize,
    pub name: String,
    pub rank: Rank,
    pub level: u32,
    pub hp: f32,
    pub max_hp: f32,
    pub attributes: Attributes,
    pub equipment: Vec<EquipmentItem>,
}

impl ExpeditionMember {
    pub fn new(
        agent_id: usize,
        name: impl Into<String>,
        rank: Rank,
        level: u32,
        attributes: Attributes,
    ) -> Self {
        let max_hp = 50.0 + attributes.vitality * 5.0 * rank.stat_multiplier();
        Self {
            agent_id,
            name: name.into(),
            rank,
            level,
            hp: max_hp,
            max_hp,
            attributes,
            equipment: Vec::new(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }

    /// Effective attack power derived from strength, rank multiplier, and weapon affixes
    pub fn effective_attack(&self) -> f32 {
        let base = self.attributes.strength * 1.5 * self.rank.stat_multiplier();
        let equip_bonus: f32 = self.equipment.iter().map(|e| e.total_attack()).sum();
        (base + equip_bonus).max(1.0)
    }

    /// Effective defense derived from vitality, rank multiplier, and armor affixes
    pub fn effective_defense(&self) -> f32 {
        let base = self.attributes.vitality * 0.8 * self.rank.stat_multiplier();
        let equip_bonus: f32 = self.equipment.iter().map(|e| e.total_defense()).sum();
        (base + equip_bonus).max(0.0)
    }

    /// Critical strike probability driven by luck
    pub fn crit_chance(&self) -> f32 {
        (0.05 + self.attributes.luck * 0.005).clamp(0.05, 0.60)
    }

    /// Restores health up to maximum capacity
    pub fn heal(&mut self, amount: f32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }
}

/// Operational state machine for an expedition party
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpeditionState {
    InTransit,
    Exploring,
    InCombat,
    Resting,
    Victorious,
    WipedOut,
    Retreated,
}

/// Active party expedition exploring an off-screen dungeon
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DungeonExpedition {
    pub id: usize,
    pub dungeon_id: usize,
    pub party: Vec<ExpeditionMember>,
    pub current_floor: usize, // 0-indexed
    pub current_room: usize,  // 0-indexed
    pub state: ExpeditionState,
    pub loot: DungeonLootDrop,
    pub exp_earned: u32,
    pub combat_log: Vec<String>,
    pub ticks_elapsed: u32,
}

impl DungeonExpedition {
    pub fn new(id: usize, dungeon_id: usize, party: Vec<ExpeditionMember>) -> Self {
        Self {
            id,
            dungeon_id,
            party,
            current_floor: 0,
            current_room: 0,
            state: ExpeditionState::Exploring,
            loot: DungeonLootDrop::default(),
            exp_earned: 0,
            combat_log: Vec::new(),
            ticks_elapsed: 0,
        }
    }

    pub fn is_party_alive(&self) -> bool {
        self.party.iter().any(|m| m.is_alive())
    }

    pub fn average_party_hp_ratio(&self) -> f32 {
        let (current, max) = self.party.iter().fold((0.0, 0.0), |(c, m), member| {
            (c + member.hp, m + member.max_hp)
        });
        if max <= 0.0 {
            0.0
        } else {
            current / max
        }
    }

    pub fn average_party_luck(&self) -> f32 {
        if self.party.is_empty() {
            return 10.0;
        }
        let total: f32 = self.party.iter().map(|m| m.attributes.luck).sum();
        total / (self.party.len() as f32)
    }
}

/// Outcome of a single off-screen encounter simulation tick
#[derive(Clone, Debug, PartialEq)]
pub enum StepResult {
    Continuing,
    RoomCleared { floor: usize, room: usize },
    FloorCleared { floor: usize },
    DungeonConquered,
    PartyDefeated,
    PartyRetreated,
}

/// High-throughput deterministic statistical combat engine resolving off-screen dungeon runs
pub struct EncounterSimulator;

impl EncounterSimulator {
    /// Advances the expedition by a single tactical round or exploration interval
    pub fn step(
        expedition: &mut DungeonExpedition,
        dungeon: &mut Dungeon,
        rng: &mut impl Rng,
        next_item_id: &mut u64,
        next_stone_id: &mut u64,
    ) -> StepResult {
        expedition.ticks_elapsed += 1;

        if matches!(
            expedition.state,
            ExpeditionState::Victorious | ExpeditionState::WipedOut | ExpeditionState::Retreated
        ) {
            return match expedition.state {
                ExpeditionState::Victorious => StepResult::DungeonConquered,
                ExpeditionState::WipedOut => StepResult::PartyDefeated,
                ExpeditionState::Retreated => StepResult::PartyRetreated,
                _ => StepResult::Continuing,
            };
        }

        // Validate floor bounds
        if expedition.current_floor >= dungeon.floors.len() {
            expedition.state = ExpeditionState::Victorious;
            dungeon.state = DungeonState::Cleared;
            return StepResult::DungeonConquered;
        }

        let floor = &mut dungeon.floors[expedition.current_floor];
        if expedition.current_room >= floor.rooms.len() {
            // Floor was completely traversed, ascend to next floor
            expedition.current_floor += 1;
            expedition.current_room = 0;
            if expedition.current_floor >= dungeon.floors.len() {
                expedition.state = ExpeditionState::Victorious;
                dungeon.state = DungeonState::Cleared;
                return StepResult::DungeonConquered;
            }
            return StepResult::FloorCleared {
                floor: expedition.current_floor - 1,
            };
        }

        let room = &mut floor.rooms[expedition.current_room];

        // Shrine room handles rest & restoration
        if room.kind == RoomKind::Shrine && !room.cleared {
            for member in &mut expedition.party {
                member.heal(member.max_hp * 0.50);
            }
            room.cleared = true;
            expedition.combat_log.push(format!(
                "Floor {} Chamber {}: Party rested at Ancient Restorative Shrine",
                expedition.current_floor + 1,
                expedition.current_room + 1
            ));
            expedition.current_room += 1;
            return StepResult::Continuing;
        }

        // Entry halls are traversed without combat
        if room.kind == RoomKind::EntryHall && !room.cleared {
            room.cleared = true;
            expedition.current_room += 1;
            return StepResult::Continuing;
        }

        // If room has living monsters, resolve combat round
        if !room.is_combat_resolved() {
            expedition.state = ExpeditionState::InCombat;
            Self::resolve_combat_round(expedition, room, rng);

            // Check if party wiped
            if !expedition.is_party_alive() {
                expedition.state = ExpeditionState::WipedOut;
                expedition
                    .combat_log
                    .push("Expedition party suffered total defeat in combat.".to_string());
                return StepResult::PartyDefeated;
            }

            // Check tactical retreat condition (< 15% party health)
            if expedition.average_party_hp_ratio() < 0.15 {
                expedition.state = ExpeditionState::Retreated;
                expedition
                    .combat_log
                    .push("Party executed tactical retreat due to critical injuries.".to_string());
                return StepResult::PartyRetreated;
            }

            // If monsters survived, continue fighting in next tick
            if !room.is_combat_resolved() {
                return StepResult::Continuing;
            }
        }

        // Room is conquered! Harvest loot, mana stones, and experience
        room.cleared = true;
        let is_boss_chamber = room.kind == RoomKind::BossLair;
        let luck = expedition.average_party_luck();

        let loot_drop = LootTable::roll_encounter_drop(
            dungeon.rank,
            is_boss_chamber,
            luck,
            rng,
            next_item_id,
            next_stone_id,
        );

        expedition.loot.gold += loot_drop.gold;
        expedition.loot.mana_stones.extend(loot_drop.mana_stones);
        expedition.loot.equipment.extend(loot_drop.equipment);
        expedition.loot.materials.extend(loot_drop.materials);

        let exp_gain = if is_boss_chamber {
            300 * dungeon.rank.stat_multiplier() as u32
        } else {
            75 * dungeon.rank.stat_multiplier() as u32
        };
        expedition.exp_earned += exp_gain;

        expedition.combat_log.push(format!(
            "Cleared Floor {} Room {} ({})! Earned {} EXP and collected loot.",
            expedition.current_floor + 1,
            expedition.current_room + 1,
            room.kind.display_name(),
            exp_gain
        ));

        let cleared_floor_idx = expedition.current_floor;
        let cleared_room_idx = expedition.current_room;

        // Advance room pointer
        expedition.current_room += 1;
        if expedition.current_room >= dungeon.floors[expedition.current_floor].rooms.len() {
            dungeon.floors[expedition.current_floor].cleared = true;
            expedition.current_floor += 1;
            expedition.current_room = 0;

            if expedition.current_floor >= dungeon.floors.len() {
                expedition.state = ExpeditionState::Victorious;
                dungeon.state = DungeonState::Cleared;
                return StepResult::DungeonConquered;
            }
            return StepResult::FloorCleared {
                floor: cleared_floor_idx,
            };
        }

        StepResult::RoomCleared {
            floor: cleared_floor_idx,
            room: cleared_room_idx,
        }
    }

    /// Simulates simultaneous exchange of offensive strikes and defensive counters
    fn resolve_combat_round(
        expedition: &mut DungeonExpedition,
        room: &mut crate::world::dungeon::DungeonRoom,
        rng: &mut impl Rng,
    ) {
        // 1. Party attacks living monsters
        for member in expedition.party.iter_mut().filter(|m| m.is_alive()) {
            if let Some(target) = room.monsters.iter_mut().find(|m| m.is_alive()) {
                let mut damage = (member.effective_attack() - target.defense * 0.4).max(1.0);
                if rng.gen_bool(member.crit_chance() as f64) {
                    damage *= 1.75;
                }
                target.take_damage(damage);
            }
        }

        // 2. Living monsters retaliate against living party members
        let living_party_indices: Vec<usize> = expedition
            .party
            .iter()
            .enumerate()
            .filter(|(_, m)| m.is_alive())
            .map(|(i, _)| i)
            .collect();

        if living_party_indices.is_empty() {
            return;
        }

        for monster in room.monsters.iter().filter(|m| m.is_alive()) {
            let target_idx = living_party_indices[rng.gen_range(0..living_party_indices.len())];
            let target = &mut expedition.party[target_idx];

            let incoming_dmg = (monster.attack_power - target.effective_defense() * 0.4).max(1.0);
            target.hp = (target.hp - incoming_dmg).max(0.0);
        }
    }
}
