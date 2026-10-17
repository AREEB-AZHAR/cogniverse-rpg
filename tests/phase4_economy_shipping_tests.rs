use cogniverse_rpg::economy::{Commodity, Market};
use cogniverse_rpg::world::shipping::{
    find_water_path, is_navigable_water, launch_ship_from_boat_yard, ShipEntity, ShipStatus,
    VesselKind,
};
use cogniverse_rpg::world::tile::{
    BiomeType, ElevationLayer, PlacedStructure, StructureKind, Tile,
};
use cogniverse_rpg::world::WorldGrid;

#[test]
fn test_supply_demand_price_dynamics_and_inertia() {
    let mut market = Market::new(1, "Port Royal Market", 20, 20);

    // 1. Baseline price check: when supply == demand, price should be near base price
    let base_grain = Commodity::Grain.base_price(); // 2.0
    let initial_target = market.calculate_target_price(Commodity::Grain);
    assert!(
        (initial_target - base_grain).abs() < 0.1,
        "Equilibrium target price should match base price: target={}, base={}",
        initial_target,
        base_grain
    );

    // 2. High scarcity: restrict supply to 2.0, increase demand to 40.0
    market.set_supply(Commodity::Grain, 2.0);
    market.set_demand(Commodity::Grain, 40.0);
    let scarce_price = market.calculate_target_price(Commodity::Grain);
    // target = 2.0 * sqrt(41 / 3) = 2.0 * sqrt(13.67) ~ 7.39
    assert!(
        scarce_price > base_grain * 2.5,
        "High demand / low supply must raise price significantly: scarce_price = {}",
        scarce_price
    );
    assert!(
        scarce_price <= base_grain * 8.0,
        "Price must not exceed 8.0x ceiling clamp"
    );

    // 3. Extreme oversupply: supply = 200.0, demand = 1.0
    market.set_supply(Commodity::Grain, 200.0);
    market.set_demand(Commodity::Grain, 1.0);
    let surplus_price = market.calculate_target_price(Commodity::Grain);
    assert_eq!(
        surplus_price,
        base_grain * 0.25,
        "Oversupplied price must be clamped at minimum floor 0.25x base price"
    );

    // 4. Price smoothing inertia: tick exponential moving average
    market.set_supply(Commodity::IronOre, 5.0);
    market.set_demand(Commodity::IronOre, 50.0);
    let target_iron = market.calculate_target_price(Commodity::IronOre);
    let start_price = market.get_price(Commodity::IronOre);

    market.tick_price_smoothing(0.20);
    let smoothed_price_1 = market.get_price(Commodity::IronOre);
    assert!(
        smoothed_price_1 > start_price && smoothed_price_1 < target_iron,
        "Price should transition smoothly towards target: start={}, step1={}, target={}",
        start_price,
        smoothed_price_1,
        target_iron
    );

    // 5. Market buy and sell transactions
    let mut buyer_wallet = 100.0;
    market.set_supply(Commodity::Wood, 30.0);
    let wood_price = market.get_price(Commodity::Wood);
    let buy_result = market.buy(Commodity::Wood, 5, &mut buyer_wallet);
    assert!(buy_result.is_ok());
    assert_eq!(market.get_supply(Commodity::Wood), 25.0);
    assert!((buyer_wallet - (100.0 - wood_price * 5.0)).abs() < 1e-3);

    let mut seller_wallet = 10.0;
    let sell_result = market.sell(Commodity::Wood, 10, &mut seller_wallet);
    assert!(sell_result.is_ok());
    assert_eq!(market.get_supply(Commodity::Wood), 35.0);
    assert!(seller_wallet > 10.0);
}

