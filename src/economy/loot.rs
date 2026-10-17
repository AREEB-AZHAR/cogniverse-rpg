use crate::agent::progression::{Attributes, Rank};
use crate::world::items::ItemId;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Equipment rarity tiers scaling from common salvage to god-forged mythics
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

impl ItemRarity {
    pub const ALL: [ItemRarity; 6] = [
        ItemRarity::Common,
        ItemRarity::Uncommon,
        ItemRarity::Rare,
        ItemRarity::Epic,
        ItemRarity::Legendary,
        ItemRarity::Mythic,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            ItemRarity::Common => "Common",
            ItemRarity::Uncommon => "Uncommon",
            ItemRarity::Rare => "Rare",
            ItemRarity::Epic => "Epic",
            ItemRarity::Legendary => "Legendary",
            ItemRarity::Mythic => "Mythic",
        }
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            ItemRarity::Common => "#9e9e9e",    // Gray
            ItemRarity::Uncommon => "#4caf50",  // Green
            ItemRarity::Rare => "#2196f3",      // Blue
            ItemRarity::Epic => "#9c27b0",      // Purple
            ItemRarity::Legendary => "#ff9800", // Orange/Gold
            ItemRarity::Mythic => "#e91e63",    // Crimson/Red
        }
    }

    pub fn max_affixes(&self) -> usize {
        match self {
            ItemRarity::Common => 0,
            ItemRarity::Uncommon => 1,
            ItemRarity::Rare => 2,
            ItemRarity::Epic => 3,
            ItemRarity::Legendary => 4,
            ItemRarity::Mythic => 5,
        }
    }

    pub fn value_multiplier(&self) -> f32 {
        match self {
            ItemRarity::Common => 1.0,
            ItemRarity::Uncommon => 1.75,
            ItemRarity::Rare => 3.0,
            ItemRarity::Epic => 6.5,
            ItemRarity::Legendary => 15.0,
            ItemRarity::Mythic => 40.0,
        }
    }
}

/// Equipment equipment slots for combatants
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Weapon,
    Armor,
    Accessory,
    Shield,
    Helmet,
}

impl EquipmentSlot {
    pub const ALL: [EquipmentSlot; 5] = [
        EquipmentSlot::Weapon,
        EquipmentSlot::Armor,
        EquipmentSlot::Accessory,
        EquipmentSlot::Shield,
        EquipmentSlot::Helmet,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            EquipmentSlot::Weapon => "Weapon",
            EquipmentSlot::Armor => "Chest Armor",
            EquipmentSlot::Accessory => "Arcane Accessory",
            EquipmentSlot::Shield => "Shield",
            EquipmentSlot::Helmet => "Headgear",
        }
    }
}

/// Stat modifications granted by magical affixes
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatModifier {
    pub strength: f32,
    pub agility: f32,
    pub vitality: f32,
    pub intelligence: f32,
    pub wisdom: f32,
    pub charisma: f32,
    pub luck: f32,
    pub bonus_attack: f32,
    pub bonus_defense: f32,
    pub crit_chance: f32,
}

impl Default for StatModifier {
    fn default() -> Self {
        Self {
            strength: 0.0,
            agility: 0.0,
            vitality: 0.0,
            intelligence: 0.0,
            wisdom: 0.0,
            charisma: 0.0,
            luck: 0.0,
            bonus_attack: 0.0,
            bonus_defense: 0.0,
            crit_chance: 0.0,
        }
    }
}

/// A procedural or enchanted affix on an equipment item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemAffix {
    pub name: String,
    pub modifier: StatModifier,
    pub tier: u32,
}

/// Complete equipment item instance with affixes and combat values
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquipmentItem {
    pub id: u64,
    pub name: String,
    pub rarity: ItemRarity,
    pub slot: EquipmentSlot,
    pub base_attack: f32,
    pub base_defense: f32,
    pub affixes: Vec<ItemAffix>,
    pub value: u32,
    pub required_rank: Rank,
    pub durability: u32,
    pub max_durability: u32,
}

