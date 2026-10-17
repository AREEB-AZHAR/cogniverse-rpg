use crate::agent::party::Party;
use crate::agent::progression::{Rank, Subclass};
use crate::economy::loot::{TitanLootChest, TitanLootTable};
use crate::world::danger::DangerField;
use crate::world::items::{Inventory, ItemId};
use crate::world::ShippingNetwork;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Archetype category of primordial world titans
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitanKind {
    AbyssalLeviathan,
    MagmaWyrm,
    VoidColossus,
}

impl TitanKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            TitanKind::AbyssalLeviathan => "🌊 Leviathan, the Abyssal Dread",
            TitanKind::MagmaWyrm => "🔥 Ignis-Ignash, the Magma Wyrm",
            TitanKind::VoidColossus => "🌌 Aethelgard, the Void Colossus",
        }
    }

    pub fn exclusive_ore(&self) -> ItemId {
        match self {
            TitanKind::AbyssalLeviathan => ItemId::AbyssaliteOre,
            TitanKind::MagmaWyrm => ItemId::PyrestoneOre,
            TitanKind::VoidColossus => ItemId::VoidstoneOre,
        }
    }

    pub fn exclusive_ingot(&self) -> ItemId {
        match self {
            TitanKind::AbyssalLeviathan => ItemId::AbyssaliteIngot,
            TitanKind::MagmaWyrm => ItemId::PyrestoneIngot,
            TitanKind::VoidColossus => ItemId::VoidstoneIngot,
        }
    }
}

/// Operational state of a primordial world titan
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitanState {
    Slumbering,
    Awakened,
    Enraged,
    Defeated,
}

/// Colossal multi-phase world boss rooted in deep biomes
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldBoss {
    pub id: usize,
    pub name: String,
    pub kind: TitanKind,
    pub x: usize,
    pub y: usize,
    pub hp: f32,
    pub max_hp: f32,
    pub attack_power: f32,
    pub defense: f32,
    pub state: TitanState,
    pub awakening_pressure: f32,
    pub disruption_radius: usize,
    pub chest_reward: Option<TitanLootChest>,
}

impl WorldBoss {
    pub fn is_alive(&self) -> bool {
        self.hp > 0.0 && self.state != TitanState::Defeated
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.hp = (self.hp - amount).max(0.0);
        if self.hp <= 0.0 {
            self.state = TitanState::Defeated;
        } else if self.hp <= self.max_hp * 0.40 && self.state == TitanState::Awakened {
            self.state = TitanState::Enraged;
        }
    }

    /// Awaken the slumbering titan into an active apocalyptic threat
    pub fn awaken(&mut self) -> String {
        self.state = TitanState::Awakened;
        format!(
            "🚨 [TITAN AWAKENS] The earth trembles as {} awakens at ({}, {})!",
            self.name, self.x, self.y
        )
    }

    /// Unveils regional cataclysms, injecting hazard and halting trade routes
    pub fn execute_cataclysm(
        &self,
        danger_field: &mut DangerField,
        shipping: &mut ShippingNetwork,
    ) {
        if !self.is_alive() || self.state == TitanState::Slumbering {
            return;
        }

        // Inject catastrophic environmental hazard
        let radius = self.disruption_radius;
        danger_field.inject_danger(self.x, self.y, radius, 85.0);

        // Leviathan tidal waves sink or ground nearby maritime shipping vessels
        if self.kind == TitanKind::AbyssalLeviathan {
            for ship in &mut shipping.ships {
                let dist = (ship.x as isize - self.x as isize).abs()
                    + (ship.y as isize - self.y as isize).abs();
                if dist <= radius as isize {
                    ship.durability = (ship.durability - 40.0).max(0.0);
                }
            }
        }
    }
}

/// Central manager supervising slumbering titans, awakening pressures, and raid rewards
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TitanManager {
    pub titans: Vec<WorldBoss>,
    pub next_titan_id: usize,
}

impl TitanManager {
    pub fn new() -> Self {
        Self {
            titans: Vec::new(),
            next_titan_id: 1,
        }
    }

    /// Seeds the 3 canonical mythic world bosses in planetary biomes
    pub fn spawn_initial_titans(width: usize, height: usize) -> Self {
        let mut mgr = Self::new();

        // 1. Leviathan in Deep Ocean
        mgr.titans.push(WorldBoss {
            id: 1,
            name: "Leviathan, the Abyssal Dread".into(),
            kind: TitanKind::AbyssalLeviathan,
            x: width / 8,
            y: height / 8,
            hp: 6000.0,
            max_hp: 6000.0,
            attack_power: 55.0,
            defense: 25.0,
            state: TitanState::Slumbering,
            awakening_pressure: 10.0,
            disruption_radius: 8,
            chest_reward: None,
        });

        // 2. Magma Wyrm in Volcanic Mountain Spine
        mgr.titans.push(WorldBoss {
            id: 2,
            name: "Ignis-Ignash, the Magma Wyrm".into(),
            kind: TitanKind::MagmaWyrm,
            x: width / 2,
            y: height / 2,
            hp: 8000.0,
            max_hp: 8000.0,
            attack_power: 70.0,
            defense: 30.0,
            state: TitanState::Slumbering,
            awakening_pressure: 15.0,
            disruption_radius: 6,
            chest_reward: None,
        });

        // 3. Void Colossus at Leyline Ruins
        mgr.titans.push(WorldBoss {
            id: 3,
            name: "Aethelgard, the Void Colossus".into(),
            kind: TitanKind::VoidColossus,
            x: (width * 3) / 4,
            y: (height * 3) / 4,
            hp: 10000.0,
            max_hp: 10000.0,
            attack_power: 90.0,
            defense: 40.0,
            state: TitanState::Slumbering,
            awakening_pressure: 5.0,
            disruption_radius: 10,
            chest_reward: None,
        });

        mgr.next_titan_id = 4;
        mgr
    }

