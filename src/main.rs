use std::sync::Arc;
use std::time::Duration;
use parking_lot::RwLock;
use tokio::net::TcpListener;

use cogniverse_rpg::server::state::SimulationEngine;
use cogniverse_rpg::server::create_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                // Uncapped Turbo: execute batch of ticks before yielding
                {
                    let mut engine = sim_ticker.write();
                    for _ in 0..50 {
                        engine.tick();
                    }
                }
                tokio::task::yield_now().await;
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
    println!("[*] Open http://localhost:{} in your browser to view the God Console.", port);

    axum::serve(listener, app).await?;

    Ok(())
}
