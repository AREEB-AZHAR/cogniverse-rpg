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

![Architecture Buildings & Mountain Ore Veins](screenshots/worldbox_buildings_and_ores.png)
*Live 2D Procedural Matrix (**128x128 Grand Continental Archipelago - 16,384 tiles**) showcasing defined, distinguishable pixel biomes, permanent village structures (**Stone Cabins with pitched roofs and chimneys, Thatched Cottages, Campfires, Granaries, Watchtowers, and Mountain Mine Shafts**), and discoverable mountain ore veins (**Glittering Auric Gold, Radiant Cyan Mithril, and Metallic Copper Iron**).*

![Knowledge & Activities Codex - Live Activity Stream](screenshots/codex_live_activities.png)
*Dedicated **🧠 Knowledge & Activities Codex Modal** featuring a real-time Live Activity Stream tracking agent coordinates, behavioral actions, inner monologue thoughts, and Kenji Doya emotional states (`Inspired`, `Content`, `Vigilant`, `Distressed`, `Calm`).*

![Civilizational Cultural Tech Tree](screenshots/codex_cultural_tech_tree.png)
*8-Tier Civilizational Technology Progression Tree (**Primitive Toolcraft, Fire Mastery & Hearth, Stone Masonry, Granary & Agriculture, Iron Metallurgy, Gold Currency & Commerce, Sacred Theocracy, Arcane Code Weaving**) driven by autonomous knowledge point accrual from mining and building.*

![Mountain Ores & Discoveries Lore](screenshots/codex_mountain_ores_lore.png)
*Mountain Ores & Discoveries Archive detailing geological properties, hardness ratings, and historical civilizational milestones recorded by exploring pioneers.*

![Settlement Anchoring & 84 Citizens Flourishing](screenshots/worldbox_roaming_population_boom.png)
*Live 2D Procedural Matrix running at **5x Sonic Speed** at **Tick 2,346 (Year 3, Month 6)**. Demonstrates complete resolution of riverbank clumping with thriving settlement anchoring: **👥 84 citizens, 👶 89 births / 💀 5 deaths, 🧬 Generation 15**.*

![60 FPS Flourishing Population & Territorial Expansion](screenshots/worldbox_60fps_population_growth.png)
*Early demographic growth phase with expanding territorial borders (`☀️ Kingdom of Solaria` golden realm vs `🌙 Lunar Sovereignty` violet realm).*

![128x128 Grand Continental Realm](screenshots/worldbox_128x128_grand_view.png)
*Detailed bird's eye view with boundless deep ocean backdrop, dual winding rivers (Great Crystal River + Mountain Tributary), sandy coastal shelves, Ironcrag Mountain Spine, Whispering Redwood Forest, Ancient Leyline Ruins, and autonomous settlements.*

![Authentic WorldBox Character Sheet](screenshots/worldbox_character_sheet.png)
*Authentic WorldBox Character Sheet Modal featuring character portrait, noble titles (`Chieftain`, `HereticRebel`), kingdom affiliation, favorite star (⭐), core RPG stats matrix (`HP`, `ATK`, `DEF`, `SPD`, `INT`, `DIP`, `AGE`, `OFF`, `KILLS`), WorldBox Trait Badges (`✦ Deceitful`, `✦ Swift`, `✦ Strong`), tabbed cognition (Inner Monologue, Kenji Doya Neuromodulators, Darwin-Gödel AST, Akashic Lineage), inventory items, and God powers (`💬 Whisper`, `✨ Bless`, `💚 Heal`, `⚡ Smite`).*

> [!TIP]
> ### 📸 User Screenshot Requests (Recent Updates)
> Please capture and add the following screenshots to the `screenshots/` directory to document the latest systems:
> 1. **Control + Click & Hold Panning & WorldBox Cursor** (`screenshots/worldbox_ctrl_click_panning.png`): Hold `Control` (or `Cmd`) on the Canvas map showing the dynamic `grab` / `grabbing` cursor while smoothly panning across the continent.
> 2. **Cellular Lava & Basalt Solidification** (`screenshots/worldbox_lava_basalt_dynamics.png`): Drop molten lava into shallow water or ocean using the `🌋 Lava` tool and capture the animated magma glow and cooling into dark basalt rock.
> 3. **Spreading Wildfire & Ash Decals** (`screenshots/worldbox_cellular_wildfire.png`): Ignite a forest using the `🔥 Wildfire` tool and capture the flame animations, smoke particles, and charred ash ground.
> 4. **Freeze Ice & Walkable Frozen Waters** (`screenshots/worldbox_freeze_ice_walking.png`): Freeze river or coastal tiles using the `🧊 Freeze Ice` tool showing characters walking across the solid ice sheets.
> 5. **Live Plots & Rebellions Codex Tab** (`screenshots/codex_plots_rebellions.png`): Open the Codex (`🧠 Codex & Activities`) and switch to the `⚔️ Live Plots & Rebellions` tab showing active civil plots, conspirators, treasuries, and progress bars.
> 6. **Monumental Town Hall & Barracks Architecture** (`screenshots/worldbox_townhall_barracks.png`): Place a Town Hall and Barracks using the `🏗️ Architecture` tool tray near a village center and capture their pixel-art rendering.
> 7. **World Archetypes & Generation Forge Modal** (`screenshots/worldbox_world_management_modal.png`): Open the `🌍 Worlds & Maps` modal showing the 7 archetype cards, seed randomizer, sea level & mountain ruggedness sliders, and dual divine Reset / Forge action buttons.

---

## 🔬 Reverse Engineering WorldBox & REA Toolchain Integration

We integrated the **Reverse Engineer Anything (REA)** skills framework (`git@github.com:morluto/rea.git`) into our workspace and global configuration, registering the REA MCP server for live autonomous inspection:
* **REA Skill Bundle**: Configured at `.agents/skills/reverse-engineer-anything/` and `~/.gemini/config/skills/reverse-engineer-anything/`.
* **REA MCP Server**: Registered in `~/.gemini/config/mcp_config.json` via `npx -y rea-agents@latest mcp`.
* **Binary Decompilation**: Used `ilspycmd` (v11.1.0 on .NET 10) on decompiled game runtime binaries (`Assembly-CSharp.dll`).

