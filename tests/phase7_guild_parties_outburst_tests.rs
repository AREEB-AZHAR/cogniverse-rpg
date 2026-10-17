use cogniverse_rpg::agent::party::{CombatRole, PartyFormationEngine, PartyStatus};
use cogniverse_rpg::agent::progression::{
    Attributes, CharacterClass, HeroicProgression, Rank, Subclass,
};
use cogniverse_rpg::agent::{Agent, CivilizationRace};
use cogniverse_rpg::brain::desire::AmbitionArchetype;
use cogniverse_rpg::factions::guild::QuestKind;
use cogniverse_rpg::factions::{CivilizationsCoordinator, DiplomaticStatus};
use cogniverse_rpg::world::dungeon::{generate_dungeon, DungeonState};
use cogniverse_rpg::world::WorldGrid;

fn create_test_hero(
    id: usize,
    name: &str,
    class: CharacterClass,
    subclass: Option<Subclass>,
    rank: Rank,
) -> Agent {
    let mut hero = Agent::new(
        id,
        name,
        CivilizationRace::SolarTerran,
        10,
        10,
        AmbitionArchetype::PowerTyrant,
        None,
    );
    hero.progression = HeroicProgression {
        rank: Some(rank),
        level: 25,
        current_xp: 5000,
        xp_for_next_level: 6000,
        class: Some(class),
        subclass,
        attributes: Attributes {
            strength: 20.0,
            vitality: 20.0,
            intelligence: 15.0,
            agility: 15.0,
            wisdom: 15.0,
            charisma: 15.0,
            luck: 10.0,
        },
        blessed_at_tick: Some(10),
    };
    hero.health = 150.0;
    hero.max_health = 150.0;
    hero
}

#[test]
fn test_mana_swell_and_outburst_rupture() {
    let mut grid = WorldGrid::new();
    let d_id = grid.dungeons[0].id;
    let d_x = grid.dungeons[0].x;
    let d_y = grid.dungeons[0].y;

    // Simulate subterranean mana accumulation
    grid.dungeons[0].mana_swell = 99.5;
    let broke = grid.dungeons[0].tick_mana_swell(1.0);
    assert!(broke, "Dungeon should break when mana swell >= 100.0");
    assert_eq!(grid.dungeons[0].state, DungeonState::Outburst);

    // Tick outburst system to spawn surface hordes and inject catastrophic hazard
    let settlements = vec![(10, 10, 100)];
    let mut civil = CivilizationsCoordinator::new();
    let events = grid.tick_outbursts_and_parties(&mut civil, &settlements, 100);

    assert!(!events.is_empty(), "Should generate outburst events");
    assert_eq!(grid.outburst_system.active_hordes.len(), 1);
    let horde = &grid.outburst_system.active_hordes[0];
    assert_eq!(horde.dungeon_id, d_id);
    assert!(horde.is_rage_mode, "Horde must be in rage mode");
    assert!(
        horde.mobs.len() >= 3,
        "Horde must have at least 3 surface mobs"
    );

    // Verify catastrophic hazard injected in danger field around dungeon
    let hazard = grid.danger_field.get_hazard(d_x, d_y);
    assert!(
        hazard >= 80.0,
        "Dungeon entrance hazard should spike to >= 80, got {}",
        hazard
    );
}

#[test]
fn test_emergency_quest_generation_and_bounty() {
    let mut civil = CivilizationsCoordinator::new();
    let dungeon = generate_dungeon(101, "Abyssal Rift", 25, 25, Rank::A, 42);

    let initial_treasury = civil.guild.treasury_gold;
    let quest_id = civil.guild.post_emergency_outburst_quest(&dungeon, 50);

    let quest = civil
        .guild
        .quests
        .iter()
        .find(|q| q.id == quest_id)
        .expect("Quest should exist");
    assert_eq!(quest.kind, QuestKind::OutbreakContainment);
    assert_eq!(quest.target_dungeon_id, Some(101));
    assert_eq!(quest.recommended_rank, Rank::A);
    assert!(
        quest.bounty_gold > 0,
        "Bounty gold must be awarded from treasury"
    );
    assert!(
        civil.guild.treasury_gold < initial_treasury,
        "Guild treasury should deduct bounty"
    );
}

