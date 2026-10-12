use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemId {
    // Basic Raw Materials
    Wood,
    Stone,
    Flint,
    IronOre,
    Grain,
    Berry,
    FreshWater,
    Fur,
    Clay,
    ManaCrystal,
    
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
            ItemId::Flint | ItemId::Clay | ItemId::Fur => 4.0,
            ItemId::TimberPlanks | ItemId::Bread => 5.0,
            ItemId::PrimitiveChopper | ItemId::Firestriker => 8.0,
            ItemId::StonePickaxe => 10.0,
            ItemId::IronOre => 12.0,
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub items: std::collections::HashMap<ItemId, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: std::collections::HashMap::new(),
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

    pub fn total_wealth(&self) -> f32 {
        self.items.iter().map(|(id, count)| id.rarity_value() * (*count as f32)).sum()
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
            RecipeId::InscribePurifyingRune => &[(ItemId::ClayTablet, 1), (ItemId::ManaCrystal, 1), (ItemId::FreshWater, 1)],
            RecipeId::InscribeStoneWallRune => &[(ItemId::ClayTablet, 1), (ItemId::Stone, 2)],
            RecipeId::InscribeShadowVeilRune => &[(ItemId::ClayTablet, 1), (ItemId::ManaCrystal, 1), (ItemId::Fur, 1)],
            RecipeId::BuildCampfire => &[(ItemId::Wood, 3), (ItemId::Stone, 1)],
            RecipeId::BuildLeanTo => &[(ItemId::Wood, 5), (ItemId::Fur, 1)],
            RecipeId::BuildStoneCabin => &[(ItemId::Stone, 10), (ItemId::Wood, 6), (ItemId::Clay, 3)],
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
            RecipeId::BuildCampfire | RecipeId::BuildLeanTo | RecipeId::BuildStoneCabin |
            RecipeId::BuildGranary | RecipeId::ErectSolarAltar | RecipeId::ErectLunarAltar => None,
        }
    }
}
