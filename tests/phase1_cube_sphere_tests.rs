use cogniverse_rpg::world::cube_sphere::CellId;
use cogniverse_rpg::world::headless::HeadlessSimulator;
use cogniverse_rpg::world::planet::CubeSphereWorld;
use cogniverse_rpg::world::saves::{SaveError, SaveSystem, WorldSaveHeader, CURRENT_SAVE_VERSION};
use cogniverse_rpg::world::tile::BiomeType;
use cogniverse_rpg::world::views::{GlobeView, RegionView};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

#[test]
fn test_cube_sphere_world_generation_and_biomes() {
    let n = 32u16;
    let seed = 424242;
    let world = CubeSphereWorld::new(n, seed);

    assert_eq!(world.face_size, n);
    assert_eq!(world.faces.len(), 6, "Must generate exactly 6 cube faces");

    let mut biomes_found = HashSet::new();
    let mut min_temp = f32::MAX;
    let mut max_temp = f32::MIN;

    for face in 0..6u8 {
        assert_eq!(
            world.faces[face as usize].len(),
            (n as usize) * (n as usize),
            "Each face must have N x N tiles"
        );

        for y in 0..n {
            for x in 0..n {
                let cell = CellId::new(face, x, y);
                let tile = world.get_tile(cell);
                biomes_found.insert(tile.biome);
                min_temp = min_temp.min(tile.temperature);
                max_temp = max_temp.max(tile.temperature);

                // Elevation must be normalized [0.0, 1.0]
                assert!(
                    (0.0..=1.0).contains(&tile.elevation),
                    "Elevation must be bounded within [0, 1], got {}",
                    tile.elevation
                );
            }
        }
    }

    // Verify presence of diverse biomes
    assert!(
        biomes_found.contains(&BiomeType::DeepOcean)
            || biomes_found.contains(&BiomeType::CloseOcean),
        "World must generate ocean bodies"
    );
    assert!(
        biomes_found.contains(&BiomeType::IroncragMountains),
        "World must generate mountains"
    );
    assert!(
        biomes_found.contains(&BiomeType::PermafrostTundra),
        "World must generate polar tundra"
    );
    assert!(
        biomes_found.contains(&BiomeType::WhisperingForest)
            || biomes_found.contains(&BiomeType::Jungle),
        "World must generate forests or jungles"
    );
    assert!(
        biomes_found.contains(&BiomeType::SunlitPlains)
            || biomes_found.contains(&BiomeType::Savanna),
        "World must generate plains or savannas"
    );

    // Verify polar vs equatorial temperature range
    assert!(
        min_temp < 5.0,
        "Poles must experience freezing temperatures, min_temp was {}",
        min_temp
    );
    assert!(
        max_temp > 20.0,
        "Equator must experience warm temperatures, max_temp was {}",
        max_temp
    );

    // Verify Points of Interest placement
    assert!(
        !world.points_of_interest.is_empty(),
        "World must generate points of interest"
    );
}

#[test]
fn test_globe_view_and_region_view() {
    let world = CubeSphereWorld::new(16, 12345);

    // 1. Globe View: Render 3D orthographic sphere
    let radius = 12;
    let globe = GlobeView::render_orthographic(&world, radius, 0.4, 0.2);

    let dim = radius * 2 + 1;
    assert_eq!(globe.len(), dim);
    assert_eq!(globe[0].len(), dim);

    let center_pixel = &globe[radius][radius];
    assert!(
        center_pixel.cell.is_some(),
        "Center pixel of the globe must hit the planet surface"
    );
    assert_ne!(
        center_pixel.render_char, ' ',
        "Center pixel must render a surface character"
    );

    // Top-left corner outside radius must be space
    let corner_pixel = &globe[0][0];
    assert!(
        corner_pixel.cell.is_none(),
        "Corner pixel outside sphere disc must be empty space"
    );
    assert_eq!(corner_pixel.render_char, ' ');

    // 2. Region View: Extract 2D window centered on a cell
    let center = CellId::new(0, 8, 8);
    let window = RegionView::extract_window(&world, center, 4, 4);

    assert_eq!(window.len(), 9);
    assert_eq!(window[0].len(), 9);
    assert_eq!(window[4][4].x, 8);
    assert_eq!(window[4][4].y, 8);
}

#[test]
fn test_atomic_binary_save_and_load() {
    let world = CubeSphereWorld::new(16, 777);
    let temp_dir = std::env::temp_dir().join("cogniverse_test_saves");
    let save_path = temp_dir.join("test_planet.cgnw");

    // 1. Atomic save
    SaveSystem::save_atomic(&world, &save_path).expect("Atomic save must succeed");

    assert!(save_path.exists(), "Save file must exist on disk");
    let tmp_path = save_path.with_extension("tmp");
    assert!(
        !tmp_path.exists(),
        "Temporary .tmp file must be cleaned up after atomic rename"
    );

    // 2. Load save and verify fidelity
    let loaded_world = SaveSystem::load_save(&save_path).expect("Loading valid save must succeed");

    assert_eq!(loaded_world.face_size, world.face_size);
    assert_eq!(loaded_world.seed, world.seed);
    assert_eq!(loaded_world.world_tick, world.world_tick);
    assert_eq!(
        loaded_world.points_of_interest.len(),
        world.points_of_interest.len()
    );

    // Spot-check tiles
    for face in 0..6u8 {
        for coord in [0, 8, 15] {
            let cell = CellId::new(face, coord, coord);
            let orig = world.get_tile(cell);
            let loaded = loaded_world.get_tile(cell);
            assert_eq!(orig.biome, loaded.biome);
            assert_eq!(orig.elevation, loaded.elevation);
        }
    }

    // 3. Test incompatible header version rejection
    let corrupt_path = temp_dir.join("corrupt_version.cgnw");
    let bad_header = WorldSaveHeader {
        magic: *b"CGNW",
        version: CURRENT_SAVE_VERSION + 999, // Incompatible version
        timestamp: 0,
    };
    let mut bad_file = File::create(&corrupt_path).unwrap();
    let header_bytes = bincode::serialize(&bad_header).unwrap();
    let header_len = header_bytes.len() as u32;
    bad_file.write_all(&header_len.to_le_bytes()).unwrap();
    bad_file.write_all(&header_bytes).unwrap();
    drop(bad_file);

    let err = SaveSystem::load_save(&corrupt_path)
        .expect_err("Loading save with incompatible version must fail");
    assert!(
        matches!(err, SaveError::IncompatibleVersion { .. }),
        "Expected IncompatibleVersion error, got {:?}",
        err
    );

    // Cleanup test artifacts
    let _ = std::fs::remove_file(save_path);
    let _ = std::fs::remove_file(corrupt_path);
}

#[test]
fn test_100_year_headless_simulation_soak_test() {
    // 100 simulated years on cube-sphere planet (100,000 ticks)
    let years = 100;
    let seed = 1234567;
    let face_size = 16; // 6 faces of 16x16 = 1,536 cells

    let result = HeadlessSimulator::run(years, seed, face_size);

    assert_eq!(
        result.simulated_years, 100,
        "Simulation must survive all 100 years"
    );
    assert_eq!(
        result.total_ticks, 100_000,
        "Simulation must complete exactly 100,000 ticks"
    );
    assert!(
        result.ticks_per_second > 1000.0,
        "Headless simulation throughput must exceed 1,000 TPS, achieved {:.0} TPS",
        result.ticks_per_second
    );
    assert!(
        result.final_poi_count > 0,
        "Planet POIs must persist throughout the 100-year epoch"
    );
}
