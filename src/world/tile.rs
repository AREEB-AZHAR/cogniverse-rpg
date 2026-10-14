use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ElevationLayer {
    DeepOcean,     // Height 0..30 (deep impassable ocean)
    CloseOcean,    // Height 30..70 (coastal water shelf)
    ShallowWaters, // Height 70..98 (wading, freezes to ice)
    Sand,          // Height 98..108 (coastal sand beach)
    SoilLow,       // Height 108..128 (lowland fertile soil)
    SoilHigh,      // Height 128..199 (elevated plains & plateaus)
    Hills,         // Height 199..210 (rocky foothills, holds lava, basalt)
    Mountains,     // Height 210..230 (steep cliffs, rich in ores)
    Summit,        // Height 230..255 (snow-capped mountain peaks)
}

impl ElevationLayer {
    pub fn from_elevation(elevation: f32) -> Self {
        if elevation < 0.12 {
            ElevationLayer::DeepOcean
        } else if elevation < 0.25 {
            ElevationLayer::CloseOcean
        } else if elevation < 0.38 {
            ElevationLayer::ShallowWaters
        } else if elevation < 0.44 {
            ElevationLayer::Sand
        } else if elevation < 0.58 {
            ElevationLayer::SoilLow
        } else if elevation < 0.76 {
            ElevationLayer::SoilHigh
        } else if elevation < 0.85 {
            ElevationLayer::Hills
        } else if elevation < 0.94 {
            ElevationLayer::Mountains
        } else {
            ElevationLayer::Summit
        }
    }

