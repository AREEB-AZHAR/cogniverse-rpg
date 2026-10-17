use rand::Rng;
use serde::{Deserialize, Serialize};

/// 7 Core RPG Attributes for characters in the simulation
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub strength: f32,     // Melee damage, carrying capacity, framing labor speed
    pub agility: f32,      // Movement speed, dodge chance, projectile accuracy
    pub vitality: f32,     // Maximum health, health regeneration, poison/plague resistance
    pub intelligence: f32, // Research progression speed, arcane power
    pub wisdom: f32,       // Mana pool, hazard & danger perception
    pub charisma: f32,     // Market trading margins, social bonding
    pub luck: f32,         // Critical hits, rare gathering finds, divine blessing luck
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            strength: 10.0,
            agility: 10.0,
            vitality: 10.0,
            intelligence: 10.0,
            wisdom: 10.0,
            charisma: 10.0,
            luck: 10.0,
        }
    }
}

/// Canonical hero and citizen ranks from F (Common) to SSS (Transcendent Chosen)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Rank {
    F,
    E,
    D,
    C,
    B,
    A,
    S,
    SPlus,
    SSS,
}

impl Rank {
    pub const ALL: [Rank; 9] = [
        Rank::F,
        Rank::E,
        Rank::D,
        Rank::C,
        Rank::B,
        Rank::A,
        Rank::S,
        Rank::SPlus,
        Rank::SSS,
    ];

    pub const CANONICAL_WEIGHTS: [u32; 9] = [300, 250, 180, 120, 80, 40, 20, 9, 1]; // sum = 1000

    pub fn display_name(&self) -> &'static str {
        match self {
            Rank::F => "F-Rank (Novice)",
            Rank::E => "E-Rank (Apprentice)",
            Rank::D => "D-Rank (Veteran)",
            Rank::C => "C-Rank (Elite)",
            Rank::B => "B-Rank (Champion)",
            Rank::A => "A-Rank (Master)",
            Rank::S => "S-Rank (Heroic)",
            Rank::SPlus => "S+-Rank (Mythic)",
            Rank::SSS => "SSS-Rank (Transcendent)",
        }
    }

    /// Stat growth multiplier applied to baseline attributes
    pub fn stat_multiplier(&self) -> f32 {
        match self {
            Rank::F => 1.0,
            Rank::E => 1.2,
            Rank::D => 1.5,
            Rank::C => 1.8,
            Rank::B => 2.2,
            Rank::A => 2.8,
            Rank::S => 3.5,
            Rank::SPlus => 4.2,
            Rank::SSS => 5.0,
        }
    }

    /// Maximum achievable level cap under this blessing rank
    pub fn level_cap(&self) -> u32 {
        match self {
            Rank::F => 20,
            Rank::E => 30,
            Rank::D => 40,
            Rank::C => 50,
            Rank::B => 65,
            Rank::A => 80,
            Rank::S => 95,
            Rank::SPlus => 110,
            Rank::SSS => 150,
        }
    }
}

/// Rolls a rank using weighted probabilities as specified in §8
pub fn roll_rank(rng: &mut impl Rng, weights: &[u32; 9]) -> Rank {
    let total: u32 = weights.iter().sum();
    debug_assert!(total > 0, "Rank roll weights must sum to > 0");

    let mut pick = rng.gen_range(0..total);
    for (rank, w) in Rank::ALL.iter().zip(weights) {
        if pick < *w {
            return *rank;
        }
        pick -= *w;
    }
    Rank::F
}

/// Primary vocation class rolled during the Temple Coming-of-Age blessing
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharacterClass {
    // Combat classes
    Warrior,
    Archer,
    Mage,
    Rogue,
    Cleric,
    // Crafting & Production classes
    Farmer,
    Miner,
    Woodcutter,
    Builder,
    Blacksmith,
    Alchemist,
    Carpenter,
    Shipwright,
    Cook,
}

impl CharacterClass {
    pub const ALL: [CharacterClass; 14] = [
        CharacterClass::Warrior,
        CharacterClass::Archer,
        CharacterClass::Mage,
        CharacterClass::Rogue,
        CharacterClass::Cleric,
        CharacterClass::Farmer,
        CharacterClass::Miner,
        CharacterClass::Woodcutter,
        CharacterClass::Builder,
        CharacterClass::Blacksmith,
        CharacterClass::Alchemist,
        CharacterClass::Carpenter,
        CharacterClass::Shipwright,
        CharacterClass::Cook,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            CharacterClass::Warrior => "Warrior",
            CharacterClass::Archer => "Archer",
            CharacterClass::Mage => "Mage",
            CharacterClass::Rogue => "Rogue",
            CharacterClass::Cleric => "Cleric",
            CharacterClass::Farmer => "Farmer",
            CharacterClass::Miner => "Miner",
            CharacterClass::Woodcutter => "Woodcutter",
            CharacterClass::Builder => "Builder",
            CharacterClass::Blacksmith => "Blacksmith",
            CharacterClass::Alchemist => "Alchemist",
            CharacterClass::Carpenter => "Carpenter",
            CharacterClass::Shipwright => "Shipwright",
            CharacterClass::Cook => "Cook",
        }
    }

    pub fn is_combat(&self) -> bool {
        matches!(
            self,
            CharacterClass::Warrior
                | CharacterClass::Archer
                | CharacterClass::Mage
                | CharacterClass::Rogue
                | CharacterClass::Cleric
        )
    }
}