impl EquipmentItem {
    /// Calculates the effective attack power of the item including affixes
    pub fn total_attack(&self) -> f32 {
        let affix_atk: f32 = self.affixes.iter().map(|a| a.modifier.bonus_attack).sum();
        (self.base_attack + affix_atk).max(0.0)
    }

    /// Calculates the effective defense value of the item including affixes
    pub fn total_defense(&self) -> f32 {
        let affix_def: f32 = self.affixes.iter().map(|a| a.modifier.bonus_defense).sum();
        (self.base_defense + affix_def).max(0.0)
    }

    /// Applies all attribute affixes from this item onto a character's core attributes
    pub fn apply_to_attributes(&self, attrs: &mut Attributes) {
        for affix in &self.affixes {
            attrs.strength += affix.modifier.strength;
            attrs.agility += affix.modifier.agility;
            attrs.vitality += affix.modifier.vitality;
            attrs.intelligence += affix.modifier.intelligence;
            attrs.wisdom += affix.modifier.wisdom;
            attrs.charisma += affix.modifier.charisma;
            attrs.luck += affix.modifier.luck;
        }
    }

    /// Checks whether this item has received divine sanctification
    pub fn is_blessed(&self) -> bool {
        self.name.contains('✦')
            || self
                .affixes
                .iter()
                .any(|a| a.name.contains("God-Forged") || a.name.contains("Celestial"))
    }

    /// Bestows divine blessing on the item, upgrading its rarity tier, infusing celestial affixes,
    /// multiplying base combat potency, restoring full durability, and consecrating its identity.
    pub fn bless(&mut self) {
        // Upgrade rarity tier up to Mythic
        self.rarity = match self.rarity {
            ItemRarity::Common => ItemRarity::Uncommon,
            ItemRarity::Uncommon => ItemRarity::Rare,
            ItemRarity::Rare => ItemRarity::Epic,
            ItemRarity::Epic => ItemRarity::Legendary,
            ItemRarity::Legendary | ItemRarity::Mythic => ItemRarity::Mythic,
        };

        // Boost base stats
        if self.base_attack > 0.0 {
            self.base_attack = (self.base_attack * 1.50).max(self.base_attack + 25.0);
        }
        if self.base_defense > 0.0 {
            self.base_defense = (self.base_defense * 1.50).max(self.base_defense + 20.0);
        }

        // Add celestial divine affix
        self.affixes.push(ItemAffix {
            name: "God-Forged Celestial Radiance".to_string(),
            modifier: StatModifier {
                strength: 15.0,
                agility: 15.0,
                vitality: 20.0,
                intelligence: 15.0,
                wisdom: 20.0,
                charisma: 15.0,
                luck: 25.0,
                bonus_attack: 25.0,
                bonus_defense: 25.0,
                crit_chance: 0.15,
            },
            tier: 10,
        });

        // Restore and expand durability
        self.max_durability = (self.max_durability + 100).max(200);
        self.durability = self.max_durability;

        // Multiply gold valuation
        self.value = ((self.value as f32) * 3.5).round() as u32;

        // Consecrate name
        if !self.name.contains('✦') {
            self.name = format!("✦ Sanctified {}", self.name);
        }
    }
}

/// Mana Stone purity and capacity grades harvested from monsters and dungeon geodes
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ManaStoneGrade {
    Flawed,
    Regular,
    Flawless,
    Radiant,
    Astral,
}

impl ManaStoneGrade {
    pub const ALL: [ManaStoneGrade; 5] = [
        ManaStoneGrade::Flawed,
        ManaStoneGrade::Regular,
        ManaStoneGrade::Flawless,
        ManaStoneGrade::Radiant,
        ManaStoneGrade::Astral,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            ManaStoneGrade::Flawed => "Flawed Mana Stone",
            ManaStoneGrade::Regular => "Regular Mana Stone",
            ManaStoneGrade::Flawless => "Flawless Mana Stone",
            ManaStoneGrade::Radiant => "Radiant Mana Stone",
            ManaStoneGrade::Astral => "Astral Mana Stone",
        }
    }

    pub fn default_capacity(&self) -> f32 {
        match self {
            ManaStoneGrade::Flawed => 50.0,
            ManaStoneGrade::Regular => 150.0,
            ManaStoneGrade::Flawless => 400.0,
            ManaStoneGrade::Radiant => 1000.0,
            ManaStoneGrade::Astral => 2500.0,
        }
    }

    pub fn base_monetary_value(&self) -> u32 {
        match self {
            ManaStoneGrade::Flawed => 15,
            ManaStoneGrade::Regular => 60,
            ManaStoneGrade::Flawless => 250,
            ManaStoneGrade::Radiant => 1000,
            ManaStoneGrade::Astral => 5000,
        }
    }
}