#[test]
fn test_trinity_synergy_and_flexible_raid_formation() {
    let tank = create_test_hero(
        1,
        "Gareth",
        CharacterClass::Warrior,
        Some(Subclass::Paladin),
        Rank::B,
    );
    let dps1 = create_test_hero(
        2,
        "Lyra",
        CharacterClass::Mage,
        Some(Subclass::Elementalist),
        Rank::B,
    );
    let dps2 = create_test_hero(
        3,
        "Vance",
        CharacterClass::Archer,
        Some(Subclass::Sniper),
        Rank::B,
    );
    let healer = create_test_hero(
        4,
        "Elowen",
        CharacterClass::Cleric,
        Some(Subclass::HighPriest),
        Rank::B,
    );

    let squad = PartyFormationEngine::form_party_for_quest(
        1,
        Rank::B,
        &[tank.clone(), dps1.clone(), dps2.clone(), healer.clone()],
        10,
        10,
    )
    .expect("Should form standard 4-member party");

    assert_eq!(squad.members.len(), 4);
    assert!(!squad.is_raid_party, "Rank B should form a standard party");
    // Verify role allocation
    assert!(squad.members.iter().any(|m| m.role == CombatRole::Tank));
    assert!(squad.members.iter().any(|m| m.role == CombatRole::Dps));
    assert!(squad.members.iter().any(|m| m.role == CombatRole::Support));
    // Verify Trinity Synergy bonus
    assert!(
        squad.synergy_multiplier >= 1.35,
        "Full Trinity should grant >= 1.35x synergy, got {}",
        squad.synergy_multiplier
    );

    // Test Flexible Raid Party scaling for Rank S (up to 8 members)
    let mut raid_candidates = Vec::new();
    for i in 0..10 {
        let (class, subclass) = match i % 3 {
            0 => (CharacterClass::Warrior, Some(Subclass::Paladin)),
            1 => (CharacterClass::Mage, Some(Subclass::Arcanist)),
            _ => (CharacterClass::Cleric, Some(Subclass::HighPriest)),
        };
        raid_candidates.push(create_test_hero(
            i + 10,
            &format!("Hero_{}", i),
            class,
            subclass,
            Rank::S,
        ));
    }

    let raid = PartyFormationEngine::form_party_for_quest(2, Rank::S, &raid_candidates, 10, 10)
        .expect("Should form grand raid brigade");

    assert_eq!(
        raid.members.len(),
        8,
        "Rank S should assemble full 8-member raid party"
    );
    assert!(raid.is_raid_party, "Should be designated as raid party");
    assert!(
        raid.composite_power > squad.composite_power * 2.0,
        "Raid party composite power should be massive"
    );
}

#[test]
fn test_outburst_horde_rage_swarm_and_subjugation() {
    let mut grid = WorldGrid::new();
    let mut civil = CivilizationsCoordinator::new();

    // Trigger outburst
    grid.dungeons[0].mana_swell = 100.0;
    grid.dungeons[0].state = DungeonState::Outburst;
    let d_x = grid.dungeons[0].x;
    let d_y = grid.dungeons[0].y;

    let target_settlement = (d_x + 5, d_y + 5, 200);
    grid.tick_outbursts_and_parties(&mut civil, &[target_settlement], 1);

    assert_eq!(grid.outburst_system.active_hordes.len(), 1);
    let horde = &mut grid.outburst_system.active_hordes[0];
    assert!(horde.is_rage_mode);

    // Form strike party at the location
    let tank = create_test_hero(
        1,
        "Aegis",
        CharacterClass::Warrior,
        Some(Subclass::Paladin),
        Rank::S,
    );
    let dps = create_test_hero(
        2,
        "Ignis",
        CharacterClass::Mage,
        Some(Subclass::Elementalist),
        Rank::S,
    );
    let sup = create_test_hero(
        3,
        "Lumina",
        CharacterClass::Cleric,
        Some(Subclass::HighPriest),
        Rank::S,
    );
    let mut party =
        PartyFormationEngine::form_party_for_quest(1, Rank::S, &[tank, dps, sup], d_x, d_y)
            .unwrap();
    party.active_quest_id = Some(1);
    grid.parties.push(party);

    // Advance ticks so the party engages the horde in battle until eradication
    for tick in 2..100 {
        if grid.parties[0].status == PartyStatus::Resting {
            break;
        }
        grid.tick_outbursts_and_parties(&mut civil, &[target_settlement], tick);
    }

    assert!(
        grid.outburst_system.active_hordes[0].is_fully_defeated(),
        "Party should defeat the outbreak horde"
    );
    assert_eq!(
        grid.dungeons[0].state,
        DungeonState::Cleared,
        "Dungeon should resolve to Cleared after outburst"
    );
    assert_eq!(grid.parties[0].status, PartyStatus::Resting);
}

#[test]
fn test_grand_coalition_treaty_and_victory() {
    let mut civil = CivilizationsCoordinator::new();
    assert_eq!(civil.diplomacy, DiplomaticStatus::PeacefulCoexistence);

    // Catastrophic S-rank outburst occurs
    let treaty_event = civil
        .evaluate_outburst_coalition(Rank::S)
        .expect("Should form Grand Coalition");
    assert!(treaty_event.contains("GRAND ANTI-CALAMITY COALITION"));
    assert_eq!(civil.diplomacy, DiplomaticStatus::GrandCoalition);

    // Victory resolves
    let victory_event = civil
        .resolve_coalition_victory(Rank::S)
        .expect("Should award coalition victory");
    assert!(victory_event.contains("COALITION TRIUMPH"));
    assert!(civil.terran_culture.knowledge_points >= 100);
    assert!(civil.lunar_culture.knowledge_points >= 100);
    assert_eq!(civil.trade_volume, 50);
}
