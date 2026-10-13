pub mod rival_god;

use serde::{Deserialize, Serialize};
use rival_god::RivalGodState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiplomaticStatus {
    PeacefulCoexistence,
    BorderTension,
    OpenWarfare,
    CommercialAlliance,
}

impl DiplomaticStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            DiplomaticStatus::PeacefulCoexistence => "Peaceful Coexistence",
            DiplomaticStatus::BorderTension => "Border Tension",
            DiplomaticStatus::OpenWarfare => "Open Warfare",
            DiplomaticStatus::CommercialAlliance => "Commercial Alliance",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CulturalTech {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub tier: u32,
    pub knowledge_required: u32,
    pub is_unlocked: bool,
    pub description: String,
    pub effect: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CivilizationCulture {
    pub faction_name: String,
    pub knowledge_points: u32,
    pub technologies: Vec<CulturalTech>,
    pub active_research: String,
    pub discoveries: Vec<String>,
}

impl CivilizationCulture {
    pub fn new(faction_name: impl Into<String>) -> Self {
        let name = faction_name.into();
        let techs = vec![
            CulturalTech {
                id: "toolmaking".into(),
                name: "Primitive Toolcraft".into(),
                icon: "🪓".into(),
                tier: 1,
                knowledge_required: 25,
                is_unlocked: false,
                description: "Knapping flint choppers and axes to accelerate resource gathering.".into(),
                effect: "+50% wood and foraging speed".into(),
            },
            CulturalTech {
                id: "fire_mastery".into(),
                name: "Fire Mastery & Hearth".into(),
                icon: "🔥".into(),
                tier: 1,
                knowledge_required: 50,
                is_unlocked: false,
                description: "Kindling village campfires for communal warmth and cooked meals.".into(),
                effect: "Enables Campfire construction and thermal protection".into(),
            },
            CulturalTech {
                id: "stone_masonry".into(),
                name: "Stone Masonry".into(),
                icon: "🏡".into(),
                tier: 2,
                knowledge_required: 100,
                is_unlocked: false,
                description: "Hewing granite and mortar into durable stone cabins.".into(),
                effect: "Enables Stone Cabin residences (+4 population capacity)".into(),
            },
            CulturalTech {
                id: "granary_farming".into(),
                name: "Granary & Agriculture".into(),
                icon: "🌾".into(),
                tier: 2,
                knowledge_required: 180,
                is_unlocked: false,
                description: "Stockpiling seasonal grain in communal wooden silos.".into(),
                effect: "Granaries preserve food rations across blizzards".into(),
            },
            CulturalTech {
                id: "iron_metallurgy".into(),
                name: "Iron Metallurgy".into(),
                icon: "⚔️".into(),
                tier: 3,
                knowledge_required: 280,
                is_unlocked: false,
                description: "Extracting raw iron ore from mountain crags to forge steel implements.".into(),
                effect: "Unlocks Iron Pickaxes and forged weapons".into(),
            },
            CulturalTech {
                id: "gold_currency".into(),
                name: "Gold Currency & Commerce".into(),
                icon: "🪙".into(),
                tier: 3,
                knowledge_required: 400,
                is_unlocked: false,
                description: "Minting gold bullion into standard coins for trade and dowries.".into(),
                effect: "Unlocks market trading and wealth-based courtship bonuses".into(),
            },
            CulturalTech {
                id: "sacred_theocracy".into(),
                name: "Sacred Theocracy".into(),
                icon: "🙏".into(),
                tier: 4,
                knowledge_required: 550,
                is_unlocked: false,
                description: "Solemn sacrificial altar ceremonies and divine covenant rituals.".into(),
                effect: "+100% divine blessing potency from altar offerings".into(),
            },
            CulturalTech {
                id: "arcane_code_weaving".into(),
                name: "Arcane Code Weaving".into(),
                icon: "🔮".into(),
                tier: 4,
                knowledge_required: 750,
                is_unlocked: false,
                description: "Infusing leyline mithril crystals into self-rewriting Gödel ASTs.".into(),
                effect: "Agents autonomously rewrite their decision code".into(),
            },
        ];

        Self {
            faction_name: name,
            knowledge_points: 0,
            technologies: techs,
            active_research: "Primitive Toolcraft".into(),
            discoveries: Vec::new(),
        }
    }

    pub fn add_knowledge(&mut self, amount: u32) -> Vec<String> {
        self.knowledge_points += amount;
        let mut newly_unlocked = Vec::new();

        for tech in self.technologies.iter_mut() {
            if !tech.is_unlocked && self.knowledge_points >= tech.knowledge_required {
                tech.is_unlocked = true;
                newly_unlocked.push(format!(
                    "🏆 [CULTURAL BREAKTHROUGH] {} unlocked '{}' (Tier {})! {}",
                    self.faction_name, tech.name, tech.tier, tech.effect
                ));
            }
        }

        // Update active research target
        if let Some(next_tech) = self.technologies.iter().find(|t| !t.is_unlocked) {
            self.active_research = next_tech.name.clone();
        } else {
            self.active_research = "Pinnacle of Civilization Achieved".into();
        }

        newly_unlocked
    }

    pub fn record_discovery(&mut self, discovery: impl Into<String>) {
        let text = discovery.into();
        if !self.discoveries.contains(&text) {
            self.discoveries.push(text);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CivilizationsCoordinator {
    pub rival_god: RivalGodState,
    pub diplomacy: DiplomaticStatus,
    pub trade_volume: u32,
    pub intermarriage_count: u32,
    pub terran_favor: f32, // Player God favor
    pub terran_culture: CivilizationCulture,
    pub lunar_culture: CivilizationCulture,
}

impl CivilizationsCoordinator {
    pub fn new() -> Self {
        Self {
            rival_god: RivalGodState::new(),
            diplomacy: DiplomaticStatus::PeacefulCoexistence,
            trade_volume: 0,
            intermarriage_count: 0,
            terran_favor: 70.0,
            terran_culture: CivilizationCulture::new("Kingdom of Solaria"),
            lunar_culture: CivilizationCulture::new("Lunar Sovereignty"),
        }
    }

    pub fn tick(&mut self, current_tick: u64, terran_count: usize, lunarian_count: usize) -> Option<String> {
        self.rival_god.evaluate_and_intervene(current_tick, terran_count, lunarian_count)
    }
}