/// A harvested or synthesized mana stone holding arcane power
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManaStone {
    pub id: u64,
    pub grade: ManaStoneGrade,
    pub purity: f32, // 0.50 to 1.00
    pub stored_mana: f32,
    pub max_mana: f32,
}

impl ManaStone {
    pub fn new(id: u64, grade: ManaStoneGrade, purity: f32) -> Self {
        let clamped_purity = purity.clamp(0.5, 1.0);
        let max_mana = grade.default_capacity() * clamped_purity;
        Self {
            id,
            grade,
            purity: clamped_purity,
            stored_mana: max_mana, // Harvested fresh with full charge
            max_mana,
        }
    }

    /// Recharges mana into the stone up to max capacity
    pub fn recharge(&mut self, amount: f32) {
        self.stored_mana = (self.stored_mana + amount).min(self.max_mana);
    }

    /// Discharges mana for arcane casting or enchantment, returning amount actually drawn
    pub fn discharge(&mut self, amount: f32) -> f32 {
        let draw = amount.min(self.stored_mana);
        self.stored_mana -= draw;
        draw
    }

    /// Calculates dynamic monetary value based on grade, purity, and remaining stored charge
    pub fn monetary_value(&self) -> u32 {
        let base = self.grade.base_monetary_value() as f32;
        let purity_mult = self.purity;
        let charge_factor = 0.5 + 0.5 * (self.stored_mana / self.max_mana.max(1.0));
        (base * purity_mult * charge_factor).round() as u32
    }

    /// Checks whether this mana stone has received divine sanctification
    pub fn is_blessed(&self) -> bool {
        self.purity >= 0.99 && self.stored_mana >= self.max_mana
    }

    /// Bestows divine blessing on the mana stone, elevating its grade, maximizing purity to 1.0,
    /// and overcharging its stored energy capacity to 125%.
    pub fn bless(&mut self) {
        self.grade = match self.grade {
            ManaStoneGrade::Flawed => ManaStoneGrade::Regular,
            ManaStoneGrade::Regular => ManaStoneGrade::Flawless,
            ManaStoneGrade::Flawless => ManaStoneGrade::Radiant,
            ManaStoneGrade::Radiant | ManaStoneGrade::Astral => ManaStoneGrade::Astral,
        };
        self.purity = 1.0;
        self.max_mana = self.grade.default_capacity() * 1.25;
        self.stored_mana = self.max_mana;
    }
}

/// Converts a standard mundane or crafted inventory item into a blessed celestial equipment relic
pub fn bless_basic_item(item: ItemId, next_item_id: &mut u64) -> EquipmentItem {
    let id = *next_item_id;
    *next_item_id += 1;
    let (slot, base_atk, base_def) = match item {
        ItemId::IronSword => (EquipmentSlot::Weapon, 35.0, 0.0),
        ItemId::HuntingBow => (EquipmentSlot::Weapon, 30.0, 0.0),
        ItemId::IronPickaxe | ItemId::StonePickaxe | ItemId::PrimitiveChopper => {
            (EquipmentSlot::Weapon, 20.0, 5.0)
        }
        ItemId::RuneSolarIgnition
        | ItemId::RunePurifyingAura
        | ItemId::RuneStoneWall
        | ItemId::RuneShadowVeil
        | ItemId::ManaCrystal => (EquipmentSlot::Accessory, 15.0, 15.0),
        _ => (EquipmentSlot::Accessory, 10.0, 10.0),
    };

    let mut equip = EquipmentItem {
        id,
        name: format!("✦ God-Forged {}", item.display_name()),
        rarity: ItemRarity::Legendary,
        slot,
        base_attack: base_atk,
        base_defense: base_def,
        affixes: vec![ItemAffix {
            name: "Celestial Sanctification".to_string(),
            modifier: StatModifier {
                strength: 12.0,
                vitality: 15.0,
                wisdom: 15.0,
                luck: 20.0,
                bonus_attack: 20.0,
                bonus_defense: 15.0,
                crit_chance: 0.10,
                ..Default::default()
            },
            tier: 7,
        }],
        value: (item.rarity_value() * 30.0).round() as u32,
        required_rank: Rank::F,
        durability: 150,
        max_durability: 150,
    };
    equip.bless();
    equip
}

