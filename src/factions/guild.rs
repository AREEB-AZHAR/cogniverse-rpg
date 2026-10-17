use crate::agent::progression::Rank;
use crate::world::dungeon::Dungeon;
use serde::{Deserialize, Serialize};

/// Archetype category of contract posted on the Adventurer Guild notice board
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestKind {
    OutbreakContainment,
    DungeonSubjugation,
    BountyHunt,
}

impl QuestKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            QuestKind::OutbreakContainment => "🚨 Emergency Outbreak Containment",
            QuestKind::DungeonSubjugation => "⚔️ Dungeon Subjugation Expedition",
            QuestKind::BountyHunt => "🎯 Monster Bounty Hunt",
        }
    }
}

/// Operational progress state of a guild quest
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestStatus {
    Open,
    Assigned,
    InProgress,
    Completed,
    Failed,
}

/// An official Adventurer Guild contract with bounty and renown rewards
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuildQuest {
    pub id: usize,
    pub title: String,
    pub kind: QuestKind,
    pub target_dungeon_id: Option<usize>,
    pub target_x: usize,
    pub target_y: usize,
    pub recommended_rank: Rank,
    pub bounty_gold: u32,
    pub renown_reward: u32,
    pub status: QuestStatus,
    pub assigned_party_id: Option<usize>,
    pub created_tick: u64,
}

impl GuildQuest {
    pub fn is_available(&self) -> bool {
        self.status == QuestStatus::Open
    }
}

/// The Adventurer Guild coordinating civilian defense, contracts, and expedition rewards
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdventurerGuild {
    pub name: String,
    pub faction_name: String,
    pub headquarters_x: usize,
    pub headquarters_y: usize,
    pub treasury_gold: u32,
    pub quests: Vec<GuildQuest>,
    pub next_quest_id: usize,
    pub completed_contracts: u32,
}

impl AdventurerGuild {
    pub fn new(
        name: impl Into<String>,
        faction_name: impl Into<String>,
        hq_x: usize,
        hq_y: usize,
    ) -> Self {
        Self {
            name: name.into(),
            faction_name: faction_name.into(),
            headquarters_x: hq_x,
            headquarters_y: hq_y,
            treasury_gold: 1000,
            quests: Vec::new(),
            next_quest_id: 1,
            completed_contracts: 0,
        }
    }

    /// Posts an urgent emergency containment quest when a dungeon breaches into Outburst.
    /// Bounties are funded dynamically from guild & civilization treasuries.
    pub fn post_emergency_outburst_quest(&mut self, dungeon: &Dungeon, tick: u64) -> usize {
        let quest_id = self.next_quest_id;
        self.next_quest_id += 1;

        let rank_mult = dungeon.rank.stat_multiplier();
        let base_bounty = (250.0 * rank_mult) as u32;
        let bounty_gold = base_bounty.min(self.treasury_gold);
        self.treasury_gold = self.treasury_gold.saturating_sub(bounty_gold);

        let quest = GuildQuest {
            id: quest_id,
            title: format!(
                "URGENT: Contain [{}] Outburst at ({}, {})",
                dungeon.name, dungeon.x, dungeon.y
            ),
            kind: QuestKind::OutbreakContainment,
            target_dungeon_id: Some(dungeon.id),
            target_x: dungeon.x,
            target_y: dungeon.y,
            recommended_rank: dungeon.rank,
            bounty_gold,
            renown_reward: (50.0 * rank_mult) as u32,
            status: QuestStatus::Open,
            assigned_party_id: None,
            created_tick: tick,
        };

        self.quests.push(quest);
        quest_id
    }

    /// Assigns an open quest to an assembling or active party
    pub fn assign_quest(&mut self, quest_id: usize, party_id: usize) -> bool {
        if let Some(quest) = self.quests.iter_mut().find(|q| q.id == quest_id) {
            if quest.status == QuestStatus::Open {
                quest.status = QuestStatus::Assigned;
                quest.assigned_party_id = Some(party_id);
                return true;
            }
        }
        false
    }

    /// Marks a quest as successfully completed and increments guild record
    pub fn complete_quest(&mut self, quest_id: usize) -> Option<GuildQuest> {
        if let Some(quest) = self.quests.iter_mut().find(|q| q.id == quest_id) {
            quest.status = QuestStatus::Completed;
            self.completed_contracts += 1;
            return Some(quest.clone());
        }
        None
    }

    /// Retrieves all currently available open contracts
    pub fn available_quests(&self) -> Vec<&GuildQuest> {
        self.quests.iter().filter(|q| q.is_available()).collect()
    }
}
