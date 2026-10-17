use cogniverse_rpg::agent::progression::{Attributes, Rank};
use cogniverse_rpg::economy::loot::{
    EquipmentItem, EquipmentSlot, ItemRarity, LootTable, ManaStoneGrade,
};
use cogniverse_rpg::world::dungeon::{generate_dungeon, DungeonState, RoomKind};
use cogniverse_rpg::world::encounter::{
    DungeonExpedition, EncounterSimulator, ExpeditionMember, ExpeditionState,
};
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn test_procedural_dungeon_generation_across_ranks() {
    let d_f = generate_dungeon(1, "Novice Cave", 10, 10, Rank::F, 12345);
    assert_eq!(d_f.floors.len(), 1);
    assert_eq!(d_f.floors[0].rooms[0].kind, RoomKind::EntryHall);
    let last_room_f = &d_f.floors[0].rooms.last().unwrap();
    assert_eq!(last_room_f.kind, RoomKind::BossLair);
    assert!(last_room_f.monsters.iter().any(|m| m.is_boss));

    let d_d = generate_dungeon(2, "Veteran Crypt", 20, 20, Rank::D, 12345);
    assert_eq!(d_d.floors.len(), 2);

    let d_b = generate_dungeon(3, "Champion Keep", 30, 30, Rank::B, 12345);
    assert_eq!(d_b.floors.len(), 4);

    let d_a = generate_dungeon(4, "Master Spire", 40, 40, Rank::A, 12345);
    assert_eq!(d_a.floors.len(), 5);

    let d_s = generate_dungeon(5, "Heroic Nexus", 50, 50, Rank::S, 12345);
    assert_eq!(d_s.floors.len(), 6);

    let d_sss = generate_dungeon(6, "Void Transcendence", 60, 60, Rank::SSS, 12345);
    assert_eq!(d_sss.floors.len(), 8);

    // Verify boss power scaling monotonically from F to SSS
    let boss_f = d_f.floors[0]
        .rooms
        .last()
        .unwrap()
        .monsters
        .iter()
        .find(|m| m.is_boss)
        .unwrap();
    let boss_sss = d_sss
        .floors
        .last()
        .unwrap()
        .rooms
        .last()
        .unwrap()
        .monsters
        .iter()
        .find(|m| m.is_boss)
        .unwrap();

    assert!(
        boss_sss.hp > boss_f.hp * 4.0,
        "SSS boss HP ({}) must far exceed F boss HP ({})",
        boss_sss.hp,
        boss_f.hp
    );
    assert!(
        boss_sss.attack_power > boss_f.attack_power * 4.0,
        "SSS boss attack ({}) must far exceed F boss attack ({})",
        boss_sss.attack_power,
        boss_f.attack_power
    );
}

#[test]
fn test_loot_table_equipment_and_affix_rolls() {
    let mut rng = StdRng::seed_from_u64(98765);

    // Common/Low rank rolls produce mostly common/uncommon
    let mut f_rarities = Vec::new();
    for _ in 0..100 {
        f_rarities.push(LootTable::roll_rarity(Rank::F, 0.0, &mut rng));
    }
    let f_common_count = f_rarities
        .iter()
        .filter(|&&r| r == ItemRarity::Common)
        .count();
    assert!(
        f_common_count > 50,
        "Rank F should drop mostly common gear: got {}/100",
        f_common_count
    );

    // SSS rank rolls with high luck should produce epic/legendary/mythic
    let mut sss_rarities = Vec::new();
    for _ in 0..100 {
        sss_rarities.push(LootTable::roll_rarity(Rank::SSS, 50.0, &mut rng));
    }
    let high_tier_count = sss_rarities
        .iter()
        .filter(|&&r| {
            r == ItemRarity::Epic || r == ItemRarity::Legendary || r == ItemRarity::Mythic
        })
        .count();
    assert!(
        high_tier_count > 60,
        "Rank SSS with high luck should drop mostly high-tier gear: got {}/100",
        high_tier_count
    );

    // Test Equipment item affix limits and attribute bonus application
    let next_id = 1;
    let epic_item =
        LootTable::generate_equipment(next_id, Rank::A, Some(ItemRarity::Epic), 25.0, &mut rng);
    assert_eq!(epic_item.rarity, ItemRarity::Epic);
    assert!(
        epic_item.affixes.len() <= ItemRarity::Epic.max_affixes(),
        "Affixes should not exceed max limit"
    );

    let mut attrs = Attributes::default();
    let initial_str = attrs.strength;
    epic_item.apply_to_attributes(&mut attrs);
    // Attributes should be modified if any attribute affixes were rolled
    let total_stat_delta = (attrs.strength - initial_str)
        + attrs.agility
        + attrs.vitality
        + attrs.intelligence
        + attrs.wisdom
        + attrs.charisma
        + attrs.luck
        - 60.0;
    assert!(
        total_stat_delta >= 0.0,
        "Affixes must apply non-negative stat bonuses"
    );
}