    /// Evaluates awakening pressure every tick and applies cataclysms
    pub fn tick(
        &mut self,
        danger_field: &mut DangerField,
        shipping: &mut ShippingNetwork,
        rng: &mut impl Rng,
    ) -> Vec<String> {
        let mut events = Vec::new();

        for titan in &mut self.titans {
            if titan.state == TitanState::Slumbering {
                titan.awakening_pressure += 0.05;
                if titan.awakening_pressure >= 100.0 {
                    events.push(titan.awaken());
                }
            } else if titan.is_alive() {
                titan.execute_cataclysm(danger_field, shipping);
            }
        }

        // Optional random volcanic rumblings
        if rng.gen_bool(0.01) {
            for titan in &mut self.titans {
                if titan.state == TitanState::Slumbering {
                    titan.awakening_pressure += 5.0;
                }
            }
        }

        events
    }

    /// Resolves one round of epic combat between a Grand Raid Brigade and a World Boss
    pub fn resolve_raid_round(
        &mut self,
        titan_id: usize,
        raid_party: &mut Party,
        next_item_id: &mut u64,
        next_stone_id: &mut u64,
        rng: &mut impl Rng,
    ) -> Option<TitanLootChest> {
        let titan = self.titans.iter_mut().find(|t| t.id == titan_id)?;
        if !titan.is_alive() || !raid_party.is_party_alive() {
            return None;
        }

        // Raid party deals damage scaled by composite power and role synergy
        let raid_atk = raid_party.composite_power * 0.15;
        let actual_damage = (raid_atk - titan.defense * 0.5).max(10.0);
        titan.take_damage(actual_damage);

        // Titan retaliates with colossal sweep attack
        let titan_dmg = match titan.state {
            TitanState::Enraged => titan.attack_power * 1.4,
            _ => titan.attack_power,
        };

        for member in &mut raid_party.members {
            if member.is_alive() {
                let damage =
                    (titan_dmg - member.defense * raid_party.synergy_multiplier * 0.3).max(5.0);
                member.take_damage(damage);
            }
        }

        // If Titan is defeated, award the Mythic Raid Treasure Chest
        if !titan.is_alive() {
            let chest = TitanLootTable::generate_chest(
                &titan.name,
                titan.kind.exclusive_ore(),
                titan.kind.exclusive_ingot(),
                next_item_id,
                next_stone_id,
                rng,
            );
            titan.chest_reward = Some(chest.clone());
            return Some(chest);
        }

        None
    }
}

/// Validates whether a worker possesses the master skills and specialized high-tier pickaxes
/// required to extract primordial boss ores without shattering the vein or tools.
pub fn validate_titan_mining(
    inventory: &Inventory,
    worker_level: u32,
    worker_rank: Option<Rank>,
    worker_subclass: Option<Subclass>,
) -> Result<u32, &'static str> {
    // 1. Tool check: Must hold a Mithril Pickaxe or Titan Pickaxe
    let has_special_pickaxe =
        inventory.count(ItemId::TitanPickaxe) > 0 || inventory.count(ItemId::MithrilPickaxe) > 0;
    if !has_special_pickaxe {
        return Err("Mining primordial titan ores requires a specialized high-quality pickaxe (Mithril Pickaxe or Titan Pickaxe). Basic tools shatter against the ore!");
    }

    // 2. Skill check: Must be a top-skilled worker (Level 25+, Rank S/SPlus/SSS, or RuneSmith subclass)
    let is_master_rank = worker_rank.map(|r| r >= Rank::S).unwrap_or(false);
    let is_master_level = worker_level >= 25;
    let is_runesmith = worker_subclass == Some(Subclass::RuneSmith);

    if !is_master_level && !is_master_rank && !is_runesmith {
        return Err("Worker lacks the master extraction skill (requires Level 25+, S-Rank, or RuneSmith) to mine primordial titan ore!");
    }

    // Yield: 2 if equipped with TitanPickaxe, 1 otherwise
    if inventory.count(ItemId::TitanPickaxe) > 0 {
        Ok(2)
    } else {
        Ok(1)
    }
}

/// Validates whether a metallurgist possesses the elite skill required to refine primordial ores into ingots
pub fn validate_titan_refining(
    worker_level: u32,
    worker_rank: Option<Rank>,
    worker_subclass: Option<Subclass>,
) -> Result<(), &'static str> {
    let is_master_rank = worker_rank.map(|r| r >= Rank::S).unwrap_or(false);
    let is_master_level = worker_level >= 25;
    let is_runesmith = worker_subclass == Some(Subclass::RuneSmith);

    if !is_master_level && !is_master_rank && !is_runesmith {
        return Err("Refining primordial boss ore into ingots requires an elite metallurgist (Level 25+, S-Rank, or RuneSmith)!");
    }

    Ok(())
}
