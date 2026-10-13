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

![Settlement Anchoring & 84 Citizens Flourishing](screenshots/worldbox_roaming_population_boom.png)
*Live 2D Procedural Matrix (**128x128 Grand Continental Archipelago - 16,384 tiles**) running at **5x Sonic Speed** at **Tick 2,346 (Year 3, Month 6)**. Demonstrates complete resolution of riverbank clumping with thriving settlement anchoring: **👥 84 citizens, 👶 89 births / 💀 5 deaths, 🧬 Generation 15**. Terrans and Lunarians maintain fluid movement between their home altars, harvesting grounds, and freshwater rivers.*

![60 FPS Flourishing Population & Territorial Expansion](screenshots/worldbox_60fps_population_growth.png)
*Early demographic growth phase with expanding territorial borders (`☀️ Kingdom of Solaria` golden realm vs `🌙 Lunar Sovereignty` violet realm).*

![128x128 Grand Continental Realm](screenshots/worldbox_128x128_grand_view.png)
*Detailed bird's eye view with boundless deep ocean backdrop, dual winding rivers (Great Crystal River + Mountain Tributary), sandy coastal shelves, Ironcrag Mountain Spine, Whispering Redwood Forest, Ancient Leyline Ruins, and autonomous settlements.*

![Authentic WorldBox Character Sheet](screenshots/worldbox_character_sheet.png)
*Authentic WorldBox Character Sheet Modal featuring character portrait, noble titles (`Chieftain`, `HereticRebel`), kingdom affiliation, favorite star (⭐), core RPG stats matrix (`HP`, `ATK`, `DEF`, `SPD`, `INT`, `DIP`, `AGE`, `OFF`, `KILLS`), WorldBox Trait Badges (`✦ Deceitful`, `✦ Swift`, `✦ Strong`), tabbed cognition (Inner Monologue, Kenji Doya Neuromodulators, Darwin-Gödel AST, Akashic Lineage), inventory items, and God powers (`💬 Whisper`, `✨ Bless`, `💚 Heal`, `⚡ Smite`).*

---

## ⚡ 60 FPS Performance Optimization & Thriving Population Dynamics

### 1. Offscreen Terrain Buffer (Zero-Lag 60 FPS Rendering)
* **The Root Cause**: On a $128 \times 128$ map, iterating through all 16,384 tiles and executing individual Canvas 2D calls (`fillRect`, `arc`, `lineTo`) every tick created over **45,000 draw calls per frame**, causing extreme frontend stutter and dropped frames.
* **The Solution**:
  - Implemented an offscreen caching canvas buffer (`terrainCanvas` at $1792 \times 1792$ resolution) that draws base terrain once on startup.
  - The main viewport renders base terrain with **a single blit call** (`ctx.drawImage(terrainCanvas, 0, 0)`).
  - Subsequent terrain alterations (e.g., God brushes, wildfires) update the offscreen buffer incrementally via a differential `modified_tiles` stream.
  - Replaced synchronous canvas redrawing with a decoupled `requestAnimationFrame(gameLoop)` loop, ensuring silky-smooth 60 FPS panning, zooming, and animation regardless of server tick frequency.

### 2. Autonomous Nutrition & Thriving Population Growth
* **The Root Cause of Stagnant Population**:
  1. *Backpack Starvation*: Agents gathered berries and grain into their backpacks, but lacked an autonomous consumption instinct, starving to death while carrying full rations.
  2. *Riverbank Hydration*: Agents were only able to drink when standing directly on water tiles; moving adjacent to riverbanks failed the water check, causing dehydration.
  3. *Excessive Decay & High Courtship Barriers*: High metabolic hunger decay ($0.0018/\text{tick}$) kept hunger levels above the strict $0.50$ courtship threshold.