/// Aggregated loot dropped from dungeon encounters or boss chambers
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DungeonLootDrop {
    pub gold: u32,
    pub mana_stones: Vec<ManaStone>,
    pub equipment: Vec<EquipmentItem>,
    pub materials: Vec<(ItemId, u32)>,
}

/// Loot table generator producing procedural drops tailored by dungeon rank and party luck
pub struct LootTable;

impl LootTable {
    /// Generates equipment item with procedural name, base stats, and affixes
    pub fn generate_equipment(
        id: u64,
        rank: Rank,
        forced_rarity: Option<ItemRarity>,
        luck_stat: f32,
        rng: &mut impl Rng,
    ) -> EquipmentItem {
        let rarity = forced_rarity.unwrap_or_else(|| Self::roll_rarity(rank, luck_stat, rng));
        let slot = EquipmentSlot::ALL[rng.gen_range(0..EquipmentSlot::ALL.len())];

        let rank_multiplier = rank.stat_multiplier();
        let base_stat_scale = 8.0 * rank_multiplier;

        let (base_attack, base_defense) = match slot {
            EquipmentSlot::Weapon => (
                rng.gen_range((base_stat_scale * 1.5)..(base_stat_scale * 2.8)),
                0.0,
            ),
            EquipmentSlot::Armor => (
                0.0,
                rng.gen_range((base_stat_scale * 1.2)..(base_stat_scale * 2.2)),
            ),
            EquipmentSlot::Shield => (
                rng.gen_range(1.0..(base_stat_scale * 0.4)),
                rng.gen_range((base_stat_scale * 0.8)..(base_stat_scale * 1.6)),
            ),
            EquipmentSlot::Helmet => (
                0.0,
                rng.gen_range((base_stat_scale * 0.6)..(base_stat_scale * 1.3)),
            ),
            EquipmentSlot::Accessory => (
                rng.gen_range((base_stat_scale * 0.3)..(base_stat_scale * 0.8)),
                rng.gen_range((base_stat_scale * 0.3)..(base_stat_scale * 0.8)),
            ),
        };

        let affixes = Self::generate_affixes(rarity, rank, rng);
        let name = Self::construct_item_name(slot, rarity, &affixes, rng);

        let base_val = (base_stat_scale * 12.0) as u32;
        let value = ((base_val as f32) * rarity.value_multiplier()).round() as u32;

        EquipmentItem {
            id,
            name,
            rarity,
            slot,
            base_attack,
            base_defense,
            affixes,
            value: value.max(5),
            required_rank: rank,
            durability: 100,
            max_durability: 100,
        }
    }

    /// Rolls an item rarity tier based on dungeon rank and party luck
    pub fn roll_rarity(rank: Rank, luck_stat: f32, rng: &mut impl Rng) -> ItemRarity {
        let luck_bonus = (luck_stat * 2.0).clamp(0.0, 150.0) as u32;

        let (w_common, w_uncommon, w_rare, w_epic, w_legendary, w_mythic): (
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
        ) = match rank {
            Rank::F => (750, 220, 30, 0, 0, 0),
            Rank::E => (600, 320, 75, 5, 0, 0),
            Rank::D => (450, 380, 150, 20, 0, 0),
            Rank::C => (280, 420, 240, 55, 5, 0),
            Rank::B => (150, 350, 360, 120, 20, 0),
            Rank::A => (60, 220, 450, 220, 48, 2),
            Rank::S => (20, 120, 420, 340, 90, 10),
            Rank::SPlus => (5, 50, 320, 450, 155, 20),
            Rank::SSS => (0, 15, 180, 480, 275, 50),
        };

        let weights = [
            w_common.saturating_sub(luck_bonus / 2),
            w_uncommon,
            w_rare + (luck_bonus / 4),
            w_epic + (luck_bonus / 6),
            w_legendary + (luck_bonus / 12),
            w_mythic + (luck_bonus / 25),
        ];

        let total: u32 = weights.iter().sum();
        let mut pick = rng.gen_range(0..total.max(1));

        for (rarity, &w) in ItemRarity::ALL.iter().zip(weights.iter()) {
            if pick < w {
                return *rarity;
            }
            pick -= w;
        }

        ItemRarity::Common
    }

