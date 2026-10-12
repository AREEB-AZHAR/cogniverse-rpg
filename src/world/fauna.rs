use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimalSpecies {
    Rabbit,
    ForestDeer,
    DireWolf,
    MountainBear,
}

impl AnimalSpecies {
    pub fn is_predator(&self) -> bool {
        matches!(self, AnimalSpecies::DireWolf | AnimalSpecies::MountainBear)
    }

    pub fn max_health(&self) -> f32 {
        match self {
            AnimalSpecies::Rabbit => 20.0,
            AnimalSpecies::ForestDeer => 60.0,
            AnimalSpecies::DireWolf => 80.0,
            AnimalSpecies::MountainBear => 160.0,
        }
    }

    pub fn attack_power(&self) -> f32 {
        match self {
            AnimalSpecies::Rabbit => 0.0,
            AnimalSpecies::ForestDeer => 2.0,
            AnimalSpecies::DireWolf => 15.0,
            AnimalSpecies::MountainBear => 30.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WildlifeEntity {
    pub id: usize,
    pub species: AnimalSpecies,
    pub x: usize,
    pub y: usize,
    pub health: f32,
    pub hunger: f32, // 0.0 (full) to 1.0 (starving)
    pub age_ticks: u64,
    pub is_alive: bool,
}

impl WildlifeEntity {
    pub fn new(id: usize, species: AnimalSpecies, x: usize, y: usize) -> Self {
        Self {
            id,
            species,
            x,
            y,
            health: species.max_health(),
            hunger: 0.2,
            age_ticks: 0,
            is_alive: true,
        }
    }

    pub fn tick(&mut self, map_width: usize, map_height: usize) {
        if !self.is_alive {
            return;
        }

        self.age_ticks += 1;
        self.hunger = (self.hunger + 0.002).min(1.0);

        // Starvation damage
        if self.hunger > 0.9 {
            self.health -= 0.5;
            if self.health <= 0.0 {
                self.is_alive = false;
                return;
            }
        }

        // Random roaming or foraging step
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.4) {
            let dx = rng.gen_range(-1..=1);
            let dy = rng.gen_range(-1..=1);

            let new_x = (self.x as isize + dx).clamp(0, map_width as isize - 1) as usize;
            let new_y = (self.y as isize + dy).clamp(0, map_height as isize - 1) as usize;
            self.x = new_x;
            self.y = new_y;
        }
    }
}