* **The Solution**:
  - **Auto-Consumption**: Implemented autonomous consumption in `tick_vitals`: any agent with $\text{hunger} > 0.45$ automatically eats berries or grain from their inventory.
  - **Bank Hydration Radius**: Allowed drinking when water or river is within $\le 2$ tiles, enabling safe hydration from riverbanks.
  - **Balanced Metabolic Decay**: Adjusted hunger decay to $0.0008/\text{tick}$ and thirst to $0.0010/\text{tick}$; reduced deprivation damage from instant death ($1.2/\text{tick}$) to gradual attrition ($0.25/\text{tick}$), with natural HP regeneration when nourished.
  - **Courtship Range & Population Cap**: Lowered the courtship attraction barrier to $0.45$, expanded mate discovery range to $6$ tiles, increased civilization carrying capacity cap to $160$ (from $40$), and provisioned newborns with starting food rations ($5$ berries, $2$ wood) for a strong survival headstart.

### 3. Settlement Anchoring, Roaming Neurons & Riverbank Dispersion
* **The Root Cause of Riverbank Congestion**:
  1. *Depleted Fallback Action*: Once an agent reached the riverbank to drink, their vitals were satisfied. The AST fell back to `ActionChoice::ChopWood`. Once the tile's initial wood was exhausted (`wood_amount == 0`), `ChopWood` had no effect and never moved the unit, causing them to freeze indefinitely on that tile.
  2. *Missing Resource Pathfinders*: In `find_nearest_target_coords`, queries for `StoneRock` and `IronVein` were unhandled (`_ => None`), halting movement whenever units sought building materials.
  3. *Absence of Home Gravity*: After drinking, agents had no instinct to return to their home village or altar.
* **The Solution**:
  - **Home Settlement Gravitation**: Pious and satisfied agents naturally gravitate toward their faction's sacred altar (`☀️ Sun Altar` at `(38, 86)` for Terrans vs `🌙 Moon Altar` at `(92, 34)` for Lunarians), creating an organic cycle between village centers and outlying resources.
  - **Post-Hydration Step**: Upon quenching thirst at the riverbank (`thirst < 0.25`), agents immediately take a step away from the water edge towards their settlement.
  - **WorldBox Roaming Neurons**: When rested and nourished, units randomly roam and explore instead of remaining static.
  - **Dynamic Resource Pathfinding**: If an agent attempts to chop wood, mine stone, or mine iron on an exhausted tile, they automatically pathfind to the nearest active resource deposit.
  - **Autonomous Shelter Construction**: Units with wood and stone now construct `StoneCabin` (`🏡`) and `Campfire` (`🔥`) structures, physically expanding their settlements.
  - **Empirical Proof**: Population expanded past **84 citizens** across **15 generations** with fluid cross-continental movement and zero riverbank congestion!
  - **Result**: Immediate, multi-generational population boom (**from 16 to 23+ citizens, 0 deaths, Gen 3** reached within 500 ticks at 5x speed).

---

## 🌍 WorldBox Features & Grand Continental Sandbox

### 1. 128x128 Grand Continental Archipelago (16,384 Tiles)
* **4x Land Expanse**: Upgraded from an initial 64x64 prototype to a massive **$128 \times 128$ (16,384 tiles)** fantasy continent.
* **Boundless Abyssal Ocean**: The world canvas renders an infinite deep ocean (`#061c33`) with animated wave shimmer lines, eliminating any void edges.
* **Multi-Tiered Procedural Biomes**:
  - **🌊 Deep Ocean Shelf**: Encircling oceanic borders.
  - **🏖️ Sandy Coastlines**: Warm golden beach rims framing bays and estuaries.
  - **🏞️ Dual River System**: The Great Crystal River traversing the continent from north to south, met by the rushing Mountain Tributary river flowing down from northeastern heights.
  - **⛰️ Ironcrag Mountain Spine**: Imposing grey peaks capped with snow facets, rich in iron veins and cavern passages.
  - **🌲 Whispering Redwood Forest**: Expansive ancient woodlands in the west teeming with game and timber.
  - **🔮 Ancient Leyline Ruins**: Obsidian and purple mystical monoliths in the heart of the realm radiating high mana density.
  - **🌾 Sunlit Plains**: Fertile meadow grasslands sprinkled with poppies and buttercups.
* **Geopolitically Balanced Kingdoms**:
  - `☀️ Kingdom of Solaria` settles in the southern fertile valley around the **Golden Sun Altar** at `(38, 86)`.
  - `🌙 Lunar Sovereignty` establishes their cavern dominion around the **Obsidian Moon Altar** at `(92, 34)`.
  - Kingdom territory borders are rendered with signature dashed borders and shaded influence areas.

