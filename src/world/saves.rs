use super::planet::CubeSphereWorld;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const CURRENT_SAVE_VERSION: u32 = 1;

#[derive(Error, Debug)]
pub enum SaveError {
    #[error("I/O error during save operation '{0}': {1}")]
    Io(String, #[source] std::io::Error),

    #[error("Serialization / deserialization error: {0}")]
    Bincode(#[from] bincode::Error),

    #[error("Unsupported save version {found}, expected {expected}")]
    IncompatibleVersion { found: u32, expected: u32 },

    #[error("Failed to locate %APPDATA% directory on host system")]
    MissingAppData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WorldSaveHeader {
    pub magic: [u8; 4], // b"CGNW" (Cogniverse World)
    pub version: u32,
    pub timestamp: u64,
}

pub struct SaveSystem;

impl SaveSystem {
    /// Returns default save folder under %APPDATA%/cogniverse-rpg/saves
    pub fn get_appdata_saves_dir() -> Result<PathBuf, SaveError> {
        let app_data = dirs::data_dir().ok_or(SaveError::MissingAppData)?;
        let saves_dir = app_data.join("cogniverse-rpg").join("saves");
        fs::create_dir_all(&saves_dir)
            .map_err(|e| SaveError::Io(saves_dir.display().to_string(), e))?;
        Ok(saves_dir)
    }

    /// Saves CubeSphereWorld atomically: writes to .tmp file, flushes, then atomic rename
    pub fn save_atomic<P: AsRef<Path>>(
        world: &CubeSphereWorld,
        destination: P,
    ) -> Result<(), SaveError> {
        let dest = destination.as_ref();
        let tmp_path = dest.with_extension("tmp");

        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| SaveError::Io(parent.display().to_string(), e))?;
        }

        let mut file = File::create(&tmp_path)
            .map_err(|e| SaveError::Io(tmp_path.display().to_string(), e))?;

        let header = WorldSaveHeader {
            magic: *b"CGNW",
            version: CURRENT_SAVE_VERSION,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        };

        // Write binary header
        let header_bytes = bincode::serialize(&header)?;
        let header_len = header_bytes.len() as u32;
        file.write_all(&header_len.to_le_bytes())
            .map_err(|e| SaveError::Io("write header len".into(), e))?;
        file.write_all(&header_bytes)
            .map_err(|e| SaveError::Io("write header".into(), e))?;

        // Write world payload
        let world_bytes = bincode::serialize(world)?;
        file.write_all(&world_bytes)
            .map_err(|e| SaveError::Io("write world bytes".into(), e))?;

        file.sync_all()
            .map_err(|e| SaveError::Io("sync file".into(), e))?;
        drop(file);

        // Atomic rename replacing existing save
        fs::rename(&tmp_path, dest)
            .map_err(|e| SaveError::Io(format!("rename {:?} to {:?}", tmp_path, dest), e))?;

        Ok(())
    }

    /// Loads CubeSphereWorld from a binary save file, validating header version
    pub fn load_save<P: AsRef<Path>>(path: P) -> Result<CubeSphereWorld, SaveError> {
        let p = path.as_ref();
        let mut file = File::open(p).map_err(|e| SaveError::Io(p.display().to_string(), e))?;

        let mut len_buf = [0u8; 4];
        file.read_exact(&mut len_buf)
            .map_err(|e| SaveError::Io("read header len".into(), e))?;
        let header_len = u32::from_le_bytes(len_buf) as usize;

        let mut header_bytes = vec![0u8; header_len];
        file.read_exact(&mut header_bytes)
            .map_err(|e| SaveError::Io("read header".into(), e))?;

        let header: WorldSaveHeader = bincode::deserialize(&header_bytes)?;
        if header.magic != *b"CGNW" || header.version != CURRENT_SAVE_VERSION {
            return Err(SaveError::IncompatibleVersion {
                found: header.version,
                expected: CURRENT_SAVE_VERSION,
            });
        }

        let mut world_bytes = Vec::new();
        file.read_to_end(&mut world_bytes)
            .map_err(|e| SaveError::Io("read world payload".into(), e))?;

        let world: CubeSphereWorld = bincode::deserialize(&world_bytes)?;
        Ok(world)
    }
}
