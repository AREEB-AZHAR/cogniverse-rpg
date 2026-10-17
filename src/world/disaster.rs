use super::danger::DangerField;
use super::tile::{BiomeType, StructureKind, Tile};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisasterKind {
    Wildfire,
    Flood,
    Earthquake,
    Drought,
    Plague,
}

impl DisasterKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            DisasterKind::Wildfire => "Raging Wildfire",
            DisasterKind::Flood => "Catastrophic Torrential Flood",
            DisasterKind::Earthquake => "Tectonic Earthquake",
            DisasterKind::Drought => "Severe Arid Drought",
            DisasterKind::Plague => "Virulent Contagion Plague",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            DisasterKind::Wildfire => "🔥",
            DisasterKind::Flood => "🌊",
            DisasterKind::Earthquake => "🌋",
            DisasterKind::Drought => "☀️",
            DisasterKind::Plague => "☣️",
        }
    }
}

/// An active localized disaster progressing across the world grid
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveDisaster {
    pub id: usize,
    pub kind: DisasterKind,
    pub x: usize,
    pub y: usize,
    pub radius: usize,
    pub remaining_ticks: u32,
    pub intensity: f32,
}

/// Settlement adaptation tracker recording cumulative hazard damage and unlocking countermeasures
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AdaptationTracker {
    pub wildfire_damage_suffered: u32,
    pub flood_damage_suffered: u32,
    pub earthquake_damage_suffered: u32,
    pub drought_damage_suffered: u32,
    pub plague_damage_suffered: u32,

    pub researched_firebreaks: bool,
    pub researched_levees: bool,
    pub researched_granaries: bool,
    pub researched_quarantine: bool,
    pub researched_reinforcement: bool,
}

impl AdaptationTracker {
    pub fn record_damage(&mut self, kind: DisasterKind, amount: u32) -> Vec<String> {
        let mut unlocked = Vec::new();
        match kind {
            DisasterKind::Wildfire => {
                self.wildfire_damage_suffered += amount;
                if self.wildfire_damage_suffered >= 50 && !self.researched_firebreaks {
                    self.researched_firebreaks = true;
                    unlocked.push(
                        "Researched Firebreak Zoning (Halts wildfire spread across settlements)"
                            .into(),
                    );
                }
            }
            DisasterKind::Flood => {
                self.flood_damage_suffered += amount;
                if self.flood_damage_suffered >= 50 && !self.researched_levees {
                    self.researched_levees = true;
                    unlocked.push(
                        "Researched Coastal Levees & Sea-Walls (Prevents coastal flood inundation)"
                            .into(),
                    );
                }
            }
            DisasterKind::Drought => {
                self.drought_damage_suffered += amount;
                if self.drought_damage_suffered >= 50 && !self.researched_granaries {
                    self.researched_granaries = true;
                    unlocked.push(
                        "Researched Communal Granaries (Protects food supplies against drought)"
                            .into(),
                    );
                }
            }
            DisasterKind::Plague => {
                self.plague_damage_suffered += amount;
                if self.plague_damage_suffered >= 50 && !self.researched_quarantine {
                    self.researched_quarantine = true;
                    unlocked.push(
                        "Researched Herbal Quarantine Protocols (Mitigates plague contagion)"
                            .into(),
                    );
                }
            }
            DisasterKind::Earthquake => {
                self.earthquake_damage_suffered += amount;
                if self.earthquake_damage_suffered >= 50 && !self.researched_reinforcement {
                    self.researched_reinforcement = true;
                    unlocked.push(
                        "Researched Timber-Braced Masonry (Prevents structure collapse in tremors)"
                            .into(),
                    );
                }
            }
        }
        unlocked
    }
}

/// Global disaster simulation engine coordinating hazard lifecycles and world interactions
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DisasterEngine {
    pub active_disasters: Vec<ActiveDisaster>,
    pub next_disaster_id: usize,
    pub adaptation: AdaptationTracker,
    pub event_log: Vec<String>,
}

impl DisasterEngine {
    pub fn new() -> Self {
        Self {
            active_disasters: Vec::new(),
            next_disaster_id: 1,
            adaptation: AdaptationTracker::default(),
            event_log: Vec::new(),
        }
    }

