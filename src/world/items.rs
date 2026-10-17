use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemId {
    // Basic Raw Materials
    Wood,
    Stone,
    Flint,
    IronOre,
    GoldOre,
    MithrilOre,
    GoldCoin,
    Grain,
    Berry,
    FreshWater,
    Fur,
    Clay,
    ManaCrystal,
    Fish,
    WoolCloth,

    // Processed & Intermediate Materials
    TimberPlanks,
    IronIngot,
    Bread,
    ClayTablet,

    // Tools & Equipment
    PrimitiveChopper,
    StonePickaxe,
    IronPickaxe,
    Firestriker,
    HuntingBow,
    IronSword,

    // Arcane Inscribed Runes
    RuneSolarIgnition,
    RunePurifyingAura,
    RuneStoneWall,
    RuneShadowVeil,
}

impl ItemId {
    pub fn display_name(&self) -> &'static str {
        match self {
            ItemId::Wood => "Wood",
            ItemId::Stone => "Stone",
            ItemId::Flint => "Flint",
            ItemId::IronOre => "Iron Ore",
            ItemId::GoldOre => "Gold Ore",
            ItemId::MithrilOre => "Arcane Mithril",
            ItemId::GoldCoin => "Gold Coin",
            ItemId::Grain => "Grain",
            ItemId::Berry => "Wild Berry",
            ItemId::FreshWater => "Fresh Water",
            ItemId::Fur => "Animal Fur",
            ItemId::Clay => "Clay",
            ItemId::ManaCrystal => "Mana Crystal",
            ItemId::TimberPlanks => "Timber Planks",
            ItemId::IronIngot => "Iron Ingot",
            ItemId::Bread => "Baked Bread",
            ItemId::ClayTablet => "Clay Tablet",
            ItemId::Fish => "Fresh Fish",
            ItemId::WoolCloth => "Luxury Wool Cloth",
            ItemId::PrimitiveChopper => "Primitive Chopper",
            ItemId::StonePickaxe => "Stone Pickaxe",
            ItemId::IronPickaxe => "Iron Pickaxe",
            ItemId::Firestriker => "Firestriker",
            ItemId::HuntingBow => "Hunting Bow",
            ItemId::IronSword => "Iron Sword",
            ItemId::RuneSolarIgnition => "Rune of Solar Ignition",
            ItemId::RunePurifyingAura => "Rune of Purifying Aura",
            ItemId::RuneStoneWall => "Rune of Stone Wall",
            ItemId::RuneShadowVeil => "Rune of Shadow Veil",
        }
    }

    pub fn rarity_value(&self) -> f32 {
        match self {
            ItemId::FreshWater | ItemId::Berry => 1.0,
            ItemId::Wood | ItemId::Stone | ItemId::Grain => 2.0,
            ItemId::Fish => 4.0,
            ItemId::Flint | ItemId::Clay | ItemId::Fur => 4.0,
            ItemId::TimberPlanks | ItemId::Bread => 5.0,
            ItemId::WoolCloth => 18.0,
            ItemId::PrimitiveChopper | ItemId::Firestriker => 8.0,
            ItemId::StonePickaxe => 10.0,
            ItemId::IronOre => 12.0,
            ItemId::GoldOre => 28.0,
            ItemId::GoldCoin => 35.0,
            ItemId::MithrilOre => 65.0,
            ItemId::IronIngot => 25.0,
            ItemId::HuntingBow | ItemId::IronPickaxe => 30.0,
            ItemId::IronSword => 45.0,
            ItemId::ManaCrystal => 50.0,
            ItemId::ClayTablet => 15.0,
            ItemId::RuneSolarIgnition | ItemId::RunePurifyingAura => 60.0,
            ItemId::RuneStoneWall | ItemId::RuneShadowVeil => 75.0,
        }
    }
}

