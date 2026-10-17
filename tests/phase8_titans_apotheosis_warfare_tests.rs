use cogniverse_rpg::agent::apotheosis::{
    ApotheosisEngine, DemigodAura, DemigodState, DivineDomain,
};
use cogniverse_rpg::agent::party::{CombatRole, Party, PartyMember, PartyStatus};
use cogniverse_rpg::agent::progression::{Rank, Subclass};
use cogniverse_rpg::agent::{Agent, CivilizationRace};
use cogniverse_rpg::brain::desire::AmbitionArchetype;
use cogniverse_rpg::factions::warfare::{SiegeEngineKind, WarCampaign};
use cogniverse_rpg::world::danger::DangerField;
use cogniverse_rpg::world::items::{Inventory, ItemId, RecipeId};
use cogniverse_rpg::world::shipping::{ShipEntity, VesselKind};
use cogniverse_rpg::world::tile::{BiomeType, PlacedStructure, StructureKind, Tile};
use cogniverse_rpg::world::titans::{
    validate_titan_mining, validate_titan_refining, TitanKind, TitanManager, TitanState,
};
use cogniverse_rpg::world::{ShippingNetwork, WorldGrid};

#[test]
fn test_titan_initialization_and_awakening_cataclysm() {
    let world = WorldGrid::new();

    // Verify 3 primordial world titans are properly placed
    assert_eq!(world.titans.titans.len(), 3);
    let leviathan = &world.titans.titans[0];
    assert_eq!(leviathan.kind, TitanKind::AbyssalLeviathan);
    assert_eq!(leviathan.state, TitanState::Slumbering);
    assert!(leviathan.hp >= 5000.0);

    let wyrm = &world.titans.titans[1];
    assert_eq!(wyrm.kind, TitanKind::MagmaWyrm);
    assert_eq!(wyrm.state, TitanState::Slumbering);

    let colossus = &world.titans.titans[2];
    assert_eq!(colossus.kind, TitanKind::VoidColossus);
    assert_eq!(colossus.state, TitanState::Slumbering);

    // Awaken the Leviathan
    let mut titan = leviathan.clone();
    let msg = titan.awaken();
    assert!(msg.contains("TITAN AWAKENS"));
    assert_eq!(titan.state, TitanState::Awakened);

    // Test regional cataclysm: injects danger into danger field and damages maritime shipping
    let mut danger = DangerField::new(world.width, world.height);
    let mut shipping = ShippingNetwork::new();
    shipping.ships.push(ShipEntity::new(
        1,
        "SS Solaria",
        VesselKind::Galleon,
        titan.x,
        titan.y,
        1,
    ));

    let initial_durability = shipping.ships[0].durability;
    titan.execute_cataclysm(&mut danger, &mut shipping);

    // Shipping vessel should have suffered tidal wave damage
    assert!(shipping.ships[0].durability < initial_durability);
    // Danger field should have catastrophic danger injected at the epicentre
    assert!(danger.sample_danger(titan.x, titan.y) >= 80.0);
}

#[test]
fn test_boss_area_exclusive_ores_and_high_skill_extraction() {
    let world = WorldGrid::new();

    // Check boss-exclusive ores seeded around boss areas
    let lev_x = world.width / 8;
    let lev_y = world.height / 8;
    let abyssal_tile = world.tile_at(lev_x, lev_y);
    assert!(
        abyssal_tile.abyssalite_ore_amount > 0,
        "Abyssalite ore must generate in Leviathan oceanic domain"
    );

    let wyrm_x = world.width / 2;
    let wyrm_y = world.height / 2;
    let pyre_tile = world.tile_at(wyrm_x, wyrm_y);
    assert!(
        pyre_tile.pyrestone_ore_amount > 0,
        "Pyrestone ore must generate in Magma Wyrm volcanic domain"
    );

    let void_x = (world.width * 3) / 4;
    let void_y = (world.height * 3) / 4;
    let void_tile = world.tile_at(void_x, void_y);
    assert!(
        void_tile.voidstone_ore_amount > 0,
        "Voidstone ore must generate in Void Colossus celestial domain"
    );

    // 1. Worker with basic inventory (no specialized pickaxe) -> Mining fails
    let mut inv = Inventory::new();
    inv.add(ItemId::StonePickaxe, 1);
    let result_no_tool =
        validate_titan_mining(&inv, 30, Some(Rank::SSS), Some(Subclass::RuneSmith));
    assert!(
        result_no_tool.is_err(),
        "Must reject mining without Mithril or Titan pickaxe"
    );

    // 2. Worker with Mithril Pickaxe but low skill (Level 5, Rank F) -> Mining fails
    inv.add(ItemId::MithrilPickaxe, 1);
    let result_low_skill = validate_titan_mining(&inv, 5, Some(Rank::F), None);
    assert!(
        result_low_skill.is_err(),
        "Must reject mining by low-skilled workers"
    );

    // 3. Master Miner (Level 28, Rank S, RuneSmith) with Mithril Pickaxe -> Succeeds!
    let result_master = validate_titan_mining(&inv, 28, Some(Rank::S), Some(Subclass::RuneSmith));
    assert!(result_master.is_ok());
    assert_eq!(result_master.unwrap(), 1);

    // 4. Titan Pickaxe wielder -> Yields bonus output (2)
    inv.add(ItemId::TitanPickaxe, 1);
    let result_titan_tool =
        validate_titan_mining(&inv, 30, Some(Rank::SSS), Some(Subclass::RuneSmith));
    assert_eq!(result_titan_tool.unwrap(), 2);

    // 5. Refining validation: requires Master skill (Level 25+ or S-Rank or RuneSmith)
    let refine_low = validate_titan_refining(10, Some(Rank::C), None);
    assert!(refine_low.is_err(), "Low skill cannot refine boss ores");

    let refine_master = validate_titan_refining(26, Some(Rank::SPlus), Some(Subclass::RuneSmith));
    assert!(
        refine_master.is_ok(),
        "Master metallurgist can refine boss ores"
    );

    // 6. Recipe outputs check
    assert_eq!(
        RecipeId::SmeltAbyssaliteIngot.output(),
        Some((ItemId::AbyssaliteIngot, 1))
    );
    assert_eq!(
        RecipeId::SmeltPyrestoneIngot.output(),
        Some((ItemId::PyrestoneIngot, 1))
    );
    assert_eq!(
        RecipeId::SmeltVoidstoneIngot.output(),
        Some((ItemId::VoidstoneIngot, 1))
    );
    assert_eq!(
        RecipeId::CraftTitanPickaxe.output(),
        Some((ItemId::TitanPickaxe, 1))
    );
}

