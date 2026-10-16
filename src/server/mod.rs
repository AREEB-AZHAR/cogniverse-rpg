pub mod covenants;
pub mod god_powers;
pub mod protocol;
pub mod state;

use std::sync::Arc;
use std::time::Duration;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use parking_lot::RwLock;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use protocol::ClientMessage;
use state::SimulationEngine;

pub type SharedSimulation = Arc<RwLock<SimulationEngine>>;

pub fn create_router(sim: SharedSimulation) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .fallback_service(ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(sim)
}

async fn ws_handler(ws: WebSocketUpgrade, State(sim): State<SharedSimulation>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, sim))
}

async fn handle_socket(socket: WebSocket, sim: SharedSimulation) {
    let (mut sender, mut receiver) = socket.split();

    // Spawn a background task to stream telemetry to the client
    let sim_stream = sim.clone();
    let mut send_task = tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let telemetry = {
                let engine = sim_stream.read();
                engine.generate_telemetry()
            };

            if let Ok(json_str) = serde_json::to_string(&telemetry) {
                if sender.send(Message::Text(json_str.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Handle incoming commands from the client
    let sim_recv = sim.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(cmd) = serde_json::from_str::<ClientMessage>(&text) {
                    let mut engine = sim_recv.write();
                    match cmd {
                        ClientMessage::SetSpeedMultiplier { speed } => {
                            engine.speed_multiplier = speed;
                            engine.log_event(format!("Time dilation updated: {:.1}x", speed));
                        }
                        ClientMessage::StepSingleTick => {
                            engine.single_step = true;
                        }
                        ClientMessage::IssueCovenant { objective, target_amount, reward, punishment } => {
                            let next_id = engine.next_covenant_id;
                            engine.next_covenant_id += 1;
                            let current_tick = engine.world.gaia.world_tick;
                            let covenant = covenants::DivineCovenant::new(
                                next_id,
                                objective,
                                target_amount,
                                reward,
                                punishment,
                                current_tick,
                            );
                            engine.log_event(format!("A NEW DIVINE COVENANT HAS BEEN FORGED: '{}'", covenant.title));
                            engine.covenants.push(covenant);
                        }
                        ClientMessage::BlessOffering { altar_id, offering_id } => {
                            if let Some(altar) = engine.world.altars.iter_mut().find(|a| a.id == altar_id) {
                                if let Some(offering) = altar.recent_offerings.iter_mut().find(|o| o.offering_id == offering_id) {
                                    offering.resolved = true;
                                    offering.outcome = Some(crate::world::altar::OfferingOutcome::Blessed {
                                        boon_type: "Vitality & Abundant Manna".into(),
                                        potency: 1.0,
                                    });
                                }
                            }
                            engine.log_event("You stretched forth your holy hand and BLESSED the altar sacrifice!");
                        }
                        ClientMessage::SmiteOffering { altar_id, offering_id } => {
                            if let Some(altar) = engine.world.altars.iter_mut().find(|a| a.id == altar_id) {
                                if let Some(offering) = altar.recent_offerings.iter_mut().find(|o| o.offering_id == offering_id) {
                                    offering.resolved = true;
                                    offering.outcome = Some(crate::world::altar::OfferingOutcome::Smitten {
                                        retribution_type: "Wrathful Retribution".into(),
                                        severity: 1.0,
                                    });
                                }
                            }
                            engine.log_event("You struck the offering with divine DISPLEASURE!");
                        }
                        ClientMessage::TriggerIntervention { intervention } => {
                            engine.execute_intervention(intervention);
                        }
                        ClientMessage::SelectAgent { agent_id } => {
                            engine.selected_agent_id = Some(agent_id);
                        }
                        ClientMessage::TerraformArea { center_x, center_y, brush, radius } => {
                            engine.terraform_area(center_x, center_y, brush, radius);
                        }
                        ClientMessage::SpawnAgentAt { race, x, y } => {
                            engine.spawn_agent_at(race, x, y);
                        }
                        ClientMessage::ResetWorld => {
                            engine.reset_world();
                            engine.log_event("The cosmos shivers as the Divine Will RESETS the living realm!");
                        }
                        ClientMessage::ChangeMap { archetype, seed, water_level, mountain_density } => {
                            engine.change_map(archetype, seed, water_level, mountain_density);
                        }
                        ClientMessage::RequestFullMap => {
                            engine.request_full_map_sync();
                        }
                        ClientMessage::ExportState => {
                            engine.trigger_export_state();
                            engine.log_event("Exporting world knowledge, achievements, and civilization history to JSON snapshot.");
                        }
                        ClientMessage::ImportState { snapshot_json } => {
                            match engine.import_run_snapshot_json(&snapshot_json) {
                                Ok(summary) => {
                                    engine.log_event(&format!("Import successful: {}", summary));
                                }
                                Err(err) => {
                                    engine.log_event(&format!("Import failed: {}", err));
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
