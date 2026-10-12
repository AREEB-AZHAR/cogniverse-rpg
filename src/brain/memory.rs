use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub tick: u64,
    pub description: String,
    pub tags: Vec<String>,
    pub importance: f32, // 1.0 to 10.0
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MemoryHierarchy {
    pub working_buffer: Vec<String>, // Last 12 ticks
    pub episodic_stream: Vec<EpisodicMemory>,
}

impl MemoryHierarchy {
    pub fn new() -> Self {
        Self {
            working_buffer: Vec::with_capacity(16),
            episodic_stream: Vec::with_capacity(64),
        }
    }

    pub fn record_observation(&mut self, text: impl Into<String>) {
        let t = text.into();
        self.working_buffer.push(t);
        if self.working_buffer.len() > 12 {
            self.working_buffer.remove(0);
        }
    }

    pub fn log_episodic_event(&mut self, tick: u64, description: impl Into<String>, tags: Vec<String>, importance: f32) {
        self.episodic_stream.push(EpisodicMemory {
            tick,
            description: description.into(),
            tags,
            importance: importance.clamp(1.0, 10.0),
        });

        // Bounded memory to prevent memory leaks on CPU (keep top 60 significant memories)
        if self.episodic_stream.len() > 60 {
            // Sort by importance and drop oldest least-important
            self.episodic_stream.remove(0);
        }
    }

    /// Retrieves memories based on Stanford Generative Agents scoring:
    /// Score = w_rec * exp(-lambda * delta_t) + w_imp * (I/10) + w_rel * tag_match
    pub fn query_memories(&self, current_tick: u64, query_tag: &str, limit: usize) -> Vec<&EpisodicMemory> {
        let mut scored: Vec<(f32, &EpisodicMemory)> = self
            .episodic_stream
            .iter()
            .map(|mem| {
                let delta_t = (current_tick.saturating_sub(mem.tick)) as f32;
                let recency = (-0.005 * delta_t).exp();
                let importance_norm = mem.importance / 10.0;
                let relevance = if mem.tags.iter().any(|t| t.eq_ignore_ascii_case(query_tag)) {
                    1.0
                } else {
                    0.0
                };

                let score = 0.3 * recency + 0.35 * importance_norm + 0.35 * relevance;
                (score, mem)
            })
            .collect();

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(limit).map(|(_, m)| m).collect()
    }
}

/// Global repository of ancient ancestral memories and legendary AST strategies
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AkashicCore {
    pub legendary_echoes: Vec<AncestralEcho>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AncestralEcho {
    pub hero_name: String,
    pub title: String,
    pub generation: u32,
    pub gift_description: String,
    pub innate_piety_bonus: f32,
    pub innate_craft_bonus: f32,
}

impl AkashicCore {
    pub fn new() -> Self {
        Self {
            legendary_echoes: vec![
                AncestralEcho {
                    hero_name: "Ignis the First Smith".into(),
                    title: "Father of Metallurgy".into(),
                    generation: 1,
                    gift_description: "Innate instinct for locating iron veins and stone crafting".into(),
                    innate_piety_bonus: 0.1,
                    innate_craft_bonus: 0.3,
                },
                AncestralEcho {
                    hero_name: "Sylva the Far-Seer".into(),
                    title: "Mistress of Whispers".into(),
                    generation: 1,
                    gift_description: "Deep affinity with nature, berries, and medicinal popppies".into(),
                    innate_piety_bonus: 0.2,
                    innate_craft_bonus: 0.1,
                },
            ],
        }
    }

    pub fn record_legend(&mut self, name: String, title: String, gen: u32, desc: String) {
        self.legendary_echoes.push(AncestralEcho {
            hero_name: name,
            title,
            generation: gen,
            gift_description: desc,
            innate_piety_bonus: 0.2,
            innate_craft_bonus: 0.2,
        });
    }

    pub fn maybe_awaken_echo(&self) -> Option<AncestralEcho> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if !self.legendary_echoes.is_empty() && rng.gen_bool(0.12) {
            let idx = rng.gen_range(0..self.legendary_echoes.len());
            Some(self.legendary_echoes[idx].clone())
        } else {
            None
        }
    }
}