#[test]
fn test_raid_combat_and_mythic_chest_loot_table() {
    let mut titan_mgr = TitanManager::spawn_initial_titans(128, 128);
    let mut rng = rand::thread_rng();

    // Create an elite Grand Raid Brigade
    let members = vec![
        PartyMember {
            agent_id: 1,
            name: "Galahad".into(),
            level: 30,
            rank: Rank::SSS,
            role: CombatRole::Tank,
            hp: 400.0,
            max_hp: 400.0,
            attack_power: 80.0,
            defense: 65.0,
        },
        PartyMember {
            agent_id: 2,
            name: "Merlina".into(),
            level: 28,
            rank: Rank::SPlus,
            role: CombatRole::Dps,
            hp: 250.0,
            max_hp: 250.0,
            attack_power: 120.0,
            defense: 25.0,
        },
        PartyMember {
            agent_id: 3,
            name: "Selene".into(),
            level: 26,
            rank: Rank::S,
            role: CombatRole::Support,
            hp: 280.0,
            max_hp: 280.0,
            attack_power: 45.0,
            defense: 40.0,
        },
    ];

    let mut raid_party = Party::new(1, "Godslayers of Solaria", 1, members, 16, 16);
    raid_party.status = PartyStatus::InBattle;
    raid_party.composite_power = 450.0;
    raid_party.synergy_multiplier = 1.35;

    let mut next_item_id = 9000;
    let mut next_stone_id = 7000;

    // Set titan's current HP to simulate the climactic final phase of the raid battle
    titan_mgr.titans[0].hp = 250.0;

    // Simulate raid rounds until Leviathan falls
    let mut chest_dropped = None;
    for _ in 0..10 {
        if let Some(chest) = titan_mgr.resolve_raid_round(
            1,
            &mut raid_party,
            &mut next_item_id,
            &mut next_stone_id,
            &mut rng,
        ) {
            chest_dropped = Some(chest);
            break;
        }
    }

    let chest = chest_dropped.expect("Titan must fall and award TitanLootChest");
    assert!(chest.gold >= 1500);
    assert!(
        !chest.raw_ores.is_empty(),
        "Boss chest must drop raw primordial ores"
    );
    assert!(
        !chest.refined_ingots.is_empty(),
        "Boss chest must drop refined ingots"
    );
    assert!(
        chest.divine_sparks >= 1,
        "Boss chest must drop Divine Spark"
    );
    assert!(
        !chest.mana_stones.is_empty(),
        "Boss chest must drop high-grade mana stones"
    );
    assert!(
        !chest.mythic_equipment.is_empty(),
        "Boss chest must drop mythic weapons/armour"
    );
}

