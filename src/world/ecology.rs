use serde::{Deserialize, Serialize};
use super::tile::{BiomeType, Tile};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherCondition {
    ClearSun,
    GentleRain,
    Thunderstorm,
    FrostBreeze,
    Heatwave,
}

impl WeatherCondition {
    pub fn display_name(&self) -> &'static str {
        match self {
            WeatherCondition::ClearSun => "Clear Sun",
            WeatherCondition::GentleRain => "Gentle Rain",
            WeatherCondition::Thunderstorm => "Thunderstorm",
            WeatherCondition::FrostBreeze => "Frost Breeze",
            WeatherCondition::Heatwave => "Heatwave",
        }
    }

    pub fn temperature_delta(&self) -> f32 {
        match self {
            WeatherCondition::ClearSun => 2.0,
            WeatherCondition::GentleRain => -2.0,
            WeatherCondition::Thunderstorm => -5.0,
            WeatherCondition::FrostBreeze => -10.0,
            WeatherCondition::Heatwave => 8.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GaiaEngine {
    pub world_tick: u64,
    pub day_length: u64, // e.g. 1000 ticks per cycle
    pub current_weather: WeatherCondition,
    pub weather_duration_remaining: u64,
    pub global_temperature_offset: f32,
    pub solar_intensity: f32, // 0.0 (midnight) to 1.0 (noon)
    pub rain_rate: f32,
}

impl GaiaEngine {
    pub fn new() -> Self {
        Self {
            world_tick: 0,
            day_length: 1000,
            current_weather: WeatherCondition::ClearSun,
            weather_duration_remaining: 300,
            global_temperature_offset: 0.0,
            solar_intensity: 0.5,
            rain_rate: 0.0,
        }
    }

    pub fn is_daytime(&self) -> bool {
        let time_of_day = self.world_tick % self.day_length;
        time_of_day >= 200 && time_of_day <= 800
    }

    pub fn tick(&mut self, tiles: &mut [Tile]) {
        self.world_tick += 1;

        // 1. Solar calculation: sinusoidal cycle over 1,000 ticks
        let cycle_pos = (self.world_tick % self.day_length) as f32 / self.day_length as f32;
        // Peak at 0.5 (noon), minimum at 0.0 and 1.0 (midnight)
        self.solar_intensity = (1.0 - (2.0 * (cycle_pos - 0.5)).abs()).clamp(0.0, 1.0);

        // 2. Weather cycle shifts
        if self.weather_duration_remaining == 0 {
            self.transition_weather();
        } else {
            self.weather_duration_remaining -= 1;
        }

        // 3. Update global temperature offset
        let solar_temp_impact = (self.solar_intensity - 0.5) * 16.0;
        let weather_temp_impact = self.current_weather.temperature_delta();
        self.global_temperature_offset = solar_temp_impact + weather_temp_impact;

        // 4. Update tiles and flora regrowth every 25 ticks
        if self.world_tick % 25 == 0 {
            for tile in tiles.iter_mut() {
                // Adjust local temperature
                let base = tile.biome.base_temperature();
                tile.temperature = base + self.global_temperature_offset;

                // Regrow flora based on moisture, soil fertility, and sunlight
                if self.solar_intensity > 0.2 {
                    let growth_factor = tile.soil_fertility * (0.8 + 0.4 * self.solar_intensity);
                    
                    if tile.biome == BiomeType::SunlitPlains && tile.grain_amount < 25 {
                        if (growth_factor * 10.0) as u32 > 5 {
                            tile.grain_amount += 1;
                        }
                    }

                    if (tile.biome == BiomeType::WhisperingForest || tile.biome == BiomeType::SunlitPlains)
                        && tile.berry_amount < 20
                    {
                        if (growth_factor * 8.0) as u32 > 4 {
                            tile.berry_amount += 1;
                        }
                    }

                    if tile.biome == BiomeType::WhisperingForest && tile.wood_amount < 50 {
                        if (growth_factor * 5.0) as u32 > 3 {
                            tile.wood_amount += 1;
                        }
                    }
                }
            }
        }
    }

    fn transition_weather(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        
        self.current_weather = if roll < 0.50 {
            WeatherCondition::ClearSun
        } else if roll < 0.75 {
            WeatherCondition::GentleRain
        } else if roll < 0.88 {
            WeatherCondition::Thunderstorm
        } else if roll < 0.95 {
            WeatherCondition::FrostBreeze
        } else {
            WeatherCondition::Heatwave
        };

        self.weather_duration_remaining = rng.gen_range(200..600);
        self.rain_rate = match self.current_weather {
            WeatherCondition::GentleRain => 0.4,
            WeatherCondition::Thunderstorm => 1.0,
            _ => 0.0,
        };
    }
}
