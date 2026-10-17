use cogniverse_rpg::agent::progression::Rank;
use cogniverse_rpg::economy::loot::{
    EquipmentItem, EquipmentSlot, ItemRarity, ManaStone, ManaStoneGrade,
};
use cogniverse_rpg::server::SimulationEngine;
use cogniverse_rpg::world::items::{Inventory, ItemId};

#[test]
fn test_equipment_item_blessing_promotion_and_affixes() {
    let mut equip = EquipmentItem {
        id: 1,
        name: "Steel Broadsword".to_string(),
        rarity: ItemRarity::Rare,
        slot: EquipmentSlot::Weapon,
        base_attack: 30.0,
        base_defense: 0.0,
        affixes: Vec::new(),
        value: 100,
        required_rank: Rank::C,
        durability: 35,
        max_durability: 100,
    };

    assert!(!equip.is_blessed());
    let initial_attack = equip.total_attack();

    // Bestow divine blessing
    equip.bless();

    assert!(equip.is_blessed());
    assert_eq!(equip.rarity, ItemRarity::Epic);
    assert!(equip.name.contains("✦ Sanctified Steel Broadsword"));
    assert!(equip.total_attack() > initial_attack + 40.0);
    assert_eq!(equip.durability, equip.max_durability);
    assert!(equip.max_durability >= 200);
    assert!(equip
        .affixes
        .iter()
        .any(|a| a.name.contains("God-Forged Celestial Radiance")));
    assert!(equip.value > 300);
}

#[test]
fn test_mana_stone_blessing_purity_and_overcharge() {
    let mut stone = ManaStone::new(10, ManaStoneGrade::Regular, 0.70);
    assert_eq!(stone.grade, ManaStoneGrade::Regular);
    assert!(!stone.is_blessed());

    // Bestow divine blessing
    stone.bless();

    assert!(stone.is_blessed());
    assert_eq!(stone.grade, ManaStoneGrade::Flawless);
    assert!((stone.purity - 1.0).abs() < 1e-4);
    assert!(stone.stored_mana >= 400.0 * 1.24);
    assert_eq!(stone.stored_mana, stone.max_mana);
}

#[test]
fn test_inventory_bless_basic_item_and_conversion() {
    let mut inv = Inventory::new();
    inv.add(ItemId::IronSword, 2);
    assert_eq!(inv.count(ItemId::IronSword), 2);
    assert!(inv.equipment.is_empty());

    let mut next_id = 500;
    let res = inv.bless_item("Iron Sword", &mut next_id);

    assert!(res.is_some());
    let title = res.unwrap();
    assert!(title.contains("✦ God-Forged Iron Sword"));
    assert_eq!(inv.count(ItemId::IronSword), 1);
    assert_eq!(inv.equipment.len(), 1);

    let created_relic = &inv.equipment[0];
    assert!(created_relic.is_blessed());
    assert_eq!(created_relic.rarity, ItemRarity::Mythic);
    assert!(created_relic.total_attack() >= 70.0);
}

#[test]
fn test_simulation_engine_bless_agent_item_miracle_and_memory() {
    let mut engine = SimulationEngine::new();
    assert!(!engine.agents.is_empty());

    let target_agent_id = engine.agents[0].id;
    // Give the agent an Iron Sword
    engine.agents[0].inventory.add(ItemId::IronSword, 1);
    let initial_piety = engine.agents[0].brain.piety;

    // Trigger divine blessing
    let result = engine.bless_agent_item(target_agent_id, "Iron Sword");
    assert!(result.is_some());
    let blessed_title = result.unwrap();
    assert!(blessed_title.contains("✦"));

    // Verify agent vitals and cognitive reaction
    let agent = &engine.agents[0];
    assert!(agent.brain.piety >= initial_piety);
    assert!((agent.brain.neuromod.dopamine - 1.0).abs() < 1e-4);

    // Verify episodic memory logged
    let has_memory = agent.brain.memory.episodic_stream.iter().any(|m| {
        m.description.contains("divine miracle") && m.description.contains(&blessed_title)
    });
    assert!(
        has_memory,
        "Agent must record holy memory of the item blessing"
    );

    // Verify world event log entry
    let has_event = engine
        .event_log
        .iter()
        .any(|e| e.contains("DIVINE MIRACLE") && e.contains(&blessed_title));
    assert!(has_event, "Engine event log must broadcast the miracle");
}
