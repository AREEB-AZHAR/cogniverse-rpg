use serde::{Deserialize, Serialize};
use super::memory::MemoryHierarchy;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeliefStatement {
    pub concept: String,
    pub confidence: f32, // 0.0 to 1.0
    pub formed_at_tick: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub beliefs: Vec<BeliefStatement>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            beliefs: Vec::new(),
        }
    }

    pub fn reflect(&mut self, memory: &MemoryHierarchy, current_tick: u64) {
        // Find high-importance memories (importance >= 7.0)
        let salient: Vec<_> = memory
            .episodic_stream
            .iter()
            .filter(|m| m.importance >= 7.0)
            .collect();

        if salient.len() >= 2 {
            // Synthesize reflection
            for m in salient.iter().take(3) {
                let concept_tag = m.tags.first().cloned().unwrap_or_else(|| "Survival".into());
                let belief_text = format!("Reflected from experience: {}", m.description);
                
                // Add or update belief
                if !self.beliefs.iter().any(|b| b.concept == concept_tag) {
                    self.beliefs.push(BeliefStatement {
                        concept: format!("{}: {}", concept_tag, belief_text),
                        confidence: 0.85,
                        formed_at_tick: current_tick,
                    });
                }
            }
        }

        // Keep maximum 15 high-level beliefs
        if self.beliefs.len() > 15 {
            self.beliefs.remove(0);
        }
    }
}
