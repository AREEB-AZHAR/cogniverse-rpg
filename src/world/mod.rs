pub mod altar;
pub mod cellular;
pub mod commands;
pub mod construction;
pub mod cube_sphere;
pub mod danger;
pub mod disaster;
pub mod dungeon;
pub mod ecology;
pub mod encounter;
pub mod fauna;
pub mod generation;
pub mod headless;
pub mod items;
pub mod outburst;
pub mod planet;
pub mod saves;
pub mod shipping;
pub mod tile;
pub mod trade_routes;
pub mod views;

pub use construction::{Blueprint, ConstructionStage, JobBoard};
pub use cube_sphere::{cell_dir, CellId};
pub use danger::DangerField;
pub use disaster::{ActiveDisaster, AdaptationTracker, DisasterEngine, DisasterKind};
pub use dungeon::{
    generate_dungeon, Dungeon, DungeonFloor, DungeonMonster, DungeonRoom, DungeonState, RoomKind,
};
pub use encounter::{
    DungeonExpedition, EncounterSimulator, ExpeditionMember, ExpeditionState, StepResult,
};
pub use headless::{HeadlessSimResult, HeadlessSimulator, TICKS_PER_YEAR};
pub use outburst::{DungeonOutburstSystem, OutbreakMob, OutburstHorde};
pub use planet::{CubeSphereWorld, PoiKind, PointOfInterest};
pub use saves::{SaveError, SaveSystem};
pub use shipping::{
    find_water_path, is_navigable_water, launch_ship_from_boat_yard, ShipEntity, ShipStatus,
    VesselKind,
};
pub use trade_routes::{DockPort, ShippingNetwork, TradeRoute};
pub use views::{GlobeView, RegionView};

use altar::Altar;
use cellular::CellularAutomataEngine;
use ecology::GaiaEngine;
pub use fauna::{
    AnimalBehaviorState, AnimalSpawnError, AnimalSpawningSystem, AnimalSpecies, EcologyManager,
    ExtinctionRecord, RepopulationEvent, SpawnAnimalRequest, WildlifeEntity,
};
pub use generation::MapArchetype;
use generation::MapGenerator;
use serde::{Deserialize, Serialize};
use tile::{BiomeType, Tile};

pub const WORLD_WIDTH: usize = 128;
pub const WORLD_HEIGHT: usize = 128;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldGrid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
    pub gaia: GaiaEngine,
    pub altars: Vec<Altar>,
    pub wildlife: Vec<WildlifeEntity>,
    pub next_entity_id: usize,
    #[serde(default)]
    pub ecology: EcologyManager,
    #[serde(default)]
    pub job_board: JobBoard,
    #[serde(default)]
    pub shipping: ShippingNetwork,
    #[serde(default)]
    pub danger_field: DangerField,
    #[serde(default)]
    pub disasters: DisasterEngine,
    #[serde(default)]
    pub dungeons: Vec<Dungeon>,
    #[serde(default)]
    pub expeditions: Vec<DungeonExpedition>,
    #[serde(default)]
    pub outburst_system: DungeonOutburstSystem,
    #[serde(default)]
    pub parties: Vec<crate::agent::party::Party>,
    #[serde(default = "default_party_seed_id")]
    pub next_party_id: usize,
    #[serde(default = "default_item_seed_id")]
    pub next_item_id: u64,
    #[serde(default = "default_stone_seed_id")]
    pub next_stone_id: u64,
}

fn default_party_seed_id() -> usize {
    1
}

fn default_item_seed_id() -> u64 {
    1001
}

fn default_stone_seed_id() -> u64 {
    5001
}