    /// Generates affixes appropriate for the item's rarity and rank
    fn generate_affixes(rarity: ItemRarity, rank: Rank, rng: &mut impl Rng) -> Vec<ItemAffix> {
        let count = rarity.max_affixes();
        let mut affixes = Vec::with_capacity(count);
        let tier = (rank as u32) + 1;
        let stat_boost = 2.0 * rank.stat_multiplier();

        let pool = [
            (
                "Titan's",
                StatModifier {
                    strength: stat_boost * 1.5,
                    ..Default::default()
                },
            ),
            (
                "Swiftwind",
                StatModifier {
                    agility: stat_boost * 1.4,
                    ..Default::default()
                },
            ),
            (
                "Juggernaut's",
                StatModifier {
                    vitality: stat_boost * 1.6,
                    ..Default::default()
                },
            ),
            (
                "Archmage's",
                StatModifier {
                    intelligence: stat_boost * 1.5,
                    ..Default::default()
                },
            ),
            (
                "Seer's",
                StatModifier {
                    wisdom: stat_boost * 1.4,
                    ..Default::default()
                },
            ),
            (
                "Sovereign's",
                StatModifier {
                    charisma: stat_boost * 1.3,
                    ..Default::default()
                },
            ),
            (
                "Gambler's",
                StatModifier {
                    luck: stat_boost * 1.8,
                    crit_chance: 0.05,
                    ..Default::default()
                },
            ),
            (
                "Vorpal",
                StatModifier {
                    bonus_attack: stat_boost * 2.0,
                    crit_chance: 0.08,
                    ..Default::default()
                },
            ),
            (
                "Aegis",
                StatModifier {
                    bonus_defense: stat_boost * 2.2,
                    ..Default::default()
                },
            ),
        ];

        let mut available_indices: Vec<usize> = (0..pool.len()).collect();
        for _ in 0..count {
            if available_indices.is_empty() {
                break;
            }
            let choice_idx = rng.gen_range(0..available_indices.len());
            let pool_idx = available_indices.remove(choice_idx);
            let (name, modif) = pool[pool_idx];
            affixes.push(ItemAffix {
                name: name.to_string(),
                modifier: modif,
                tier,
            });
        }

        affixes
    }

    /// Generates item names based on slot, rarity, and affixes
    fn construct_item_name(
        slot: EquipmentSlot,
        rarity: ItemRarity,
        affixes: &[ItemAffix],
        rng: &mut impl Rng,
    ) -> String {
        let base_noun = match slot {
            EquipmentSlot::Weapon => match rng.gen_range(0..4) {
                0 => "Broadsword",
                1 => "Warblade",
                2 => "Spear",
                _ => "Staff",
            },
            EquipmentSlot::Armor => match rng.gen_range(0..3) {
                0 => "Cuirass",
                1 => "Plate Armor",
                _ => "Mantle",
            },
            EquipmentSlot::Shield => "Tower Shield",
            EquipmentSlot::Helmet => "Great Helm",
            EquipmentSlot::Accessory => match rng.gen_range(0..3) {
                0 => "Amulet",
                1 => "Ring",
                _ => "Pendant",
            },
        };

        if affixes.is_empty() {
            format!("{} {}", rarity.display_name(), base_noun)
        } else {
            let prefix = &affixes[0].name;
            if affixes.len() > 1 {
                format!("{} {} of {}", prefix, base_noun, affixes[1].name)
            } else {
                format!("{} {}", prefix, base_noun)
            }
        }
    }