#[test]
fn test_mana_stone_harvesting_and_energy_capacity() {
    let mut rng = StdRng::seed_from_u64(445566);
    let mut stone_id = 1;

    // Harvest low rank stone
    let stone_f = LootTable::roll_mana_stone(Rank::F, &mut rng, &mut stone_id);
    assert_eq!(stone_f.grade, ManaStoneGrade::Flawed);
    assert!(stone_f.max_mana <= 50.0);

    // Harvest transcendent rank stone
    let stone_sss = LootTable::roll_mana_stone(Rank::SSS, &mut rng, &mut stone_id);
    assert!(
        stone_sss.grade == ManaStoneGrade::Astral || stone_sss.grade == ManaStoneGrade::Radiant
    );
    assert!(stone_sss.max_mana >= 600.0);

    // Test discharge, recharge, and monetary valuation
    let mut test_stone = stone_sss.clone();
    let initial_val = test_stone.monetary_value();
    let drawn = test_stone.discharge(500.0);
    assert!((drawn - 500.0).abs() < 1e-4);
    assert!(test_stone.stored_mana < test_stone.max_mana);

    let discharged_val = test_stone.monetary_value();
    assert!(
        discharged_val < initial_val,
        "Depleted mana stone ({}) must be worth less than full charge ({})",
        discharged_val,
        initial_val
    );

    test_stone.recharge(500.0);
    assert!((test_stone.stored_mana - test_stone.max_mana).abs() < 1e-4);
    assert_eq!(test_stone.monetary_value(), initial_val);
}

#[test]
fn test_off_screen_encounter_combat_resolution() {
    let mut rng = StdRng::seed_from_u64(112233);
    let mut next_item_id = 100;
    let mut next_stone_id = 200;

    // 1. Victorious expedition: Veteran B-rank party vs F-rank Novice Dungeon
    let mut d_easy = generate_dungeon(10, "Goblin Hollow", 5, 5, Rank::F, 777);

    let hero_attrs = Attributes {
        strength: 35.0,
        agility: 25.0,
        vitality: 30.0,
        intelligence: 15.0,
        wisdom: 20.0,
        charisma: 15.0,
        luck: 25.0,
    };
    let leader = ExpeditionMember::new(1, "Commander Vane", Rank::B, 25, hero_attrs);
    let companion = ExpeditionMember::new(2, "Ranger Lyra", Rank::B, 24, hero_attrs);

    let mut party = vec![leader, companion];
    // Equip leader with a strong weapon
    party[0].equipment.push(EquipmentItem {
        id: 1,
        name: "Steel Greataxe".to_string(),
        rarity: ItemRarity::Rare,
        slot: EquipmentSlot::Weapon,
        base_attack: 45.0,
        base_defense: 0.0,
        affixes: Vec::new(),
        value: 120,
        required_rank: Rank::B,
        durability: 100,
        max_durability: 100,
    });

    let mut expedition = DungeonExpedition::new(1, d_easy.id, party);

    let mut steps = 0;
    let max_steps = 100;
    while steps < max_steps
        && !matches!(
            expedition.state,
            ExpeditionState::Victorious | ExpeditionState::WipedOut | ExpeditionState::Retreated
        )
    {
        EncounterSimulator::step(
            &mut expedition,
            &mut d_easy,
            &mut rng,
            &mut next_item_id,
            &mut next_stone_id,
        );
        steps += 1;
    }

    assert_eq!(
        expedition.state,
        ExpeditionState::Victorious,
        "High rank party should conquer novice dungeon"
    );
    assert_eq!(d_easy.state, DungeonState::Cleared);
    assert!(expedition.loot.gold > 0, "Expedition should earn gold");
    assert!(
        expedition.exp_earned > 0,
        "Expedition should earn combat EXP"
    );
    assert!(
        !expedition.combat_log.is_empty(),
        "Combat log should record events"
    );

    // 2. High hazard expedition: 1 fragile novice commoner vs S-Rank Dungeon
    let mut d_lethal = generate_dungeon(20, "Dragon Abyss", 50, 50, Rank::S, 888);
    let weak_attrs = Attributes {
        strength: 5.0,
        agility: 5.0,
        vitality: 5.0,
        intelligence: 5.0,
        wisdom: 5.0,
        charisma: 5.0,
        luck: 5.0,
    };
    let fragile_novice = ExpeditionMember::new(3, "Unlucky Peasant", Rank::F, 1, weak_attrs);
    let mut doomed_expedition = DungeonExpedition::new(2, d_lethal.id, vec![fragile_novice]);

    let mut doomed_steps = 0;
    while doomed_steps < 50
        && !matches!(
            doomed_expedition.state,
            ExpeditionState::WipedOut | ExpeditionState::Retreated
        )
    {
        EncounterSimulator::step(
            &mut doomed_expedition,
            &mut d_lethal,
            &mut rng,
            &mut next_item_id,
            &mut next_stone_id,
        );
        doomed_steps += 1;
    }

    assert!(
        matches!(
            doomed_expedition.state,
            ExpeditionState::WipedOut | ExpeditionState::Retreated
        ),
        "Fragile commoner must be wiped out or forced to retreat in S-rank dungeon"
    );
}
