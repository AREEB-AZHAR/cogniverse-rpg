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
pub struct CivilizationsCoordinator {
    pub rival_god: RivalGodState,
    pub diplomacy: DiplomaticStatus,
    pub trade_volume: u32,
    pub intermarriage_count: u32,
    pub terran_favor: f32, // Player God favor
}

impl CivilizationsCoordinator {
    pub fn new() -> Self {
        Self {
            rival_god: RivalGodState::new(),
            diplomacy: DiplomaticStatus::PeacefulCoexistence,
            trade_volume: 0,
            intermarriage_count: 0,
            terran_favor: 70.0,
        }
    }

    pub fn tick(&mut self, current_tick: u64, terran_count: usize, lunarian_count: usize) -> Option<String> {
        self.rival_god.evaluate_and_intervene(current_tick, terran_count, lunarian_count)
    }
}