### WorldBox Architectural Systems Reverse-Engineered & Adapted

| WorldBox C# System | Decompiled Structure & Mechanics | Cogniverse Rust Engine Equivalent |
|---|---|---|
| **`TileLibrary` & `MapGenerator`** | 9-layer elevation hierarchy (`deep_ocean`, `close_ocean`, `shallow_waters`, `sand`, `soil_low`, `soil_high`, `hills`, `mountains`, `summit`) generated via multi-octave Perlin noise with radial distance falloff. | **`ElevationLayer` & `MapGenerator` (`src/world/tile.rs`, `src/world/generation.rs`)**: 9-tier elevation hierarchy with continental distance falloff mask, coherent noise, mountain spring pools, and non-potable outer ocean margins. |
| **`WorldBehaviourActionFire`** | Spreads across flammable materials (trees, grain, wood buildings) based on moisture and heat; water tiles extinguish immediately; leaves burned ground that decays to ash before regenerating fertility. | **`CellularAutomataEngine::tick_fire` (`src/world/cellular.rs`)**: Cellular wildfire engine with neighbor ignition, fuel consumption, structure fire damage, ash recovery stages (`burned_stages`), and animated flame particles in `static/app.js`. |
| **`WorldBehaviourActionLava`** | Molten lava flows downhill into lower adjacent elevations, melts frozen ice, destroys wood structures, and **solidifies into basalt stone (`TileLibrary.hills`) when touching water**. | **`CellularAutomataEngine::tick_lava` (`src/world/cellular.rs`)**: Molten lava cellular automata with downhill gravity flow, temperature spikes (95°C), and basalt rock/hills formation upon water contact. |
| **`WorldBehaviourTilesTemperatureFreeze`** | Temperature diffusion where subzero temperatures freeze shallow waters into walkable ice sheets, snow caps summits, and warm heat melts ice back into water. | **`CellularAutomataEngine::tick_freeze_thaw` (`src/world/cellular.rs`)**: Dynamic temperature model where shallow waters freeze into solid walkable ice (`FrozenState::Ice`), enabling entities to walk across bodies of water. |
| **`WorldBehaviourActionBiomes`** | Biomes compete and creep into neighboring tiles based on `grow_strength` ratings (`Randy.randomInt`). | **`CellularAutomataEngine::tick_biome_creep` (`src/world/cellular.rs`)**: Stochastic competitive biome creeping for Savanna, Lush Jungle, Corrupted Infernal, and Permafrost Tundra. |
| **`PlotsLibrary` / `PlotAsset`** | Conspirators gather ambition, initiate schemes (Rebellion, War, Alliance, Secession), tick progress toward a threshold, and trigger diplomatic shifts. | **`CivilPlot` & `PlotType` (`src/factions/plots.rs`)**: Emergent political plots (`Rebellion`, `HolyWar`, `CommercialAlliance`, `HeresySchism`, `GoldenAgePact`) driven by faction treasuries, mutual sentiment, and conspirator counts. |
| **`ActorTraitLibrary` / `ActorTrait`** | Traits possess `rate_birth`, `rate_inherit`, and stat modifiers (`Immortal`, `Regeneration`, `Strong`, `Fast`, `Wise`). | **`ReproductionManager::inherit_traits` (`src/brain/reproduction.rs`)**: 65% hereditary transmission from parents to newborns, plus rare spontaneous mutations (`Immortal`, `Regeneration`, `WiseScholar`, `EagleEyed`, `Pyromancer`, `Energetic`, `Frail`). |
| **`City` & `BuildingAsset`** | Settlement centers and military bastions hold population caps, provide defense, and anchor territory borders. | **`TownHall` & `Barracks` (`src/world/tile.rs`)**: High-tier architecture. Town Hall (HP: 350, Cap: 8) anchors civil governance; Barracks (HP: 220, Cap: 6) serves as faction garrison. Complete with custom pixel-art renderings in `static/app.js`. |
| **`GodPower` & `PowerAction`** | Divine interventions that alter world state, inspire citizens, or unleash devastation. | **`DivineInterventionKind` (`src/server/god_powers.rs`)**: Added `DivineInspiration` (+Dopamine & KP), `CurseOfDoubt` (+Noradrenaline & paranoia), `CataclysmEarthquake`, `SpawnLava`, `IgniteWildfire`, `FreezeIce`, and `ThawHeat`. |
| **`WorldGenerator` & `MapTemplate`** | Multi-template generator supporting island clusters, pangaea, atolls, with noise sliders and world reset commands. | **`MapGenerator` & `MapArchetype` (`src/world/generation.rs`, `src/server/protocol.rs`)**: 7 procedural map archetypes with custom seeds, sea level sliders, mountain ruggedness, dynamic settlement relocation, and 1-packet base64 WebSocket sync. |
| **`SaveManager` & `WorldStateSnapshot`** | Serialization of map tiles, units, traits, cities, and relations to persistent storage. | **`CivilizationRunSnapshot` (`src/server/protocol.rs`, `src/server/state.rs`)**: Complete JSON state export & import preserving 100% of agent brain ASTs, ancestral echoes, cultural tech progression trees, covenants, active plots, and historical milestones. |

---

## 💾 Full-Fidelity JSON State Export & Import Engine

Genesis Cogniverse features a complete, lossless **JSON State Serialization Pipeline**. Unlike traditional sims that only save high-level stats, Cogniverse exports the exact state of every single autonomous agent brain, ancestral memory, and civilizational achievement:

