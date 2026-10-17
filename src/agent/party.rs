use crate::agent::progression::{CharacterClass, HeroicProgression, Rank, Subclass};
use crate::agent::Agent;
use crate::world::outburst::OutburstHorde;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Combat tactical role derived from vocation and subclass
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombatRole {
    Tank,
    Dps,
    Support,
}

impl CombatRole {
    pub fn display_name(&self) -> &'static str {
        match self {
            CombatRole::Tank => "🛡️ Vanguard / Tank",
            CombatRole::Dps => "⚔️ Striker / DPS",
            CombatRole::Support => "💖 Healer / Support",
        }
    }

    /// Infers tactical role from hero's class and subclass
    pub fn from_progression(prog: &HeroicProgression) -> Self {
        if let Some(subclass) = prog.subclass {
            match subclass {
                Subclass::Paladin | Subclass::Berserker => CombatRole::Tank,
                Subclass::HighPriest | Subclass::Inquisitor => CombatRole::Support,
                Subclass::Sniper
                | Subclass::Ranger
                | Subclass::Elementalist
                | Subclass::Arcanist
                | Subclass::Assassin
                | Subclass::ShadowDancer => CombatRole::Dps,
                _ => CombatRole::Dps,
            }
        } else if let Some(class) = prog.class {
            match class {
                CharacterClass::Warrior => CombatRole::Tank,
                CharacterClass::Cleric => CombatRole::Support,
                CharacterClass::Mage | CharacterClass::Archer | CharacterClass::Rogue => {
                    CombatRole::Dps
                }
                _ => CombatRole::Dps,
            }
        } else {
            CombatRole::Dps
        }
    }
}

/// Operational state of an active adventurer party
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyStatus {
    Assembling,
    EnRoute,
    InBattle,
    Resting,
    Disbanded,
}

/// A member enrolled in a tactical party
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartyMember {
    pub agent_id: usize,
    pub name: String,
    pub role: CombatRole,
    pub rank: Rank,
    pub level: u32,
    pub hp: f32,
    pub max_hp: f32,
    pub attack_power: f32,
    pub defense: f32,
}

impl PartyMember {
    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.hp = (self.hp - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }
}

/// Tactical party or multi-nation raid formation responding to threats and guild quests
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Party {
    pub id: usize,
    pub name: String,
    pub leader_id: usize,
    pub members: Vec<PartyMember>,
    pub status: PartyStatus,
    pub x: usize,
    pub y: usize,
    pub target_x: usize,
    pub target_y: usize,
    pub active_quest_id: Option<usize>,
    pub synergy_multiplier: f32,
    pub composite_power: f32,
    pub is_raid_party: bool,
}

impl Party {
    pub fn new(
        id: usize,
        name: impl Into<String>,
        leader_id: usize,
        members: Vec<PartyMember>,
        start_x: usize,
        start_y: usize,
    ) -> Self {
        let is_raid = members.len() > 4;
        let mut party = Self {
            id,
            name: name.into(),
            leader_id,
            members,
            status: PartyStatus::Assembling,
            x: start_x,
            y: start_y,
            target_x: start_x,
            target_y: start_y,
            active_quest_id: None,
            synergy_multiplier: 1.0,
            composite_power: 0.0,
            is_raid_party: is_raid,
        };
        party.recalculate_stats();
        party
    }

    /// Evaluates role balance and awards the Trinity Synergy Bonus
    pub fn recalculate_stats(&mut self) {
        let has_tank = self.members.iter().any(|m| m.role == CombatRole::Tank);
        let has_dps = self.members.iter().any(|m| m.role == CombatRole::Dps);
        let has_support = self.members.iter().any(|m| m.role == CombatRole::Support);

        let mut mult = 1.0;
        if has_tank {
            mult += 0.15;
        }
        if has_dps {
            mult += 0.15;
        }
        if has_support {
            mult += 0.20;
        }
        // Trinity Bonus: all three roles present
        if has_tank && has_dps && has_support {
            mult += 0.15;
        }

        self.synergy_multiplier = mult;

        // Composite power = synergy * sum of individual stats
        let raw_power: f32 = self
            .members
            .iter()
            .map(|m| m.attack_power + 1.2 * m.defense + 0.3 * m.max_hp)
            .sum();

        self.composite_power = raw_power * mult;
    }

    pub fn is_party_alive(&self) -> bool {
        self.members.iter().any(|m| m.is_alive())
    }

    pub fn alive_member_count(&self) -> usize {
        self.members.iter().filter(|m| m.is_alive()).count()
    }

    /// Steps the party toward the assigned target coordinates
    pub fn step_toward_target(&mut self, width: usize, height: usize) {
        let mut new_x = self.x as isize;
        let mut new_y = self.y as isize;

        if (self.target_x as isize) > new_x {
            new_x += 1;
        } else if (self.target_x as isize) < new_x {
            new_x -= 1;
        }

        if (self.target_y as isize) > new_y {
            new_y += 1;
        } else if (self.target_y as isize) < new_y {
            new_y -= 1;
        }

        self.x = (new_x.max(0) as usize).min(width.saturating_sub(1));
        self.y = (new_y.max(0) as usize).min(height.saturating_sub(1));
    }