#[test]
fn test_boat_construction_and_launch_at_boat_yard() {
    let mut world = WorldGrid::new();
    let width = world.width;

    let yard_x = 30;
    let yard_y = 30;
    let water_x = 30;
    let water_y = 31;

    // Set surrounding 3x3 neighborhood to land first
    for y in 29..=31 {
        for x in 29..=31 {
            let t = world.tile_at_mut(x, y);
            t.elevation_layer = ElevationLayer::SoilLow;
            t.biome = BiomeType::SunlitPlains;
            t.structure = None;
        }
    }

    // Set yard tile as land with a BoatYard structure
    {
        let yard_tile = world.tile_at_mut(yard_x, yard_y);
        yard_tile.elevation_layer = ElevationLayer::SoilLow;
        yard_tile.biome = BiomeType::SunlitPlains;
        yard_tile.structure = Some(PlacedStructure::new(StructureKind::BoatYard));
    }

    // Set adjacent tile as navigable river/water
    {
        let water_tile = world.tile_at_mut(water_x, water_y);
        water_tile.elevation_layer = ElevationLayer::ShallowWaters;
        water_tile.biome = BiomeType::CrystalRiverlands;
        assert!(is_navigable_water(water_tile));
    }

    // Register a coastal dock
    let dock_id = world
        .shipping
        .register_dock("Sunken Cove Dock", yard_x + 1, yard_y, 1);

    // Launch a CargoCog from the BoatYard
    let ship_result = launch_ship_from_boat_yard(
        yard_x,
        yard_y,
        VesselKind::CargoCog,
        "The Gilded Albatross",
        &world.tiles,
        width,
        world.height,
        dock_id,
        world.shipping.next_ship_id,
    );

    assert!(
        ship_result.is_ok(),
        "Ship should launch into adjacent navigable water"
    );
    let ship = ship_result.unwrap();
    assert_eq!(ship.x, water_x);
    assert_eq!(ship.y, water_y);
    assert_eq!(ship.kind, VesselKind::CargoCog);
    assert_eq!(ship.available_capacity(), 150);
    assert_eq!(ship.durability, 150.0);

    // Add to shipping network
    world.shipping.add_ship(ship);
    let registered_dock = world.shipping.get_dock(dock_id).unwrap();
    assert!(registered_dock.docked_ships.contains(&1));
}

#[test]
fn test_maritime_water_pathfinding_navigation() {
    let width = 30;
    let height = 30;
    let mut tiles = Vec::with_capacity(width * height);

    // Initialize all tiles as impassable land
    for y in 0..height {
        for x in 0..width {
            let mut t = Tile::new(x, y, BiomeType::SunlitPlains, 15.0, 0.5);
            t.elevation_layer = ElevationLayer::SoilHigh;
            t.biome = BiomeType::SunlitPlains;
            tiles.push(t);
        }
    }

    // Create a winding canal of navigable water from (2, 2) -> (2, 12) -> (15, 12) -> (15, 20)
    let mut canal_points = Vec::new();
    for y in 2..=12 {
        canal_points.push((2, y));
    }
    for x in 2..=15 {
        canal_points.push((x, 12));
    }
    for y in 12..=20 {
        canal_points.push((15, y));
    }

    for &(cx, cy) in &canal_points {
        let idx = cy * width + cx;
        tiles[idx].elevation_layer = ElevationLayer::ShallowWaters;
        tiles[idx].biome = BiomeType::CrystalRiverlands;
    }

    let start = (2, 2);
    let goal = (15, 20);

    let path_opt = find_water_path(start, goal, &tiles, width, height);
    assert!(
        path_opt.is_some(),
        "Water pathfinder must find open canal route"
    );

    let mut path = path_opt.unwrap();
    assert!(!path.is_empty());

    // Verify all steps in path are on navigable water
    let mut current = start;
    while let Some((nx, ny)) = path.pop() {
        let idx = ny * width + nx;
        assert!(
            is_navigable_water(&tiles[idx]),
            "Every step of maritime path must be navigable water: ({}, {})",
            nx,
            ny
        );
        let dist =
            (nx as isize - current.0 as isize).abs() + (ny as isize - current.1 as isize).abs();
        assert_eq!(dist, 1, "Path steps must be orthogonal adjacent");
        current = (nx, ny);
    }
    assert_eq!(current, goal, "Path should reach the destination goal");

    // Negative test: completely landlocked goal should return None
    let landlocked_goal = (28, 28);
    let blocked_path = find_water_path(start, landlocked_goal, &tiles, width, height);
    assert!(
        blocked_path.is_none(),
        "Impassable land destination must return None"
    );
}

