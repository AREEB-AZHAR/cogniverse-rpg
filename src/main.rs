#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use parking_lot::RwLock;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

use cogniverse_rpg::server::create_router;
use cogniverse_rpg::server::state::SimulationEngine;
use cogniverse_rpg::world::headless::HeadlessSimulator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut seed: u64 = 42;
    let mut headless_years: Option<u32> = None;
    let mut face_size: u16 = 64;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" if i + 1 < args.len() => {
                seed = args[i + 1].parse().unwrap_or(42);
                i += 1;
            }
            "--headless-years" if i + 1 < args.len() => {
                headless_years = args[i + 1].parse().ok();
                i += 1;
            }
            "--face-size" if i + 1 < args.len() => {
                face_size = args[i + 1].parse().unwrap_or(64);
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }

    if let Some(years) = headless_years {
        HeadlessSimulator::run(years, seed, face_size);
        return Ok(());
    }

    println!("========================================================");
    println!("     GENESIS COGNIVERSE: AUTONOMOUS AI RPG SIMULATION   ");
    println!("  CPU-First Darwin-Gödel AST & Dual Pantheon Engine     ");
    println!("========================================================");

    let sim_engine = Arc::new(RwLock::new(SimulationEngine::new()));

    // Spawn background simulation loop
    let sim_ticker = sim_engine.clone();
    tokio::spawn(async move {
        loop {
            let (speed, single_step) = {
                let mut engine = sim_ticker.write();
                let spd = engine.speed_multiplier;
                let step = engine.single_step;
                if step {
                    engine.single_step = false;
                    engine.tick();
                }
                (spd, step)
            };

            if single_step {
                tokio::time::sleep(Duration::from_millis(50)).await;
                continue;
            }

            if speed <= 0.0 {
                // Paused
                tokio::time::sleep(Duration::from_millis(100)).await;
            } else if speed >= 1000.0 {
                // Sonic Turbo: execute smooth batch of ticks and yield cooperatively to prevent lock starvation
                {
                    let mut engine = sim_ticker.write();
                    for _ in 0..15 {
                        engine.tick();
                    }
                }
                tokio::time::sleep(Duration::from_millis(2)).await;
            } else {
                // Dynamic tick pace
                {
                    let mut engine = sim_ticker.write();
                    engine.tick();
                }

                // 1x = 100ms (10 TPS), 10x = 10ms (100 TPS), 100x = 1ms (1000 TPS)
                let sleep_ms = (100.0 / speed).max(1.0) as u64;
                tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
            }
        }
    });

    let app = create_router(sim_engine);
    let port = 3000;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    println!("[*] Simulation server running at http://localhost:{}", port);
    println!(
        "[*] Open http://localhost:{} in your browser to view the God Console.",
        port
    );

    axum::serve(listener, app).await?;

    Ok(())
}