use crate::economy::loot::{bless_basic_item, EquipmentItem, ManaStone};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub items: std::collections::HashMap<ItemId, u32>,
    #[serde(default)]
    pub equipment: Vec<EquipmentItem>,
    #[serde(default)]
    pub mana_stones: Vec<ManaStone>,
    #[serde(default)]
    pub blessed_item_names: std::collections::HashSet<String>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: std::collections::HashMap::new(),
            equipment: Vec::new(),
            mana_stones: Vec::new(),
            blessed_item_names: std::collections::HashSet::new(),
        }
    }

    pub fn count(&self, item: ItemId) -> u32 {
        *self.items.get(&item).unwrap_or(&0)
    }

    pub fn add(&mut self, item: ItemId, amount: u32) {
        *self.items.entry(item).or_insert(0) += amount;
    }

    pub fn remove(&mut self, item: ItemId, amount: u32) -> bool {
        if self.count(item) >= amount {
            let entry = self.items.entry(item).or_insert(0);
            *entry -= amount;
            if *entry == 0 {
                self.items.remove(&item);
            }
            true
        } else {
            false
        }
    }

    pub fn add_equipment(&mut self, item: EquipmentItem) {
        self.equipment.push(item);
    }

    pub fn add_mana_stone(&mut self, stone: ManaStone) {
        self.mana_stones.push(stone);
    }

    /// Bestows a divine blessing on a matching item in the inventory (equipment, mana stone, or item stack).
    /// Returns the sanctified item's title upon success.
    pub fn bless_item(&mut self, target_name: &str, next_item_id: &mut u64) -> Option<String> {
        // 1. Check equipment items first
        if let Some(equip) = self.equipment.iter_mut().find(|e| {
            e.name.eq_ignore_ascii_case(target_name)
                || target_name.contains(&e.name)
                || e.name.contains(target_name)
        }) {
            equip.bless();
            self.blessed_item_names.insert(equip.name.clone());
            return Some(equip.name.clone());
        }

        // 2. Check mana stones
        if let Some(stone) = self.mana_stones.iter_mut().find(|s| {
            s.grade.display_name().eq_ignore_ascii_case(target_name)
                || target_name.contains(s.grade.display_name())
        }) {
            stone.bless();
            let name = format!("✦ Sanctified {}", stone.grade.display_name());
            self.blessed_item_names.insert(name.clone());
            return Some(name);
        }

        // 3. Check basic item stacks: convert 1 unit to a divine equipment relic
        let mut found_item = None;
        for (item_id, count) in &mut self.items {
            if (item_id.display_name().eq_ignore_ascii_case(target_name)
                || target_name.contains(item_id.display_name()))
                && *count > 0
            {
                *count -= 1;
                found_item = Some(*item_id);
                break;
            }
        }

        if let Some(item_id) = found_item {
            let blessed_equip = bless_basic_item(item_id, next_item_id);
            let blessed_name = blessed_equip.name.clone();
            self.blessed_item_names.insert(blessed_name.clone());
            self.equipment.push(blessed_equip);
            return Some(blessed_name);
        }

        // 4. Mark name as blessed if carried
        for item_id in self.items.keys() {
            let name = item_id.display_name();
            if name.eq_ignore_ascii_case(target_name) || target_name.contains(name) {
                let blessed_name = format!("✦ Sanctified {}", name);
                self.blessed_item_names.insert(blessed_name.clone());
                return Some(blessed_name);
            }
        }

        None
    }

    pub fn total_wealth(&self) -> f32 {
        let stack_wealth: f32 = self
            .items
            .iter()
            .map(|(id, count)| id.rarity_value() * (*count as f32))
            .sum();
        let equip_wealth: f32 = self.equipment.iter().map(|e| e.value as f32).sum();
        let stone_wealth: f32 = self
            .mana_stones
            .iter()
            .map(|s| s.monetary_value() as f32)
            .sum();
        stack_wealth + equip_wealth + stone_wealth
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecipeId {
    CraftPrimitiveChopper,
    CraftFirestriker,
    CraftStonePickaxe,
    CraftTimberPlanks,
    BakeBread,
    SmeltIronIngot,
    CraftIronPickaxe,
    CraftIronSword,
    InscribeSolarRune,
    InscribePurifyingRune,
    InscribeStoneWallRune,
    InscribeShadowVeilRune,
    BuildCampfire,
    BuildLeanTo,
    BuildStoneCabin,
    BuildGranary,
    ErectSolarAltar,
    ErectLunarAltar,
}

impl RecipeId {
    pub fn requirements(&self) -> &'static [(ItemId, u32)] {
        match self {
            RecipeId::CraftPrimitiveChopper => &[(ItemId::Wood, 1), (ItemId::Flint, 1)],
            RecipeId::CraftFirestriker => &[(ItemId::Flint, 1), (ItemId::Stone, 1)],
            RecipeId::CraftStonePickaxe => &[(ItemId::Wood, 2), (ItemId::Stone, 2)],
            RecipeId::CraftTimberPlanks => &[(ItemId::Wood, 3)],
            RecipeId::BakeBread => &[(ItemId::Grain, 3), (ItemId::FreshWater, 1)],
            RecipeId::SmeltIronIngot => &[(ItemId::IronOre, 2), (ItemId::Wood, 2)],
            RecipeId::CraftIronPickaxe => &[(ItemId::Wood, 2), (ItemId::IronIngot, 2)],
            RecipeId::CraftIronSword => &[(ItemId::Wood, 1), (ItemId::IronIngot, 3)],
            RecipeId::InscribeSolarRune => &[(ItemId::ClayTablet, 1), (ItemId::ManaCrystal, 1)],
            RecipeId::InscribePurifyingRune => &[
                (ItemId::ClayTablet, 1),
                (ItemId::ManaCrystal, 1),
                (ItemId::FreshWater, 1),
            ],
            RecipeId::InscribeStoneWallRune => &[(ItemId::ClayTablet, 1), (ItemId::Stone, 2)],
            RecipeId::InscribeShadowVeilRune => &[
                (ItemId::ClayTablet, 1),
                (ItemId::ManaCrystal, 1),
                (ItemId::Fur, 1),
            ],
            RecipeId::BuildCampfire => &[(ItemId::Wood, 3), (ItemId::Stone, 1)],
            RecipeId::BuildLeanTo => &[(ItemId::Wood, 5), (ItemId::Fur, 1)],
            RecipeId::BuildStoneCabin => {
                &[(ItemId::Stone, 10), (ItemId::Wood, 6), (ItemId::Clay, 3)]
            }
            RecipeId::BuildGranary => &[(ItemId::Wood, 12), (ItemId::Clay, 4)],
            RecipeId::ErectSolarAltar => &[(ItemId::Stone, 15), (ItemId::ManaCrystal, 1)],
            RecipeId::ErectLunarAltar => &[(ItemId::Stone, 15), (ItemId::ManaCrystal, 1)],
        }
    }

    pub fn output(&self) -> Option<(ItemId, u32)> {
        match self {
            RecipeId::CraftPrimitiveChopper => Some((ItemId::PrimitiveChopper, 1)),
            RecipeId::CraftFirestriker => Some((ItemId::Firestriker, 1)),
            RecipeId::CraftStonePickaxe => Some((ItemId::StonePickaxe, 1)),
            RecipeId::CraftTimberPlanks => Some((ItemId::TimberPlanks, 2)),
            RecipeId::BakeBread => Some((ItemId::Bread, 2)),
            RecipeId::SmeltIronIngot => Some((ItemId::IronIngot, 1)),
            RecipeId::CraftIronPickaxe => Some((ItemId::IronPickaxe, 1)),
            RecipeId::CraftIronSword => Some((ItemId::IronSword, 1)),
            RecipeId::InscribeSolarRune => Some((ItemId::RuneSolarIgnition, 1)),
            RecipeId::InscribePurifyingRune => Some((ItemId::RunePurifyingAura, 1)),
            RecipeId::InscribeStoneWallRune => Some((ItemId::RuneStoneWall, 1)),
            RecipeId::InscribeShadowVeilRune => Some((ItemId::RuneShadowVeil, 1)),
            // Structures don't produce inventory items; they are placed in world
            RecipeId::BuildCampfire
            | RecipeId::BuildLeanTo
            | RecipeId::BuildStoneCabin
            | RecipeId::BuildGranary
            | RecipeId::ErectSolarAltar
            | RecipeId::ErectLunarAltar => None,
        }
    }
}