impl Default for WorldGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldGrid {
    pub fn new() -> Self {
        let width = WORLD_WIDTH;
        let height = WORLD_HEIGHT;

        // Generate map with coherent fractal noise, elevation layers, and faction resource bases
        let (mut tiles, altars) = MapGenerator::generate_world(width, height);
        Self::enrich_minerals(&mut tiles);
        let (wildlife, next_id) = Self::spawn_initial_wildlife();
        let dungeons = Self::spawn_initial_dungeons(width, height, 42);

        Self {
            width,
            height,
            tiles,
            gaia: GaiaEngine::new(),
            altars,
            wildlife,
            next_entity_id: next_id,
            ecology: EcologyManager::default(),
            job_board: JobBoard::new(),
            shipping: ShippingNetwork::new(),
            danger_field: DangerField::new(width, height),
            disasters: DisasterEngine::new(),
            dungeons,
            expeditions: Vec::new(),
            outburst_system: DungeonOutburstSystem::new(),
            parties: Vec::new(),
            next_party_id: 1,
            next_item_id: 1001,
            next_stone_id: 5001,
        }
    }

    /// Resets the world grid with the specified map archetype and procedural parameters
    pub fn reset_with_params(
        &mut self,
        archetype: MapArchetype,
        seed: u64,
        water_level: f32,
        mountain_density: f32,
    ) -> String {
        let (mut tiles, altars, map_title) = MapGenerator::generate_world_with_params(
            self.width,
            self.height,
            archetype,
            seed,
            water_level,
            mountain_density,
        );

        Self::enrich_minerals(&mut tiles);
        let (wildlife, next_id) = Self::spawn_initial_wildlife();

        self.tiles = tiles;
        self.altars = altars;
        self.wildlife = wildlife;
        self.next_entity_id = next_id;
        self.gaia = GaiaEngine::new();
        self.ecology = EcologyManager::default();
        self.job_board = JobBoard::new();
        self.shipping = ShippingNetwork::new();
        self.dungeons = Self::spawn_initial_dungeons(self.width, self.height, seed);
        self.expeditions.clear();
        self.outburst_system = DungeonOutburstSystem::new();
        self.parties.clear();

        map_title
    }

    fn enrich_minerals(tiles: &mut [Tile]) {
        for t in tiles.iter_mut() {
            if t.biome == BiomeType::IroncragMountains {
                if (t.x + t.y) % 11 == 0 {
                    t.iron_ore_amount += 40; // Dense Iron Outcrop
                }
                if (t.x * 3 + t.y) % 17 == 0 {
                    t.gold_ore_amount += 35; // Vein of Gold
                }
                if (t.x * 5 + t.y * 2) % 23 == 0 {
                    t.mithril_amount += 30; // Mithril Gem Pocket
                }
            } else if t.biome == BiomeType::AncientLeylineRuins && (t.x + t.y) % 7 == 0 {
                t.mithril_amount += 35;
            }
        }
    }

    fn spawn_initial_wildlife() -> (Vec<WildlifeEntity>, usize) {
        let mut wildlife = Vec::new();
        let mut next_id = 1;

        // Spawn deer herds
        for i in 0..28 {
            let wx = 14 + (i * 5) % 40;
            let wy = 16 + (i * 7) % 65;
            wildlife.push(WildlifeEntity::new(
                next_id,
                AnimalSpecies::ForestDeer,
                wx,
                wy,
            ));
            next_id += 1;
        }

        // Spawn dire wolves
        for i in 0..14 {
            let wx = 75 + (i * 4) % 42;
            let wy = 14 + (i * 6) % 40;
            wildlife.push(WildlifeEntity::new(
                next_id,
                AnimalSpecies::DireWolf,
                wx,
                wy,
            ));
            next_id += 1;
        }

        // Spawn rabbits
        for i in 0..36 {
            let wx = 22 + (i * 7) % 80;
            let wy = 60 + (i * 5) % 52;
            wildlife.push(WildlifeEntity::new(next_id, AnimalSpecies::Rabbit, wx, wy));
            next_id += 1;
        }

        // Spawn mountain bears
        for i in 0..12 {
            let wx = 70 + (i * 5) % 45;
            let wy = 20 + (i * 6) % 35;
            wildlife.push(WildlifeEntity::new(
                next_id,
                AnimalSpecies::MountainBear,
                wx,
                wy,
            ));
            next_id += 1;
        }

        (wildlife, next_id)
    }