### What Is Preserved in the JSON Snapshot (`format_version: "1.0.0"`)
1. **World & Map Metadata**: Map ID, archetype, seed, elapsed tick count, elapsed calendar years, water ratio, and mountain ruggedness.
2. **Autonomous Agent Neural States**:
   - Every agent's fuel-metered Darwin-Gödel Abstract Syntax Tree (AST) decision genome (`mutations_count`, `generation`, `last_action_gas`).
   - Ancestral Akashic memories & awakened echoes.
   - Kenji Doya three-factor neuromodulation states (Dopamine, Serotonin, Noradrenaline).
   - Core RPG attributes (HP, ATK, DEF, SPD, INT, DIP, Age, Kills, Piety) and inherited WorldBox traits (`Immortal`, `Regeneration`, `WiseScholar`, etc.).
   - Inventory items, equipped tools, and metabolic vital levels.
3. **Civilizational Tech & Cultural Trees**:
   - Complete 8-tier technology tree state for both Solaria and Lunaria (`Primitive Toolcraft` through `Arcane Code Weaving`).
   - Unlocked vs locked research nodes and accumulated Knowledge Points (KP).
   - Historical discoveries archive recorded by exploring pioneers.
   - Active emergent civil plots, conspirators, progress percentages, and kingdom treasuries.
4. **Pantheon Covenants & Altars**:
   - Active divine covenants, required sacrifices, target thresholds, and promised miracles.
   - Altar locations, consecrated offerings, and citizen sincerity ratings.
5. **Multi-World Evolutionary Milestones**:
   - Cumulative discoveries, peak AST generations, longest observed lifespans, and historical world logs.

### Usage
* **Exporting**: Click `💾 Export` in the Top HUD or from the Game Start Panel. The engine packages the world state and triggers an immediate browser download of `cogniverse_run_<realm>_tick<N>.json`. A snapshot is also archived locally in `localStorage` for cross-world comparison.
* **Importing**: Click `📂 Import` in the Top HUD or the Game Start Panel. Select any `.json` snapshot file. The engine validates the snapshot schema, broadcasts `ClientMessage::ImportState`, resets the simulation grid, and restores all agents and knowledge into the live realm.

---

## 🚀 Game Start Panel & World Hub (`#modal-game-start`)

Clicking the `🚀 Game Hub` button in the Top HUD opens the centralized **World Hub** modal:
* **Continue Active Realm**: Seamlessly resumes the living world in motion. Shows live status (e.g., *Year 2 | 24 Citizens | Peaceful*).
* **Forge New World**: Direct gateway to the Procedural Map Generator to select one of 7 map archetypes, configure seeds, or reset the realm.
* **Import Chronicle (JSON)**: File picker to load external or shared world files.
* **Quick Tools Strip**: Instant shortcuts for exporting world JSON, launching learning analytics, switching aesthetic themes, and toggling GPU tiers.
* **Saved Chronicles & Runs Archive**: Visual cards of past worlds and imported runs displaying years lived, citizen count, and accumulated knowledge points with instant **Load Run** and **Delete** actions.

---

## 📊 Multi-World Learning Analytics & Multi-Style Canvas Graphs (`#modal-learning-stats`)

The **Multi-World Learning & Evolution Analytics** modal provides comprehensive cross-world synthesis comparing how synthetic societies learn, adapt, and survive across different procedural worlds and elapsed years:

### 4 Dynamic Canvas Graph Styles
1. **📈 Timeline Curves (Area / Line Chart)**:
   - High-DPI canvas graph displaying multi-series temporal curves across elapsed calendar years.
   - Selectable metric focus: *Total Knowledge Points (KP)*, *Technologies Mastered*, *AST Generational Depth*, or *Historical Discoveries*.
   - Features glowing curve paths, translucent area fills under active curves, and dashed lines for archived runs.
2. **🕸️ Cognitive Radar (6-Axis Spider Chart)**:
   - 6-axis competency web assessing civilizational capabilities across:
     - 🌾 **Agronomy**: Wild berry foraging & grain cultivation.
     - ⛏️ **Metallurgy**: Iron and gold ore mining & extraction.
     - 🛠️ **Tool Crafting**: Tool making and inventory equipment.
     - 🏛️ **Architecture**: Stone cabins, cottages, and civic buildings erected.
     - 🛐 **Theology & Piety**: Altar offerings and covenant fidelity.
     - 🗺️ **Exploration**: Continental navigation & boundary expansion.
   - Shows active world polygon in glowing Gold (`#f59e0b`) overlaid against archived worlds in Emerald (`#10b981`).
3. **📊 Comparative Bars (Grouped Bar Chart)**:
   - Side-by-side comparative bars contrasting active world vs archived worlds across Knowledge (KP / 10), Technologies Unlocked, Max Generation, Lifespan (Years), and Altar Offerings.
4. **🍩 Domain Donut (Concentric Knowledge Ring)**:
   - Polar donut chart visualizing civilizational knowledge distribution across functional domains: *Agronomy (30%)*, *Metallurgy (25%)*, *Arcane Theology (20%)*, *Architecture (15%)*, and *Social & Diplomacy (10%)*.

### Cross-World Progression Ledger
A live tabular ledger displaying side-by-side statistics for all active and archived worlds: Realm Name, Archetype & Seed, Years Lived, Population, Knowledge (KP), Tech Mastered, Altars Consecrated, Diplomatic Stance, and Action buttons.

---

## 🎨 Obsidian & Mythic Gold Aesthetic (Non-Dark-Blue Theme Overhaul)

