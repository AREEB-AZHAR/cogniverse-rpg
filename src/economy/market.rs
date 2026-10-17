use crate::world::items::ItemId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standard tradable commodities in the civilizational market network
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Commodity {
    Grain,
    Berry,
    Bread,
    Fish,
    Wood,
    Stone,
    IronOre,
    IronIngot,
    Tools,
    LuxuryCloth,
}

impl Commodity {
    pub const ALL: [Commodity; 10] = [
        Commodity::Grain,
        Commodity::Berry,
        Commodity::Bread,
        Commodity::Fish,
        Commodity::Wood,
        Commodity::Stone,
        Commodity::IronOre,
        Commodity::IronIngot,
        Commodity::Tools,
        Commodity::LuxuryCloth,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            Commodity::Grain => "Grain Rations",
            Commodity::Berry => "Wild Berries",
            Commodity::Bread => "Baked Bread",
            Commodity::Fish => "Fresh Fish",
            Commodity::Wood => "Timber Wood",
            Commodity::Stone => "Quarried Stone",
            Commodity::IronOre => "Iron Ore",
            Commodity::IronIngot => "Refined Iron Ingot",
            Commodity::Tools => "Crafted Tools",
            Commodity::LuxuryCloth => "Luxury Woven Cloth",
        }
    }

    pub fn base_price(&self) -> f32 {
        match self {
            Commodity::Berry | Commodity::Grain => 2.0,
            Commodity::Bread | Commodity::Fish => 5.0,
            Commodity::Wood | Commodity::Stone => 3.0,
            Commodity::IronOre => 10.0,
            Commodity::IronIngot => 22.0,
            Commodity::Tools => 35.0,
            Commodity::LuxuryCloth => 25.0,
        }
    }

    pub fn as_item_id(&self) -> ItemId {
        match self {
            Commodity::Grain => ItemId::Grain,
            Commodity::Berry => ItemId::Berry,
            Commodity::Bread => ItemId::Bread,
            Commodity::Fish => ItemId::Fish,
            Commodity::Wood => ItemId::Wood,
            Commodity::Stone => ItemId::Stone,
            Commodity::IronOre => ItemId::IronOre,
            Commodity::IronIngot => ItemId::IronIngot,
            Commodity::Tools => ItemId::IronPickaxe,
            Commodity::LuxuryCloth => ItemId::WoolCloth,
        }
    }
}

/// A localized economic market node with supply/demand dynamics and price inertia
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Market {
    pub id: usize,
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub supplies: HashMap<Commodity, f32>,
    pub demands: HashMap<Commodity, f32>,
    pub prices: HashMap<Commodity, f32>,
    pub treasury_coins: f32,
}

impl Market {
    pub fn new(id: usize, name: impl Into<String>, x: usize, y: usize) -> Self {
        let mut supplies = HashMap::new();
        let mut demands = HashMap::new();
        let mut prices = HashMap::new();

        for &c in &Commodity::ALL {
            supplies.insert(c, 20.0);
            demands.insert(c, 20.0);
            prices.insert(c, c.base_price());
        }

        Self {
            id,
            name: name.into(),
            x,
            y,
            supplies,
            demands,
            prices,
            treasury_coins: 500.0,
        }
    }

    /// Calculates ideal instantaneous target price based on non-linear square root ratio
    /// P_target = P_base * sqrt((Demand + 1) / (Supply + 1)), clamped to [0.25 * P_base, 8.0 * P_base]
    pub fn calculate_target_price(&self, commodity: Commodity) -> f32 {
        let base = commodity.base_price();
        let supply = *self.supplies.get(&commodity).unwrap_or(&1.0);
        let demand = *self.demands.get(&commodity).unwrap_or(&1.0);

        let ratio = (demand + 1.0) / (supply + 1.0);
        let multiplier = ratio.sqrt();
        let raw_price = base * multiplier;

        raw_price.clamp(base * 0.25, base * 8.0)
    }

    /// Smooths current prices toward target prices via exponential moving average (inertia)
    pub fn tick_price_smoothing(&mut self, alpha: f32) {
        for &commodity in &Commodity::ALL {
            let target = self.calculate_target_price(commodity);
            let current = *self.prices.get(&commodity).unwrap_or(&target);
            let updated = current + alpha * (target - current);
            self.prices.insert(commodity, updated);
        }
    }

    pub fn get_price(&self, commodity: Commodity) -> f32 {
        *self
            .prices
            .get(&commodity)
            .unwrap_or(&commodity.base_price())
    }

    pub fn set_supply(&mut self, commodity: Commodity, amount: f32) {
        self.supplies.insert(commodity, amount.max(0.0));
    }

    pub fn set_demand(&mut self, commodity: Commodity, amount: f32) {
        self.demands.insert(commodity, amount.max(0.0));
    }

    pub fn get_supply(&self, commodity: Commodity) -> f32 {
        *self.supplies.get(&commodity).unwrap_or(&0.0)
    }

    pub fn get_demand(&self, commodity: Commodity) -> f32 {
        *self.demands.get(&commodity).unwrap_or(&0.0)
    }

    /// Purchases goods from the market: deducts supply, increases demand, deposits coins into treasury
    pub fn buy(
        &mut self,
        commodity: Commodity,
        quantity: u32,
        buyer_coins: &mut f32,
    ) -> Result<f32, &'static str> {
        if quantity == 0 {
            return Ok(0.0);
        }

        let avail_supply = self.get_supply(commodity);
        if avail_supply < quantity as f32 {
            return Err("Insufficient market supply");
        }

        let price_per_unit = self.get_price(commodity);
        let total_cost = price_per_unit * (quantity as f32);

        if *buyer_coins < total_cost {
            return Err("Buyer has insufficient coins");
        }

        // Execute transaction
        *buyer_coins -= total_cost;
        self.treasury_coins += total_cost;
        self.set_supply(commodity, avail_supply - quantity as f32);

        let current_demand = self.get_demand(commodity);
        self.set_demand(commodity, current_demand + (quantity as f32) * 0.5);

        Ok(total_cost)
    }

    /// Sells goods to the market: increases supply, deducts treasury coins, pays seller
    pub fn sell(
        &mut self,
        commodity: Commodity,
        quantity: u32,
        seller_coins: &mut f32,
    ) -> Result<f32, &'static str> {
        if quantity == 0 {
            return Ok(0.0);
        }

        let price_per_unit = self.get_price(commodity);
        let total_payout = price_per_unit * (quantity as f32);

        if self.treasury_coins < total_payout {
            return Err("Market treasury has insufficient coins to purchase goods");
        }

        // Execute transaction
        self.treasury_coins -= total_payout;
        *seller_coins += total_payout;

        let current_supply = self.get_supply(commodity);
        self.set_supply(commodity, current_supply + quantity as f32);

        let current_demand = self.get_demand(commodity);
        self.set_demand(
            commodity,
            (current_demand - (quantity as f32) * 0.3).max(0.0),
        );

        Ok(total_payout)
    }
}
