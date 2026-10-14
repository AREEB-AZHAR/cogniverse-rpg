use serde::{Deserialize, Serialize};
use super::covenants::{DivineCovenant, QuestObjectiveType, CovenantRewardType};
use super::god_powers::DivineInterventionKind;
use super::super::agent::CivilizationRace;
use super::super::world::altar::Altar;
use super::super::world::ecology::WeatherCondition;
use super::super::world::tile::{BiomeType, StructureKind};
use super::super::brain::desire::AmbitionArchetype;
use super::super::brain::ast::BehavioralGenome;
use super::super::brain::neuromod::NeuromodulationState;
use super::super::brain::memory::EpisodicMemory;

use super::super::factions::CivilizationCulture;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerraformBrush {
    RaiseMountain,
    CarveRiver,
    PlantForest,
    FertilizePlains,
    InfuseLeylineRuins,
    EnrichIronVein,
    EnrichGoldVein,
    EnrichMithrilVein,
    BuildCabin,
    BuildCottage,
    BuildCampfire,
    BuildTownHall,
    BuildBarracks,
    BuildGranary,
    BuildWatchtower,
    BuildMineShaft,
    ClearLand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    SetSpeedMultiplier { speed: f32 },
    StepSingleTick,
    IssueCovenant {
        objective: QuestObjectiveType,
        target_amount: u32,
        reward: CovenantRewardType,
        punishment: String,
    },
    BlessOffering { altar_id: usize, offering_id: usize },
    SmiteOffering { altar_id: usize, offering_id: usize },
    TriggerIntervention { intervention: DivineInterventionKind },
    SelectAgent { agent_id: usize },
    TerraformArea {
        center_x: usize,
        center_y: usize,
        brush: TerraformBrush,
        radius: usize,
    },
    SpawnAgentAt {
        race: CivilizationRace,
        x: usize,
        y: usize,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DemographicsTelemetry {
    pub total_population: usize,
    pub terran_count: usize,
    pub lunarian_count: usize,
    pub wildlife_count: usize,
    pub births_total: u64,
    pub deaths_total: u64,
    pub max_generation: u32,
    pub avg_health: f32,
    pub avg_hunger: f32,
    pub avg_piety: f32,
    pub total_circulating_wealth: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlacedStructureSummary {
    pub x: usize,
    pub y: usize,
    pub kind: StructureKind,
    pub health: f32,
    pub max_health: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentSummary {
    pub id: usize,
    pub name: String,
    pub race: CivilizationRace,
    pub x: usize,
    pub y: usize,
    pub health: f32,
    pub hunger: f32,
    pub energy: f32,
    pub warmth: f32,
    pub archetype: AmbitionArchetype,
    pub current_thought: String,
    pub last_action: String,
    pub generation: u32,
    pub traits: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedAgentDetail {
    pub id: usize,
    pub name: String,
    pub race: CivilizationRace,
    pub x: usize,
    pub y: usize,
    pub health: f32,
    pub hunger: f32,
    pub thirst: f32,
    pub energy: f32,
    pub warmth: f32,
    pub social_need: f32,
    pub piety: f32,
    pub mana: f32,
    pub archetype: AmbitionArchetype,
    pub neuromod: NeuromodulationState,
    pub current_thought: String,
    pub inventory_items: Vec<(String, u32)>,
    pub recent_memories: Vec<EpisodicMemory>,
    pub beliefs: Vec<String>,
    pub genome: BehavioralGenome,
    pub partner_id: Option<usize>,
    pub children_count: u32,
    pub ancestral_echo: Option<String>,
    pub traits: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentActivitySummary {
    pub tick: u64,
    pub agent_id: usize,
    pub agent_name: String,
    pub race: CivilizationRace,
    pub activity: String,
    pub location: (usize, usize),
    pub emotion: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldStateTelemetry {
    pub world_tick: u64,
    pub speed_multiplier: f32,
    pub weather: WeatherCondition,
    pub solar_intensity: f32,
    pub is_daytime: bool,
    pub global_temperature_offset: f32,
    pub demographics: DemographicsTelemetry,
    pub agents: Vec<AgentSummary>,
    pub structures: Vec<PlacedStructureSummary>,
    pub altars: Vec<Altar>,
    pub active_covenants: Vec<DivineCovenant>,
    pub rival_god_favor: f32,
    pub rival_god_directive: String,
    pub event_log: Vec<String>,
    pub selected_agent: Option<SelectedAgentDetail>,
    pub modified_tiles: Vec<(usize, usize, BiomeType)>,
    pub activities: Vec<AgentActivitySummary>,
    pub terran_culture: CivilizationCulture,
    pub lunar_culture: CivilizationCulture,
    pub active_plots: Vec<super::super::factions::plots::CivilPlot>,
    pub terran_treasury: u32,
    pub lunar_treasury: u32,
    pub diplomacy_status: String,
}