The user interface has been completely redesigned to move away from midnight dark blue in favor of an **Obsidian Charcoal & Mythic Burnished Gold** design system:
* **Obsidian Charcoal Foundation**: Primary backgrounds utilize velvety charcoal (`#121316`, `#181920`, `#0a0b0d`), eliminating saturated blues.
* **Mythic Gold & Amber Accents**: Borders, buttons, and HUD badges glow with burnished gold (`#f59e0b`, `#fbbf24`) and warm amber (`#d97706`).
* **Abyssal Slate Ocean Rendering**: Deep ocean tiles render in deep obsidian slate (`#111317` with subtle `#1e2129` ripples) with outer wave lines tuned to the active palette.
* **4 Curated Theme Palettes**:
  1. 🎨 **Obsidian Gold** (Default): Velvety charcoal and warm burnished gold.
  2. 🎨 **Alchemical Amber**: Rich umber charcoal, antique parchment, and radiant amber.
  3. 🎨 **Emerald Sanctum**: Deep jade forest charcoal with glowing emerald accents.
  4. 🎨 **Crimson Basalt**: Basalt dark charcoal with molten ruby highlights.

---

## ⚡ GPU Acceleration & Computational Power Tiers (Low / Medium / Ultra)

To ensure smooth performance across all machines, from high-end gaming rigs to battery-constrained laptops, Cogniverse features a three-tier GPU scaling system:

| GPU Tier | Target FPS | Rendering Pipeline & Post-Processing Effects | Target Hardware |
|---|---|---|---|
| **⚡ Ultra** | 60–120 FPS | Full hardware-accelerated WebGL / Canvas compositing, dynamic altar & monarch aura bloom shaders, water caustics shimmer on coastlines, and smooth subpixel camera interpolation. | Discrete GPUs (NVIDIA RTX, AMD Radeon, Apple Silicon) |
| **⚡ Medium** | 60 FPS | Hardware-accelerated 2D canvas with offscreen terrain buffer (`terrainCanvas`), layered dirty-rect rendering, and ambient lighting passes. | Standard Desktops & Modern Integrated GPUs (Intel Iris, AMD Vega) |
| **⚡ Low** | 30 FPS Cap | Battery-saver mode with frame-rate throttling, disabled particle systems, minimal compositing, and reduced draw calls. | Laptops on Battery, Older PCs & Ultra-Portable Devices |

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

## 🧭 Root Cause Analysis: Why Chthonic Lunarians Walked to (0, 0) & Permanent Solution

### 1. The Investigation
When observing the simulation over extended ticks, several Chthonic Lunarians (`🌙`) spawned at the northeastern mountain stronghold `(92, 34)` would eventually migrate across the map and clump at coordinate `(0, 0)` in the top-left ocean corner.

Through code and telemetry tracing, we discovered **three compounding root causes**:
1. **Border Ocean Misclassification as Potable Riverlands**:
   In `src/world/mod.rs` (legacy generator), the ocean margin check classified `x < 8 || y < 8` as `BiomeType::CrystalRiverlands` with `has_fresh_water = true` and `berry_amount = 8`.
2. **Resource Starvation in Northeastern Mountains**:
   The mountain terrain around `(92, 34)` initially lacked dedicated food sources and fresh water springs. As Lunarian vitals dropped, their Darwin-Gödel AST evaluated `ActionChoice::DrinkWater` and `ActionChoice::ForageFood`. Looking across the continent for the nearest food and water, the sensor queries detected the northern border edge (`y <= 7`) as the closest fresh water. Agents walked north to the edge, followed the river/coastline west along the top border (`y = 0..7`), and accumulated at `(0, 0)`.
3. **Unbounded Idle Roaming**:
   When agents entered `ActionChoice::IdleRest`, their roaming logic generated step offsets without anchoring to their home settlement or kingdom altar, drifting further northwest into the boundary corner.

### 2. The Architectural Solution
1. **Multi-Octave Coherent Fractal Continental Generator (`src/world/generation.rs`)**:
   - Implemented a coherent noise elevation model with a radial distance falloff mask.
   - Encircling oceanic borders are strictly assigned to `ElevationLayer::DeepOcean` and `ElevationLayer::CloseOcean` with `has_fresh_water = false` and `berry_amount = 0`. Agents never seek or drink non-potable ocean water.
2. **Mountain Springs & Cave Berry Reserves at Lunar Bastion `(92, 34)`**:
   - Carved natural mountain spring pools (`has_fresh_water = true`) and rich cave berry bushes directly around the Obsidian Moon Altar at `(92, 34)`.
   - Updated `src/world/ecology.rs` so flora regrowth replenishes mountain berry bushes and spring pools while ignoring deep ocean tiles.
3. **Territorial Settlement Gravitation & Roaming Anchors (`src/server/state.rs`)**:
   - Anchored roaming, drinking, and foraging to the home settlement coordinates:
     - `☀️ Solar Terrans` anchor to `(38, 86)` in the southern fertile valley.
     - `🌙 Chthonic Lunarians` anchor to `(92, 34)` in the northeastern mountain caverns.
   - Verified with automated test `test_lunar_people_do_not_wander_to_origin`: 100% of Lunarians remain in their mountain dominion across hundreds of ticks without a single unit wandering to `(0, 0)`.

---

## 🎮 Smooth Camera Controls & Control + Click & Hold Panning

Navigating the grand $128 \times 128$ continent is now buttery-smooth and responsive with signature RTS/God simulator controls:

* **Control + Click & Hold Pan**: Hold `Control` (or `Cmd` on macOS) and left-click drag anywhere on the canvas to smoothly pan the camera.
* **Dynamic Cursor Feedback**:
  - Pressing `Ctrl`, `Cmd`, or `Spacebar` immediately transitions the mouse cursor to a `grab` hand icon (`🤚`).
  - Clicking and dragging transitions the cursor to `grabbing` (`✊`) with real-time viewport translation.
  - Releasing the key or mouse button smoothly reverts the cursor.
* **Multi-Button Pan Support**:
  - `Ctrl + Left Click & Drag` / `Cmd + Left Click & Drag`
  - `Spacebar + Left Click & Drag`
  - `Middle Mouse Button (Scrollwheel) Click & Drag`
  - `Right Click & Drag`
* **Zoom Matrix**: Smooth exponential zoom focused on mouse cursor via mouse scrollwheel, or quick +/- buttons on the HUD.

---

## 🗺️ WorldBox Cellular Automata Engine & Map Dynamics

