pub mod altar;
pub mod cellular;
pub mod commands;
pub mod construction;
pub mod cube_sphere;
pub mod danger;
pub mod disaster;
pub mod ecology;
pub mod fauna;
pub mod generation;
pub mod headless;
pub mod items;
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
pub use headless::{HeadlessSimResult, HeadlessSimulator, TICKS_PER_YEAR};
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
}
