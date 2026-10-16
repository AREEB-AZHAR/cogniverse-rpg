use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataLoadError {
    #[error("I/O error reading data file '{0}': {1}")]
    Io(String, #[source] std::io::Error),

    #[error("RON deserialization error in '{0}': {1}")]
    RonParse(String, #[source] ron::error::SpannedError),

    #[error("Validation error in definition '{id}' of type '{data_type}': {reason}")]
    Validation {
        data_type: &'static str,
        id: String,
        reason: String,
    },

    #[error("Missing definition with id '{0}'")]
    NotFound(String),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MineDef {
    pub id: String,
    pub name: String,
    pub min_tech: String,
    pub tool_tier: u8,
    pub yield_range: (u32, u32),
    pub biomes: Vec<String>,
    pub sprite: String,
}

impl MineDef {
    pub fn validate(&self) -> Result<(), DataLoadError> {
        if self.id.trim().is_empty() {
            return Err(DataLoadError::Validation {
                data_type: "MineDef",
                id: "<empty>".into(),
                reason: "Mine ID cannot be empty".into(),
            });
        }
        if self.yield_range.0 == 0 || self.yield_range.1 < self.yield_range.0 {
            return Err(DataLoadError::Validation {
                data_type: "MineDef",
                id: self.id.clone(),
                reason: format!("Invalid yield range {:?}", self.yield_range),
            });
        }
        if self.biomes.is_empty() {
            return Err(DataLoadError::Validation {
                data_type: "MineDef",
                id: self.id.clone(),
                reason: "Mine must specify at least one valid biome".into(),
            });
        }
        if self.sprite.trim().is_empty() {
            return Err(DataLoadError::Validation {
                data_type: "MineDef",
                id: self.id.clone(),
                reason: "Sprite identifier cannot be empty".into(),
            });
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DietType {
    Herbivore,
    Carnivore,
    Omnivore,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct AnimalDef {
    pub id: String,
    pub name: String,
    pub diet: DietType,
    pub biomes: Vec<String>,
    pub herd_size: (u32, u32),
    pub lifespan: u32,
    pub aggression: f32,
    pub domesticable: bool,
    pub products: Vec<String>,
    pub sprite: String,
    pub max_health: f32,
    pub attack_power: f32,
}

impl AnimalDef {
    pub fn validate(&self) -> Result<(), DataLoadError> {
        if self.id.trim().is_empty() {
            return Err(DataLoadError::Validation {
                data_type: "AnimalDef",
                id: "<empty>".into(),
                reason: "Animal species ID cannot be empty".into(),
            });
        }
        if self.herd_size.0 == 0 || self.herd_size.1 < self.herd_size.0 {
            return Err(DataLoadError::Validation {
                data_type: "AnimalDef",
                id: self.id.clone(),
                reason: format!("Invalid herd size {:?}", self.herd_size),
            });
        }
        if self.max_health <= 0.0 {
            return Err(DataLoadError::Validation {
                data_type: "AnimalDef",
                id: self.id.clone(),
                reason: format!("Max health must be positive, got {}", self.max_health),
            });
        }
        if self.biomes.is_empty() {
            return Err(DataLoadError::Validation {
                data_type: "AnimalDef",
                id: self.id.clone(),
                reason: "Animal must specify at least one valid biome".into(),
            });
        }
        if self.sprite.trim().is_empty() {
            return Err(DataLoadError::Validation {
                data_type: "AnimalDef",
                id: self.id.clone(),
                reason: "Sprite identifier cannot be empty".into(),
            });
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct GameDataRegistry {
    pub mines: HashMap<String, MineDef>,
    pub animals: HashMap<String, AnimalDef>,
}

impl GameDataRegistry {
    pub fn new() -> Self {
        Self {
            mines: HashMap::new(),
            animals: HashMap::new(),
        }
    }

    pub fn load_from_directory<P: AsRef<Path>>(
        &mut self,
        dir_path: P,
    ) -> Result<(), DataLoadError> {
        let p = dir_path.as_ref();
        let mines_file = p.join("mines.ron");
        let animals_file = p.join("animals.ron");

        self.load_mines_file(&mines_file)?;
        self.load_animals_file(&animals_file)?;

        Ok(())
    }

    pub fn load_mines_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), DataLoadError> {
        let path_str = path.as_ref().display().to_string();
        let content =
            fs::read_to_string(&path).map_err(|e| DataLoadError::Io(path_str.clone(), e))?;

        let mines: Vec<MineDef> =
            ron::from_str(&content).map_err(|e| DataLoadError::RonParse(path_str, e))?;

        for mine in mines {
            mine.validate()?;
            self.mines.insert(mine.id.clone(), mine);
        }

        Ok(())
    }

    pub fn load_animals_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), DataLoadError> {
        let path_str = path.as_ref().display().to_string();
        let content =
            fs::read_to_string(&path).map_err(|e| DataLoadError::Io(path_str.clone(), e))?;

        let animals: Vec<AnimalDef> =
            ron::from_str(&content).map_err(|e| DataLoadError::RonParse(path_str, e))?;

        for animal in animals {
            animal.validate()?;
            self.animals.insert(animal.id.clone(), animal);
        }

        Ok(())
    }

    pub fn get_mine(&self, id: &str) -> Option<&MineDef> {
        self.mines.get(id)
    }

    pub fn get_animal(&self, id: &str) -> Option<&AnimalDef> {
        self.animals.get(id)
    }
}