Reverse-engineered from WorldBox runtime assemblies (`TileLibrary`, `WorldBehaviourActionBiomes`, `WorldBehaviourActionFire`, `WorldBehaviourActionLava`, `WorldBehaviourTilesTemperatureFreeze`), Cogniverse now features an authentic cellular automata simulation engine (`src/world/cellular.rs`):

### 1. 9-Layer Elevation Hierarchy (`ElevationLayer`)
1. `DeepOcean`: Deep abyssal ocean; impassable without seafaring vessels.
2. `CloseOcean`: Coastal oceanic shelf.
3. `ShallowWaters`: Wading water; flash-freezes into solid walkable ice sheets under freezing temperatures.
4. `Sand`: Coastal beaches and dunes.
5. `SoilLow`: Lowland fertile soil suitable for agriculture and forestry.
6. `SoilHigh`: Elevated plains and plateaus.
7. `Hills`: Rugged rocky foothills; cools flowing lava into solid basalt rock.
8. `Mountains`: Towering granite peaks rich in iron, gold, and mithril ores.
9. `Summit`: High altitude snow-capped mountain peaks permanently frozen with snowdrifts.

### 2. Molten Lava & Basalt Solidification
* **Downhill Gravity Flow**: Molten lava (`lava_level: 1..3`) flows downhill from higher elevations to lower adjacent tiles.
* **Heat Radiation**: Radiates intense heat (temperature spikes to 95°C), vaporizing snow and ice.
* **Basalt Solidification on Water Contact**: When lava contacts water tiles (`DeepOcean`, `CloseOcean`, `ShallowWaters`), the water immediately quenches the lava, creating permanent dark basalt rock and foothills (`ElevationLayer::Hills`, `+stone_amount`, `+flint_amount`).

### 3. Spreading Wildfire & Ash Decal Recovery
* **Fuel & Moisture Ignition**: Wildfire ignites flammable vegetation (forests, plains grass, granaries, wooden cottages) based on tile moisture levels.
* **Decay to Ash**: Once wood/fuel is consumed, the fire decays into charred blackened ash ground (`burned_stages: 4`, `soil_fertility: 0.2`).
* **Ecological Regeneration**: Over subsequent cycles, ash gradually breaks down, enriching soil fertility back to healthy levels.

### 4. Subzero Freezing & Walkable Ice Sheets
* **Walkable Ice Formation**: When cold weather or the Divine Freeze power lowers temperature below -2°C, shallow waters freeze into solid ice sheets (`FrozenState::Ice`).
* **Solid Ice Navigation**: Units can freely walk across frozen ice sheets (`is_walkable() = true`), opening temporary winter pathways across river valleys!
* **Thawing Cycle**: Warm sunlight thaws ice back into flowing freshwater.

### 5. Competitive Biome Creeping
Biomes dynamically compete and expand into adjacent uncolonized soil based on their species `grow_strength`:
* **Savanna Grasslands** (`grow_strength: 3`): Warm amber grasses and acacia shrubs.
* **Verdant Jungle** (`grow_strength: 4`): Dense tropical canopies with vibrant orchids.
* **Corrupted Infernal Biomass** (`grow_strength: 5`): High-virulence alien purple creep with pulsing eyes and spores.
* **Permafrost Tundra** (`grow_strength: 3`): Glacial subzero lichen and frost crystal sparkles.

### 6. Signature God Powers in Bottom Bar
* **World & Nature**: Plains, Forest, River, Ocean, Mountain, Leyline Ruins, Iron/Gold/Mithril Ore Veins, Savanna Seed, Jungle Seed, Corrupted Seed, Permafrost Seed, Sponge Cleanse.
* **Architecture**: Stone Cabin, Wood Cottage, Town Hall, Barracks, Campfire Hearth, Granary Silo, Watchtower, Mine Shaft.
* **Cataclysms & Miracles**: Molten Lava, Spreading Wildfire, Freeze Ice, Thaw Heat, Divine Inspiration, Wrathful Lightning, Tectonic Earthquake, Curse of Doubt, Golden Peace, Meteor Strike.

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
* **🪙 Enrich Gold**: Seed glittering auric gold veins into mountain peaks.
* **💎 Enrich Mithril**: Infuse rare glowing cyan/purple mithril crystal veins.
* **🧹 Clear Land**: Cleanse scorched terrain and clear obsolete structures.

### 5. Dedicated Architecture God Tools & Living Housing System
A dedicated **"🏗️ Architecture"** God category in the bottom power dock grants full creative control over civilizational construction, while autonomous agents naturally erect and inhabit shelters:
* **🏡 Stone Cabin**: Durable masonry residence housing up to **4 citizens**. Resting inside provides $+0.40$ warmth, $+0.15$ energy/tick, and natural health regeneration ($+2.0$ HP/tick).
* **🛖 Wood Cottage**: Cozy timber-log cottage with thatched straw roof housing **2 citizens**. Provides $+0.35$ warmth and $+0.12$ energy/tick.
* **🔥 Campfire Hearth**: Communal stone fire pit providing $+0.45$ warmth and thermal safety during nighttime freezes and blizzards.
* **🌾 Granary Silo**: Elevated wooden grain silo on stilts that stockpiles food rations for entire villages to survive long winters.
* **🗼 Guard Watchtower**: Fortified observation tower with high timber battlements and a royal banner overlooking kingdom borders.
* **⛏️ Mountain Mine Shaft**: Timbered underground cavern entrance enabling deep mineral extraction from the Ironcrag mountains.

### 6. Mountain Ore Discovery & Civilizational Tech Progression
* **Visible, Sparkling Ore Deposits**:
  - **⛏️ Ironcrag Iron Ore**: Metallic copper/rust ore flecks mined to forge pickaxes and weapons ($+15$ Knowledge Points).
  - **🪙 Auric Gold Veins**: Glittering yellow and amber gold nuggets mined to mint kingdom currency ($+25$ Knowledge Points).
  - **💎 Leyline Mithril Crystals**: Rare radiant cyan & violet crystalline veins resonating with cosmic mana ($+35$ Knowledge Points).