/// Advanced subclass specialization unlocked at Level 25+
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Subclass {
    // Combat
    Paladin,
    Berserker,
    Sniper,
    Ranger,
    Elementalist,
    Arcanist,
    Assassin,
    ShadowDancer,
    HighPriest,
    Inquisitor,
    // Crafting
    MasterAgronomist,
    DeepDelver,
    Forester,
    MasterArchitect,
    RuneSmith,
    GrandApothecary,
    ArtisanWoodworker,
    NavalArchitect,
    MasterCaterer,
}

impl Subclass {
    pub fn display_name(&self) -> &'static str {
        match self {
            Subclass::Paladin => "Holy Paladin",
            Subclass::Berserker => "Berserker",
            Subclass::Sniper => "Sniper",
            Subclass::Ranger => "Wilderness Ranger",
            Subclass::Elementalist => "Elementalist",
            Subclass::Arcanist => "Grand Arcanist",
            Subclass::Assassin => "Shadow Assassin",
            Subclass::ShadowDancer => "Shadow Dancer",
            Subclass::HighPriest => "High Priest",
            Subclass::Inquisitor => "Inquisitor",
            Subclass::MasterAgronomist => "Master Agronomist",
            Subclass::DeepDelver => "Deep Delver",
            Subclass::Forester => "Master Forester",
            Subclass::MasterArchitect => "Master Architect",
            Subclass::RuneSmith => "Rune Smith",
            Subclass::GrandApothecary => "Grand Apothecary",
            Subclass::ArtisanWoodworker => "Artisan Woodworker",
            Subclass::NavalArchitect => "Naval Architect",
            Subclass::MasterCaterer => "Master Caterer",
        }
    }

    pub fn default_for_class(class: CharacterClass) -> Self {
        match class {
            CharacterClass::Warrior => Subclass::Paladin,
            CharacterClass::Archer => Subclass::Sniper,
            CharacterClass::Mage => Subclass::Elementalist,
            CharacterClass::Rogue => Subclass::Assassin,
            CharacterClass::Cleric => Subclass::HighPriest,
            CharacterClass::Farmer => Subclass::MasterAgronomist,
            CharacterClass::Miner => Subclass::DeepDelver,
            CharacterClass::Woodcutter => Subclass::Forester,
            CharacterClass::Builder => Subclass::MasterArchitect,
            CharacterClass::Blacksmith => Subclass::RuneSmith,
            CharacterClass::Alchemist => Subclass::GrandApothecary,
            CharacterClass::Carpenter => Subclass::ArtisanWoodworker,
            CharacterClass::Shipwright => Subclass::NavalArchitect,
            CharacterClass::Cook => Subclass::MasterCaterer,
        }
    }
}

/// Character heroic progression state tracking levels, XP, rank, and attributes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeroicProgression {
    pub level: u32,
    pub current_xp: u32,
    pub xp_for_next_level: u32,
    pub rank: Option<Rank>,
    pub class: Option<CharacterClass>,
    pub subclass: Option<Subclass>,
    pub attributes: Attributes,
    pub blessed_at_tick: Option<u64>,
}

impl Default for HeroicProgression {
    fn default() -> Self {
        Self::new()
    }
}

impl HeroicProgression {
    pub fn new() -> Self {
        Self {
            level: 1,
            current_xp: 0,
            xp_for_next_level: 100,
            rank: None,
            class: None,
            subclass: None,
            attributes: Attributes::default(),
            blessed_at_tick: None,
        }
    }

    /// Bestows a divine Temple Blessing at age 16: rolls rank, assigns class, scales stats
    pub fn bless_at_temple(
        &mut self,
        rng: &mut impl Rng,
        weights: &[u32; 9],
        chosen_class: CharacterClass,
        tick: u64,
    ) -> Rank {
        let rolled_rank = roll_rank(rng, weights);
        let mult = rolled_rank.stat_multiplier();

        self.rank = Some(rolled_rank);
        self.class = Some(chosen_class);
        self.blessed_at_tick = Some(tick);

        // Apply rank multipliers to baseline attributes
        self.attributes.strength *= mult;
        self.attributes.agility *= mult;
        self.attributes.vitality *= mult;
        self.attributes.intelligence *= mult;
        self.attributes.wisdom *= mult;
        self.attributes.charisma *= mult;
        self.attributes.luck *= mult;

        rolled_rank
    }

    /// Awards experience points and handles level ups & subclass unlocking
    pub fn gain_xp(&mut self, amount: u32) -> bool {
        let max_lvl = self.rank.map_or(20, |r| r.level_cap());
        if self.level >= max_lvl {
            return false;
        }

        self.current_xp += amount;
        let mut leveled_up = false;

        while self.current_xp >= self.xp_for_next_level && self.level < max_lvl {
            self.current_xp -= self.xp_for_next_level;
            self.level += 1;
            leveled_up = true;
            self.xp_for_next_level = (self.xp_for_next_level as f32 * 1.25) as u32;

            // Stat growth per level scaled by rank
            let growth = self.rank.map_or(1.0, |r| r.stat_multiplier());
            self.attributes.strength += 1.5 * growth;
            self.attributes.agility += 1.5 * growth;
            self.attributes.vitality += 2.0 * growth;
            self.attributes.intelligence += 1.5 * growth;
            self.attributes.wisdom += 1.5 * growth;

            // Check for subclass unlock at Level 25
            if self.level >= 25 && self.subclass.is_none() {
                if let Some(c) = self.class {
                    self.subclass = Some(Subclass::default_for_class(c));
                }
            }
        }

        leveled_up
    }
}
