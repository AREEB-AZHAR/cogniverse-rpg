use super::ast::BehavioralGenome;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CourtshipProposal {
    pub suitor_id: usize,
    pub suitor_name: String,
    pub attraction_score: f32,
    pub accepted: bool,
}

pub struct ReproductionManager;

impl ReproductionManager {
    /// Evaluates mutual attraction between two agents
    pub fn calculate_attraction(
        charisma_b: f32,
        wealth_b: f32,
        reputation_b: f32,
        health_b: f32,
    ) -> f32 {
        let norm_wealth = (wealth_b / 50.0).clamp(0.0, 1.0);
        let norm_rep = ((reputation_b + 1.0) / 2.0).clamp(0.0, 1.0);
        let norm_health = (health_b / 100.0).clamp(0.0, 1.0);

        0.30 * charisma_b + 0.25 * norm_wealth + 0.25 * norm_rep + 0.20 * norm_health
    }

    /// Creates child genome by combining parents' genomes via crossover and mutation
    pub fn create_child_genome(
        parent_a_genome: &BehavioralGenome,
        parent_b_genome: &BehavioralGenome,
    ) -> BehavioralGenome {
        let mut child = BehavioralGenome::crossover(parent_a_genome, parent_b_genome);
        // Epigenetic mutation
        child.mutate(0.2);
        child
    }

    /// Inherits traits from parents with genetic transmission and spontaneous mutation inspired by WorldBox
    pub fn inherit_traits(parent_a_traits: &[String], parent_b_traits: &[String]) -> Vec<String> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut child_traits = Vec::new();

        for trait_name in parent_a_traits.iter() {
            if rng.gen_bool(0.65) && !child_traits.contains(trait_name) {
                child_traits.push(trait_name.clone());
            }
        }
        for trait_name in parent_b_traits.iter() {
            if rng.gen_bool(0.65) && !child_traits.contains(trait_name) {
                child_traits.push(trait_name.clone());
            }
        }

        // Spontaneous genetic mutation (WorldBox-style rare trait emergence)
        if rng.gen_bool(0.12) {
            let rare_traits = [
                "Immortal",
                "Regeneration",
                "WiseScholar",
                "EagleEyed",
                "Pyromancer",
                "HardSkin",
                "Sunblessed",
                "Moonshadow",
            ];
            let mutation = rare_traits[rng.gen_range(0..rare_traits.len())];
            if !child_traits.contains(&mutation.to_string()) {
                child_traits.push(mutation.to_string());
            }
        }

        if child_traits.is_empty() {
            child_traits.push("HardSkin".to_string());
        }

        child_traits
    }
}