* **Episodic Discovery Logging**: Whenever agents explore mountains and unearth an ore vein, they record an episodic memory milestone and award civilizational Knowledge Points (KP).

### 7. 🧠 Civilizational Knowledge & Activity Codex
Accessible via the dedicated top HUD button **`🧠 Codex & Activities`**, opening a tabbed codex window:
* **📡 Live Activity Stream**: Real-time ticker showing every citizen's current action, inner thoughts, grid coordinates, and emotional state (`Inspired`, `Content`, `Vigilant`, `Distressed`, `Calm`). Filterable by **All**, **☀️ Solar Terrans**, or **🌙 Lunarians**.
* **🏛️ Cultural Technology Tree**: Displays 8 unlockable civilizational breakthroughs with progress bars and Knowledge Point requirements:
  1. *Primitive Toolcraft* (Tier 1, 25 KP): $+50\%$ wood and foraging speed.
  2. *Fire Mastery & Hearth* (Tier 1, 50 KP): Enables Campfire construction and thermal protection.
  3. *Stone Masonry* (Tier 2, 100 KP): Enables permanent Stone Cabins ($+4$ population capacity).
  4. *Granary & Agriculture* (Tier 2, 180 KP): Granary silos preserve food across blizzards.
  5. *Iron Metallurgy* (Tier 3, 280 KP): Unlocks forged iron pickaxes and tools.
  6. *Gold Currency & Commerce* (Tier 3, 400 KP): Minting gold coins enables market trading and dowries.
  7. *Sacred Theocracy* (Tier 4, 550 KP): $+100\%$ divine blessing potency from altar offerings.
  8. *Arcane Code Weaving* (Tier 4, 750 KP): Infusing mithril crystals allows agents to autonomously rewrite their decision code AST.
* **💎 Mountain Ores & Lore Archive**: Geological compendium detailing ore hardness and recording civilizational breakthroughs.

### 8. WorldBox Character Sheet Inspector
Click any character or use the "Inspect" tool to open a detailed WorldBox character sheet:
* **⭐ Favorite Star Button**: Star your favorite agents to track them; enable camera follow to watch them live as they go about their daily life.
* **WorldBox Trait Badges**: Dynamic traits derived from behavior, heritage, and genetics (e.g. `👑 Leader`, `🌟 Genius`, `⚔️ Strong`, `✦ Swift`, `✦ Deceitful`, `🕊️ Pacifist`, `🔮 Wise`, `💰 Greedy`).
* **💭 Live Inner Monologue**: Unfiltered stream of consciousness reflecting their current needs, fears, and plans.
* **Vitals & RPG Stats**: Health, Energy, Hunger, Piety, Attack, Defense, Speed, and Intelligence.
* **Kenji Doya Neuromodulator Gauges**: Real-time biological cognition (Dopamine, Serotonin, Noradrenaline).
* **Inventory & Gear**: Visible items carried in the agent's pack (e.g. `Wild Berry x5`, `Wood x2`, `Iron Ore`, `Gold Ore`, `Mithril Ore`).
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
| **Core Language** | **Rust (Edition 2021 / 1.80+)** | Zero-cost abstractions, memory safety, thread safety without garbage collection pauses. |
| **Concurrency** | **Tokio & Rayon** | Multi-threaded simulation ticks saturating available CPU cores; asynchronous WebSocket dispatch. |
| **Web Server** | **Axum 0.8** | Ultra-lightweight asynchronous HTTP/WebSocket server embedded in the same binary. |
| **Front-End / God Console**| **HTML5 Canvas + Vanilla CSS** | Zero npm bloat, instant loading, renders 60fps canvas world with zero external GPU dependencies. |
| **Reverse Engineering** | **REA Skills & ilspycmd** | Reverse Engineer Anything skill ecosystem + .NET IL decompilation of Unity Mono engines. |
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
3. *(Optional - Reverse Engineering Subsystem)*:
   - **.NET SDK** (v8.0+ or v10.0+): Required if using `ilspycmd` for decompiling Unity Mono `Assembly-CSharp.dll`.
     ```bash
     dotnet tool install --global ilspycmd
     ```
   - **Node.js & npm** (v18+): Required if executing the REA agents MCP server (`npx -y rea-agents@latest mcp`).

