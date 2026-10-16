use super::super::world::items::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuneEffect {
    SolarIgnition,
    PurifyingAura,
    RaiseStoneWall,
    ShadowVeil,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledRune {
    pub rune_item: ItemId,
    pub effect: RuneEffect,
    pub mana_cost: f32,
    pub potency: f32,
    pub description: String,
}

impl CompiledRune {
    pub fn from_item(item: ItemId) -> Option<Self> {
        match item {
            ItemId::RuneSolarIgnition => Some(Self {
                rune_item: item,
                effect: RuneEffect::SolarIgnition,
                mana_cost: 15.0,
                potency: 25.0,
                description: "Ignites a crackling sacred flame, providing immediate heat and clearing pests.".into(),
            }),
            ItemId::RunePurifyingAura => Some(Self {
                rune_item: item,
                effect: RuneEffect::PurifyingAura,
                mana_cost: 20.0,
                potency: 40.0,
                description: "Infuses life energy, healing physical wounds and calming stressed nerves.".into(),
            }),
            ItemId::RuneStoneWall => Some(Self {
                rune_item: item,
                effect: RuneEffect::RaiseStoneWall,
                mana_cost: 25.0,
                potency: 100.0,
                description: "Manipulates binary leyline harmonics to raise an impenetrable barrier of stone.".into(),
            }),
            ItemId::RuneShadowVeil => Some(Self {
                rune_item: item,
                effect: RuneEffect::ShadowVeil,
                mana_cost: 18.0,
                potency: 0.8,
                description: "Envelops the caster in a cloak of dark resonance, confounding predators and foes.".into(),
            }),
            _ => None,
        }
    }
}
