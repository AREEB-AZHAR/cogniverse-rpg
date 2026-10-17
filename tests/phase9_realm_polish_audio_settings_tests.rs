use cogniverse_rpg::agent::apotheosis::{DemigodAura, DemigodState, DivineDomain};
use cogniverse_rpg::agent::CivilizationRace;
use cogniverse_rpg::factions::warfare::{SiegeEngineKind, WarCampaign};
use cogniverse_rpg::world::titans::TitanState;
use cogniverse_rpg::world::WorldGrid;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct MinimapProjection {
    world_size: f32,
    minimap_size: f32,
}

impl MinimapProjection {
    fn new(world_size: f32, minimap_size: f32) -> Self {
        Self {
            world_size,
            minimap_size,
        }
    }

    fn scale(&self) -> f32 {
        self.minimap_size / self.world_size
    }

    fn world_to_minimap(&self, wx: f32, wy: f32) -> (f32, f32) {
        let s = self.scale();
        (wx * s, wy * s)
    }

    fn minimap_to_world(&self, mx: f32, my: f32) -> (usize, usize) {
        let s = self.scale();
        let wx = (mx / s).clamp(0.0, self.world_size - 1.0).round() as usize;
        let wy = (my / s).clamp(0.0, self.world_size - 1.0).round() as usize;
        (wx, wy)
    }

    fn camera_frustum_minimap_rect(
        &self,
        cam_x: f32,
        cam_y: f32,
        cam_zoom: f32,
        canvas_w: f32,
        canvas_h: f32,
        tile_px: f32,
    ) -> (f32, f32, f32, f32) {
        let scaled_tile_px = tile_px * cam_zoom;
        let min_wx = (-cam_x / scaled_tile_px).clamp(0.0, self.world_size);
        let min_wy = (-cam_y / scaled_tile_px).clamp(0.0, self.world_size);
        let max_wx = ((-cam_x + canvas_w) / scaled_tile_px).clamp(0.0, self.world_size);
        let max_wy = ((-cam_y + canvas_h) / scaled_tile_px).clamp(0.0, self.world_size);

        let (mx, my) = self.world_to_minimap(min_wx, min_wy);
        let mw = (max_wx - min_wx) * self.scale();
        let mh = (max_wy - min_wy) * self.scale();
        (mx, my, mw.max(4.0), mh.max(4.0))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RealmAudioSettings {
    master_volume: f32,
    sfx_volume: f32,
    ambient_volume: f32,
    is_muted: bool,
}

impl Default for RealmAudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.80,
            sfx_volume: 0.85,
            ambient_volume: 0.40,
            is_muted: false,
        }
    }
}

impl RealmAudioSettings {
    fn effective_sfx_gain(&self) -> f32 {
        if self.is_muted {
            0.0
        } else {
            (self.master_volume * self.sfx_volume).clamp(0.0, 1.0)
        }
    }

    fn effective_ambient_gain(&self) -> f32 {
        if self.is_muted {
            0.0
        } else {
            (self.master_volume * self.ambient_volume).clamp(0.0, 1.0)
        }
    }

    fn set_master(&mut self, val: f32) {
        self.master_volume = val.clamp(0.0, 1.0);
    }

    fn toggle_mute(&mut self) -> bool {
        self.is_muted = !self.is_muted;
        self.is_muted
    }
}

#[test]
fn test_physical_audio_assets_on_disk_integrity() {
    let audio_dir = Path::new("static/audio");
    assert!(
        audio_dir.exists(),
        "The static/audio directory must exist on disk"
    );

    let required_assets = [
        "click.wav",
        "mining.wav",
        "explosion.wav",
        "titan_roar.wav",
        "chime.wav",
        "victory.wav",
        "defeat.wav",
        "ambient_breeze.wav",
    ];

    for asset in &required_assets {
        let file_path = audio_dir.join(asset);
        assert!(
            file_path.exists(),
            "Audio asset file must exist on disk: {:?}",
            file_path
        );

        let mut file = File::open(&file_path)
            .unwrap_or_else(|_| panic!("Failed to open audio asset: {:?}", file_path));
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .unwrap_or_else(|_| panic!("Failed to read audio asset: {:?}", file_path));

        // Verify minimum valid WAV size (44-byte header + data)
        assert!(
            bytes.len() > 44,
            "WAV asset {:?} is too small: {} bytes",
            asset,
            bytes.len()
        );

        // Verify RIFF / WAVE header structure
        assert_eq!(
            &bytes[0..4],
            b"RIFF",
            "Asset {:?} must begin with 'RIFF'",
            asset
        );
        assert_eq!(
            &bytes[8..12],
            b"WAVE",
            "Asset {:?} format must be 'WAVE'",
            asset
        );
        assert_eq!(
            &bytes[12..16],
            b"fmt ",
            "Asset {:?} must contain 'fmt ' chunk",
            asset
        );

        // Verify audio format is 1 (PCM uncompressed)
        let audio_format = u16::from_le_bytes([bytes[20], bytes[21]]);
        assert_eq!(
            audio_format, 1,
            "Asset {:?} must be uncompressed PCM (format 1)",
            asset
        );

        // Verify sample rate is 44,100 Hz
        let sample_rate = u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]);
        assert_eq!(
            sample_rate, 44100,
            "Asset {:?} sample rate must be 44.1 kHz",
            asset
        );

        // Verify bit depth is 16-bit
        let bits_per_sample = u16::from_le_bytes([bytes[34], bytes[35]]);
        assert_eq!(
            bits_per_sample, 16,
            "Asset {:?} must be 16-bit audio",
            asset
        );
    }
}

