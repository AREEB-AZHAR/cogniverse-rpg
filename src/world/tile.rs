use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiomeType {
    SunlitPlains,
    WhisperingForest,
    CrystalRiverlands,
    IroncragMountains,
    AncientLeylineRuins,
}

impl BiomeType {
    pub fn display_name(&self) -> &'static str {
        match self {
            BiomeType::SunlitPlains => "Sunlit Plains",
            BiomeType::WhisperingForest => "Whispering Forest",
            BiomeType::CrystalRiverlands => "Crystal Riverlands",
            BiomeType::IroncragMountains => "Ironcrag Mountains",
            BiomeType::AncientLeylineRuins => "Ancient Leyline Ruins",
        }
    }

    pub fn color_hex(&self) -> &'static str {
        match self {
            BiomeType::SunlitPlains => "#4d7c0f",       // Lush grassy green
            BiomeType::WhisperingForest => "#14532d",   // Deep forest green
            BiomeType::CrystalRiverlands => "#0284c7",  // Azure water blue
            BiomeType::IroncragMountains => "#78716c",  // Stone mountain gray
            BiomeType::AncientLeylineRuins => "#7e22ce", // Arcane mystical purple
        }
    }

    pub fn base_temperature(&self) -> f32 {
        match self {
            BiomeType::SunlitPlains => 20.0,
            BiomeType::WhisperingForest => 16.0,
            BiomeType::CrystalRiverlands => 18.0,
            BiomeType::IroncragMountains => 6.0,
            BiomeType::AncientLeylineRuins => 14.0,
        }
    }

    pub fn movement_cost(&self) -> f32 {
        match self {
            BiomeType::SunlitPlains => 1.0,
            BiomeType::WhisperingForest => 1.4,
            BiomeType::CrystalRiverlands => 2.0, // Water wading
            BiomeType::IroncragMountains => 2.2, // Steep rocks
            BiomeType::AncientLeylineRuins => 1.1,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub biome: BiomeType,
    pub elevation: f32,       // 0.0 to 1.0
    pub moisture: f32,        // 0.0 to 1.0
    pub soil_fertility: f32,  // 0.0 to 1.0
    pub temperature: f32,     // Celsius
    pub mana_density: f32,    // 0.0 to 1.0 (Leyline concentration)
    
    // Natural harvestable resources present on tile
    pub wood_amount: u32,
    pub stone_amount: u32,
    pub flint_amount: u32,
    pub iron_ore_amount: u32,
    pub grain_amount: u32,
    pub berry_amount: u32,
    pub has_fresh_water: bool,
    
    // Constructed structure on this tile (if any)
    pub structure: Option<PlacedStructure>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructureKind {
    Campfire,
    LeanTo,
    StoneCabin,
    Granary,
    SolarAltar,
    LunarAltar,
    StoneWall,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlacedStructure {
    pub kind: StructureKind,
    pub owner_id: Option<usize>,
    pub health: f32,
    pub max_health: f32,
}

impl Tile {
    pub fn new(x: usize, y: usize, biome: BiomeType, elevation: f32, moisture: f32) -> Self {
        let mut tile = Self {
            x,
            y,
            biome,
            elevation,
            moisture,
            soil_fertility: match biome {
                BiomeType::SunlitPlains => 0.9,
                BiomeType::WhisperingForest => 0.8,
                BiomeType::CrystalRiverlands => 0.85,
                BiomeType::IroncragMountains => 0.2,
                BiomeType::AncientLeylineRuins => 0.4,
            },
            temperature: biome.base_temperature(),
            mana_density: match biome {
                BiomeType::AncientLeylineRuins => 0.95,
                BiomeType::CrystalRiverlands => 0.6,
                BiomeType::WhisperingForest => 0.4,
                BiomeType::IroncragMountains => 0.3,
                BiomeType::SunlitPlains => 0.2,
            },
            wood_amount: 0,
            stone_amount: 0,
            flint_amount: 0,
            iron_ore_amount: 0,
            grain_amount: 0,
            berry_amount: 0,
            has_fresh_water: false,
            structure: None,
        };

        // Populate natural resources according to biome
        match biome {
            BiomeType::SunlitPlains => {
                tile.grain_amount = 15;
                tile.berry_amount = 10;
                tile.wood_amount = 3;
                tile.flint_amount = 4;
            }
            BiomeType::WhisperingForest => {
                tile.wood_amount = 35;
                tile.berry_amount = 18;
                tile.flint_amount = 5;
                tile.stone_amount = 5;
            }
            BiomeType::CrystalRiverlands => {
                tile.has_fresh_water = true;
                tile.wood_amount = 8;
                tile.berry_amount = 8;
                tile.flint_amount = 8;
                tile.stone_amount = 6;
            }
            BiomeType::IroncragMountains => {
                tile.stone_amount = 40;
                tile.flint_amount = 15;
                tile.iron_ore_amount = 25;
                tile.wood_amount = 2;
            }
            BiomeType::AncientLeylineRuins => {
                tile.stone_amount = 20;
                tile.flint_amount = 10;
                tile.iron_ore_amount = 5;
            }
        }

        tile
    }
}
