use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DivineInterventionKind {
    DivineWhisper { target_agent_id: Option<usize>, message: String },
    MiracleManna { x: usize, y: usize },
    PrometheanFire { x: usize, y: usize },
    SmiteLightning { x: usize, y: usize },
    TowerOfBabel,
    GenesisEvolutionSpark,
}

impl DivineInterventionKind {
    pub fn name(&self) -> &'static str {
        match self {
            DivineInterventionKind::DivineWhisper { .. } => "Divine Whisper",
            DivineInterventionKind::MiracleManna { .. } => "Miracle of Manna",
            DivineInterventionKind::PrometheanFire { .. } => "Promethean Sacred Fire",
            DivineInterventionKind::SmiteLightning { .. } => "Wrathful Lightning Smite",
            DivineInterventionKind::TowerOfBabel => "Tower of Babel (Linguistic Perturbation)",
            DivineInterventionKind::GenesisEvolutionSpark => "Genesis Evolution Spark",
        }
    }
}