### Clone & Build
```bash
git clone https://github.com/your-username/cogniverse-rpg.git
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
Executes **12 comprehensive unit & integration tests** covering:
- AST gas metering and sandbox recursion prevention
- Darwin-Gödel AST semantic mutations
- Kenji Doya neuromodulation kinetics
- Altar offerings and sincerity scoring
- Covenant fulfillment and divine reward disbursement
- Multi-generational population dispersion and auto-nutrition
- Architecture placement (Town Halls, Barracks, Stone Cabins)
- Mountain ore vein enrichment and discoverability
- Civilizational cultural technology progression
- Hereditary trait inheritance (65% Mendelian rate) & rare mutations
- Emergent civil plot ticking, progression, and hatching
- Divine interventions & cataclysms (`DivineInspiration`, `CurseOfDoubt`, `Earthquake`, `GoldenAgePeace`)

---

## 📂 Project Architecture & File Hierarchy

```
cogniverse-rpg/
├── .agents/
│   └── skills/
│       └── reverse-engineer-anything/ # Local REA reverse engineering agent skills
├── Cargo.toml                         # Project dependencies (Tokio, Axum, Rayon, Serde, Rand)
├── README.md                          # Comprehensive documentation & setup instructions
├── screenshots/                       # High-resolution simulation screenshots
│   ├── worldbox_buildings_and_ores.png
│   ├── codex_live_activities.png
│   ├── codex_cultural_tech_tree.png
│   ├── codex_mountain_ores_lore.png
│   ├── worldbox_roaming_population_boom.png
│   ├── worldbox_60fps_population_growth.png
│   ├── worldbox_128x128_grand_view.png
│   └── worldbox_character_sheet.png
├── src/
│   ├── lib.rs                         # Exported library root for modular architecture & tests
│   ├── main.rs                        # System entrypoint, CLI runner, background simulation ticker
│   ├── world/                         # Environment & Living Nature
│   │   ├── mod.rs                     # 128x128 spatial grid coordinator and tick dispatcher
│   │   ├── tile.rs                    # Biomes, elevation layers, structures (TownHall, Barracks, etc.)
│   │   ├── generation.rs              # 7 procedural map archetypes with custom seeds & noise tuning
│   │   ├── cellular.rs                # WorldBox cellular automata (Lava, Wildfire, Freeze/Thaw, Biome Creep)
│   │   ├── items.rs                   # Item catalog, crafting recipe matrix, inventory system
│   │   ├── ecology.rs                 # Gaia Engine: Thermodynamic solar cycle, flora logistic growth
│   │   ├── fauna.rs                   # Adaptive wildlife AI (Predator/prey carrying capacity)
│   │   └── altar.rs                   # Solar/Lunar Altars, sacrificial deposits, sincerity scoring
│   ├── brain/                         # Cognitive Engine
│   │   ├── mod.rs                     # Cognitive pipeline coordinator
│   │   ├── ast.rs                     # Fuel-metered Darwin-Gödel AST virtual machine
│   │   ├── neuromod.rs                # Kenji Doya Dopamine, Serotonin, Noradrenaline dynamics
│   │   ├── memory.rs                  # Working, Episodic, Associative memory & Akashic Core
│   │   ├── reflection.rs              # Semantic reflection & Knowledge Graph synthesis
│   │   ├── world_model.rs             # Predictive state transitions ("Dreamer" RSSM)
│   │   ├── desire.rs                  # Radical free-will motivation archetypes & inner monologue
│   │   ├── reproduction.rs            # Courtship, attraction, Mendelian trait inheritance & mutation
│   │   ├── magic.rs                   # Arcane Code Weaving (Magic as compiled AST runes)
│   │   └── justice.rs                 # Reputation tracking, taboos & costly third-party punishment
│   ├── factions/                      # Competing Races & Dual Pantheons
│   │   ├── mod.rs                     # Civilization coordinator, treasuries & diplomatic status
│   │   ├── plots.rs                   # Civil plots (Rebellions, Holy Wars, Commercial Alliances)
│   │   └── rival_god.rs               # "The Shadow Weaver" autonomous Rival AI Deity
│   └── server/                        # Pantheon God Console
│       ├── mod.rs                     # Axum HTTP server & WebSocket real-time broadcaster
│       ├── state.rs                   # Thread-safe simulation state (SimulationEngine)
│       ├── covenants.rs               # Divine quest creator, tracking & reward distribution
│       ├── god_powers.rs              # Divine interventions (Whisper, Inspiration, Curse, Earthquake, Peace)
│       └── protocol.rs                # Real-time JSON telemetry packets & terraform protocol
├── static/                            # Embedded WorldBox God Panel Web Frontend
│   ├── index.html                     # God dashboard, canvas map, world management modal, codex modal
│   ├── style.css                      # Sleek dark-mode glassmorphic styling, archetype cards, progress bars
│   └── app.js                         # High-performance 60fps canvas renderer, pixel-art structures, WS client
└── tests/
    └── simulation_tests.rs            # 19-case comprehensive unit & integration test suite
