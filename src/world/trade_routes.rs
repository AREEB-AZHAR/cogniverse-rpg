use super::shipping::{find_water_path, ShipEntity, ShipStatus};
use super::tile::Tile;
use crate::economy::{Commodity, Market};
use serde::{Deserialize, Serialize};

/// A coastal harbor port facilitating vessel docking, cargo transfers, and trade
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DockPort {
    pub id: usize,
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub berth_capacity: usize,
    pub docked_ships: Vec<usize>,
    pub market_id: usize,
}

impl DockPort {
    pub fn new(id: usize, name: impl Into<String>, x: usize, y: usize, market_id: usize) -> Self {
        Self {
            id,
            name: name.into(),
            x,
            y,
            berth_capacity: 4,
            docked_ships: Vec::new(),
            market_id,
        }
    }

    pub fn has_available_berth(&self) -> bool {
        self.docked_ships.len() < self.berth_capacity
    }

    pub fn dock_vessel(&mut self, ship_id: usize) -> bool {
        if self.has_available_berth() && !self.docked_ships.contains(&ship_id) {
            self.docked_ships.push(ship_id);
            true
        } else {
            false
        }
    }

    pub fn undock_vessel(&mut self, ship_id: usize) {
        self.docked_ships.retain(|&id| id != ship_id);
    }
}

/// A maritime trade shipping route between two coastal ports
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeRoute {
    pub id: usize,
    pub origin_dock_id: usize,
    pub destination_dock_id: usize,
    pub commodity: Commodity,
    pub return_commodity: Option<Commodity>,
    pub assigned_ships: Vec<usize>,
    pub is_active: bool,
    pub total_volume_traded: u32,
    pub total_profit_generated: f32,
}

impl TradeRoute {
    pub fn new(
        id: usize,
        origin_dock_id: usize,
        destination_dock_id: usize,
        commodity: Commodity,
        return_commodity: Option<Commodity>,
    ) -> Self {
        Self {
            id,
            origin_dock_id,
            destination_dock_id,
            commodity,
            return_commodity,
            assigned_ships: Vec::new(),
            is_active: true,
            total_volume_traded: 0,
            total_profit_generated: 0.0,
        }
    }
}

/// Central maritime trade and shipping manager coordinating ports, vessels, and routes
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ShippingNetwork {
    pub docks: Vec<DockPort>,
    pub ships: Vec<ShipEntity>,
    pub routes: Vec<TradeRoute>,
    pub next_dock_id: usize,
    pub next_ship_id: usize,
    pub next_route_id: usize,
}

impl ShippingNetwork {
    pub fn new() -> Self {
        Self {
            docks: Vec::new(),
            ships: Vec::new(),
            routes: Vec::new(),
            next_dock_id: 1,
            next_ship_id: 1,
            next_route_id: 1,
        }
    }

    pub fn register_dock(
        &mut self,
        name: impl Into<String>,
        x: usize,
        y: usize,
        market_id: usize,
    ) -> usize {
        let id = self.next_dock_id;
        self.next_dock_id += 1;
        self.docks.push(DockPort::new(id, name, x, y, market_id));
        id
    }

    pub fn get_dock(&self, id: usize) -> Option<&DockPort> {
        self.docks.iter().find(|d| d.id == id)
    }

    pub fn get_dock_mut(&mut self, id: usize) -> Option<&mut DockPort> {
        self.docks.iter_mut().find(|d| d.id == id)
    }

    pub fn get_ship(&self, id: usize) -> Option<&ShipEntity> {
        self.ships.iter().find(|s| s.id == id)
    }

    pub fn get_ship_mut(&mut self, id: usize) -> Option<&mut ShipEntity> {
        self.ships.iter_mut().find(|s| s.id == id)
    }

    pub fn add_ship(&mut self, ship: ShipEntity) {
        if let Some(dock) = self.get_dock_mut(ship.home_dock) {
            dock.dock_vessel(ship.id);
        }
        self.ships.push(ship);
    }

    pub fn create_route(
        &mut self,
        origin_dock_id: usize,
        destination_dock_id: usize,
        commodity: Commodity,
        return_commodity: Option<Commodity>,
    ) -> usize {
        let id = self.next_route_id;
        self.next_route_id += 1;
        self.routes.push(TradeRoute::new(
            id,
            origin_dock_id,
            destination_dock_id,
            commodity,
            return_commodity,
        ));
        id
    }

    pub fn assign_ship_to_route(&mut self, route_id: usize, ship_id: usize) -> bool {
        if let Some(route) = self.routes.iter_mut().find(|r| r.id == route_id) {
            if !route.assigned_ships.contains(&ship_id) {
                route.assigned_ships.push(ship_id);
                return true;
            }
        }
        false
    }

