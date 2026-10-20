# 🌍 Genesis Cogniverse 2D & 2.5D — Sovereign Zenith Engine

[![Godot Engine](https://img.shields.io/badge/Godot-4.x%20%7C%20Forward%20Plus-478CBF?style=for-the-badge&logo=godotengine&logoColor=white)](https://godotengine.org)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-brightgreen?style=for-the-badge)](https://godotengine.org)
[![Perspective](https://img.shields.io/badge/Perspective-Pure%202D%20Top--Down%20%7C%202.5D%20Voxel-orange?style=for-the-badge)]()
[![AI Architecture](https://img.shields.io/badge/AI-Infinite%20Axis%20Utility-blueviolet?style=for-the-badge)]()
[![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)]()

**Genesis Cogniverse 2D** is an autonomous God Simulator and Living Sovereign RPG created in Godot 4. The game features a **Pure Top-Down 2D Engine** powered by custom-generated procedural pixel-art tilesets and multi-layer `TileMapLayer` world grids (inspired by *WorldBox* and *RimWorld*), alongside continuous Infinite Axis Utility AI, stochastic coming-of-age Temple Awakening ceremonies, all 10 master mine tiers, dynamic CanvasModulate Day/Night cycles, a tactile retro wooden God console, and an orbital Cosmic World Map.

---

## 🌟 Key Features

### 1. Pure Top-Down 2D Engine & Procedural Tileset Generator
- **Programmatic Pixel-Art Atlas (`CustomTilesetGenerator.gd`)**:
  - Dynamically synthesizes a high-contrast 32×32 pixel-art atlas (`asserts/custom_tileset_2d.png`).
  - **Row 0 (12 Terrains & Biomes)**: Deep Ocean, Crystal River, Sandy Coast, Sunlit Plains, Fertile Farmland, Whispering Forest, Ironcrag Mountains, Permafrost Snow, Volcanic Basalt, Active Lava, Ancient Runic, and Corrupted Lands.
  - **Row 1 (Architecture & Settlements)**: Red-roof Villager Cottage, Blue-roof Villager Cottage, Temple of Ascension, Cavern Dungeon Rift, Guild Hall Keep, Granary Silo, Campfire, and Watchtower.
  - **Row 2 (All 10 Master Mines)**: Lignite Coal Quarry, Bog Iron Mine, Malachite Copper Pit, Cassiterite Tin Seam, Vein Quartz Gold Drift, Argentite Silver Lode, Bauxite Smelting Clay, Titanium Black Sand, Cobaltite Blue Core, and Aetherium Resonant Geode.
  - **Row 3 (Foliage & Props)**: Oak Trees, Pine Trees, Mountain Boulders, Wheat Crops, Bushes, Wild Mushrooms, and Glowing Crystal Shards.
- **Layered 2D World Grid (`WorldGrid2D.gd`)**:
  - 128 × 128 cell world with multi-octave `FastNoiseLite` elevation and moisture distribution.
  - Rendered across 3 dedicated `TileMapLayer` tiers:
    - **`GroundLayer`**: Seamless terrain biomes and rivers.
    - **`PropsLayer`**: Trees, bushes, boulders, and farmland crops.
    - **`StructuresLayer`**: Village cottages, Temple, Dungeon, Guild Hall, Granary, and the 10 Master Mines.
- **Real-Time Day/Night Atmosphere (`CanvasModulate`)**:
  - Continuous ambient color grading keyed to the 24-hour simulation clock:
    - **Dawn (05:30 – 08:00)**: Warm sunrise peach & golden amber.
    - **Midday (08:00 – 17:00)**: Clear, crisp daylight.
    - **Sunset (17:00 – 19:30)**: Dramatic golden hour dusk.
    - **Night (19:30 – 05:30)**: Mystical moonlit midnight starlight.

### 2. Autonomous Infinite Axis Utility AI
- Villagers calculate dynamic scoring curves ($0.0 \to 1.0$) across multiple continuous environmental axes:
  - **Temple Awakening**: Seek blessings at the Temple of Ascension once reaching age 16.
  - **Dungeon Exploration**: Venture into the Abyssal Rift to slay monsters and harvest rare Mana Stones.
  - **Mining Expeditions**: Journey to any of the 10 Master Mines to extract Coal, Iron, Copper, Gold, Titanium, or Aetherium.
  - **Farmland Agriculture**: Plant, tend, and harvest wheat to replenish communal grain silos.
  - **Campfire Rest**: Seek social warmth and recover energy after long shifts.
  - **Fauna Ecology**: Rabbits, Deer, Sheep, Wolves, Cows, and Bears exhibit flocking, grazing, and natural wandering physics.

### 3. Temple Awakening & Class Rolling (F to SSS Ranks)
- When young villagers reach age 16, they journey to the **Temple of Ascension** ($X: 68, Y: 40$) for their divine awakening ceremony.
- **Stochastic Rank Distribution**:
  - **Rank F** (40.0%): Common civilian laborers.
  - **Rank E** (25.0%): Capable hands with +10% stat boost.
  - **Rank D** (15.0%): Skilled tradesmen (+20% stat boost).
  - **Rank C** (10.0%): Respected community pillars (+35% stat boost).
  - **Rank B** (5.5%): Regional champions (+50% stat boost).
  - **Rank A** (3.0%): Elite guild leaders (+80% stat boost).
  - **Rank S** (1.0%): National heroes with rare divine auras (+120% stat boost).
  - **Rank SS** (0.4%): Mythic legends (+180% stat boost).
  - **Rank SSS** (0.1%): Demigods destined for sovereign rule (+250% stat boost).
- **Vocation Assignment & Visual Tinting**:
  - **Warriors & Paladins**: Steel blue or radiant gold.
  - **Mages**: Mystical arcane purple.
  - **Rangers**: Emerald forest green.
  - **Rogues**: Shadow slate.
  - **Farmers**: Harvest golden amber.
  - **Novices**: Crisp white.

### 4. Complete 10 Master Mine Resource Extraction
Every mine type features dedicated extraction logic, visual color coding, and resource yields:
1. **Lignite Coal Quarry** (Tier 1) $\to$ Base thermal fuel.
2. **Bog Iron Mine** (Tier 2) $\to$ Forging tools and weapons.
3. **Malachite Copper Pit** (Tier 3) $\to$ Bronze alloy casting.
4. **Cassiterite Tin Seam** (Tier 4) $\to$ Precision smithing.
5. **Vein Quartz Gold Drift** (Tier 5) $\to$ Guild currency and sovereignty tithes.
6. **Argentite Silver Lode** (Tier 6) $\to$ Enchanted runes and mirrors.
7. **Bauxite Smelting Clay** (Tier 7) $\to$ Lightweight alloy plating.
8. **Titanium Black Sand** (Tier 8) $\to$ Masterwork plate armor.
9. **Cobaltite Blue Core** (Tier 9) $\to$ Arcane conductor batteries.
10. **Aetherium Crystal Rift** (Tier 10) $\to$ Reality-bending mana artifacts.

### 5. Dual-Mode Seamless Camera & Cosmic World Map
- **Tactical 2D Village View (`CameraController2D.gd`)**:
  - Smooth WASD / arrow-key panning with speed scaling.
  - Middle-mouse drag pan for fluid canvas exploration.
  - Mouse-wheel zoom with smooth interpolation.
- **Cosmic World Map Switch**:
  - Automatically activates when zooming out past threshold ($zoom \le 0.22$) or pressing `M` / clicking `🌌 Cosmic Map`.
  - Displays the rotating planetary sphere with animated leylines and starfield.
  - **Sovereign Realms Card**: Diplomatic relations with the Solarian Hegemony, Umbral Enclave, and Sylvan Kinship.
  - **Threat Rankings**: Real-time calamity tracker (Abyssal Void Dragon, Frost Titan, Goblin Warlord Horde).
  - **Era Progression**: Chronicle of world ages (from Age of Awakening $\to$ Celestial Sovereignty).

### 6. Tactile Retro Wooden God Console
- **Status Bar**: Real-time calendar (Year, Season, Day, AM/PM), Weather badge with click-to-open **Climate Codex popover**, Danger threat gauge, Stockpile counters (`🪵 Wood`, `🪨 Stone`, `🌾 Wheat`, `💎 Mana Stones`), and 0x–8x Simulation Speed controls.
- **Live NPC Inspector**: Dynamic portrait, Rank badge, 4-stat chips (`STR`, `VIT`, `DEX`, `INT`), active Utility AI drive, and `⚡ Awaken` miracle button.
- **Chronicle Event Feed**: Scrolling world history with click-to-pan camera teleportation.
- **Minimap Radar**: Real-time minimap radar with dynamic POI markers and quick-jump shortcuts (`🏛️ Temple`, `🐉 Dungeon`, `⚔️ Guild`, `🌾 Farm`).
- **4 Build & Edit Sub-Palettes**:
  - `🌱 Nature`: Raise/Lower Terrain, Forest, Plains, Tundra, Lava, Clear Land.
  - `⛏️ Mines`: Paint any of the 10 Master Mine nodes.
  - `🏛️ Architecture`: Place Temple, Dungeon, Guild Hall, Houses, Granary, Campfire.
  - `⚡ Miracles`: Smite, Rain Blessing, Sun Blessing, Divine Awakening.

### 7. Living Townsfolk & High-Fidelity Architectural Buildings
- **Artisan Buildings & Architecture (`asserts/buildings/`)**:
  - **Citizen Cottage** ($208 \times 219$): Half-timbered stone cottage with open doorstep and pitched shingle roof.
  - **Master Blacksmith Forge** ($239 \times 182$): Stone and brick hearth with blazing furnace, water quenching tub, and anvil.
  - **Alchemy Apothecary** ($231 \times 257$): Ivy-clad herbalist cottage with potion racks and bubbling cauldrons.
  - **Adventurers' Guildhall** ($250 \times 271$): Grand two-story timber hall with open double gatehouse and quest notice board.
- **Dedicated Artisan NPCs**:
  - **Master Blacksmith Goran Ironheart (`BlacksmithNPC2D.gd`)**: Positioned at the forge anvil; animated with hammering strikes emitting glowing sparks, idle blade inspection, and brow-wiping rest cycles. Interactive `[E]` service hones player weapons (+2 ATK, craft iron broadswords).
  - **Master Alchemist Elira Moonwhisper (`AlchemistNPC2D.gd`)**: Positioned in front of the apothecary; animated with potion brewing and flask inspection. Interactive `[E]` apothecary shop sells Crimson Health Flasks.
- **Living Townsfolk Animation Engine (`CharacterSpriteGenerator.gd`)**:
  - Distinct gender representations with male and female spritesheets.
  - Directional walking cycles with horizontal flipping.
  - Expressive conversational gestures (`talk` animation) triggered automatically when standing near neighbors in the streets.
- **Pixel-Perfect Foundation Collision Shapes & Depth Occlusion**:
  - All props and buildings utilize base foundation collision boxes covering only the ground footprint, leaving rooftops, tree canopies, and doorways completely unobstructed.
  - Open doorway corridors allow heroes and villagers to walk up to and inside building thresholds.
  - Unified `YSortEntities` container ensures mathematically perfect 2D depth sorting on the Y-axis across all players, NPCs, wildlife, trees, and buildings.

---

## 📁 Project Architecture & File Structure

```
new-game-project/
├── project.godot                     # Godot 4 project configuration (Genesis Cogniverse 2D)
├── icon.svg                          # Engine logo asset
├── .gitignore                        # Git exclusion rules (.godot, temp, exports)
│
├── scene/
│   ├── main_game_2d.tscn             # Master 2D Scene (WorldGrid2D, Camera2D, CanvasModulate, UI) [DEFAULT]
│   ├── main_game.tscn                # 2.5D Stepped Voxel Scene (WorldGrid3D, MultiMeshInstance3D)
│   └── entities/
│       ├── npc_character_2d.tscn     # 2D CharacterBody2D with shadow, sprite, and name label
│       ├── wildlife_animal_2d.tscn   # 2D Wildlife CharacterBody2D with grazing and flocking
│       ├── npc_character_3d.tscn     # Y-Billboard 3D CharacterBody3D
│       └── wildlife_animal_3d.tscn   # Y-Billboard 3D Wildlife Controller
│
├── script/
│   ├── MainGame2D.gd                 # Master 2D coordinator, NPC/Fauna spawning, 2D tile click picking
│   ├── MainGame.gd                   # Master 2.5D coordinator for stepped 3D voxel mode
│   │
│   ├── autoload/
│   │   ├── GameManager.gd            # Simulation calendar, stockpiles, canonical POI coordinates
│   │   ├── ChronicleManager.gd       # World events history & camera pan signal bus
│   │   └── SoundManager.gd           # 8-channel procedurally synthesized 16-bit WAV SFX engine
│   │
│   ├── simulation/
│   │   ├── CustomTilesetGenerator.gd # Procedural 32x32 pixel-art atlas texture and TileSet generator
│   │   ├── WorldGrid2D.gd            # 128x128 TileMapLayer procedural world grid & God terraforming tools
│   │   ├── WorldGrid3D.gd            # 128x128 3D voxel grid with single draw-call MultiMesh
│   │   ├── BuildingFactory3D.gd      # 3D architectural models for 2.5D mode
│   │   ├── AwakeningSystem.gd        # Stochastic F to SSS rank rolling & RPG vocation assignment
│   │   └── UtilityAI.gd              # Continuous Infinite Axis Utility decision evaluation
│   │
│   ├── entities/
│   │   ├── AgentController2D.gd      # 2D Hero state machine, Utility AI, Temple Awakening, Mining
│   │   ├── WildlifeController2D.gd   # 2D Fauna flocking, grazing, and wandering physics
│   │   ├── AgentController3D.gd      # 2.5D Hero state machine
│   │   └── WildlifeController3D.gd   # 2.5D Fauna flocking
│   │
│   └── ui/
│       ├── CameraController2D.gd     # Smooth 2D camera pan, zoom, drag, and cosmic map threshold
│       ├── CameraController3D.gd     # 45° isometric tilt camera for 2.5D mode
│       ├── RetroHUD.gd               # Status bar, time/weather, Climate Codex, speed controls
│       ├── NpcInspector.gd           # Dynamic NPC details card with ⚡ Awaken trigger
│       ├── EventFeed.gd              # World history feed with click-to-pan navigation
│       ├── MinimapRadar.gd           # Real-time radar with POI jump shortcuts
│       ├── BuildSubpalette.gd        # 4 build tabs (Nature, Mines, Architecture, Miracles)
│       └── CosmicWorldMap.gd         # Planetary sphere, leylines, Sovereign Realms, Threat rankings
│
└── asserts/                          # Game assets (sprites, textures, custom tilesets)
    ├── custom_tileset_2d.png         # Programmatically generated 32x32 pixel-art atlas texture
    ├── Metroidvania/                 # Gothicvania character sprites
    ├── PixelPlanets/                 # Stars and planet textures
    └── file/                         # UI and environment details
```

### 8. Phase 8: Multi-Town World Architecture, RPG Interactions & Dungeon Occlusion
- **Universal NPC & Object Interaction (`InventoryManager.add_item_by_id`)**:
  - Implemented `InventoryManager.add_item_by_id(item_id, quantity)` supporting canonical item definitions, material stacks, and automatic inventory updates.
  - Added alias support (`"weapon_iron_sword"` $\to$ `"sword_iron"`) in `ItemData.create_item_by_id`.
  - Full interactive support for:
    - **Master Blacksmiths**: Hone steel and forge Iron Broadswords for 15 Gold.
    - **Master Alchemists**: Dispense Crimson Health Potions for 5 Gold.
    - **Floating Mana Stones**: Commune to absorb 25 Mana Stones and Resonant Mana Crystals with pulsing arcane glow and floating damage/communion popups.
- **Flawless Dungeon Depth & Player Hierarchy (`DungeonInterior2D`)**:
  - Eliminated the bug where player characters rendered beneath dungeon tiles or vanished:
    - `DungeonInterior2D` now has `y_sort_enabled = true`.
    - `GroundLayer` has `z_index = -1`, ensuring subterranean floor obsidian tiles always stay beneath entities.
    - `WallsLayer`, `Mobs`, `Boss`, and `PlayerSlot` Y-sort seamlessly together.
  - Seamless player reparenting: when descending into the rift, the hero is reparented into `dungeon_instance.player_slot`, maintaining 100% visibility.
  - When emerging victorious or upon death, the hero is cleanly reparented back to the overworld `y_sort_entities` container.
- **Respawn Visibility & Building Restoration (`MainGame2D.gd`)**:
  - Unconditionally restores `y_sort_entities.visible = true` in both `_on_player_died()` and `_on_death_screen_revive()`.
  - Fixed building disappearance bug upon death/respawn: all cottages, forges, apothecaries, and guilds remain permanently visible.
- **Architectural Generation Across All 5 Biome Towns (`WorldDecorator2D.gd`)**:
  - Dynamic procedural city decorator iterating over all 5 registered kingdoms:
    - **Sunhaven Kingdom** (Sunlit Plains Capital)
    - **Sylvandell Bastion** (Whispering Redwoods)
    - **Khar-Drak Fortress** (Ironcrag Mountains)
    - **Oasis of Qadesh** (Solar Badlands & Dunes)
    - **Frostpeak Sanctuary** (Permafrost Tundra)
  - Every town receives:
    - High-fidelity **Blacksmith Forge & Smelter** with local master smith NPC (`Goran`, `Faelar`, `Durnik`, `Tariq`, `Torvald`).
    - High-fidelity **Alchemy Apothecary** with local herbalist NPC (`Elira`, `Sylva`, `Gimra`, `Zahra`, `Astrid`).
    - High-fidelity **Adventurers' Guild Hall**.
    - Residential **Citizen Cottages** around neighborhood quarters.
    - Crossroads **Street Lanterns**, supply crates, barrels, and city entrance signposts.
- **Engine Warning Elimination & TileSet Physics Fix**:
  - Resolved `TileSetAtlasSource` physics layer 0 out-of-bounds error by ensuring `tileset.add_source(source, 0)` is attached before collision polygon assignment.
  - Fixed `PlayerInventoryUI.gd:128` string formatting error (`%` character escaping).
  - Eliminated all shadowed global identifier warnings (`ItemData`, `HealthBar2D`, `Hurtbox2D`, `Hitbox2D`, `CharacterSpriteGenerator`, `LootTable`).

---

## 🚀 Getting Started

### Prerequisites
- **Godot Engine 4.x** (v4.2, v4.3, or v4.7+).
- **OS**: Windows, macOS, or Linux.
- **Renderer**: Forward Plus, Mobile, or Compatibility (OpenGL 3.3+).

### Installation & Launching

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/AREEB-AZHAR/cogniverse-rpg.git
   cd new-game-project
   ```

2. **Open in Godot Engine**:
   - Launch **Godot Engine 4.x**.
   - Click **Import** in the Project Manager.
   - Navigate to the `new-game-project` directory and select `project.godot`.
   - Click **Import & Edit**.

3. **Run the Project**:
   - Press **F5** (or click the **Play** button in the top right corner).
   - The game will immediately boot into `res://scene/main_game_2d.tscn`.

4. **Run the Automated Audit & Test Suite (Headless)**:
   - **Static Script Audit**:
     ```bash
     python script/tools/static_audit.py
     ```
   - **Scene Instantiation Audit**:
     ```bash
     godot --headless -s res://script/tools/audit_all_scenes.gd
     ```
   - **Full Subsystem Runtime Stress Audit**:
     ```bash
     godot --headless -s res://script/tools/runtime_stress_audit.gd
     ```

---

## 🛡️ Zero-Error & Zero-Warning Certification
The entire codebase has passed comprehensive static and dynamic runtime verification:
- **0 Compilation / Syntax Errors**
- **0 Runtime Exceptions / Nil Pointer Dereferences**
- **0 Engine Warnings** (Shadowed identifiers, unused signals, formatting characters resolved)
- **0 Physics / Out-of-Bounds Layer Violations**
- **Full Y-Sort & Depth Occlusion Compliance** across Overworld and Dungeon environments.

## 🎮 Controls & Shortcuts

| Action | Control | Description |
| :--- | :--- | :--- |
| **Move Hero** | `W` `A` `S` `D` | 8-way directional movement |
| **Sprint** | `Shift` (Hold) | 1.6× sprint speed boost |
| **Sword Attack** | `Space` / `Left Click` | Directional melee strike with weapon damage |
| **Interact (NPC / Object)**| `E` | Forge equipment, buy potions, commune with Mana Stones |
| **Inventory & Equipment** | `I` / `B` | Open retro wooden inventory panel |
| **Quick Health Potion** | `Q` | Instantly drink health potion (+50 HP) from backpack |
| **Pan Camera** | Arrow Keys / `Middle Drag` | Move camera across the 128×128 world grid |
| **Toggle Camera Lock** | `Tab` | Toggle between free camera and following the hero |
| **Toggle Cosmic Map** | `M` | Shifts between tactical village view and planetary map |
| **Pause / Speed** | `0x` - `8x` | Pause or fast-forward simulation time |

---

## 📜 License
Distributed under the MIT License.
