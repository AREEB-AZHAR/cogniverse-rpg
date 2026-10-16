use super::items::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeityAlignment {
    SolarPlayerGod,
    LunarShadowWeaver,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SacrificialOffering {
    pub offering_id: usize,
    pub agent_id: usize,
    pub agent_name: String,
    pub item: ItemId,
    pub amount: u32,
    pub tick: u64,
    pub sincerity_score: f32, // 0.0 to 1.0+
    pub resolved: bool,
    pub outcome: Option<OfferingOutcome>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OfferingOutcome {
    Blessed {
        boon_type: String,
        potency: f32,
    },
    Smitten {
        retribution_type: String,
        severity: f32,
    },
    Ignored,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Altar {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub deity: DeityAlignment,
    pub name: String,
    pub consecrated_level: u32,
    pub accumulated_mana: f32,
    pub recent_offerings: Vec<SacrificialOffering>,
}

impl Altar {
    pub fn new(
        id: usize,
        x: usize,
        y: usize,
        deity: DeityAlignment,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            x,
            y,
            deity,
            name: name.into(),
            consecrated_level: 1,
            accumulated_mana: 50.0,
            recent_offerings: Vec::new(),
        }
    }

    /// Calculates sincerity score: high if the agent gave something rare relative to their wealth, multiplied by piety
    pub fn calculate_sincerity(
        item: ItemId,
        amount: u32,
        agent_wealth: f32,
        agent_piety: f32,
    ) -> f32 {
        let item_value = item.rarity_value() * (amount as f32);
        let sacrifice_ratio = (item_value / (agent_wealth + 5.0)).clamp(0.05, 1.0);
        let raw_score = sacrifice_ratio * (0.5 + 0.5 * agent_piety.clamp(0.0, 1.0));
        (raw_score * 1.5).clamp(0.0, 1.0)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn receive_offering(
        &mut self,
        offering_id: usize,
        agent_id: usize,
        agent_name: String,
        item: ItemId,
        amount: u32,
        agent_wealth: f32,
        agent_piety: f32,
        tick: u64,
    ) -> usize {
        let sincerity_score = Self::calculate_sincerity(item, amount, agent_wealth, agent_piety);
        self.accumulated_mana += item.rarity_value() * (amount as f32) * 0.5;

        self.recent_offerings.push(SacrificialOffering {
            offering_id,
            agent_id,
            agent_name,
            item,
            amount,
            tick,
            sincerity_score,
            resolved: false,
            outcome: None,
        });

        // Keep last 30 offerings
        if self.recent_offerings.len() > 30 {
            self.recent_offerings.remove(0);
        }

        self.recent_offerings.len() - 1
    }
}
