use super::super::world::{WorldGrid, WORLD_WIDTH, WORLD_HEIGHT};
use super::super::world::items::ItemId;
use super::super::agent::{Agent, CivilizationRace};
use super::super::brain::ast::{ActionChoice, EntityFilter};
use super::super::brain::memory::AkashicCore;
use super::super::brain::desire::AmbitionArchetype;
use super::super::brain::reproduction::ReproductionManager;
use super::super::factions::CivilizationsCoordinator;
use super::covenants::{DivineCovenant, QuestObjectiveType, CovenantRewardType};
use super::god_powers::DivineInterventionKind;
use super::protocol::{AgentSummary, SelectedAgentDetail, WorldStateTelemetry};

pub struct SimulationEngine {
    pub world: WorldGrid,
    pub agents: Vec<Agent>,
    pub civilizations: CivilizationsCoordinator,
    pub akashic_core: AkashicCore,
    pub covenants: Vec<DivineCovenant>,
    pub speed_multiplier: f32, // 0.0 = paused, 1.0 = real-time, 10.0, 100.0, 1000.0 (turbo)
    pub single_step: bool,
    pub selected_agent_id: Option<usize>,
    pub event_log: Vec<String>,
    pub next_agent_id: usize,
    pub next_covenant_id: usize,
}

impl SimulationEngine {
    pub fn new() -> Self {
        let world = WorldGrid::new();
        let akashic_core = AkashicCore::new();
        let mut agents = Vec::new();
        let mut next_id = 1;

        // Spawn initial Solar Terrans near Sun Altar (x: 24, y: 36)
        let terran_names = ["Kael", "Lyra", "Orion", "Aria", "Darek", "Elora", "Theron", "Mira"];
        for (i, name) in terran_names.iter().enumerate() {
            let x = 20 + (i * 2) % 10;
            let y = 33 + (i * 3) % 8;
            let archetype = AmbitionArchetype::random_archetype();
            let echo = akashic_core.maybe_awaken_echo();
            agents.push(Agent::new(next_id, *name, CivilizationRace::SolarTerran, x, y, archetype, echo));
            next_id += 1;
        }

        // Spawn initial Chthonic Lunarians near Moon Altar (x: 48, y: 16)
        let lunar_names = ["Vane", "Nyx", "Malik", "Selene", "Corvus", "Morrigan", "Dusk", "Hesper"];
        for (i, name) in lunar_names.iter().enumerate() {
            let x = 44 + (i * 2) % 10;
            let y = 14 + (i * 3) % 8;
            let archetype = AmbitionArchetype::random_archetype();
            let echo = akashic_core.maybe_awaken_echo();
            agents.push(Agent::new(next_id, *name, CivilizationRace::ChthonicLunarian, x, y, archetype, echo));
            next_id += 1;
        }

        // Initial default covenant
        let mut covenants = Vec::new();
        covenants.push(DivineCovenant::new(
            1,
            QuestObjectiveType::PresentAltarOfferings,
            3,
            CovenantRewardType::RainOfManna,
            "A chilling frost will descend upon the unfaithful.",
            0,
        ));

        let mut event_log = Vec::new();
        event_log.push("The Genesis Cogniverse has awakened! The world matrix is in harmony.".into());
        event_log.push("Solar Terrans settled near the Golden Sun Altar (24, 36).".into());
        event_log.push("Chthonic Lunarians established presence at the Obsidian Moon Altar (48, 16).".into());

        Self {
            world,
            agents,
            civilizations: CivilizationsCoordinator::new(),
            akashic_core,
            covenants,
            speed_multiplier: 1.0,
            single_step: false,
            selected_agent_id: Some(1),
            event_log,
            next_agent_id: next_id,
            next_covenant_id: 2,
        }
    }

    pub fn log_event(&mut self, text: impl Into<String>) {
        let t = text.into();
        self.event_log.push(t);
        if self.event_log.len() > 30 {
            self.event_log.remove(0);
        }
    }

