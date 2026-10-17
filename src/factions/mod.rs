pub mod guild;
pub mod plots;
pub mod rival_god;
pub mod warfare;

use crate::agent::progression::Rank;
use crate::agent::CivilizationRace;
use guild::AdventurerGuild;
use plots::{CivilPlot, PlotType};
use rival_god::RivalGodState;
use serde::{Deserialize, Serialize};
pub use warfare::{SiegeEngineEntity, SiegeEngineKind, TerritoryGrid, WarCampaign};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiplomaticStatus {
    PeacefulCoexistence,
    BorderTension,
    OpenWarfare,
    CommercialAlliance,
    MutualDefensePact,
    GrandCoalition,
}

impl DiplomaticStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            DiplomaticStatus::PeacefulCoexistence => "Peaceful Coexistence",
            DiplomaticStatus::BorderTension => "Border Tension",
            DiplomaticStatus::OpenWarfare => "Open Warfare",
            DiplomaticStatus::CommercialAlliance => "Commercial Alliance",
            DiplomaticStatus::MutualDefensePact => "🛡️ Mutual Defense Pact",
            DiplomaticStatus::GrandCoalition => "⚔️ Grand Anti-Calamity Coalition",
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
                description: "Knapping flint choppers and axes to accelerate resource gathering."
                    .into(),
                effect: "+50% wood and foraging speed".into(),
            },
            CulturalTech {
                id: "fire_mastery".into(),
                name: "Fire Mastery & Hearth".into(),
                icon: "🔥".into(),
                tier: 1,
                knowledge_required: 50,
                is_unlocked: false,
                description: "Kindling village campfires for communal warmth and cooked meals."
                    .into(),
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
                description:
                    "Extracting raw iron ore from mountain crags to forge steel implements.".into(),
                effect: "Unlocks Iron Pickaxes and forged weapons".into(),
            },
            CulturalTech {
                id: "gold_currency".into(),
                name: "Gold Currency & Commerce".into(),
                icon: "🪙".into(),
                tier: 3,
                knowledge_required: 400,
                is_unlocked: false,
                description: "Minting gold bullion into standard coins for trade and dowries."
                    .into(),
                effect: "Unlocks market trading and wealth-based courtship bonuses".into(),
            },
            CulturalTech {
                id: "sacred_theocracy".into(),
                name: "Sacred Theocracy".into(),
                icon: "🙏".into(),
                tier: 4,
                knowledge_required: 550,
                is_unlocked: false,
                description: "Solemn sacrificial altar ceremonies and divine covenant rituals."
                    .into(),
                effect: "+100% divine blessing potency from altar offerings".into(),
            },
            CulturalTech {
                id: "arcane_code_weaving".into(),
                name: "Arcane Code Weaving".into(),
                icon: "🔮".into(),
                tier: 4,
                knowledge_required: 750,
                is_unlocked: false,
                description: "Infusing leyline mithril crystals into self-rewriting Gödel ASTs."
                    .into(),
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
    pub terran_capital: (usize, usize),
    pub lunar_capital: (usize, usize),
    pub terran_treasury: u32,
    pub lunar_treasury: u32,
    pub active_plots: Vec<CivilPlot>,
    pub next_plot_id: usize,
    pub guild: AdventurerGuild,
    pub warfare: WarCampaign,
}

impl Default for CivilizationsCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl CivilizationsCoordinator {
    pub fn new() -> Self {
        let plots = vec![
            CivilPlot::new(
                1,
                7,
                "Theron",
                CivilizationRace::SolarTerran,
                PlotType::CommercialAlliance,
                "Grand Trans-Continental Caravan Pact",
                "Establish duty-free timber and ore trading route across Crystal River",
            ),
            CivilPlot::new(
                2,
                12,
                "Corvus",
                CivilizationRace::ChthonicLunarian,
                PlotType::HeresySchism,
                "Cult of the Midnight Leyline",
                "Consecrate ancient leyline ruins independent of orthodox Moon Altar priesthood",
            ),
        ];

        Self {
            rival_god: RivalGodState::new(),
            diplomacy: DiplomaticStatus::PeacefulCoexistence,
            trade_volume: 0,
            intermarriage_count: 0,
            terran_favor: 70.0,
            terran_culture: CivilizationCulture::new("Kingdom of Solaria"),
            lunar_culture: CivilizationCulture::new("Lunar Sovereignty"),
            terran_capital: (38, 86),
            lunar_capital: (92, 34),
            terran_treasury: 250,
            lunar_treasury: 250,
            active_plots: plots,
            next_plot_id: 3,
            guild: AdventurerGuild::new(
                "Grand Pan-Continental Adventurer Guild",
                "Neutral Concord",
                65,
                60,
            ),
            warfare: WarCampaign::new(128, 128),
        }
    }

    pub fn tick(
        &mut self,
        current_tick: u64,
        terran_count: usize,
        lunarian_count: usize,
    ) -> Option<String> {
        self.rival_god
            .evaluate_and_intervene(current_tick, terran_count, lunarian_count)
    }

    pub fn hatch_plot(
        &mut self,
        initiator_id: usize,
        initiator_name: impl Into<String>,
        initiator_race: CivilizationRace,
        plot_type: PlotType,
        title: impl Into<String>,
        target_desc: impl Into<String>,
    ) {
        let plot = CivilPlot::new(
            self.next_plot_id,
            initiator_id,
            initiator_name,
            initiator_race,
            plot_type,
            title,
            target_desc,
        );
        self.next_plot_id += 1;
        self.active_plots.push(plot);
        if self.active_plots.len() > 10 {
            self.active_plots.remove(0);
        }
    }