    pub fn tile_at(&self, x: usize, y: usize) -> &Tile {
        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);
        &self.tiles[y * self.width + x]
    }

    pub fn tile_at_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);
        let idx = y * self.width + x;
        &mut self.tiles[idx]
    }

    pub fn tick_cellular(&mut self) -> Vec<(usize, usize)> {
        CellularAutomataEngine::tick(
            &mut self.tiles,
            self.width,
            self.height,
            self.gaia.global_temperature_offset,
            self.gaia.world_tick,
        )
    }

    pub fn tick(&mut self) {
        // Tick Gaia climate and flora
        self.gaia.tick(&mut self.tiles);

        // Tick wildlife
        let w = self.width;
        let h = self.height;
        for animal in self.wildlife.iter_mut() {
            animal.tick(w, h);
        }

        // Prune dead wildlife
        self.wildlife.retain(|a| a.is_alive);

        // Wildlife reproduction if population drops below carrying capacity
        if self.wildlife.len() < 60 && self.gaia.world_tick.is_multiple_of(80) {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let rx = rng.gen_range(12..w - 12);
            let ry = rng.gen_range(12..h - 12);
            let id = self.next_entity_id;
            self.next_entity_id += 1;
            let species = match rng.gen_range(0..4) {
                0 => AnimalSpecies::Rabbit,
                1 => AnimalSpecies::ForestDeer,
                2 => AnimalSpecies::MountainBear,
                _ => AnimalSpecies::DireWolf,
            };
            self.wildlife.push(WildlifeEntity::new(id, species, rx, ry));
        }

        // Tick danger field and active natural disasters
        self.danger_field.tick();
        self.disasters.tick(
            &mut self.danger_field,
            &mut self.tiles,
            self.width,
            self.height,
        );

        // Tick dungeons & expeditions
        self.tick_dungeons();
    }

    /// Spawns the initial canonical set of diverse procedural dungeons across ranks
    pub fn spawn_initial_dungeons(width: usize, height: usize, seed: u64) -> Vec<Dungeon> {
        use crate::agent::progression::Rank;
        vec![
            generate_dungeon(
                1,
                "Whispering Caverns",
                width / 4,
                height / 4,
                Rank::F,
                seed.wrapping_add(101),
            ),
            generate_dungeon(
                2,
                "Blighted Catacombs",
                (width * 3) / 4,
                height / 4,
                Rank::D,
                seed.wrapping_add(102),
            ),
            generate_dungeon(
                3,
                "Minotaur Labyrinth",
                width / 2,
                height / 2,
                Rank::C,
                seed.wrapping_add(103),
            ),
            generate_dungeon(
                4,
                "Sunken Citadel",
                width / 4,
                (height * 3) / 4,
                Rank::B,
                seed.wrapping_add(104),
            ),
            generate_dungeon(
                5,
                "Infernal Spire",
                (width * 3) / 4,
                (height * 3) / 4,
                Rank::A,
                seed.wrapping_add(105),
            ),
            generate_dungeon(
                6,
                "Astral Sanctum",
                width / 2,
                height / 6,
                Rank::S,
                seed.wrapping_add(106),
            ),
        ]
    }

    /// Advances active dungeon expeditions and respawn timers
    pub fn tick_dungeons(&mut self) {
        let mut rng = rand::thread_rng();

        // Respawn cleared dungeons if respawn cooldown elapses
        let world_seed = 9999;
        for dungeon in &mut self.dungeons {
            if dungeon.state == DungeonState::Cleared {
                dungeon.respawn_timer += 1;
                if dungeon.respawn_timer >= 200 {
                    dungeon.respawn(world_seed);
                }
            }
        }

        // Advance active expeditions
        for expedition in &mut self.expeditions {
            if let Some(dungeon) = self
                .dungeons
                .iter_mut()
                .find(|d| d.id == expedition.dungeon_id)
            {
                EncounterSimulator::step(
                    expedition,
                    dungeon,
                    &mut rng,
                    &mut self.next_item_id,
                    &mut self.next_stone_id,
                );
            }
        }
    }

    /// Executes one tick of autonomous construction work for an agent on the world's job board
    pub fn perform_construction_step(
        &mut self,
        agent_id: usize,
        inventory: &mut items::Inventory,
        needs: &mut crate::agent::needs::NpcNeeds,
    ) -> bool {
        crate::agent::utility_ai::UtilityAi::perform_construction_step(
            agent_id,
            inventory,
            needs,
            &mut self.job_board,
            &mut self.tiles,
            self.width,
        )
    }

    /// Advances maritime trade, shipping fleets, and cargo transfers across coastal docks
    pub fn tick_shipping(&mut self, markets: &mut [crate::economy::Market]) {
        self.shipping
            .tick_trade_arbitrage(&self.tiles, self.width, self.height, markets);
    }

    /// Advances dungeon outbursts, surface hordes, guild quest responses, and tactical party maneuvers
    pub fn tick_outbursts_and_parties(
        &mut self,
        civil_coordinator: &mut crate::factions::CivilizationsCoordinator,
        settlements: &[(usize, usize, usize)],
        current_tick: u64,
    ) -> Vec<String> {
        let mut events = Vec::new();
        let mut rng = rand::thread_rng();

        // 1. Tick dungeon mana swells & surface outburst hordes
        let new_breaks = self.outburst_system.tick(
            &mut self.dungeons,
            &mut self.danger_field,
            settlements,
            self.width,
            self.height,
            &mut rng,
        );

        // 2. Process emergency quests and geopolitical coalition appeals
        for d_id in new_breaks {
            if let Some(dungeon) = self.dungeons.iter().find(|d| d.id == d_id) {
                let q_id = civil_coordinator
                    .guild
                    .post_emergency_outburst_quest(dungeon, current_tick);
                events.push(format!(
                    "🚨 [DUNGEON OUTBURST] Mana rupture at [{}] (Rank {})! Surface horde is marching in rage toward settlements! Quest #{} issued.",
                    dungeon.name,
                    dungeon.rank.display_name(),
                    q_id
                ));

                if let Some(coalition_event) =
                    civil_coordinator.evaluate_outburst_coalition(dungeon.rank)
                {
                    events.push(coalition_event);
                }
            }
        }

        // 3. Advance tactical parties toward target and engage in combat
        for party in &mut self.parties {
            if !party.is_party_alive() {
                party.status = crate::agent::party::PartyStatus::Disbanded;
                continue;
            }

            // Find target horde if party has an active quest
            if let Some(q_id) = party.active_quest_id {
                let target_d_id = civil_coordinator
                    .guild
                    .quests
                    .iter()
                    .find(|q| q.id == q_id)
                    .and_then(|q| q.target_dungeon_id);

                if let Some(d_id) = target_d_id {
                    if let Some(horde) = self
                        .outburst_system
                        .active_hordes
                        .iter_mut()
                        .find(|h| h.dungeon_id == d_id && !h.is_contained)
                    {
                        let dist = (party.x as isize - horde.origin_x as isize).abs()
                            + (party.y as isize - horde.origin_y as isize).abs();

                        if dist <= 2 {
                            party.status = crate::agent::party::PartyStatus::InBattle;
                            let engaged = party.engage_outbreak_round(horde, &mut rng);
                            if engaged && horde.is_fully_defeated() {
                                horde.is_contained = true;
                                if let Some(dungeon) =
                                    self.dungeons.iter_mut().find(|d| d.id == d_id)
                                {
                                    dungeon.resolve_outburst();
                                }
                                party.status = crate::agent::party::PartyStatus::Resting;
                                civil_coordinator.guild.complete_quest(q_id);
                                if let Some(triumph) =
                                    civil_coordinator.resolve_coalition_victory(horde.rank)
                                {
                                    events.push(triumph);
                                }
                                events.push(format!(
                                    "⚔️ [OUTBURST CONTAINED] Party '{}' successfully eradicated the horde at ({}, {})!",
                                    party.name, horde.origin_x, horde.origin_y
                                ));
                            }
                        } else {
                            party.status = crate::agent::party::PartyStatus::EnRoute;
                            party.target_x = horde.origin_x;
                            party.target_y = horde.origin_y;
                            party.step_toward_target(self.width, self.height);
                        }
                    }
                }
            }
        }

        events
    }
}