    /// Advances the entire simulation by 1 tick
    pub fn tick(&mut self) {
        let current_tick = self.world.gaia.world_tick;

        // 1. Tick the world grid (Gaia engine, flora regrowth, wildlife)
        self.world.tick();

        // 2. Tick civilizations and Rival AI God
        let terran_count = self.agents.iter().filter(|a| a.is_alive && a.race == CivilizationRace::SolarTerran).count();
        let lunarian_count = self.agents.iter().filter(|a| a.is_alive && a.race == CivilizationRace::ChthonicLunarian).count();
        if let Some(rival_msg) = self.civilizations.tick(current_tick, terran_count, lunarian_count) {
            self.log_event(rival_msg);
        }

        // 3. Pre-compute environmental sensory data and decide agent actions
        // (Fast, cache-friendly iteration)
        let weather_delta = self.world.gaia.current_weather.temperature_delta();
        let mut actions_to_execute = Vec::with_capacity(self.agents.len());

        for agent in self.agents.iter_mut() {
            if !agent.is_alive {
                continue;
            }

            // Tick physiological vitals
            let tile = self.world.tile_at(agent.x, agent.y);
            agent.tick_vitals(tile, weather_delta);

            if !agent.is_alive {
                continue;
            }

            // Calculate sensory distances
            let distances = agent.compute_sensory_distances(&self.world);
            let has_fn = |item_name: &str| -> bool {
                match item_name {
                    "Wood" => agent.inventory.count(ItemId::Wood) > 0,
                    "Stone" => agent.inventory.count(ItemId::Stone) > 0,
                    "Iron" => agent.inventory.count(ItemId::IronOre) > 0,
                    "Food" => agent.inventory.count(ItemId::Berry) > 0 || agent.inventory.count(ItemId::Grain) > 0,
                    _ => false,
                }
            };

            // Cognitive brain evaluates AST
            let chosen_action = agent.brain.decide_action(
                agent.hunger,
                agent.thirst,
                agent.energy,
                agent.warmth,
                agent.social_need,
                agent.health,
                &distances,
                &has_fn,
                current_tick,
                &agent.name,
            );

            actions_to_execute.push((agent.id, chosen_action));
        }

        // 4. Resolve actions in physical world
        let mut new_borns = Vec::new();
        let mut offerings_to_record = Vec::new();

        for (agent_id, action) in actions_to_execute {
            let mut agent_idx = None;
            for (idx, a) in self.agents.iter().enumerate() {
                if a.id == agent_id {
                    agent_idx = Some(idx);
                    break;
                }
            }

            let Some(idx) = agent_idx else { continue };

            match action {
                ActionChoice::IdleRest => {
                    let agent = &mut self.agents[idx];
                    agent.energy = (agent.energy + 0.05).min(1.0);
                    agent.brain.neuromod.update(0.1, 1.0 - agent.hunger, false, false, 0.0);
                }
                ActionChoice::MoveToNearest(filter) => {
                    let target_pos = {
                        let agent = &self.agents[idx];
                        Self::find_nearest_target_coords(&self.world, &self.agents, agent, filter)
                    };
                    if let Some((tx, ty)) = target_pos {
                        let agent = &mut self.agents[idx];
                        agent.step_towards(tx, ty, WORLD_WIDTH, WORLD_HEIGHT);
                    }
                }
                ActionChoice::ForageFood => {
                    let (ax, ay) = (self.agents[idx].x, self.agents[idx].y);
                    let tile = self.world.tile_at_mut(ax, ay);
                    if tile.berry_amount > 0 {
                        tile.berry_amount -= 1;
                        let agent = &mut self.agents[idx];
                        agent.inventory.add(ItemId::Berry, 1);
                        agent.hunger = (agent.hunger - 0.25).max(0.0);
                        agent.brain.neuromod.update(0.5, 1.0 - agent.hunger, false, false, 0.0);
                        agent.brain.memory.record_observation("Foraged delicious wild berries.");
                    } else if tile.grain_amount > 0 {
                        tile.grain_amount -= 1;
                        let agent = &mut self.agents[idx];
                        agent.inventory.add(ItemId::Grain, 1);
                        agent.hunger = (agent.hunger - 0.15).max(0.0);
                        agent.brain.neuromod.update(0.3, 1.0 - agent.hunger, false, false, 0.0);
                        agent.brain.memory.record_observation("Harvested wild grain.");
                    }
                }
                ActionChoice::DrinkWater => {
                    let (ax, ay) = (self.agents[idx].x, self.agents[idx].y);
                    let has_water = self.world.tile_at(ax, ay).has_fresh_water;
                    if has_water {
                        let agent = &mut self.agents[idx];
                        agent.thirst = (agent.thirst - 0.4).max(0.0);
                        agent.brain.neuromod.update(0.4, 1.0 - agent.thirst, false, false, 0.0);
                        agent.brain.memory.record_observation("Drank cold, clean river water.");
                    }
                }
                ActionChoice::ChopWood => {
                    let (ax, ay) = (self.agents[idx].x, self.agents[idx].y);
                    let tile = self.world.tile_at_mut(ax, ay);
                    if tile.wood_amount > 0 {
                        tile.wood_amount -= 1;
                        let agent = &mut self.agents[idx];
                        agent.inventory.add(ItemId::Wood, 1);
                        agent.energy = (agent.energy - 0.03).max(0.0);
                        agent.brain.neuromod.update(0.3, 0.5, false, false, 0.0);
                    }
                }
                ActionChoice::MineStone => {
                    let (ax, ay) = (self.agents[idx].x, self.agents[idx].y);
                    let tile = self.world.tile_at_mut(ax, ay);
                    if tile.stone_amount > 0 {
                        tile.stone_amount -= 1;
                        let agent = &mut self.agents[idx];
                        agent.inventory.add(ItemId::Stone, 1);
                        agent.energy = (agent.energy - 0.04).max(0.0);
                        agent.brain.neuromod.update(0.3, 0.5, false, false, 0.0);
                    }
                }
                ActionChoice::MineIron => {
                    let (ax, ay) = (self.agents[idx].x, self.agents[idx].y);
                    let tile = self.world.tile_at_mut(ax, ay);
                    if tile.iron_ore_amount > 0 {
                        tile.iron_ore_amount -= 1;
                        let agent = &mut self.agents[idx];
                        agent.inventory.add(ItemId::IronOre, 1);
                        agent.energy = (agent.energy - 0.06).max(0.0);
                        agent.brain.neuromod.update(0.7, 0.5, false, false, 0.1);
                        agent.brain.memory.log_episodic_event(
                            current_tick,
                            "Struck an iron vein! Extracted raw iron ore.",
                            vec!["Iron".into(), "Crafting".into()],
                            7.5,
                        );
                    }
                }
                ActionChoice::CraftTool => {
                    let agent = &mut self.agents[idx];
                    if agent.inventory.count(ItemId::Wood) >= 1 && agent.inventory.count(ItemId::Flint) >= 1 {
                        agent.inventory.remove(ItemId::Wood, 1);
                        agent.inventory.remove(ItemId::Flint, 1);
                        agent.inventory.add(ItemId::PrimitiveChopper, 1);
                        agent.brain.neuromod.update(0.8, 0.7, false, false, 0.1);
                        agent.brain.memory.log_episodic_event(
                            current_tick,
                            "Crafted a Primitive Chopper! Wood harvesting speed doubled.",
                            vec!["Crafting".into(), "Mastery".into()],
                            8.0,
                        );
                    }
                }
                ActionChoice::CourtNearestMate => {
                    let (ax, ay, a_name, a_race, a_char, a_wealth, a_health) = {
                        let a = &self.agents[idx];
                        (a.x, a.y, a.name.clone(), a.race, a.brain.charisma, a.inventory.total_wealth(), a.health)
                    };

                    let mut mate_idx = None;
                    for (other_idx, other) in self.agents.iter().enumerate() {
                        if other_idx != idx && other.is_alive && other.race == a_race {
                            let d = (other.x as isize - ax as isize).abs() + (other.y as isize - ay as isize).abs();
                            if d <= 3 {
                                mate_idx = Some(other_idx);
                                break;
                            }
                        }
                    }

                    if let Some(m_idx) = mate_idx {
                        let attraction = ReproductionManager::calculate_attraction(
                            a_char,
                            a_wealth,
                            self.agents[m_idx].brain.social_ledger.get_reputation(self.agents[idx].id),
                            a_health,
                        );

                        if attraction > 0.55 {
                            // Successful courtship!
                            self.agents[idx].social_need = (self.agents[idx].social_need - 0.5).max(0.0);
                            self.agents[m_idx].social_need = (self.agents[m_idx].social_need - 0.5).max(0.0);
                            self.agents[idx].partner_id = Some(self.agents[m_idx].id);
                            self.agents[m_idx].partner_id = Some(self.agents[idx].id);

                            // Reproduction if both well fed
                            if self.agents[idx].hunger < 0.5 && self.agents[m_idx].hunger < 0.5 && self.agents.len() < 35 {
                                let child_genome = ReproductionManager::create_child_genome(
                                    &self.agents[idx].brain.genome,
                                    &self.agents[m_idx].brain.genome,
                                );
                                new_borns.push((
                                    a_race,
                                    ax,
                                    ay,
                                    child_genome,
                                    format!("Child of {} and {}", a_name, self.agents[m_idx].name),
                                ));
                                self.agents[idx].children_count += 1;
                                self.agents[m_idx].children_count += 1;
                            }
                        }
                    }
                }
                ActionChoice::MakeAltarOffering => {
                    let (ax, ay, a_id, a_name, a_piety, a_wealth) = {
                        let a = &self.agents[idx];
                        (a.x, a.y, a.id, a.name.clone(), a.brain.piety, a.inventory.total_wealth())
                    };

                    for (altar_idx, altar) in self.world.altars.iter().enumerate() {
                        let d = (altar.x as isize - ax as isize).abs() + (altar.y as isize - ay as isize).abs();
                        if d <= 2 {
                            let item_to_give = if self.agents[idx].inventory.count(ItemId::IronOre) > 0 {
                                Some((ItemId::IronOre, 1))
                            } else if self.agents[idx].inventory.count(ItemId::Berry) > 0 {
                                Some((ItemId::Berry, 1))
                            } else if self.agents[idx].inventory.count(ItemId::Wood) > 0 {
                                Some((ItemId::Wood, 1))
                            } else {
                                None
                            };

                            if let Some((item, count)) = item_to_give {
                                self.agents[idx].inventory.remove(item, count);
                                offerings_to_record.push((altar_idx, a_id, a_name.clone(), item, count, a_wealth, a_piety));
                            }
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        // Record offerings to altars and advance covenants
        for (altar_idx, a_id, a_name, item, count, a_wealth, a_piety) in offerings_to_record {
            let offer_id = self.world.altars[altar_idx].recent_offerings.len() + 1;
            let sincerity = self.world.altars[altar_idx].receive_offering(
                offer_id,
                a_id,
                a_name.clone(),
                item,
                count,
                a_wealth,
                a_piety,
                current_tick,
            );

            self.log_event(format!(
                "{} brought an offering of {}x {} to {}! (Sincerity: {:.2})",
                a_name, count, item.display_name(), self.world.altars[altar_idx].name, sincerity
            ));

            let mut completed_covenants = Vec::new();
            // Advance covenant progress if applicable
            for cov in self.covenants.iter_mut() {
                if cov.objective == QuestObjectiveType::PresentAltarOfferings {
                    if cov.advance(1) {
                        completed_covenants.push(format!(
                            "DIVINE COVENANT FULFILLED! '{}' was completed! Granted {}",
                            cov.title, cov.promised_reward.display_name()
                        ));
                    }
                }
            }
            for msg in completed_covenants {
                self.log_event(msg);
            }
        }

        // Spawn newborn agents
        for (race, x, y, genome, name) in new_borns {
            let id = self.next_agent_id;
            self.next_agent_id += 1;
            let archetype = AmbitionArchetype::random_archetype();
            let echo = self.akashic_core.maybe_awaken_echo();
            let mut baby = Agent::new(id, name.clone(), race, x, y, archetype, echo);
            baby.brain.genome = genome;
            self.log_event(format!("A new child has been born: {}! Inherited genetic AST.", name));
            self.agents.push(baby);
        }

        // Check deceased agents and commit legends to Akashic Core
        for agent in self.agents.iter_mut() {
            if !agent.is_alive && agent.age_ticks > 0 {
                // If they lived a long life, enshrine them into Akashic Core
                if agent.age_ticks > 400 {
                    self.akashic_core.record_legend(
                        agent.name.clone(),
                        format!("Elder of the {}th generation", agent.brain.genome.generation),
                        agent.brain.genome.generation,
                        format!("Survived {} ticks and mastered the art of {}", agent.age_ticks, agent.brain.desire.archetype.display_name()),
                    );
                }
                agent.age_ticks = 0; // Mark processed
            }
        }

        // Remove dead agents (keep population clean)
        self.agents.retain(|a| a.is_alive);
    }

    fn find_nearest_target_coords(
        world: &WorldGrid,
        agents: &[Agent],
        agent: &Agent,
        filter: EntityFilter,
    ) -> Option<(usize, usize)> {
        match filter {
            EntityFilter::Altar => {
                let mut best = None;
                let mut best_dist = 999.0;
                for altar in &world.altars {
                    let d = (altar.x as f32 - agent.x as f32).hypot(altar.y as f32 - agent.y as f32);
                    if d < best_dist {
                        best_dist = d;
                        best = Some((altar.x, altar.y));
                    }
                }
                best
            }
            EntityFilter::FoodSource => {
                let mut best = None;
                let mut best_dist = 999.0;
                for y in 0..world.height {
                    for x in 0..world.width {
                        let t = world.tile_at(x, y);
                        if t.berry_amount > 0 || t.grain_amount > 0 {
                            let d = (x as f32 - agent.x as f32).hypot(y as f32 - agent.y as f32);
                            if d < best_dist {
                                best_dist = d;
                                best = Some((x, y));
                            }
                        }
                    }
                }
                best
            }
            EntityFilter::WaterSource => {
                let mut best = None;
                let mut best_dist = 999.0;
                for y in 0..world.height {
                    for x in 0..world.width {
                        let t = world.tile_at(x, y);
                        if t.has_fresh_water {
                            let d = (x as f32 - agent.x as f32).hypot(y as f32 - agent.y as f32);
                            if d < best_dist {
                                best_dist = d;
                                best = Some((x, y));
                            }
                        }
                    }
                }
                best
            }
            EntityFilter::Tree => {
                let mut best = None;
                let mut best_dist = 999.0;
                for y in 0..world.height {
                    for x in 0..world.width {
                        let t = world.tile_at(x, y);
                        if t.wood_amount > 0 {
                            let d = (x as f32 - agent.x as f32).hypot(y as f32 - agent.y as f32);
                            if d < best_dist {
                                best_dist = d;
                                best = Some((x, y));
                            }
                        }
                    }
                }
                best
            }
            EntityFilter::PotentialMate => {
                let mut best = None;
                let mut best_dist = 999.0;
                for other in agents {
                    if other.id != agent.id && other.is_alive && other.race == agent.race {
                        let d = (other.x as f32 - agent.x as f32).hypot(other.y as f32 - agent.y as f32);
                        if d < best_dist {
                            best_dist = d;
                            best = Some((other.x, other.y));
                        }
                    }
                }
                best
            }
            _ => None,
        }
    }

    /// Handles a God intervention from the Admin Console
    pub fn execute_intervention(&mut self, intervention: DivineInterventionKind) {
        match intervention {
            DivineInterventionKind::DivineWhisper { target_agent_id, message } => {
                if let Some(target_id) = target_agent_id {
                    let mut found_name = None;
                    if let Some(agent) = self.agents.iter_mut().find(|a| a.id == target_id) {
                        agent.brain.memory.log_episodic_event(
                            self.world.gaia.world_tick,
                            format!("God's Voice whispered directly to my mind: '{}'", message),
                            vec!["DivineWhisper".into(), "Revelation".into()],
                            10.0,
                        );
                        found_name = Some(agent.name.clone());
                    }
                    if let Some(name) = found_name {
                        self.log_event(format!("You whispered directly into {}'s mind: '{}'", name, message));
                    }
                } else {
                    for agent in self.agents.iter_mut() {
                        agent.brain.memory.log_episodic_event(
                            self.world.gaia.world_tick,
                            format!("A booming divine voice echoed from the heavens: '{}'", message),
                            vec!["DivineWhisper".into(), "HeavenlyRevelation".into()],
                            10.0,
                        );
                    }
                    self.log_event(format!("You spoke from the heavens to all living souls: '{}'", message));
                }
            }
            DivineInterventionKind::MiracleManna { x, y } => {
                let x = x.min(WORLD_WIDTH - 1);
                let y = y.min(WORLD_HEIGHT - 1);
                let tile = self.world.tile_at_mut(x, y);
                tile.berry_amount += 30;
                tile.grain_amount += 30;
                self.log_event(format!("MIRACLE: Manna fell from the heavens at coordinates ({}, {})!", x, y));
            }
            DivineInterventionKind::SmiteLightning { x, y } => {
                let x = x.min(WORLD_WIDTH - 1);
                let y = y.min(WORLD_HEIGHT - 1);
                for agent in self.agents.iter_mut() {
                    let d = (agent.x as isize - x as isize).abs() + (agent.y as isize - y as isize).abs();
                    if d <= 2 {
                        agent.health -= 45.0;
                        agent.brain.neuromod.noradrenaline = 0.95; // Extreme fear
                        agent.brain.memory.log_episodic_event(
                            self.world.gaia.world_tick,
                            "Struck by wrathful divine lightning bolt!",
                            vec!["Wrath".into(), "Cataclysm".into()],
                            10.0,
                        );
                    }
                }
                self.log_event(format!("WRATH: A blinding lightning bolt smote the earth at ({}, {})!", x, y));
            }
            DivineInterventionKind::TowerOfBabel => {
                for agent in self.agents.iter_mut() {
                    agent.brain.genome.mutate(0.8);
                }
                self.log_event("TOWER OF BABEL: All emergent behavioral dialects were scrambled by divine decree!");
            }
            DivineInterventionKind::GenesisEvolutionSpark => {
                for agent in self.agents.iter_mut() {
                    agent.brain.genome.mutate(0.9);
                }
                self.log_event("GENESIS SPARK: Forced an immediate evolutionary epoch across all living agents!");
            }
            _ => {}
        }
    }

    /// Generates telemetry payload for the God Panel Web Console
    pub fn generate_telemetry(&self) -> WorldStateTelemetry {
        let agent_summaries: Vec<AgentSummary> = self
            .agents
            .iter()
            .map(|a| AgentSummary {
                id: a.id,
                name: a.name.clone(),
                race: a.race,
                x: a.x,
                y: a.y,
                health: a.health,
                hunger: a.hunger,
                energy: a.energy,
                warmth: a.warmth,
                archetype: a.brain.desire.archetype,
                current_thought: a.brain.current_thought.clone(),
                last_action: format!("{:?}", a.brain.last_action),
                generation: a.brain.genome.generation,
            })
            .collect();

        let selected_agent_detail = self.selected_agent_id.and_then(|id| {
            self.agents.iter().find(|a| a.id == id).map(|a| {
                let items: Vec<(String, u32)> = a
                    .inventory
                    .items
                    .iter()
                    .map(|(k, v)| (k.display_name().to_string(), *v))
                    .collect();

                let beliefs = a.brain.knowledge.beliefs.iter().map(|b| b.concept.clone()).collect();
                let recent_memories = a.brain.memory.episodic_stream.iter().rev().take(10).cloned().collect();

                SelectedAgentDetail {
                    id: a.id,
                    name: a.name.clone(),
                    race: a.race,
                    x: a.x,
                    y: a.y,
                    health: a.health,
                    hunger: a.hunger,
                    thirst: a.thirst,
                    energy: a.energy,
                    warmth: a.warmth,
                    social_need: a.social_need,
                    piety: a.brain.piety,
                    mana: a.brain.mana,
                    archetype: a.brain.desire.archetype,
                    neuromod: a.brain.neuromod.clone(),
                    current_thought: a.brain.current_thought.clone(),
                    inventory_items: items,
                    recent_memories,
                    beliefs,
                    genome: a.brain.genome.clone(),
                    partner_id: a.partner_id,
                    children_count: a.children_count,
                    ancestral_echo: a.ancestral_echo.as_ref().map(|e| format!("{}: {}", e.hero_name, e.gift_description)),
                }
            })
        });

        WorldStateTelemetry {
            world_tick: self.world.gaia.world_tick,
            speed_multiplier: self.speed_multiplier,
            weather: self.world.gaia.current_weather,
            solar_intensity: self.world.gaia.solar_intensity,
            global_temperature_offset: self.world.gaia.global_temperature_offset,
            agents: agent_summaries,
            altars: self.world.altars.clone(),
            wildlife_count: self.world.wildlife.len(),
            active_covenants: self.covenants.clone(),
            rival_god_favor: self.civilizations.rival_god.shadow_favor_meter,
            rival_god_directive: self.civilizations.rival_god.active_directive.clone(),
            event_log: self.event_log.clone(),
            selected_agent: selected_agent_detail,
        }
    }
}
