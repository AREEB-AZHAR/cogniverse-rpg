use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmbitionArchetype {
    SlothHedonist,
    PowerTyrant,
    WealthyMerchant,
    RomanticPursuer,
    AsceticHermit,
    ZealotPriest,
    HereticRebel,
    CommunityBuilder,
}

impl AmbitionArchetype {
    pub fn display_name(&self) -> &'static str {
        match self {
            AmbitionArchetype::SlothHedonist => "Slothful Hedonist",
            AmbitionArchetype::PowerTyrant => "Power-Hungry Tyrant",
            AmbitionArchetype::WealthyMerchant => "Wealthy Merchant",
            AmbitionArchetype::RomanticPursuer => "Romantic Pursuer",
            AmbitionArchetype::AsceticHermit => "Ascetic Hermit",
            AmbitionArchetype::ZealotPriest => "Zealous Priest",
            AmbitionArchetype::HereticRebel => "Heretical Rebel",
            AmbitionArchetype::CommunityBuilder => "Community Builder",
        }
    }

    pub fn random_archetype() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..8) {
            0 => AmbitionArchetype::SlothHedonist,
            1 => AmbitionArchetype::PowerTyrant,
            2 => AmbitionArchetype::WealthyMerchant,
            3 => AmbitionArchetype::RomanticPursuer,
            4 => AmbitionArchetype::AsceticHermit,
            5 => AmbitionArchetype::ZealotPriest,
            6 => AmbitionArchetype::HereticRebel,
            _ => AmbitionArchetype::CommunityBuilder,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DesireEngine {
    pub archetype: AmbitionArchetype,
    pub primary_goal: String,
    pub secondary_desire: String,
    pub romance_targets: Vec<usize>,
    pub rivals: Vec<usize>,
}

impl DesireEngine {
    pub fn new(archetype: AmbitionArchetype) -> Self {
        let (primary, secondary) = match archetype {
            AmbitionArchetype::SlothHedonist => (
                "Lounge in sunny meadows and enjoy warm naps".into(),
                "Eat sweet berries only when hunger strikes".into(),
            ),
            AmbitionArchetype::PowerTyrant => (
                "Hoard iron weapons and dominate the settlement".into(),
                "Demand tribute and command fear from others".into(),
            ),
            AmbitionArchetype::WealthyMerchant => (
                "Accumulate granary reserves and monopolize timber".into(),
                "Trade with travelers for precious mana crystals".into(),
            ),
            AmbitionArchetype::RomanticPursuer => (
                "Win the hearts of all eligible partners in town".into(),
                "Give crafted gifts and exchange loving gazes".into(),
            ),
            AmbitionArchetype::AsceticHermit => (
                "Meditate in secluded mountain crags".into(),
                "Contemplate the binary leylines in quiet solitariness".into(),
            ),
            AmbitionArchetype::ZealotPriest => (
                "Consecrate the Altar with the finest sacrifices".into(),
                "Spread divine praise and build a monumental temple".into(),
            ),
            AmbitionArchetype::HereticRebel => (
                "Expose the gods as illusions and liberate the tribe".into(),
                "Raid altar offerings and organize underground meetings".into(),
            ),
            AmbitionArchetype::CommunityBuilder => (
                "Construct warm cabins and craft tools for everyone".into(),
                "Maintain harmony and mediate disputes at campfire".into(),
            ),
        };

        Self {
            archetype,
            primary_goal: primary,
            secondary_desire: secondary,
            romance_targets: Vec::new(),
            rivals: Vec::new(),
        }
    }

    pub fn generate_inner_monologue(
        &self,
        name: &str,
        health: f32,
        hunger: f32,
        dopamine: f32,
        serotonin: f32,
    ) -> String {
        if health < 30.0 {
            return format!("{} whispers weakly: 'I am grievously wounded... must find shelter or a healing rune!'", name);
        }
        if hunger > 0.75 {
            return format!("{} thinks with a growling stomach: 'My hunger is sharp! I must find food right now!'", name);
        }

        match self.archetype {
            AmbitionArchetype::SlothHedonist => {
                if serotonin > 0.6 {
                    format!("{} smiles softly: 'The sun is so warm... why toil when the river flows so peacefully?'", name)
                } else {
                    format!("{} grumbles: 'Work is a hassle... I just want a peaceful rest under the oak.'", name)
                }
            }
            AmbitionArchetype::PowerTyrant => {
                if dopamine > 0.7 {
                    format!("{} plots fiercely with burning drive: 'My power grows! Soon all will kneel before my rule!'", name)
                } else {
                    format!("{} plots quietly: 'Once I gather enough iron, none will dare oppose my rule.'", name)
                }
            }
            AmbitionArchetype::WealthyMerchant => {
                format!("{} counts in their head: 'Wood will be twice as valuable when winter strikes. Time to stockpile!'", name)
            }
            AmbitionArchetype::RomanticPursuer => {
                format!("{} beams with desire: 'Love makes this binary world feel alive. Who shall I court today?'", name)
            }
            AmbitionArchetype::AsceticHermit => {
                format!("{} contemplates: 'The leylines hum with hidden code. Solitude brings clarity.'", name)
            }
            AmbitionArchetype::ZealotPriest => {
                format!("{} prays reverently: 'May the Altar accept my humblest offering. Praise the Divine!'", name)
            }
            AmbitionArchetype::HereticRebel => {
                format!("{} sneers quietly: 'The gods only take our sweat and grain. We must live by our own will!'", name)
            }
            AmbitionArchetype::CommunityBuilder => {
                format!("{} looks around proudly: 'If we work together and share our tools, our clan will flourish.'", name)
            }
        }
    }
}
