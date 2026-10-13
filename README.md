# ✦ Genesis Cogniverse
### Autonomous Neuro-Symbolic Self-Learning AI RPG World & WorldBox God Pantheon Engine in Pure Rust

[![Rust](https://img.shields.io/badge/Language-Rust%202021-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Architecture](https://img.shields.io/badge/Architecture-Data--Oriented%20ECS-green.svg)]()
[![Inspiration](https://img.shields.io/badge/Inspiration-WorldBox%20God%20Simulator-purple.svg)]()
[![Hardware](https://img.shields.io/badge/Hardware-Ordinary%20PC%20%2F%20CPU--First-success.svg)]()

> **Genesis Cogniverse** is a persistent, CPU-optimized artificial life simulation and fantasy RPG sandbox inspired by **WorldBox**, where **every entity is an autonomous, self-learning AI**. Non-player characters (NPCs) live with absolute free will, gather resources, build cabins, present altar offerings, mate, experience biological neuromodulation, and **recursively rewrite their own decision code over generations** using fuel-metered Abstract Syntax Trees (ASTs).

---

## 📸 WorldBox-Inspired God Admin Console

![WorldBox God Admin Console](screenshots/worldbox_god_console.png)
*Live 2D Procedural Matrix (64x64 grid) with interactive Pan & Zoom, WorldBox Bottom God Power Bar (Terrain Brushes, Civilization Spawners, Cataclysms, Camera Inspect), Top Macro Demographics Bar, Weather Particles (Rain/Snow/Thunder), Dynamic Day/Night Lighting with radial light cutouts around campfires & cabins, and WorldBox Character Sheet with Trait Badges, Favorite Star ⭐, Vitals, Inner Monologue, and Darwin-Gödel AST Genome.*

---

## 🌍 WorldBox Features & God Sandbox Controls

### 1. Macro Demographics & Civilization Metrics Bar
Located at the very top of the God Console, providing real-time macro telemetry across the entire world:
* **POP**: Total living sentient population in the simulation.
* **FACTIONS**: Balance of power between `☀️ Solar Terrans` and `🌙 Chthonic Lunarians`.
* **FAUNA**: Autonomous wildlife carrying capacity (Deer, Wolves, Boars, Rabbits).
* **BIRTH/DEATH**: Lifetime birth counter (👶) and death counter (💀).
* **MAX GEN**: Maximum evolutionary generation reached through genetic Darwin-Gödel mutation.
* **AVG VITALS**: Population-wide average health (❤️) and hunger (🍞) levels.
* **DEVOTION**: Average civilizational piety and sincere religious devotion (🙏).
* **WEALTH**: Total circulating economic goods, tools, and ores (🪙).
* **ENVIRONMENT**: Current thermodynamic day/night phase and dynamic weather conditions.

### 2. Interactive World Matrix with Smooth Pan & Zoom
* **Mouse-Wheel Zoom**: Seamlessly zoom in (up to 4.0x) and out (down to 0.5x) centered directly on the cursor.
* **Click-and-Drag Pan**: Click and drag with the middle mouse button, right mouse button, or while holding Spacebar to explore the 64x64 procedural world.
* **Camera Toolbar**: Floating canvas overlay with `➕ Zoom In`, `➖ Zoom Out`, `🎯 Reset View`, and `🔭 Follow Character` mode.
* **Find Leader Button**: Instantly locates the most senior elder or highest-piety civilizational leader and centers the camera on them.

### 3. Dynamic Terraforming Brushes with Variable Radius
Transform the physical world in real-time using God Brushes with `1x1`, `3x3`, and `5x5` radius sizing:
* **🌾 Plains Brush**: Fertilize lush grassland, planting wild grains and berries.
* **🌲 Forest Brush**: Seed dense whispering woodland with harvestable timber and game.
* **🌊 River Brush**: Carve freshwater rivers for drinking, irrigation, and hydration.
* **⛰️ Mountain Brush**: Raise rocky Ironcrag peaks rich in stone, flint, and iron ore veins.
* **🔮 Leyline Ruins**: Infuse ancient arcane leyline monoliths radiating high mana density.
* **⛏️ Enrich Iron**: Concentrate rich underground iron deposits into existing rock formations.
* **🧹 Clear Land**: Cleanse scorched terrain and clear obsolete structures.

### 4. WorldBox Character Sheet Inspector
Click any character or use the "Inspect" tool to open a detailed WorldBox character sheet:
* **⭐ Favorite Star Button**: Star your favorite agents to track them; enable camera follow to watch them live as they go about their daily life.
* **WorldBox Trait Badges**: Dynamic traits derived from behavior, heritage, and genetics (e.g. `👑 Leader`, `🌟 Genius`, `⚔️ Strong`, `✨ Charming`, `🕊️ Pacifist`, `🔮 Wise`, `💰 Greedy`).
* **💭 Live Inner Monologue**: Unfiltered stream of consciousness reflecting their current needs, fears, and plans.
* **Vitals Meters**: Health, Energy, Hunger, and Piety percentages.
* **Kenji Doya Neuromodulator Gauges**: Real-time biological cognition (Dopamine, Serotonin, Noradrenaline).
* **Inventory & Gear**: Visible items carried in the agent's pack (e.g. `Wild Berry x8`, `Wood x2`, `Iron Ore`).
* **Ancestral Lineage & Echo**: Heritage details from deceased legendary ancestors enshrined in the Akashic Core.
* **Darwin-Gödel AST Genome**: Full code tree inspection showing the exact self-rewritten logic steering the NPC.
* **Direct God Actions**: `💬 Whisper`, `✨ Bless Piety`, `💚 Heal`, and `⚡ Smite`.

### 5. Atmospheric Weather & Dynamic Lighting
* **Gentle Rain & Thunderstorms**: Animated falling raindrop streaks with ripple splashes and occasional dramatic lightning flashes that illuminate the world.
* **Frost Breeze**: Drifting hexagonal snow crystal particles during cold snaps.
* **Day & Night Cycles**: 1,000-tick orbital cycle with a rich navy darkness overlay during nighttime, illuminated by **soft radial glowing lights** around campfires, cabins, altars, and torches.

---

## 🌟 Cognitive Engine & Foundational Breakthroughs

### 1. Darwin-Gödel Machine AST Self-Rewriting Engine
* **Theoretical Origin**: Sakana AI, UBC, and Jürgen Schmidhuber (2024–2025).
* **The Breakthrough**: Classical Gödel Machines failed because proving code safety mathematically is NP-hard. The **Darwin Gödel Machine** paradigm replaces formal proofs with **empirical sandbox evaluation and semantic genetic programming**.
* **In Cogniverse**: Each NPC carries an executable Abstract Syntax Tree (`BehavioralGenome`). Agents record failure traces, synthesize code patches via **Reaction-Norm Semantic Mutation**, and exchange successful behavioral subtrees during cultural interactions via **Mendelian Crossover**.
* **CPU Safety**: Execution is sandboxed with a strict **500-gas budget** per tick, mathematically preventing infinite recursion or lag on ordinary PC CPUs.

### 2. Biological Neuromodulation (Kenji Doya Formulation)
Rather than static utility weights, each NPC's cognition is co-regulated in real-time by three computational neuromodulators:
* **Dopamine ($\delta_t$)**: Temporal Difference Reward Error ($RPE$) and incentive salience. Reinforces successful tool crafting, foraging, and quest completion.
* **Serotonin ($5\text{-HT}_t$)**: Regulates temporal horizon ($\gamma$) and opponent behavioral inhibition. High 5-HT yields patient long-term planning (e.g. building permanent stone cabins); low 5-HT induces frantic, impulsive foraging.
* **Noradrenaline ($\text{NA}_t$)**: Measures environmental volatility and unexpected surprise. Spikes during predator encounters, blizzards, or divine smites, **multiplying the genetic mutation rate** to trigger rapid evolutionary rewiring.

### 3. Predictive World Model ("Dreamer" RSSM)
* Inspired by David Ha, Jürgen Schmidhuber, and Danijar Hafner's *Dreamer* algorithms.
* Agents maintain an internal transition model: $P(S_{t+1} \mid S_t, A_t)$.
* Before executing high-risk physical actions, agents simulate $K$-step forward mental rollouts ("dreams") to anticipate injury or starvation without risking actual bodily harm.

### 4. Dual Pantheons & The Autonomous Rival AI God
* **The Solar Terrans (Player-Aligned)**: Agrarian, stone-builders, toolmakers, guided by You (The Player God) via the Pantheon Console.
* **The Chthonic Lunarians (Rival Race)**: Nomadic, alchemical brewers, subterranean miners, shadow mages, guided by **The Shadow Weaver** (an autonomous **Meta-Learning Rival AI Deity**).
* **Rival God Intelligence**: The Shadow Weaver independently monitors Terran strength, issues shadow directives to its Lunarians, and grants dark boons without any human prompts!

### 5. Sacrificial Altars, Offerings & Sincerity Scoring
* NPCs carry harvested goods, iron ingots, or rare gems to sacred Altars.
* **Offering Sincerity Formulation**:
  $$\mathcal{S}_{\text{offering}} = \frac{\text{ItemRarity} \cdot \text{CostToAgent}}{\text{AgentWealth} + \epsilon} \times \text{AgentPiety}$$
* Generous peasant sacrifices yield high sincerity, prompting divine blessings. Insulting gifts from hoarders risk divine wrath.

### 6. Nomadic Resurgence & Civilizational Resilience
* If extreme famine or catastrophe causes a faction's population to dwindle, nomadic pioneers migrate into the valley from beyond the mountains, ensuring civilizations never permanently vanish.

---

## 🛠️ Technology Stack & Hardware Guarantees

| Component | Choice | Rationale |
|---|---|---|
| **Core Language** | **Rust (Edition 2021 / 1.97+)** | Zero-cost abstractions, memory safety, thread safety without garbage collection pauses. |
| **Concurrency** | **Tokio & Rayon** | Multi-threaded simulation ticks saturating available CPU cores; asynchronous WebSocket dispatch. |
| **Web Server** | **Axum 0.8** | Ultra-lightweight asynchronous HTTP/WebSocket server embedded in the same binary. |
| **Front-End / God Console**| **HTML5 Canvas + Vanilla CSS** | Zero npm bloat, instant loading, renders 60fps canvas world with zero external GPU dependencies. |
| **Hardware Target** | **Ordinary PC / CPU-First** | Single or multi-core x86_64 CPU; consumes < 150MB RAM out-of-the-box. |

---

## 🚀 Quickstart Guide (Run Locally)

### Prerequisites
1. **Rust Toolchain**: Install Rust (1.80+ or 1.97+) via [rustup.rs](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   # On Windows: download and run rustup-init.exe
   ```
2. Verify installation:
   ```bash
   rustc --version
   cargo --version
   ```

### Clone & Build
```bash
git clone https://github.com/areeb/cogniverse-rpg.git
cd cogniverse-rpg
cargo build --release
```

### Running the Simulation
```bash
cargo run --release
```
Console output:
```
========================================================
     GENESIS COGNIVERSE: AUTONOMOUS AI RPG SIMULATION   
  CPU-First Darwin-Gödel AST & Dual Pantheon Engine     
========================================================
[*] Simulation server running at http://localhost:3000
[*] Open http://localhost:3000 in your browser to view the God Console.
```

Open **`http://localhost:3000`** in any web browser to enter the live WorldBox God Admin Console!

### Running the Test Suite
```bash
cargo test
```
Executes 6 unit & integration tests covering AST gas metering, Darwin-Gödel mutations, Kenji Doya neuromodulation, altar offerings, and covenant fulfillment.

---

## 📂 Project Architecture & File Hierarchy

```
cogniverse-rpg/
├── Cargo.toml                    # Project dependencies (Tokio, Axum, Rayon, Serde, Rand)
├── README.md                     # Comprehensive documentation & setup instructions
├── screenshots/
│   ├── worldbox_god_console.png  # Live WorldBox God Console screenshot
│   └── god_console_overview.png  # Legacy overview screenshot
├── src/
│   ├── lib.rs                    # Exported library root for modular architecture & tests
│   ├── main.rs                   # System entrypoint, CLI runner, background simulation ticker
│   ├── world/                    # Environment & Living Nature
│   │   ├── mod.rs                # 64x64 spatial grid coordinator and tick dispatcher
│   │   ├── tile.rs               # Biomes, elevation, moisture, temperatures, placed structures
│   │   ├── items.rs              # Item catalog, crafting recipe matrix, inventory system
│   │   ├── ecology.rs            # Gaia Engine: Thermodynamic solar cycle, flora logistic growth
│   │   ├── fauna.rs              # Adaptive wildlife AI (Predator/prey carrying capacity)
│   │   └── altar.rs              # Solar/Lunar Altars, sacrificial deposits, sincerity scoring
│   ├── brain/                    # Cognitive Engine
│   │   ├── mod.rs                # Cognitive pipeline coordinator
│   │   ├── ast.rs                # Fuel-metered Darwin-Gödel AST virtual machine
│   │   ├── neuromod.rs           # Kenji Doya Dopamine, Serotonin, Noradrenaline dynamics
│   │   ├── memory.rs             # Working, Episodic, Associative memory & Akashic Core
│   │   ├── reflection.rs         # Semantic reflection & Knowledge Graph synthesis
│   │   ├── world_model.rs        # Predictive state transitions ("Dreamer" RSSM)
│   │   ├── desire.rs             # Radical free-will motivation archetypes & inner monologue
│   │   ├── reproduction.rs       # Courtship, attraction compatibility & AST crossover
│   │   ├── magic.rs              # Arcane Code Weaving (Magic as compiled AST runes)
│   │   └── justice.rs            # Reputation tracking, taboos & costly third-party punishment
│   ├── factions/                 # Competing Races & Dual Pantheons
│   │   ├── mod.rs                # Civilization coordinator & diplomatic status
│   │   └── rival_god.rs          # "The Shadow Weaver" autonomous Rival AI Deity
│   └── server/                   # Pantheon God Console
│       ├── mod.rs                # Axum HTTP server & WebSocket real-time broadcaster
│       ├── state.rs              # Thread-safe simulation state (SimulationEngine)
│       ├── covenants.rs          # Divine quest creator, tracking & reward distribution
│       ├── god_powers.rs         # Divine interventions (Whisper, Miracle, Smite, Babel)
│       └── protocol.rs           # Real-time JSON telemetry packets & terraform protocol
├── static/                       # Embedded WorldBox God Panel Web Frontend
│   ├── index.html                # God dashboard, canvas map, agent inspector UI
│   ├── style.css                 # Sleek dark-mode glassmorphic styling & WorldBox God bar
│   └── app.js                    # High-performance 60fps canvas renderer & WS client
└── tests/
    └── simulation_tests.rs       # Complete unit & integration test suite
```

---

## 🎮 God Console Controls & Key Features

1. **Chronos Time Dilation**:
   * **Pause (0x)**: Freezes world ticks; allows deep inspection of frozen thoughts.
   * **1x (Real-time)**: 10 ticks/second (human-observable speed).
   * **10x & 100x**: Accelerated village growth and tool development.
   * **⚡ TURBO (1000x)**: Multi-threaded execution for multi-generational evolutionary jumps.
   * **STEP**: Advances the simulation by exactly 1 tick for behavioral debugging.
2. **Divine Interventions & Cataclysms**:
   * **Divine Whisper**: Telepathically inject high-salience thoughts into any chosen agent or all living souls.
   * **Manna Drop**: Spawns emergency berry and grain stockpiles at target coordinates.
   * **Wrathful Smite**: Calls down a lightning bolt to punish wrongdoers.
   * **Tower of Babel**: Scrambles emergent dialect trees to test linguistic re-adaptation.
   * **Genesis Spark**: Forces an immediate mutation epoch across all agents with elevated plasticity.
3. **Divine Covenant & Quest Forge**:
   * Issue commandments (deliver offerings, stockpile grain, forge iron tools, inscribe runes).
   * Set promised rewards (Rain of Manna, Fertility blessing, Promethean inspiration).
   * Observe whether NPCs zealously obey or heretically rebel!

---

## 📜 License

MIT License. Designed and built with research in Artificial Life, Multi-Agent Reinforcement Learning, and Neuro-Symbolic Self-Improving AI.