### 2. Macro Demographics & Civilization Metrics Bar
Located at the very top of the God Console, providing real-time macro telemetry across the entire world:
* **POP**: Total living sentient population in the simulation.
* **FACTIONS**: Balance of power between `☀️ Solar Terrans` and `🌙 Chthonic Lunarians`.
* **FAUNA**: Autonomous wildlife carrying capacity (Deer, Dire Wolves, Mountain Bears, Rabbits).
* **BIRTH/DEATH**: Lifetime birth counter (👶) and death counter (💀).
* **MAX GEN**: Maximum evolutionary generation reached through genetic Darwin-Gödel mutation.
* **AVG VITALS**: Population-wide average health (❤️) and hunger (🍞) levels.
* **DEVOTION**: Average civilizational piety and sincere religious devotion (🙏).
* **WEALTH**: Total circulating economic goods, tools, and ores (🪙).
* **ENVIRONMENT**: Current thermodynamic day/night phase and dynamic weather conditions.

### 3. Interactive World Matrix with Smooth Pan & Zoom
* **Mouse-Wheel Zoom**: Seamlessly zoom in (up to 5.0x) and out (down to 0.18x) centered directly on the cursor.
* **Click-and-Drag Pan**: Click and drag with the middle mouse button, right mouse button, or while holding Spacebar to explore the 128x128 continent.
* **Camera Toolbar**: Floating canvas overlay with `➕ Zoom In`, `➖ Zoom Out`, `🎯 Reset View`, and `🔭 Follow Character` mode.
* **Find Leader Button**: Instantly locates the most senior elder or highest-piety civilizational leader and centers the camera on them.

### 4. Dynamic Terraforming Brushes with Variable Radius
Transform the physical world in real-time using God Brushes with `●1`, `●3`, `●5`, and `●9` radius sizing:
* **🌾 Plains Brush**: Fertilize lush grassland, planting wild grains and berries.
* **🌲 Forest Brush**: Seed dense whispering woodland with harvestable timber and game.
* **🌊 River Brush**: Carve freshwater rivers for drinking, irrigation, and hydration.
* **⛰️ Mountain Brush**: Raise rocky Ironcrag peaks rich in stone, flint, and iron ore veins.
* **🔮 Leyline Ruins**: Infuse ancient arcane leyline monoliths radiating high mana density.
* **⛏️ Enrich Iron**: Concentrate rich underground iron deposits into existing rock formations.
* **🧹 Clear Land**: Cleanse scorched terrain and clear obsolete structures.

### 5. WorldBox Character Sheet Inspector
Click any character or use the "Inspect" tool to open a detailed WorldBox character sheet:
* **⭐ Favorite Star Button**: Star your favorite agents to track them; enable camera follow to watch them live as they go about their daily life.
* **WorldBox Trait Badges**: Dynamic traits derived from behavior, heritage, and genetics (e.g. `👑 Leader`, `🌟 Genius`, `⚔️ Strong`, `✦ Swift`, `✦ Deceitful`, `🕊️ Pacifist`, `🔮 Wise`, `💰 Greedy`).
* **💭 Live Inner Monologue**: Unfiltered stream of consciousness reflecting their current needs, fears, and plans.
* **Vitals & RPG Stats**: Health, Energy, Hunger, Piety, Attack, Defense, Speed, and Intelligence.
* **Kenji Doya Neuromodulator Gauges**: Real-time biological cognition (Dopamine, Serotonin, Noradrenaline).
* **Inventory & Gear**: Visible items carried in the agent's pack (e.g. `Wild Berry x5`, `Wood x2`, `Iron Ore`).
* **Ancestral Lineage & Echo**: Heritage details from deceased legendary ancestors enshrined in the Akashic Core.
* **Darwin-Gödel AST Genome**: Full code tree inspection showing the exact self-rewritten logic steering the NPC.
* **Direct God Actions**: `💬 Whisper`, `✨ Bless Piety`, `💚 Heal`, and `⚡ Smite`.

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