#[test]
fn test_demigod_apotheosis_ascension_trial_and_heirloom() {
    let mut agent = Agent::new(
        100,
        "Aurelius the Radiant",
        CivilizationRace::SolarTerran,
        50,
        50,
        AmbitionArchetype::PowerTyrant,
        None,
    );

    // Initial check: Low level agent cannot ascend
    assert!(!ApotheosisEngine::can_ascend(&agent));

    // Elevate agent to Level 30, SSS-Rank, and grant a Divine Spark
    agent.progression.level = 30;
    agent.progression.rank = Some(Rank::SSS);
    agent.progression.subclass = Some(Subclass::Paladin);
    agent.inventory.add(ItemId::DivineSpark, 1);

    assert!(
        ApotheosisEngine::can_ascend(&agent),
        "Elite SSS hero with Divine Spark should satisfy Ascension prerequisites"
    );

    let initial_max_hp = agent.max_health;
    let mut demigod_state = DemigodState::default();
    let mut next_item_id = 9999;

    let heirloom_res = ApotheosisEngine::perform_ascension(
        &mut agent,
        DivineDomain::SunHerald,
        &mut demigod_state,
        &mut next_item_id,
    );

    assert!(heirloom_res.is_ok());
    let heirloom = heirloom_res.unwrap();

    // Verify apotheosis transformations:
    // 1. Divine Spark consumed
    assert_eq!(agent.inventory.count(ItemId::DivineSpark), 0);
    // 2. Health increased
    assert!(agent.max_health > initial_max_hp);
    // 3. Demigod state set
    assert!(demigod_state.is_ascended);
    assert_eq!(demigod_state.domain, Some(DivineDomain::SunHerald));
    assert!(demigod_state.aura.is_some());
    // 4. Dynasty Heirloom forged
    assert_eq!(heirloom.creator_name, "Aurelius the Radiant");
    assert!(heirloom.name.contains("Ancestral Crown"));
    assert!(heirloom.attack_bonus >= 30.0);
}

#[test]
fn test_demigod_aura_party_enhancements() {
    let aura = DemigodAura {
        domain: DivineDomain::WarBringer,
        radius: 8,
        potency: 1.25,
    };

    let members = vec![PartyMember {
        agent_id: 10,
        name: "Valerius".into(),
        level: 20,
        rank: Rank::A,
        role: CombatRole::Tank,
        hp: 200.0,
        max_hp: 200.0,
        attack_power: 50.0,
        defense: 40.0,
    }];

    let mut party = Party::new(5, "Legion of Dawn", 10, members, 20, 20);
    party.status = PartyStatus::EnRoute;
    party.composite_power = 100.0;
    party.synergy_multiplier = 1.0;

    let pre_atk = party.members[0].attack_power;
    let pre_def = party.members[0].defense;

    ApotheosisEngine::apply_aura_to_party(&aura, &mut party);

    assert!(
        party.members[0].attack_power > pre_atk,
        "WarBringer aura must increase attack"
    );
    assert!(
        party.members[0].defense > pre_def,
        "WarBringer aura must increase defense"
    );
}

#[test]
fn test_kingdom_siege_warfare_bombardment_and_territory_shift() {
    let width = 64;
    let height = 64;
    let mut campaign = WarCampaign::new(width, height);
    campaign.declare_war(
        CivilizationRace::SolarTerran,
        CivilizationRace::ChthonicLunarian,
    );

    let mut tiles = vec![Tile::new(0, 0, BiomeType::WhisperingForest, 100.0, 50.0); width * height];
    for y in 0..height {
        for x in 0..width {
            tiles[y * width + x].x = x;
            tiles[y * width + x].y = y;
        }
    }

    // Place an enemy fortification (Barracks) at (20, 20)
    let tgt_x = 20;
    let tgt_y = 20;
    tiles[tgt_y * width + tgt_x].structure = Some(PlacedStructure {
        kind: StructureKind::Barracks,
        owner_id: None,
        health: 150.0,
        max_health: 150.0,
        custom_type: None,
    });

    // Deploy a Terran Heavy Catapult within firing range at (20, 24) (distance = 4 <= range 6)
    let engine_id = campaign.build_siege_engine(
        SiegeEngineKind::Catapult,
        CivilizationRace::SolarTerran,
        20,
        24,
        tgt_x,
        tgt_y,
    );
    assert_eq!(engine_id, 1);

    // First bombardment round
    let events1 = campaign.tick(&mut tiles, width, height);
    assert!(events1.iter().any(|e| e.contains("SIEGE BOMBARDMENT")));
    assert!(tiles[tgt_y * width + tgt_x].structure.is_some());

    // Second bombardment round -> Structural collapse!
    let events2 = campaign.tick(&mut tiles, width, height);
    assert!(events2.iter().any(|e| e.contains("FORTRESS COLLAPSED")));
    assert!(
        tiles[tgt_y * width + tgt_x].structure.is_none(),
        "Fortification must be reduced to rubble"
    );
    assert_eq!(campaign.sieges_won, 1);
    assert_eq!(campaign.gold_plundered, 250);

    // Territorial sovereignty should have shifted toward the Terran conquerors
    let new_owner = campaign.territory.get_owner(tgt_x, tgt_y);
    assert_eq!(new_owner, Some(CivilizationRace::SolarTerran));
}