    /// Harvests a Mana Stone from a defeated monster or geode
    pub fn roll_mana_stone(rank: Rank, rng: &mut impl Rng, next_stone_id: &mut u64) -> ManaStone {
        let grade = match rank {
            Rank::F => ManaStoneGrade::Flawed,
            Rank::E => {
                if rng.gen_bool(0.7) {
                    ManaStoneGrade::Flawed
                } else {
                    ManaStoneGrade::Regular
                }
            }
            Rank::D => {
                if rng.gen_bool(0.6) {
                    ManaStoneGrade::Regular
                } else {
                    ManaStoneGrade::Flawed
                }
            }
            Rank::C => {
                if rng.gen_bool(0.6) {
                    ManaStoneGrade::Regular
                } else {
                    ManaStoneGrade::Flawless
                }
            }
            Rank::B => {
                if rng.gen_bool(0.6) {
                    ManaStoneGrade::Flawless
                } else {
                    ManaStoneGrade::Regular
                }
            }
            Rank::A => {
                if rng.gen_bool(0.65) {
                    ManaStoneGrade::Flawless
                } else {
                    ManaStoneGrade::Radiant
                }
            }
            Rank::S => {
                if rng.gen_bool(0.6) {
                    ManaStoneGrade::Radiant
                } else {
                    ManaStoneGrade::Flawless
                }
            }
            Rank::SPlus => {
                if rng.gen_bool(0.6) {
                    ManaStoneGrade::Radiant
                } else {
                    ManaStoneGrade::Astral
                }
            }
            Rank::SSS => {
                if rng.gen_bool(0.7) {
                    ManaStoneGrade::Astral
                } else {
                    ManaStoneGrade::Radiant
                }
            }
        };

        let purity = rng.gen_range(0.65..1.0);
        let id = *next_stone_id;
        *next_stone_id += 1;
        ManaStone::new(id, grade, purity)
    }

    /// Rolls complete encounter drop including gold, mana stones, equipment, and crafting materials
    pub fn roll_encounter_drop(
        rank: Rank,
        is_boss: bool,
        luck_stat: f32,
        rng: &mut impl Rng,
        next_item_id: &mut u64,
        next_stone_id: &mut u64,
    ) -> DungeonLootDrop {
        let rank_mult = rank.stat_multiplier();
        let boss_mult = if is_boss { 3.5 } else { 1.0 };

        // Gold coin drop
        let base_gold = (rng.gen_range(15..45) as f32) * rank_mult * boss_mult;
        let gold = base_gold.round() as u32;

        // Mana stone drops
        let stone_count = if is_boss {
            rng.gen_range(2..=4)
        } else if rng.gen_bool(0.55) {
            1
        } else {
            0
        };

        let mut mana_stones = Vec::with_capacity(stone_count);
        for _ in 0..stone_count {
            mana_stones.push(Self::roll_mana_stone(rank, rng, next_stone_id));
        }

        // Equipment drops
        let equip_chance = if is_boss {
            1.0
        } else {
            0.35 + (luck_stat * 0.005).min(0.25)
        };
        let mut equipment = Vec::new();
        if rng.gen_bool(equip_chance as f64) {
            let id = *next_item_id;
            *next_item_id += 1;
            equipment.push(Self::generate_equipment(id, rank, None, luck_stat, rng));
        }
        if is_boss && rng.gen_bool(0.5) {
            let id = *next_item_id;
            *next_item_id += 1;
            equipment.push(Self::generate_equipment(id, rank, None, luck_stat, rng));
        }

        // Crafting materials
        let mut materials = Vec::new();
        if is_boss {
            materials.push((ItemId::MithrilOre, rng.gen_range(2..=6)));
            materials.push((ItemId::ManaCrystal, rng.gen_range(3..=8)));
        } else {
            if rng.gen_bool(0.4) {
                materials.push((ItemId::IronOre, rng.gen_range(1..=4)));
            }
            if rng.gen_bool(0.2) {
                materials.push((ItemId::ManaCrystal, rng.gen_range(1..=2)));
            }
        }

        DungeonLootDrop {
            gold,
            mana_stones,
            equipment,
            materials,
        }
    }
}