    /// Resolves one round of statistical combat between this party and a surface outbreak horde
    pub fn engage_outbreak_round(
        &mut self,
        horde: &mut OutburstHorde,
        _rng: &mut impl Rng,
    ) -> bool {
        if !self.is_party_alive() || horde.is_fully_defeated() {
            return false;
        }

        // Party deals damage to living horde mobs
        let party_atk = self.composite_power * 0.12;
        for mob in &mut horde.mobs {
            if mob.is_alive() {
                let damage = (party_atk - mob.defense * 0.4).max(5.0);
                mob.take_damage(damage);
                break; // Focus target highest priority mob
            }
        }

        // Horde strikes back at living party members
        if let Some(mob) = horde.mobs.iter().find(|m| m.is_alive()) {
            let horde_atk = mob.attack_power;
            // Tanks absorb hit if available
            let target_idx = self
                .members
                .iter()
                .position(|m| m.is_alive() && m.role == CombatRole::Tank)
                .or_else(|| self.members.iter().position(|m| m.is_alive()));

            if let Some(idx) = target_idx {
                let target = &mut self.members[idx];
                let def = target.defense * self.synergy_multiplier;
                let damage = (horde_atk - def * 0.35).max(3.0);
                target.take_damage(damage);
            }
        }

        // Support role heals damaged allies
        let has_support = self
            .members
            .iter()
            .any(|m| m.is_alive() && m.role == CombatRole::Support);
        if has_support {
            for member in &mut self.members {
                if member.is_alive() && member.hp < member.max_hp {
                    member.heal(12.0 * self.synergy_multiplier);
                }
            }
        }

        true
    }
}

/// Engine to dynamically form standard squads (3-4 members) or flexible raid parties (up to 8 members)
pub struct PartyFormationEngine;

impl PartyFormationEngine {
    /// Assembles an optimal party from available agents, sizing up to 8 members for A/S/SS/SSS threat ranks
    pub fn form_party_for_quest(
        party_id: usize,
        quest_rank: Rank,
        candidates: &[Agent],
        origin_x: usize,
        origin_y: usize,
    ) -> Option<Party> {
        let max_size = if quest_rank >= Rank::A { 8 } else { 4 };
        let min_size = 3;

        // Filter conscious and combat-ready agents
        let ready_agents: Vec<&Agent> = candidates
            .iter()
            .filter(|a| a.is_alive && (a.health / a.max_health) >= 0.60)
            .collect();

        if ready_agents.len() < min_size {
            return None;
        }

        // Segregate by role
        let mut tanks = Vec::new();
        let mut dps = Vec::new();
        let mut supports = Vec::new();

        for agent in &ready_agents {
            let role = CombatRole::from_progression(&agent.progression);
            match role {
                CombatRole::Tank => tanks.push(*agent),
                CombatRole::Dps => dps.push(*agent),
                CombatRole::Support => supports.push(*agent),
            }
        }

        let mut selected_members = Vec::new();

        // 1. Pick at least one tank if available
        if let Some(tank) = tanks.pop() {
            selected_members.push(Self::agent_to_party_member(tank, CombatRole::Tank));
        }

        // 2. Pick at least one healer/support if available
        if let Some(support) = supports.pop() {
            selected_members.push(Self::agent_to_party_member(support, CombatRole::Support));
        }

        // 3. Fill with DPS and remaining roles up to max_size
        while selected_members.len() < max_size {
            if let Some(d) = dps.pop() {
                selected_members.push(Self::agent_to_party_member(d, CombatRole::Dps));
            } else if let Some(t) = tanks.pop() {
                selected_members.push(Self::agent_to_party_member(t, CombatRole::Tank));
            } else if let Some(s) = supports.pop() {
                selected_members.push(Self::agent_to_party_member(s, CombatRole::Support));
            } else {
                break;
            }
        }

        if selected_members.len() < min_size {
            return None;
        }

        // Leader is highest rank / level member
        let leader_id = selected_members
            .iter()
            .max_by_key(|m| (m.rank as u32, m.level))
            .map(|m| m.agent_id)
            .unwrap_or(selected_members[0].agent_id);

        let name = if selected_members.len() > 4 {
            format!("Grand Raid Brigade #{}", party_id)
        } else {
            format!("Vanguard Strike Squad #{}", party_id)
        };

        Some(Party::new(
            party_id,
            name,
            leader_id,
            selected_members,
            origin_x,
            origin_y,
        ))
    }

    fn agent_to_party_member(agent: &Agent, role: CombatRole) -> PartyMember {
        let rank = agent.progression.rank.unwrap_or(Rank::F);
        let rank_mult = rank.stat_multiplier();
        PartyMember {
            agent_id: agent.id,
            name: agent.name.clone(),
            role,
            rank,
            level: agent.progression.level,
            hp: agent.health,
            max_hp: agent.max_health,
            attack_power: agent.progression.attributes.strength * 1.5 * rank_mult,
            defense: agent.progression.attributes.vitality * 0.8 * rank_mult,
        }
    }
}