    /// Spawns a new active disaster at (x, y) with specified radius, duration, and intensity
    pub fn trigger_disaster(
        &mut self,
        kind: DisasterKind,
        x: usize,
        y: usize,
        radius: usize,
        duration_ticks: u32,
        intensity: f32,
    ) -> usize {
        let id = self.next_disaster_id;
        self.next_disaster_id += 1;

        let disaster = ActiveDisaster {
            id,
            kind,
            x,
            y,
            radius,
            remaining_ticks: duration_ticks,
            intensity,
        };

        self.event_log.push(format!(
            "⚠️ [DISASTER AWAKENED] {} strike at ({}, {}) with radius {}!",
            kind.display_name(),
            x,
            y,
            radius
        ));

        self.active_disasters.push(disaster);
        id
    }

    /// Ticks all active disasters, injects hazard intensity into DangerField, and damages structures
    pub fn tick(
        &mut self,
        danger_field: &mut DangerField,
        tiles: &mut [Tile],
        grid_width: usize,
        grid_height: usize,
    ) {
        let mut expired_ids = Vec::new();

        for disaster in self.active_disasters.iter_mut() {
            // 1. Inject danger into the dynamic danger field
            let danger_injection = disaster.intensity * 25.0;
            danger_field.inject_danger(disaster.x, disaster.y, disaster.radius, danger_injection);

            // 2. Apply environmental impact based on disaster type
            let min_x = disaster.x.saturating_sub(disaster.radius);
            let max_x = (disaster.x + disaster.radius).min(grid_width.saturating_sub(1));
            let min_y = disaster.y.saturating_sub(disaster.radius);
            let max_y = (disaster.y + disaster.radius).min(grid_height.saturating_sub(1));

            for cy in min_y..=max_y {
                for cx in min_x..=max_x {
                    let idx = cy * grid_width + cx;

                    if let Some(tile) = tiles.get_mut(idx) {
                        match disaster.kind {
                            DisasterKind::Wildfire => {
                                // Flammable biomes ignite
                                if tile.biome == BiomeType::WhisperingForest
                                    || tile.biome == BiomeType::SunlitPlains
                                {
                                    tile.is_on_fire = true;
                                    tile.fire_ticks = tile.fire_ticks.saturating_add(10);
                                }
                                // Wooden structures take fire damage unless firebreaks are researched
                                if let Some(ref mut s) = tile.structure {
                                    if !self.adaptation.researched_firebreaks
                                        && (s.kind == StructureKind::WoodCottage
                                            || s.kind == StructureKind::Granary
                                            || s.kind == StructureKind::Dock)
                                    {
                                        s.health = (s.health - 10.0).max(0.0);
                                        self.adaptation.record_damage(DisasterKind::Wildfire, 2);
                                    }
                                }
                            }
                            DisasterKind::Earthquake => {
                                // Tremors damage structures unless reinforcement is researched
                                if let Some(ref mut s) = tile.structure {
                                    if !self.adaptation.researched_reinforcement {
                                        s.health = (s.health - 15.0).max(0.0);
                                        self.adaptation.record_damage(DisasterKind::Earthquake, 3);
                                    }
                                }
                            }
                            DisasterKind::Flood => {
                                // Floods damage coastal/river facilities unless levees are built
                                if let Some(ref mut s) = tile.structure {
                                    if !self.adaptation.researched_levees
                                        && (s.kind == StructureKind::Dock
                                            || s.kind == StructureKind::BoatYard)
                                    {
                                        s.health = (s.health - 8.0).max(0.0);
                                        self.adaptation.record_damage(DisasterKind::Flood, 2);
                                    }
                                }
                            }
                            DisasterKind::Drought => {
                                self.adaptation.record_damage(DisasterKind::Drought, 1);
                            }
                            DisasterKind::Plague => {
                                self.adaptation.record_damage(DisasterKind::Plague, 1);
                            }
                        }
                    }
                }
            }

            // Decrement remaining duration
            disaster.remaining_ticks = disaster.remaining_ticks.saturating_sub(1);
            if disaster.remaining_ticks == 0 {
                expired_ids.push(disaster.id);
            }
        }

        // Clean up expired disasters
        self.active_disasters
            .retain(|d| !expired_ids.contains(&d.id));
    }
}