    /// Evaluates arbitrage opportunities between markets and dispatches trade voyages
    pub fn tick_trade_arbitrage(
        &mut self,
        tiles: &[Tile],
        width: usize,
        height: usize,
        markets: &mut [Market],
    ) {
        // Collect active routes and their configurations
        let route_configs: Vec<(usize, usize, usize, Commodity, Option<Commodity>)> = self
            .routes
            .iter()
            .filter(|r| r.is_active)
            .map(|r| {
                (
                    r.id,
                    r.origin_dock_id,
                    r.destination_dock_id,
                    r.commodity,
                    r.return_commodity,
                )
            })
            .collect();

        for (route_id, origin_id, dest_id, outbound_good, return_good) in route_configs {
            let origin_dock = match self.get_dock(origin_id) {
                Some(d) => d.clone(),
                None => continue,
            };
            let dest_dock = match self.get_dock(dest_id) {
                Some(d) => d.clone(),
                None => continue,
            };

            for ship in self.ships.iter_mut() {
                // Check if ship is assigned to this route or idle at origin dock
                let is_assigned = self
                    .routes
                    .iter()
                    .any(|r| r.id == route_id && r.assigned_ships.contains(&ship.id));
                if !is_assigned {
                    continue;
                }

                match ship.status {
                    ShipStatus::AtPort { dock_id } => {
                        if dock_id == origin_id {
                            // At origin port: load outbound commodity from market
                            let origin_market =
                                markets.iter_mut().find(|m| m.id == origin_dock.market_id);
                            if let Some(m) = origin_market {
                                let capacity = ship.available_capacity();
                                let available = m.get_supply(outbound_good) as u32;
                                let to_load = capacity.min(available);

                                if to_load > 0 {
                                    let mut treasury = 9999.0;
                                    let _ = m.buy(outbound_good, to_load, &mut treasury);
                                    ship.load_cargo(outbound_good, to_load);
                                }
                            }

                            // Calculate water route to destination dock
                            if let Some(path) = find_water_path(
                                (ship.x, ship.y),
                                (dest_dock.x, dest_dock.y),
                                tiles,
                                width,
                                height,
                            ) {
                                ship.path = path;
                                ship.status = ShipStatus::EnRoute {
                                    destination_dock: dest_id,
                                    target_x: dest_dock.x,
                                    target_y: dest_dock.y,
                                };
                            }
                        } else if dock_id == dest_id {
                            // At destination port: unload and sell outbound cargo
                            let dest_market =
                                markets.iter_mut().find(|m| m.id == dest_dock.market_id);
                            if let Some(m) = dest_market {
                                let cargo_amount = *ship.cargo.get(&outbound_good).unwrap_or(&0);
                                if cargo_amount > 0 {
                                    let mut seller_wallet = 0.0;
                                    let _ = m.sell(outbound_good, cargo_amount, &mut seller_wallet);
                                    ship.unload_cargo(outbound_good, cargo_amount);

                                    // Update route stats
                                    if let Some(route) =
                                        self.routes.iter_mut().find(|r| r.id == route_id)
                                    {
                                        route.total_volume_traded += cargo_amount;
                                        route.total_profit_generated += seller_wallet;
                                    }
                                }

                                // Optionally load return cargo
                                if let Some(ret_good) = return_good {
                                    let capacity = ship.available_capacity();
                                    let available = m.get_supply(ret_good) as u32;
                                    let to_load = capacity.min(available);
                                    if to_load > 0 {
                                        let mut treasury = 9999.0;
                                        let _ = m.buy(ret_good, to_load, &mut treasury);
                                        ship.load_cargo(ret_good, to_load);
                                    }
                                }
                            }

                            // Sail back to origin port
                            if let Some(path) = find_water_path(
                                (ship.x, ship.y),
                                (origin_dock.x, origin_dock.y),
                                tiles,
                                width,
                                height,
                            ) {
                                ship.path = path;
                                ship.status = ShipStatus::EnRoute {
                                    destination_dock: origin_id,
                                    target_x: origin_dock.x,
                                    target_y: origin_dock.y,
                                };
                            }
                        }
                    }
                    ShipStatus::EnRoute {
                        destination_dock,
                        target_x,
                        target_y,
                    } => {
                        // Advance vessel along calculated water path
                        let moved = ship.step_along_path();
                        if !moved || (ship.x == target_x && ship.y == target_y) {
                            ship.status = ShipStatus::AtPort {
                                dock_id: destination_dock,
                            };
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