    pub fn is_water(&self) -> bool {
        matches!(self, ElevationLayer::DeepOcean | ElevationLayer::CloseOcean | ElevationLayer::ShallowWaters)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ElevationLayer::DeepOcean => "Deep Ocean",
            ElevationLayer::CloseOcean => "Close Ocean",
            ElevationLayer::ShallowWaters => "Shallow Waters",
            ElevationLayer::Sand => "Sand Beach",
            ElevationLayer::SoilLow => "Lowland Soil",
            ElevationLayer::SoilHigh => "Highland Plateau",
            ElevationLayer::Hills => "Foothills",
            ElevationLayer::Mountains => "High Mountain",
            ElevationLayer::Summit => "Snow Summit",
        }
    }

    pub fn color_hex(&self) -> &'static str {
        match self {
            ElevationLayer::DeepOcean => "#0f3871",
            ElevationLayer::CloseOcean => "#1963b3",
            ElevationLayer::ShallowWaters => "#38a3ee",
            ElevationLayer::Sand => "#f3e497",
            ElevationLayer::SoilLow => "#5ea338",
            ElevationLayer::SoilHigh => "#448324",
            ElevationLayer::Hills => "#5b5e5c",
            ElevationLayer::Mountains => "#414545",
            ElevationLayer::Summit => "#f8fafc",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrozenState {
    None,
    Ice,
    Snow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiomeType {
    SunlitPlains,
    WhisperingForest,
    CrystalRiverlands,
    IroncragMountains,
    AncientLeylineRuins,
    Savanna,
    Jungle,
    CorruptedInfernal,
    PermafrostTundra,
    Swamp,
}

impl BiomeType {
    pub fn display_name(&self) -> &'static str {
        match self {
            BiomeType::SunlitPlains => "Sunlit Plains",
            BiomeType::WhisperingForest => "Whispering Forest",
            BiomeType::CrystalRiverlands => "Crystal Riverlands",
            BiomeType::IroncragMountains => "Ironcrag Mountains",
            BiomeType::AncientLeylineRuins => "Ancient Leyline Ruins",
            BiomeType::Savanna => "Savanna",
            BiomeType::Jungle => "Lush Jungle",
            BiomeType::CorruptedInfernal => "Infernal Wasteland",
            BiomeType::PermafrostTundra => "Permafrost Tundra",
            BiomeType::Swamp => "Murky Swamp",
        }
    }

    pub fn color_hex(&self) -> &'static str {
        match self {
            BiomeType::SunlitPlains => "#4d7c0f",       // Lush grassy green
            BiomeType::WhisperingForest => "#14532d",   // Deep forest green
            BiomeType::CrystalRiverlands => "#0284c7",  // Azure water blue
            BiomeType::IroncragMountains => "#78716c",  // Stone mountain gray
            BiomeType::AncientLeylineRuins => "#7e22ce", // Arcane mystical purple
            BiomeType::Savanna => "#ca8a04",            // Golden savanna yellow
            BiomeType::Jungle => "#15803d",             // Vivid emerald jungle
            BiomeType::CorruptedInfernal => "#991b1b",   // Ash volcanic red
            BiomeType::PermafrostTundra => "#e0f2fe",   // Pale frosted snow
            BiomeType::Swamp => "#3f6212",              // Murky moss olive
        }
    }

    pub fn base_temperature(&self) -> f32 {
        match self {
            BiomeType::SunlitPlains => 20.0,
            BiomeType::WhisperingForest => 16.0,
            BiomeType::CrystalRiverlands => 18.0,
            BiomeType::IroncragMountains => 4.0,
            BiomeType::AncientLeylineRuins => 14.0,
            BiomeType::Savanna => 32.0,
            BiomeType::Jungle => 28.0,
            BiomeType::CorruptedInfernal => 45.0,
            BiomeType::PermafrostTundra => -8.0,
            BiomeType::Swamp => 22.0,
        }
    }

    pub fn grow_strength(&self) -> u8 {
        match self {
            BiomeType::SunlitPlains => 5,
            BiomeType::WhisperingForest => 6,
            BiomeType::Savanna => 5,
            BiomeType::Jungle => 7,
            BiomeType::AncientLeylineRuins => 8,
            BiomeType::CorruptedInfernal => 9,
            BiomeType::PermafrostTundra => 6,
            BiomeType::Swamp => 6,
            BiomeType::CrystalRiverlands => 0,
            BiomeType::IroncragMountains => 0,
        }
    }

    pub fn is_flammable(&self) -> bool {
        match self {
            BiomeType::WhisperingForest | BiomeType::SunlitPlains | BiomeType::Savanna | BiomeType::Jungle | BiomeType::Swamp => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub biome: BiomeType,
    pub elevation: f32,             // 0.0 to 1.0
    pub elevation_layer: ElevationLayer,
    pub moisture: f32,              // 0.0 to 1.0
    pub soil_fertility: f32,        // 0.0 to 1.0
    pub temperature: f32,           // Celsius
    pub mana_density: f32,          // 0.0 to 1.0 (Leyline concentration)
    
    // Cellular Automata Dynamic States
    pub is_on_fire: bool,
    pub fire_ticks: u16,
    pub burned_stages: u8,          // 0 = healthy, 1..5 = ash recovering
    pub lava_level: u8,             // 0 = none, 1..3 = molten lava flow
    pub frozen_state: FrozenState,  // None, Ice, Snow
    
    // Natural harvestable resources present on tile
    pub wood_amount: u32,
    pub stone_amount: u32,
    pub flint_amount: u32,
    pub iron_ore_amount: u32,
    pub gold_ore_amount: u32,
    pub mithril_amount: u32,
    pub grain_amount: u32,
    pub berry_amount: u32,
    pub has_fresh_water: bool,
    
    // Constructed structure on this tile (if any)
    pub structure: Option<PlacedStructure>,
}

impl Tile {
    pub fn movement_cost(&self) -> f32 {
        if self.lava_level > 0 {
            return 3.5;
        }
        if self.frozen_state == FrozenState::Ice {
            return 1.1; // Solid walkable ice!
        }
        match self.elevation_layer {
            ElevationLayer::DeepOcean | ElevationLayer::CloseOcean => 99.0, // Impassable water without boat or ice
            ElevationLayer::ShallowWaters => 2.0,                           // Wading
            ElevationLayer::Sand => 1.2,
            ElevationLayer::SoilLow | ElevationLayer::SoilHigh => 1.0,
            ElevationLayer::Hills => 1.5,
            ElevationLayer::Mountains => 2.2,
            ElevationLayer::Summit => 2.8,
        }
    }

    pub fn is_walkable(&self) -> bool {
        if self.frozen_state == FrozenState::Ice {
            return true;
        }
        !matches!(self.elevation_layer, ElevationLayer::DeepOcean | ElevationLayer::CloseOcean)
    }

    pub fn can_catch_fire(&self) -> bool {
        if self.elevation_layer.is_water() || self.lava_level > 0 || self.is_on_fire {
            return false;
        }
        if self.wood_amount > 0 || self.grain_amount > 0 || self.biome.is_flammable() {
            return true;
        }
        if let Some(ref s) = self.structure {
            matches!(s.kind, StructureKind::WoodCottage | StructureKind::Granary | StructureKind::Campfire)
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructureKind {
    Campfire,
    LeanTo,
    WoodCottage,
    StoneCabin,
    TownHall,
    Barracks,
    Granary,
    Watchtower,
    MineShaft,
    SolarAltar,
    LunarAltar,
    StoneWall,
}

impl StructureKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            StructureKind::Campfire => "Campfire",
            StructureKind::LeanTo => "Lean-To Shelter",
            StructureKind::WoodCottage => "Wood Cottage",
            StructureKind::StoneCabin => "Stone Cabin",
            StructureKind::TownHall => "Chieftain's Town Hall",
            StructureKind::Barracks => "Warrior Barracks",
            StructureKind::Granary => "Granary Silo",
            StructureKind::Watchtower => "Guard Watchtower",
            StructureKind::MineShaft => "Mountain Mine Shaft",
            StructureKind::SolarAltar => "Golden Sun Altar",
            StructureKind::LunarAltar => "Obsidian Moon Altar",
            StructureKind::StoneWall => "Fortified Stone Wall",
        }
    }

    pub fn housing_capacity(&self) -> usize {
        match self {
            StructureKind::TownHall => 8,
            StructureKind::Barracks => 6,
            StructureKind::StoneCabin => 4,
            StructureKind::WoodCottage => 2,
            StructureKind::LeanTo => 1,
            _ => 0,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            StructureKind::Campfire => "🔥",
            StructureKind::LeanTo => "⛺",
            StructureKind::WoodCottage => "🛖",
            StructureKind::StoneCabin => "🏡",
            StructureKind::TownHall => "🏛️",
            StructureKind::Barracks => "🛡️",
            StructureKind::Granary => "🌾",
            StructureKind::Watchtower => "🗼",
            StructureKind::MineShaft => "⛏️",
            StructureKind::SolarAltar => "☀️",
            StructureKind::LunarAltar => "🌙",
            StructureKind::StoneWall => "🧱",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlacedStructure {
    pub kind: StructureKind,
    pub owner_id: Option<usize>,
    pub health: f32,
    pub max_health: f32,
}

impl PlacedStructure {
    pub fn new(kind: StructureKind) -> Self {
        let max_hp = match kind {
            StructureKind::StoneCabin => 120.0,
            StructureKind::WoodCottage => 90.0,
            StructureKind::TownHall => 350.0,
            StructureKind::Barracks => 220.0,
            StructureKind::Granary => 100.0,
            StructureKind::Watchtower => 150.0,
            StructureKind::MineShaft => 200.0,
            StructureKind::SolarAltar | StructureKind::LunarAltar => 500.0,
            StructureKind::Campfire => 80.0,
            StructureKind::LeanTo => 50.0,
            StructureKind::StoneWall => 200.0,
        };
        Self {
            kind,
            owner_id: None,
            health: max_hp,
            max_health: max_hp,
        }
    }
}

impl Tile {
    pub fn new(x: usize, y: usize, biome: BiomeType, elevation: f32, moisture: f32) -> Self {
        let elevation_layer = ElevationLayer::from_elevation(elevation);
        let frozen_state = if elevation_layer == ElevationLayer::Summit {
            FrozenState::Snow
        } else {
            FrozenState::None
        };

        let mut tile = Self {
            x,
            y,
            biome,
            elevation,
            elevation_layer,
            moisture,
            soil_fertility: match biome {
                BiomeType::SunlitPlains => 0.9,
                BiomeType::WhisperingForest => 0.8,
                BiomeType::CrystalRiverlands => 0.85,
                BiomeType::Jungle => 0.95,
                BiomeType::Savanna => 0.65,
                BiomeType::Swamp => 0.88,
                BiomeType::IroncragMountains => 0.25,
                BiomeType::AncientLeylineRuins => 0.45,
                BiomeType::PermafrostTundra => 0.15,
                BiomeType::CorruptedInfernal => 0.10,
            },
            temperature: biome.base_temperature(),
            mana_density: match biome {
                BiomeType::AncientLeylineRuins => 0.95,
                BiomeType::CorruptedInfernal => 0.85,
                BiomeType::CrystalRiverlands => 0.6,
                BiomeType::WhisperingForest => 0.4,
                BiomeType::IroncragMountains => 0.35,
                _ => 0.2,
            },
            is_on_fire: false,
            fire_ticks: 0,
            burned_stages: 0,
            lava_level: 0,
            frozen_state,
            wood_amount: 0,
            stone_amount: 0,
            flint_amount: 0,
            iron_ore_amount: 0,
            gold_ore_amount: 0,
            mithril_amount: 0,
            grain_amount: 0,
            berry_amount: 0,
            has_fresh_water: false,
            structure: None,
        };

        // Populate natural resources according to biome and elevation layer
        if elevation_layer == ElevationLayer::DeepOcean || elevation_layer == ElevationLayer::CloseOcean {
            // Salt ocean water - cannot be drunk, no surface trees or land berries
            tile.has_fresh_water = false;
            tile.wood_amount = 0;
            tile.berry_amount = 0;
            tile.grain_amount = 0;
            return tile;
        }

        match biome {
            BiomeType::SunlitPlains => {
                tile.grain_amount = 18;
                tile.berry_amount = 12;
                tile.wood_amount = 4;
                tile.flint_amount = 4;
            }
            BiomeType::WhisperingForest => {
                tile.wood_amount = 40;
                tile.berry_amount = 20;
                tile.flint_amount = 6;
                tile.stone_amount = 6;
            }
            BiomeType::CrystalRiverlands => {
                tile.has_fresh_water = true;
                tile.wood_amount = 10;
                tile.berry_amount = 10;
                tile.flint_amount = 8;
                tile.stone_amount = 6;
            }
            BiomeType::Savanna => {
                tile.grain_amount = 12;
                tile.wood_amount = 15;
                tile.stone_amount = 8;
                tile.flint_amount = 6;
            }
            BiomeType::Jungle => {
                tile.wood_amount = 50;
                tile.berry_amount = 25;
                tile.grain_amount = 10;
                tile.flint_amount = 5;
            }
            BiomeType::IroncragMountains => {
                tile.stone_amount = 45;
                tile.flint_amount = 18;
                tile.iron_ore_amount = 30;
                tile.gold_ore_amount = 14;
                tile.wood_amount = 3;
                // Mountain cave berries, lichen, and underground springs
                // prevent Lunarians from starving or fleeing across the continent to (0, 0)!
                tile.berry_amount = 8; 
                tile.has_fresh_water = (x + y) % 3 == 0;
            }
            BiomeType::AncientLeylineRuins => {
                tile.stone_amount = 25;
                tile.flint_amount = 10;
                tile.iron_ore_amount = 8;
                tile.gold_ore_amount = 10;
                tile.mithril_amount = 22;
                tile.berry_amount = 6;
                tile.has_fresh_water = (x + y) % 4 == 0;
            }
            BiomeType::CorruptedInfernal => {
                tile.stone_amount = 20;
                tile.iron_ore_amount = 15;
                tile.gold_ore_amount = 10;
                tile.mithril_amount = 8;
            }
            BiomeType::PermafrostTundra => {
                tile.stone_amount = 15;
                tile.flint_amount = 12;
                tile.berry_amount = 4;
            }
            BiomeType::Swamp => {
                tile.has_fresh_water = true;
                tile.wood_amount = 25;
                tile.berry_amount = 14;
                tile.flint_amount = 8;
            }
        }

        tile
    }
}
