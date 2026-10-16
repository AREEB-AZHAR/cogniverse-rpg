use crate::agent::CivilizationRace;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlotType {
    Rebellion,
    HolyWar,
    CommercialAlliance,
    HeresySchism,
    GoldenAgePact,
}

impl PlotType {
    pub fn display_name(&self) -> &'static str {
        match self {
            PlotType::Rebellion => "Provincial Rebellion",
            PlotType::HolyWar => "Crusade & Holy War",
            PlotType::CommercialAlliance => "Treaty of Free Commerce",
            PlotType::HeresySchism => "Schismatic Heresy",
            PlotType::GoldenAgePact => "Pact of the Golden Age",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            PlotType::Rebellion => "⚔️",
            PlotType::HolyWar => "⚡",
            PlotType::CommercialAlliance => "🤝",
            PlotType::HeresySchism => "🔥",
            PlotType::GoldenAgePact => "✨",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CivilPlot {
    pub id: usize,
    pub initiator_id: usize,
    pub initiator_name: String,
    pub initiator_race: CivilizationRace,
    pub plot_type: PlotType,
    pub title: String,
    pub target_description: String,
    pub progress: f32, // 0.0 to 100.0%
    pub supporter_ids: Vec<usize>,
    pub is_resolved: bool,
}

impl CivilPlot {
    pub fn new(
        id: usize,
        initiator_id: usize,
        initiator_name: impl Into<String>,
        initiator_race: CivilizationRace,
        plot_type: PlotType,
        title: impl Into<String>,
        target_description: impl Into<String>,
    ) -> Self {
        Self {
            id,
            initiator_id,
            initiator_name: initiator_name.into(),
            initiator_race,
            plot_type,
            title: title.into(),
            target_description: target_description.into(),
            progress: 0.0,
            supporter_ids: vec![initiator_id],
            is_resolved: false,
        }
    }

    pub fn add_supporter(&mut self, agent_id: usize) {
        if !self.supporter_ids.contains(&agent_id) {
            self.supporter_ids.push(agent_id);
        }
    }

    pub fn advance_progress(&mut self, delta: f32) -> bool {
        self.progress = (self.progress + delta).min(100.0);
        if self.progress >= 100.0 && !self.is_resolved {
            self.is_resolved = true;
            true
        } else {
            false
        }
    }
}
