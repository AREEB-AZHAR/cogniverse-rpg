use serde::{Deserialize, Serialize};
use super::covenants::{DivineCovenant, QuestObjectiveType, CovenantRewardType};
use super::god_powers::DivineInterventionKind;
use super::super::agent::CivilizationRace;
use super::super::world::altar::Altar;
pub use super::super::world::MapArchetype;
use super::super::world::ecology::WeatherCondition;
use super::super::world::tile::{BiomeType, ElevationLayer, FrozenState, StructureKind};
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
    // WorldBox Dynamic Nature Brushes
    SpawnLava,
    IgniteWildfire,
    FreezeIce,
    ThawHeat,
    SeedSavanna,
    SeedJungle,
    SeedCorrupted,
    SeedPermafrost,
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
    ResetWorld,
    ChangeMap {
        archetype: MapArchetype,
        seed: Option<u64>,
        water_level: Option<f32>,
        mountain_density: Option<f32>,
    },
    RequestFullMap,
    ExportState,
    ImportState { snapshot_json: String },
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
pub struct DynamicTileSummary {
    pub x: usize,
    pub y: usize,
    pub biome: BiomeType,
    pub elevation_layer: ElevationLayer,
    pub is_on_fire: bool,
    pub burned_stages: u8,
    pub lava_level: u8,
    pub frozen_state: FrozenState,
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
    pub dynamic_tiles: Vec<DynamicTileSummary>,
    pub activities: Vec<AgentActivitySummary>,
    pub terran_culture: CivilizationCulture,
    pub lunar_culture: CivilizationCulture,
    pub active_plots: Vec<super::super::factions::plots::CivilPlot>,
    pub terran_treasury: u32,
    pub lunar_treasury: u32,
    pub diplomacy_status: String,
    pub map_id: u64,
    pub map_name: String,
    pub map_archetype: MapArchetype,
    pub full_map_sync: Option<String>,
    pub exported_snapshot: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldMetadataSnapshot {
    pub world_name: String,
    pub map_archetype: MapArchetype,
    pub map_seed: u64,
    pub map_water_level: f32,
    pub map_mountain_density: f32,
    pub world_tick: u64,
    pub elapsed_years: u64,
    pub export_timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LearningMilestonesSnapshot {
    pub total_technologies_unlocked: usize,
    pub total_discoveries_recorded: usize,
    pub peak_generation: u32,
    pub total_births: u64,
    pub total_deaths: u64,
    pub total_wealth: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CivilizationRunSnapshot {
    pub engine_version: String,
    pub metadata: WorldMetadataSnapshot,
    pub milestones: LearningMilestonesSnapshot,
    pub civilizations: super::super::factions::CivilizationsCoordinator,
    pub covenants: Vec<DivineCovenant>,
    pub agents: Vec<super::super::agent::Agent>,
    pub event_log: Vec<String>,
}

pub fn encode_base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((bytes.len() + 2) / 3 * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };

        let idx0 = (b0 >> 2) as usize;
        let idx1 = (((b0 & 0x03) << 4) | (b1 >> 4)) as usize;
        let idx2 = (((b1 & 0x0f) << 2) | (b2 >> 6)) as usize;
        let idx3 = (b2 & 0x3f) as usize;

        out.push(TABLE[idx0] as char);
        out.push(TABLE[idx1] as char);
        if chunk.len() > 1 {
            out.push(TABLE[idx2] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[idx3] as char);
        } else {
            out.push('=');
        }
    }
    out
}


