use serde::{Deserialize, Serialize};
use super::covenants::{DivineCovenant, QuestObjectiveType, CovenantRewardType};
use super::god_powers::DivineInterventionKind;
use super::super::agent::CivilizationRace;
use super::super::world::altar::Altar;
use super::super::world::ecology::WeatherCondition;
use super::super::brain::desire::AmbitionArchetype;
use super::super::brain::ast::BehavioralGenome;
use super::super::brain::neuromod::NeuromodulationState;
use super::super::brain::memory::EpisodicMemory;

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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldStateTelemetry {
    pub world_tick: u64,
    pub speed_multiplier: f32,
    pub weather: WeatherCondition,
    pub solar_intensity: f32,
    pub global_temperature_offset: f32,
    pub agents: Vec<AgentSummary>,
    pub altars: Vec<Altar>,
    pub wildlife_count: usize,
    pub active_covenants: Vec<DivineCovenant>,
    pub rival_god_favor: f32,
    pub rival_god_directive: String,
    pub event_log: Vec<String>,
    pub selected_agent: Option<SelectedAgentDetail>,
}
