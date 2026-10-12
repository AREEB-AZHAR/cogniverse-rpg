use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestObjectiveType {
    BuildTemple,
    StockpileGrain,
    CraftIronTools,
    InscribeArcaneRunes,
    PresentAltarOfferings,
}

impl QuestObjectiveType {
    pub fn display_name(&self) -> &'static str {
        match self {
            QuestObjectiveType::BuildTemple => "Erect a Sacred Stone Temple",
            QuestObjectiveType::StockpileGrain => "Stockpile 30 Grain in Granary",
            QuestObjectiveType::CraftIronTools => "Forge 5 Iron Tools",
            QuestObjectiveType::InscribeArcaneRunes => "Inscribe 3 Arcane Runes",
            QuestObjectiveType::PresentAltarOfferings => "Deliver 5 Sacrificial Offerings",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CovenantRewardType {
    RainOfManna,
    BlessingOfFertility,
    PrometheanInspiration,
    CelestialAegis,
}

impl CovenantRewardType {
    pub fn display_name(&self) -> &'static str {
        match self {
            CovenantRewardType::RainOfManna => "Rain of Manna (Abundant Food)",
            CovenantRewardType::BlessingOfFertility => "Blessing of Fertility (+Offspring)",
            CovenantRewardType::PrometheanInspiration => "Promethean Inspiration (Advanced AST)",
            CovenantRewardType::CelestialAegis => "Celestial Aegis (Predator Immunity)",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DivineCovenant {
    pub id: usize,
    pub title: String,
    pub objective: QuestObjectiveType,
    pub target_progress: u32,
    pub current_progress: u32,
    pub promised_reward: CovenantRewardType,
    pub punishment_description: String,
    pub is_completed: bool,
    pub is_defied: bool,
    pub issued_tick: u64,
}

impl DivineCovenant {
    pub fn new(
        id: usize,
        objective: QuestObjectiveType,
        target_progress: u32,
        promised_reward: CovenantRewardType,
        punishment: impl Into<String>,
        issued_tick: u64,
    ) -> Self {
        Self {
            id,
            title: format!("Divine Mandate: {}", objective.display_name()),
            objective,
            target_progress,
            current_progress: 0,
            promised_reward,
            punishment_description: punishment.into(),
            is_completed: false,
            is_defied: false,
            issued_tick,
        }
    }

    pub fn advance(&mut self, amount: u32) -> bool {
        if self.is_completed || self.is_defied {
            return false;
        }

        self.current_progress += amount;
        if self.current_progress >= self.target_progress {
            self.is_completed = true;
            true
        } else {
            false
        }
    }
}
