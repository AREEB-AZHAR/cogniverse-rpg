use crate::agent::party::Party;
use crate::agent::progression::Rank;
use crate::agent::Agent;
use crate::world::items::ItemId;
use serde::{Deserialize, Serialize};

/// Celestial domain bestowed upon an ascended Demigod champion
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivineDomain {
    SunHerald,
    WarBringer,
    VoidWalker,
}

impl DivineDomain {
    pub fn display_name(&self) -> &'static str {
        match self {
            DivineDomain::SunHerald => "☀️ Herald of the Radiant Sun",
            DivineDomain::WarBringer => "⚔️ Sovereign Bringer of War",
            DivineDomain::VoidWalker => "🌌 Walker of the Infinite Void",
        }
    }
}

/// Dynamic area-of-effect aura projected by an active Demigod
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DemigodAura {
    pub domain: DivineDomain,
    pub radius: usize,
    pub potency: f32,
}

/// Permanent sacred artifact forged by an ascended Demigod to preserve their lineage
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynastyHeirloom {
    pub id: u64,
    pub name: String,
    pub creator_name: String,
    pub domain: DivineDomain,
    pub attack_bonus: f32,
    pub defense_bonus: f32,
    pub generation: u32,
}

/// Demigod ascension state and aura tracking
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DemigodState {
    pub is_ascended: bool,
    pub domain: Option<DivineDomain>,
    pub ascension_tier: u32,
    pub aura: Option<DemigodAura>,
    pub heirloom: Option<DynastyHeirloom>,
}

/// Engine supervising the Sacred Trial of Apotheosis and aura buffs
pub struct ApotheosisEngine;

impl ApotheosisEngine {
    /// Checks whether an agent satisfies the stringent requirements for Demigod Ascension
    pub fn can_ascend(agent: &Agent) -> bool {
        let is_high_rank = agent
            .progression
            .rank
            .map(|r| r >= Rank::S)
            .unwrap_or(false);
        let is_veteran = agent.progression.level >= 25;
        let has_spark = agent.inventory.count(ItemId::DivineSpark) > 0;

        is_high_rank && is_veteran && has_spark
    }

    /// Performs the sacred trial of apotheosis, consuming a Divine Spark and forging a Dynasty Heirloom
    pub fn perform_ascension(
        agent: &mut Agent,
        domain: DivineDomain,
        demigod_state: &mut DemigodState,
        next_item_id: &mut u64,
    ) -> Result<DynastyHeirloom, &'static str> {
        if !Self::can_ascend(agent) {
            return Err("Candidate does not meet Demigod Ascension prerequisites");
        }

        // Consume Divine Spark
        agent.inventory.remove(ItemId::DivineSpark, 1);

        // Elevate agent attributes & vitals
        agent.health = agent.max_health * 1.5;
        agent.max_health *= 1.5;
        agent.progression.attributes.strength += 25.0;
        agent.progression.attributes.vitality += 25.0;
        agent.progression.attributes.wisdom += 25.0;

        // Forge Dynasty Heirloom
        let heirloom_id = *next_item_id;
        *next_item_id += 1;

        let heirloom = DynastyHeirloom {
            id: heirloom_id,
            name: format!("✦ Ancestral Crown of {}", agent.name),
            creator_name: agent.name.clone(),
            domain,
            attack_bonus: 35.0,
            defense_bonus: 30.0,
            generation: 1,
        };

        demigod_state.is_ascended = true;
        demigod_state.domain = Some(domain);
        demigod_state.ascension_tier = 1;
        demigod_state.aura = Some(DemigodAura {
            domain,
            radius: 8,
            potency: 1.25,
        });
        demigod_state.heirloom = Some(heirloom.clone());

        Ok(heirloom)
    }

    /// Applies the Demigod's domain aura to a tactical party, multiplying attack and defense
    pub fn apply_aura_to_party(aura: &DemigodAura, party: &mut Party) {
        let atk_buff = match aura.domain {
            DivineDomain::WarBringer => 0.35 * aura.potency,
            DivineDomain::SunHerald => 0.20 * aura.potency,
            DivineDomain::VoidWalker => 0.25 * aura.potency,
        };

        let def_buff = match aura.domain {
            DivineDomain::SunHerald => 0.30 * aura.potency,
            DivineDomain::WarBringer => 0.15 * aura.potency,
            DivineDomain::VoidWalker => 0.20 * aura.potency,
        };

        for member in &mut party.members {
            if member.is_alive() {
                member.attack_power *= 1.0 + atk_buff;
                member.defense *= 1.0 + def_buff;
            }
        }
        party.recalculate_stats();
    }
}
