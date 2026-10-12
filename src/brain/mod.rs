pub mod ast;
pub mod neuromod;
pub mod memory;
pub mod reflection;
pub mod world_model;
pub mod desire;
pub mod justice;
pub mod magic;
pub mod reproduction;

use serde::{Deserialize, Serialize};
use ast::{ActionChoice, AstVmContext, BehavioralGenome, EntityFilter};
use neuromod::NeuromodulationState;
use memory::MemoryHierarchy;
use reflection::KnowledgeGraph;
use world_model::WorldModel;
use desire::DesireEngine;
use justice::SocialJusticeLedger;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CognitiveBrain {
    pub genome: BehavioralGenome,
    pub neuromod: NeuromodulationState,
    pub memory: MemoryHierarchy,
    pub knowledge: KnowledgeGraph,
    pub world_model: WorldModel,
    pub desire: DesireEngine,
    pub social_ledger: SocialJusticeLedger,
    pub mana: f32,
    pub charisma: f32,
    pub piety: f32,
    pub current_thought: String,
    pub last_action: ActionChoice,
    pub gas_consumed_last_tick: u32,
}

impl CognitiveBrain {
    pub fn new(archetype: desire::AmbitionArchetype, charisma: f32, piety: f32) -> Self {
        Self {
            genome: BehavioralGenome::new_default(),
            neuromod: NeuromodulationState::new(),
            memory: MemoryHierarchy::new(),
            knowledge: KnowledgeGraph::new(),
            world_model: WorldModel::new(),
            desire: DesireEngine::new(archetype),
            social_ledger: SocialJusticeLedger::new(),
            mana: 50.0,
            charisma,
            piety,
            current_thought: "Awakened into the world.".into(),
            last_action: ActionChoice::IdleRest,
            gas_consumed_last_tick: 0,
        }
    }

    /// Evaluates perception and runs the gas-metered AST
    pub fn decide_action(
        &mut self,
        hunger: f32,
        thirst: f32,
        energy: f32,
        warmth: f32,
        social_need: f32,
        health: f32,
        nearest_distances: &std::collections::HashMap<EntityFilter, f32>,
        inventory_has: &dyn Fn(&str) -> bool,
        current_tick: u64,
        agent_name: &str,
    ) -> ActionChoice {
        // 1. Periodic reflection
        if current_tick % 100 == 0 {
            self.knowledge.reflect(&self.memory, current_tick);
        }

        // 2. Prepare VM context
        let vm_ctx = AstVmContext {
            hunger,
            thirst,
            energy,
            warmth,
            social_need,
            piety: self.piety,
            health,
            dopamine: self.neuromod.dopamine,
            serotonin: self.neuromod.serotonin,
            noradrenaline: self.neuromod.noradrenaline,
            nearest_distances,
            inventory_has,
        };

        // 3. Evaluate AST with 500-gas budget
        let (action, gas_used) = self.genome.evaluate(&vm_ctx);
        self.gas_consumed_last_tick = gas_used;
        self.last_action = action;

        // 4. Update inner monologue thought
        self.current_thought = self.desire.generate_inner_monologue(
            agent_name,
            health,
            hunger,
            self.neuromod.dopamine,
            self.neuromod.serotonin,
        );

        action
    }
}