#[test]
fn test_automated_sea_trade_arbitrage_cycle() {
    let mut world = WorldGrid::new();

    // Create a clear straight water channel between Port A (10, 10) and Port B (10, 16)
    for y in 10..=16 {
        let tile = world.tile_at_mut(10, y);
        tile.elevation_layer = ElevationLayer::DeepOcean;
        tile.biome = BiomeType::DeepOcean;
    }

    let dock_a_id = world.shipping.register_dock("Harbor Alpha", 10, 10, 101);
    let dock_b_id = world.shipping.register_dock("Harbor Beta", 10, 16, 102);

    // Setup two regional markets
    let mut market_a = Market::new(101, "Alpha Port Market", 10, 10);
    // Port A: massive Grain surplus
    market_a.set_supply(Commodity::Grain, 100.0);
    market_a.set_demand(Commodity::Grain, 5.0);

    let mut market_b = Market::new(102, "Beta Port Market", 10, 16);
    // Port B: severe Grain famine
    market_b.set_supply(Commodity::Grain, 0.0);
    market_b.set_demand(Commodity::Grain, 80.0);
    // Port B: rich in Iron Ingots
    market_b.set_supply(Commodity::IronIngot, 50.0);

    let mut markets = vec![market_a, market_b];

    // Create a trade route from Port A to Port B carrying Grain, returning with IronIngot
    let route_id = world.shipping.create_route(
        dock_a_id,
        dock_b_id,
        Commodity::Grain,
        Some(Commodity::IronIngot),
    );

    // Build and assign CargoCog
    let ship_id = 99;
    let mut ship = ShipEntity::new(
        ship_id,
        "Sea Serpent",
        VesselKind::CargoCog,
        10,
        10,
        dock_a_id,
    );
    ship.status = ShipStatus::AtPort { dock_id: dock_a_id };
    world.shipping.add_ship(ship);
    world.shipping.assign_ship_to_route(route_id, ship_id);

    // 1. Tick 1: Ship loads grain at Port A and sets sail towards Port B
    world.tick_shipping(&mut markets);

    let ship = world.shipping.get_ship(ship_id).unwrap();
    assert!(
        ship.cargo_count() > 0,
        "Ship should have loaded grain cargo at Port A"
    );
    assert_eq!(*ship.cargo.get(&Commodity::Grain).unwrap(), 100);
    assert!(matches!(ship.status, ShipStatus::EnRoute { .. }));

    // 2. Advance voyage ticks until vessel reaches Port B
    for _ in 0..10 {
        world.tick_shipping(&mut markets);
        let s = world.shipping.get_ship(ship_id).unwrap();
        if matches!(s.status, ShipStatus::AtPort { dock_id } if dock_id == dock_b_id) {
            break;
        }
    }

    // 3. Tick while docked at Port B: unloads and sells grain, loads return cargo (IronIngots)
    world.tick_shipping(&mut markets);

    let ship = world.shipping.get_ship(ship_id).unwrap();
    assert_eq!(
        ship.cargo.get(&Commodity::Grain),
        None,
        "Grain should be unloaded at Port B"
    );
    let route = world
        .shipping
        .routes
        .iter()
        .find(|r| r.id == route_id)
        .unwrap();
    assert_eq!(
        route.total_volume_traded, 100,
        "100 units of grain should be traded at destination"
    );
    assert!(
        route.total_profit_generated > 0.0,
        "Trade route should generate revenue"
    );
    assert!(
        markets[1].get_supply(Commodity::Grain) >= 100.0,
        "Market B must receive the grain delivery"
    );
}