#[test]
fn test_minimap_projection_and_bounds_clamping() {
    let proj = MinimapProjection::new(128.0, 160.0);
    assert!((proj.scale() - 1.25).abs() < 1e-5);

    // World to minimap projections
    let (m_origin_x, m_origin_y) = proj.world_to_minimap(0.0, 0.0);
    assert_eq!(m_origin_x, 0.0);
    assert_eq!(m_origin_y, 0.0);

    let (m_center_x, m_center_y) = proj.world_to_minimap(64.0, 64.0);
    assert_eq!(m_center_x, 80.0);
    assert_eq!(m_center_y, 80.0);

    let (m_max_x, m_max_y) = proj.world_to_minimap(127.0, 127.0);
    assert_eq!(m_max_x, 158.75);
    assert_eq!(m_max_y, 158.75);

    // Minimap to world inversions with clamping
    let (w_center_x, w_center_y) = proj.minimap_to_world(80.0, 80.0);
    assert_eq!(w_center_x, 64);
    assert_eq!(w_center_y, 64);

    let (w_clamp_low_x, w_clamp_low_y) = proj.minimap_to_world(-20.0, -50.0);
    assert_eq!(w_clamp_low_x, 0);
    assert_eq!(w_clamp_low_y, 0);

    let (w_clamp_high_x, w_clamp_high_y) = proj.minimap_to_world(250.0, 300.0);
    assert_eq!(w_clamp_high_x, 127);
    assert_eq!(w_clamp_high_y, 127);

    // Camera Frustum Viewport calculation
    // Camera centered on world at zoom 1.0 (canvas 1280x720, tile 14px)
    let frustum = proj.camera_frustum_minimap_rect(0.0, 0.0, 1.0, 1280.0, 720.0, 14.0);
    assert_eq!(frustum.0, 0.0);
    assert_eq!(frustum.1, 0.0);
    assert!(frustum.2 > 0.0);
    assert!(frustum.3 > 0.0);
}

#[test]
fn test_war_room_telemetry_fidelity_and_serialization() {
    let mut world = WorldGrid::new();

    // Verify titans are present and serialize cleanly
    assert_eq!(world.titans.titans.len(), 3);
    let titans_json =
        serde_json::to_string(&world.titans.titans).expect("Titans must serialize to JSON");
    assert!(titans_json.contains("AbyssalLeviathan"));
    assert!(titans_json.contains("MagmaWyrm"));
    assert!(titans_json.contains("VoidColossus"));

    // Awaken a titan and check state change
    world.titans.titans[0].awaken();
    assert_eq!(world.titans.titans[0].state, TitanState::Awakened);

    // Verify Demigod Apotheosis state serialization
    let demigod = DemigodState {
        is_ascended: true,
        domain: Some(DivineDomain::SunHerald),
        aura: Some(DemigodAura {
            domain: DivineDomain::SunHerald,
            radius: 6,
            potency: 1.25,
        }),
        ascension_tier: 1,
        ..Default::default()
    };

    let apoth_json = serde_json::to_string(&demigod).expect("Demigod must serialize to JSON");
    assert!(apoth_json.contains("SunHerald"));
    assert!(apoth_json.contains("true"));

    // Verify Siege Engine Warfare Campaign list
    let mut campaign = WarCampaign::new(64, 64);
    let engine_id = campaign.build_siege_engine(
        SiegeEngineKind::Catapult,
        CivilizationRace::SolarTerran,
        20,
        24,
        20,
        20,
    );
    assert_eq!(engine_id, 1);
    assert_eq!(campaign.engines.len(), 1);

    let siege_json =
        serde_json::to_string(&campaign.engines).expect("Siege engines must serialize to JSON");
    assert!(siege_json.contains("Catapult"));
    assert!(siege_json.contains("SolarTerran"));
}

#[test]
fn test_settings_audio_volume_attenuation_and_mute_logic() {
    let mut settings = RealmAudioSettings::default();
    assert!((settings.master_volume - 0.80).abs() < 1e-5);
    assert!((settings.sfx_volume - 0.85).abs() < 1e-5);
    assert!((settings.ambient_volume - 0.40).abs() < 1e-5);
    assert!(!settings.is_muted);

    // Initial effective gains
    let sfx_gain = settings.effective_sfx_gain();
    assert!((sfx_gain - (0.80 * 0.85)).abs() < 1e-4);

    let amb_gain = settings.effective_ambient_gain();
    assert!((amb_gain - (0.80 * 0.40)).abs() < 1e-4);

    // Mute toggle
    let muted = settings.toggle_mute();
    assert!(muted);
    assert_eq!(settings.effective_sfx_gain(), 0.0);
    assert_eq!(settings.effective_ambient_gain(), 0.0);

    // Unmute restores previous level
    let unmuted = settings.toggle_mute();
    assert!(!unmuted);
    assert!((settings.effective_sfx_gain() - (0.80 * 0.85)).abs() < 1e-4);

    // Master volume clamp
    settings.set_master(1.5);
    assert_eq!(settings.master_volume, 1.0);
    settings.set_master(-0.5);
    assert_eq!(settings.master_volume, 0.0);
}