    pub fn tick_plots(&mut self) -> Vec<String> {
        let mut events = Vec::new();
        for plot in self.active_plots.iter_mut() {
            if plot.is_resolved {
                continue;
            }
            // Advance progress gradually based on supporters count
            let rate = 0.35 + (plot.supporter_ids.len() as f32 * 0.15);
            if plot.advance_progress(rate) {
                match plot.plot_type {
                    PlotType::CommercialAlliance => {
                        self.diplomacy = DiplomaticStatus::CommercialAlliance;
                        self.terran_treasury += 150;
                        self.lunar_treasury += 150;
                        events.push(format!(
                            "📜 [PLOT RESOLVED] {}'s plot '{}' SUCCEEDED! Realms entered Commercial Alliance (+150 gold to treasuries).",
                            plot.initiator_name, plot.title
                        ));
                    }
                    PlotType::Rebellion => {
                        self.diplomacy = DiplomaticStatus::BorderTension;
                        events.push(format!(
                            "⚔️ [PLOT RESOLVED] {}'s rebellion '{}' IGNITED! Provincial unrest flares up along realm borders.",
                            plot.initiator_name, plot.title
                        ));
                    }
                    PlotType::HolyWar => {
                        self.diplomacy = DiplomaticStatus::OpenWarfare;
                        events.push(format!(
                            "⚡ [PLOT RESOLVED] {}'s Crusade '{}' COMMENCED! Open warfare erupts between Solaria and Lunar Sovereignty!",
                            plot.initiator_name, plot.title
                        ));
                    }
                    PlotType::HeresySchism => {
                        events.push(format!(
                            "🔥 [PLOT RESOLVED] {}'s Heretical Schism '{}' COMPLETED! Heretical cultists consecrated independent leyline shrines.",
                            plot.initiator_name, plot.title
                        ));
                    }
                    PlotType::GoldenAgePact => {
                        self.diplomacy = DiplomaticStatus::PeacefulCoexistence;
                        self.terran_culture.add_knowledge(60);
                        self.lunar_culture.add_knowledge(60);
                        events.push(format!(
                            "✨ [PLOT RESOLVED] {}'s Golden Age Pact '{}' TRIUMPHED! Philosophers ushered in an era of mutual enlightenment (+60 Knowledge).",
                            plot.initiator_name, plot.title
                        ));
                    }
                }
            }
        }
        events
    }

    /// Evaluates geopolitical panic when an Outburst occurs.
    /// High-threat outbursts (A, S, SS, SSS) compel nations to forge Mutual Defense Pacts or Grand Coalitions.
    pub fn evaluate_outburst_coalition(&mut self, dungeon_rank: Rank) -> Option<String> {
        if self.diplomacy == DiplomaticStatus::OpenWarfare {
            return None; // Bitter wartime rivals refuse joint treaties until open truce
        }

        if dungeon_rank >= Rank::S {
            if self.diplomacy != DiplomaticStatus::GrandCoalition {
                self.diplomacy = DiplomaticStatus::GrandCoalition;
                let terran_contrib = self.terran_treasury / 4;
                let lunar_contrib = self.lunar_treasury / 4;
                self.terran_treasury = self.terran_treasury.saturating_sub(terran_contrib);
                self.lunar_treasury = self.lunar_treasury.saturating_sub(lunar_contrib);
                self.guild.treasury_gold += terran_contrib + lunar_contrib;

                return Some(format!(
                    "🌐 [GEOPOLITICAL TREATY] Catastrophic {}-Rank Outburst ignited realm-wide panic! Solar Terrans and Chthonic Lunarians ratified the GRAND ANTI-CALAMITY COALITION! Pooled {} gold into Guild bounty treasury.",
                    dungeon_rank.display_name(),
                    terran_contrib + lunar_contrib
                ));
            }
        } else if dungeon_rank >= Rank::A
            && self.diplomacy != DiplomaticStatus::MutualDefensePact
            && self.diplomacy != DiplomaticStatus::GrandCoalition
        {
            self.diplomacy = DiplomaticStatus::MutualDefensePact;
            return Some(format!(
                "🛡️ [DIPLOMATIC TREATY] Severe {}-Rank Outburst led nations to sign a MUTUAL DEFENSE PACT to coordinate garrison defenses.",
                dungeon_rank.display_name()
            ));
        }

        None
    }

    /// Rewards cooperating nations upon successful eradication of an outburst threat under coalition
    pub fn resolve_coalition_victory(&mut self, dungeon_rank: Rank) -> Option<String> {
        if self.diplomacy == DiplomaticStatus::GrandCoalition
            || self.diplomacy == DiplomaticStatus::MutualDefensePact
        {
            let knowledge_gain = match dungeon_rank {
                Rank::SSS | Rank::SPlus | Rank::S => 100,
                Rank::A => 60,
                _ => 30,
            };

            self.terran_culture.add_knowledge(knowledge_gain);
            self.lunar_culture.add_knowledge(knowledge_gain);
            self.trade_volume += 50;

            Some(format!(
                "🕊️ [COALITION TRIUMPH] Joint allied forces eradicated the {}-Rank Outburst! Both civilizations celebrate unified victory (+{} Knowledge, +50 Trade Volume).",
                dungeon_rank.display_name(),
                knowledge_gain
            ))
        } else {
            None
        }
    }
}
