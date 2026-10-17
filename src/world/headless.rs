use super::planet::CubeSphereWorld;
use std::time::Instant;

pub const TICKS_PER_YEAR: u64 = 1000;

#[derive(Clone, Debug)]
pub struct HeadlessSimResult {
    pub total_ticks: u64,
    pub simulated_years: u32,
    pub elapsed_millis: u128,
    pub ticks_per_second: f64,
    pub final_poi_count: usize,
}

pub struct HeadlessSimulator;

impl HeadlessSimulator {
    /// Runs a headless soak simulation for the specified number of years and seed without panics
    pub fn run(years: u32, seed: u64, face_size: u16) -> HeadlessSimResult {
        let start = Instant::now();
        let total_ticks = years as u64 * TICKS_PER_YEAR;

        println!("========================================================");
        println!("       COGNIVERSE HEADLESS SIMULATION SOAK TEST         ");
        println!(
            "  Target: {} simulated years ({} ticks)                ",
            years, total_ticks
        );
        println!(
            "  Grid: 6 faces of {}x{} cells (Total: {} cells)       ",
            face_size,
            face_size,
            (face_size as usize).pow(2) * 6
        );
        println!(
            "  Seed: {}                                              ",
            seed
        );
        println!("========================================================");

        let mut world = CubeSphereWorld::new(face_size, seed);
        let poi_count = world.points_of_interest.len();

        for tick in 1..=total_ticks {
            world.tick();

            // Progress heartbeat every 20 simulated years
            if tick % (20 * TICKS_PER_YEAR) == 0 {
                let current_year = tick / TICKS_PER_YEAR;
                let elapsed_secs = start.elapsed().as_secs_f64();
                let current_tps = tick as f64 / elapsed_secs.max(0.001);
                println!(
                    "[*] Progress: Year {} / {} (Tick {}) | Running at {:.0} TPS | POIs: {}",
                    current_year, years, tick, current_tps, poi_count
                );
            }
        }

        let elapsed = start.elapsed();
        let elapsed_millis = elapsed.as_millis();
        let tps = if elapsed.as_secs_f64() > 0.0 {
            total_ticks as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        println!("========================================================");
        println!("   HEADLESS SIMULATION COMPLETE: ALL YEARS SURVIVED!    ");
        println!(
            "  Total Ticks: {} in {:.2?} ({:.0} TPS)                ",
            total_ticks, elapsed, tps
        );
        println!("========================================================");

        HeadlessSimResult {
            total_ticks,
            simulated_years: years,
            elapsed_millis,
            ticks_per_second: tps,
            final_poi_count: world.points_of_interest.len(),
        }
    }
}