```

---

## 🌍 Procedural World Archetypes & Divine Map Forge

The God Player now has sovereign control to reset the living realm or forge entirely new procedural realities on the fly. Accessed via the **`🌍 Worlds & Maps`** button in the Top HUD or the **`🔄 Reset/Map`** tool in the Nature dock tray.

### The 7 WorldBox-Inspired Map Archetypes

| Archetype | Geological & Biome Profile | Strategic Geopolitical Dynamic |
|---|---|---|
| **🌍 Grand Archipelago** *(Default Genesis)* | Elliptical continental landmass encircled by deep ocean, split by the Great Crystal River, eastern tributaries, and the Ironcrag mountain spine. | Balanced initial proving ground with fertile plains, lush redwoods, and mountain caves. |
| **🗺️ Pangaea Supercontinent** | Colossal monolithic supercontinent spanning 75% of the world grid with inland freshwater seas, sprawling forest heartlands, and dense central peaks. | High border tension, massive territorial empires, and land trade routes. |
| **🏝️ Scattered Islands** | Cellular maritime archipelago featuring 15–20 distinct tropical, forested, and volcanic isles separated by azure ocean channels. | Fragmented tribal territories, island resource competition, and naval frontiers. |
| **⭕ Sacred Ring Atoll** | Giant circular volcanic atoll rim surrounding a sheltered turquoise lagoon with an ancient arcane sanctuary at its heart. | Natural caldera fortress with circular migration corridors and sacred central leyline altar. |
| **⚖️ Twin Continents** | Two colossal rival continental plates separated by a Grand Central Ocean Rift running north to south. | Hemispheric rivalry: Western realm of the Solar Terrans vs. Eastern fortress of the Chthonic Lunarians. |
| **❄️ Glacial Wastes** | Subzero northern permafrost continent dominated by frozen bays, glaciers, blizzards, and cold pine taiga. | Extreme survival stakes: requires fires, insulated stone cabins, and hearth management. |
| **🌋 Volcanic Caldera** | Scorched infernal wasteland with basalt ridges, magma cauldrons, obsidian spires, savanna deserts, and ashlands. | Dangerous environment with active lava flows and rich mineral/mithril pockets. |

### Procedural Generator Controls

1. **Custom Seed System**:
   * Deterministic seed input allows replaying or sharing favorite world configurations.
   * **`🎲 Random` Button**: Instantly rolls fresh numeric seeds for unexpected landmass geometry.
2. **Sea Level & Ocean Ratio Slider**:
   * Modulates continental shelf emergence:
     - *Dry Basin ($0\% - 35\%$)*: Ocean recedes, exposing massive land bridges and fertile seabed valleys.
     - *Standard ($36\% - 65\%$)*: Classical coastal balance with pristine beaches and river deltas.
     - *High Seas ($66\% - 100\%$)*: Ocean floods inland, turning continents into archipelagos and fjord networks.
3. **Mountain Ruggedness Slider**:
   * Modulates tectonic uplift and mineral density:
     - *Rolling Hills ($0\% - 35\%$)*: Gentle slopes and vast arable plains.
     - *Moderate Crags ($36\% - 65\%$)*: Realistic alpine chains with iron and gold veins.
     - *Extreme Peaks ($66\% - 100\%$)*: Sky-piercing jagged peaks and deep mountain caverns.

### Divine Actions: Reset vs. Forge

* **🔄 Reset Active World**:
  - Preserves current terrain geometry and seed.
  - Cleanses the realm of old structures, resets simulation ticks and demography back to Year 1.
  - Re-seeds starting pioneer cohorts at the Golden Sun and Obsidian Moon Altars.
* **✨ Forge New Map Realm**:
  - Synthesizes fresh multi-octave harmonic noise from the chosen archetype and seed.
  - Automatically identifies optimal, habitable settlement coordinates for both factions (plains near fresh water for Terrans; mineral-rich foothill caves for Lunarians).
  - Transmits a compact $16,\!384$-byte base64 tile payload over WebSocket, instantly updating the 60 FPS offscreen terrain canvas without page reloads.

---

## 🎮 God Console Controls & Key Features

1. **World Management & Map Forge**:
   * **`🌍 Worlds & Maps`**: Opens the Realm Forge modal to select archetypes, randomize seeds, adjust ocean/mountain sliders, and reset or generate worlds.
   * **Title Click**: Click the world title badge (`World of Genesis ✎`) in the top HUD to quickly open the World Manager.
2. **Chronos Time Dilation**:
   * **Pause (0x)**: Freezes world ticks; allows deep inspection of frozen thoughts.
   * **1x (Real-time)**: 10 ticks/second (human-observable speed).
   * **2x & 5x**: Accelerated village growth and tool development.
   * **⚡ SONIC (1000x)**: Multi-threaded execution for multi-generational evolutionary jumps.
   * **STEP**: Advances the simulation by exactly 1 tick for behavioral debugging.
3. **Divine Interventions & Cataclysms**:
   * **Divine Whisper**: Telepathically inject high-salience thoughts into any chosen agent or all living souls.
   * **Divine Inspiration**: Flood an agent with dopamine ($+0.35$) and award civilizational Knowledge Points.
   * **Curse of Doubt**: Infuse paranoia and distrust, spiking noradrenaline and dampening piety.
   * **Cataclysmic Earthquake**: Strike target coordinates with a seismic rupture, shattering structures within 3 tiles.
   * **Golden Age of Peace**: Divine amnesty declaring universal peace, settling active civil plots and replenishing treasuries.
   * **Manna Drop**: Spawns emergency berry and grain stockpiles at target coordinates.
   * **Wrathful Smite**: Calls down a lightning bolt to punish wrongdoers.
   * **Tower of Babel**: Scrambles emergent dialect trees to test linguistic re-adaptation.
   * **Genesis Spark**: Forces an immediate mutation epoch across all agents with elevated plasticity.
4. **Architecture Placement Tools**:
   * **Town Hall**: Deploy administrative civic centers housing up to 8 citizens.
   * **Barracks**: Deploy military garrison forts housing up to 6 warriors.
   * **Stone Cabin & Cottage**: Erect peasant and noble residences.
   * **Watchtower & Granary**: Establish border fortifications and survival silos.
6. **🚀 Cogniverse Realm Hub & 1-Click World Creation**:
   * **Instant World Forging**: Select any archetype directly from the Start Hub and click **`✨ Forge & Launch World`** to start a brand new procedural simulation in one click without dialog interruptions.
   * **Archetype Card Direct Forging**: In the World Manager, double-click any archetype card or click **`✨ Forge This Realm`** for instantaneous procedural generation.
   * **Full Connection Health Reporting**: Automatic WebSocket health detection with non-blocking visual toast feedback.
7. **📂 Full-Fidelity JSON State Export & Import**:
   * **Complete Multiversal Snapshots**: Save and restore full world states including AST genomes, Akashic memories, civilizational technologies, and historical demographic logs.
   * **Client & Server Synchronized**: Instant client downloads and drag-and-drop file imports for seamless multi-world archiving.
8. **📊 Multiversal Learning Analytics & Comparative Statistics**:
   * **Cognitive Radar Chart**: Visualizes 5 core learning dimensions (Semantic Reflection, AST Complexity, Technological Mastery, Linguistic Dialect Cohesion, and Survival Efficiency).
   * **Cross-Epoch Epoch Comparison**: Bar and line charts tracking knowledge growth across different worlds over simulated years.
9. **⚡ Hardware-Adaptive GPU Tier Scaling**:
   * **Low (30 FPS)**: Battery-saver and low-spec hardware optimization.
   * **Medium (Smooth 60 FPS)**: Full particle effects and double-buffered canvas rendering.
   * **High (Ultra 60+ FPS)**: Dynamic celestial ambient lighting, water wave distortion, and particle trails.
10. **🎨 Obsidian Charcoal & Burnished Gold Theme Engine**:
    * Clean departure from generic blue tones into an elegant Obsidian Charcoal (`#0b0c0e`, `#14171d`), Burnished Amber Gold (`#f59e0b`), Emerald Jade, and Crimson Ruby palette.

---

## 📜 License

MIT License. Designed and built with research in Artificial Life, Multi-Agent Reinforcement Learning, and Neuro-Symbolic Self-Improving AI.
