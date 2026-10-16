use super::tile::{BiomeType, ElevationLayer, PlacedStructure};
use super::WorldGrid;
use std::collections::HashSet;

pub const CHUNK_SIZE: usize = 16;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct CellId {
    pub face: u8,
    pub x: u16,
    pub y: u16,
}

impl CellId {
    pub fn new(face: u8, x: u16, y: u16) -> Self {
        Self { face, x, y }
    }

    pub fn chunk_coords(&self) -> (u8, u16, u16) {
        (
            self.face,
            self.x / (CHUNK_SIZE as u16),
            self.y / (CHUNK_SIZE as u16),
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct DirtyChunks {
    dirty: HashSet<(u8, u16, u16)>,
}

impl DirtyChunks {
    pub fn new() -> Self {
        Self {
            dirty: HashSet::new(),
        }
    }

    pub fn mark_dirty(&mut self, cell: CellId) {
        self.dirty.insert(cell.chunk_coords());
    }

    pub fn is_dirty(&self, face: u8, cx: u16, cy: u16) -> bool {
        self.dirty.contains(&(face, cx, cy))
    }

    pub fn has_dirty(&self) -> bool {
        !self.dirty.is_empty()
    }

    pub fn drain_dirty(&mut self) -> Vec<(u8, u16, u16)> {
        self.dirty.drain().collect()
    }

    pub fn count(&self) -> usize {
        self.dirty.len()
    }
}

/// Mesh representation cache per chunk (rebuilt when chunk is dirty)
#[derive(Clone, Debug, PartialEq)]
pub struct ChunkMesh {
    pub chunk_x: u16,
    pub chunk_y: u16,
    pub vertex_count: usize,
    pub revision: u64,
}

/// Cache of chunk meshes ensuring renders do not go stale
#[derive(Clone, Debug, Default)]
pub struct ChunkMeshCache {
    pub meshes: std::collections::HashMap<(u8, u16, u16), ChunkMesh>,
}

impl ChunkMeshCache {
    pub fn new() -> Self {
        Self {
            meshes: std::collections::HashMap::new(),
        }
    }

    pub fn rebuild_chunk(&mut self, face: u8, cx: u16, cy: u16, world: &WorldGrid) {
        let mut vertex_count = 0;
        let start_x = (cx as usize) * CHUNK_SIZE;
        let start_y = (cy as usize) * CHUNK_SIZE;

        for dy in 0..CHUNK_SIZE {
            let y = start_y + dy;
            if y >= world.height {
                continue;
            }
            for dx in 0..CHUNK_SIZE {
                let x = start_x + dx;
                if x >= world.width {
                    continue;
                }
                let tile = world.tile_at(x, y);
                // Each non-empty land/structure tile emits vertices into the chunk mesh
                vertex_count += 4;
                if tile.structure.is_some() {
                    vertex_count += 4;
                }
            }
        }

        let entry = self.meshes.entry((face, cx, cy)).or_insert(ChunkMesh {
            chunk_x: cx,
            chunk_y: cy,
            vertex_count: 0,
            revision: 0,
        });

        entry.vertex_count = vertex_count;
        entry.revision += 1;
    }
}

/// Cached pathfinding costs per cell (recalculated when chunk is dirty)
#[derive(Clone, Debug)]
pub struct PathCostCache {
    pub width: usize,
    pub height: usize,
    pub costs: Vec<u8>, // 0 = impassable (ocean/deep lava), 1 = road/plains, 5 = mountain/forest
}

impl PathCostCache {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            costs: vec![1; width * height],
        }
    }

    pub fn get_cost(&self, x: usize, y: usize) -> u8 {
        if x < self.width && y < self.height {
            self.costs[y * self.width + x]
        } else {
            0
        }
    }

    pub fn update_chunk(&mut self, _face: u8, cx: u16, cy: u16, world: &WorldGrid) {
        let start_x = (cx as usize) * CHUNK_SIZE;
        let start_y = (cy as usize) * CHUNK_SIZE;

        for dy in 0..CHUNK_SIZE {
            let y = start_y + dy;
            if y >= world.height {
                continue;
            }
            for dx in 0..CHUNK_SIZE {
                let x = start_x + dx;
                if x >= world.width {
                    continue;
                }
                let tile = world.tile_at(x, y);
                let cost = if tile.elevation_layer.is_water() {
                    0 // Impassable deep/close ocean without boat or ice
                } else {
                    match tile.biome {
                        BiomeType::CrystalRiverlands => 4, // Shallow fordable water
                        BiomeType::IroncragMountains => 6,
                        BiomeType::WhisperingForest | BiomeType::Jungle => 3,
                        BiomeType::CorruptedInfernal => 8,
                        BiomeType::SunlitPlains | BiomeType::Savanna => 1,
                        BiomeType::AncientLeylineRuins | BiomeType::Swamp => 2,
                        BiomeType::PermafrostTundra => 3,
                    }
                };
                self.costs[y * self.width + x] = cost;
            }
        }
    }
}

/// The unified EditCommand trait ensuring all god-mode mutations go through a single undoable pipeline
pub trait EditCommand: Send + Sync {
    fn apply(&mut self, world: &mut WorldGrid, dirty: &mut DirtyChunks);
    fn undo(&mut self, world: &mut WorldGrid, dirty: &mut DirtyChunks);
    fn description(&self) -> String;
}

/// Terrain properties for a cell (biome + elevation layer)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Terrain {
    pub biome: BiomeType,
    pub elevation_layer: ElevationLayer,
}

impl Terrain {
    pub fn new(biome: BiomeType, elevation_layer: ElevationLayer) -> Self {
        Self {
            biome,
            elevation_layer,
        }
    }
}

/// Painting terrain biome and elevation on one or more cells
pub struct PaintTerrain {
    pub edits: Vec<(CellId, Terrain /* new */, Terrain /* old */)>,
}

impl PaintTerrain {
    pub fn new(edits: Vec<(CellId, Terrain, Terrain)>) -> Self {
        Self { edits }
    }

    pub fn single(cell: CellId, new_terrain: Terrain, old_terrain: Terrain) -> Self {
        Self {
            edits: vec![(cell, new_terrain, old_terrain)],
        }
    }
}

impl EditCommand for PaintTerrain {
    fn apply(&mut self, world: &mut WorldGrid, dirty: &mut DirtyChunks) {
        for &(cell, new_terrain, _) in &self.edits {
            let x = cell.x as usize;
            let y = cell.y as usize;
            let tile = world.tile_at_mut(x, y);
            tile.biome = new_terrain.biome;
            tile.elevation_layer = new_terrain.elevation_layer;
            dirty.mark_dirty(cell);
        }
    }

    fn undo(&mut self, world: &mut WorldGrid, dirty: &mut DirtyChunks) {
        for &(cell, _, old_terrain) in &self.edits {
            let x = cell.x as usize;
            let y = cell.y as usize;
            let tile = world.tile_at_mut(x, y);
            tile.biome = old_terrain.biome;
            tile.elevation_layer = old_terrain.elevation_layer;
            dirty.mark_dirty(cell);
        }
    }

    fn description(&self) -> String {
        format!("PaintTerrain: {} cell(s)", self.edits.len())
    }
}

/// Elevation modification command (raise / lower elevation)
pub struct ModifyElevation {
    pub edits: Vec<(CellId, f32 /* new */, f32 /* old */)>,
}

impl ModifyElevation {
    pub fn new(edits: Vec<(CellId, f32, f32)>) -> Self {
        Self { edits }
    }
}

impl EditCommand for ModifyElevation {
    fn apply(&mut self, world: &mut WorldGrid, dirty: &mut DirtyChunks) {
        for &(cell, new_elev, _) in &self.edits {
            let x = cell.x as usize;
            let y = cell.y as usize;
            let tile = world.tile_at_mut(x, y);
            tile.elevation = new_elev;
            tile.elevation_layer = ElevationLayer::from_elevation(new_elev);
            dirty.mark_dirty(cell);
        }
    }

    fn undo(&mut self, world: &mut WorldGrid, dirty: &mut DirtyChunks) {
        for &(cell, _, old_elev) in &self.edits {
            let x = cell.x as usize;
            let y = cell.y as usize;
            let tile = world.tile_at_mut(x, y);
            tile.elevation = old_elev;
            tile.elevation_layer = ElevationLayer::from_elevation(old_elev);
            dirty.mark_dirty(cell);
        }
    }

    fn description(&self) -> String {
        format!("ModifyElevation: {} cell(s)", self.edits.len())
    }
}

/// Place a data-driven mine on a cell
pub struct PlaceMineCommand {
    pub cell: CellId,
    pub mine_id: String,
    pub old_structure: Option<PlacedStructure>,
}

impl PlaceMineCommand {
    pub fn new(cell: CellId, mine_id: String, old_structure: Option<PlacedStructure>) -> Self {
        Self {
            cell,
            mine_id,
            old_structure,
        }
    }
}

impl EditCommand for PlaceMineCommand {
    fn apply(&mut self, world: &mut WorldGrid, dirty: &mut DirtyChunks) {
        let x = self.cell.x as usize;
        let y = self.cell.y as usize;
        let tile = world.tile_at_mut(x, y);
        let structure = PlacedStructure::new_mine(self.mine_id.clone());
        tile.structure = Some(structure);
        dirty.mark_dirty(self.cell);
    }

    fn undo(&mut self, world: &mut WorldGrid, dirty: &mut DirtyChunks) {
        let x = self.cell.x as usize;
        let y = self.cell.y as usize;
        let tile = world.tile_at_mut(x, y);
        tile.structure = self.old_structure.clone();
        dirty.mark_dirty(self.cell);
    }

    fn description(&self) -> String {
        format!(
            "PlaceMine: {} at ({}, {})",
            self.mine_id, self.cell.x, self.cell.y
        )
    }
}

/// The command history engine managing undo, redo, and automatic cache synchronization
pub struct CommandPipeline {
    pub undo_stack: Vec<Box<dyn EditCommand>>,
    pub redo_stack: Vec<Box<dyn EditCommand>>,
    pub dirty_chunks: DirtyChunks,
    pub mesh_cache: ChunkMeshCache,
    pub path_cost_cache: PathCostCache,
}

impl CommandPipeline {
    pub fn new(world_width: usize, world_height: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            dirty_chunks: DirtyChunks::new(),
            mesh_cache: ChunkMeshCache::new(),
            path_cost_cache: PathCostCache::new(world_width, world_height),
        }
    }

    /// Executes an edit command, pushes it onto the undo stack, and syncs dirty caches
    pub fn execute(&mut self, mut command: Box<dyn EditCommand>, world: &mut WorldGrid) {
        command.apply(world, &mut self.dirty_chunks);
        self.undo_stack.push(command);
        self.redo_stack.clear();
        self.sync_dirty_chunks(world);
    }

    /// Undoes the last edit command and syncs dirty caches
    pub fn undo(&mut self, world: &mut WorldGrid) -> bool {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo(world, &mut self.dirty_chunks);
            self.redo_stack.push(command);
            self.sync_dirty_chunks(world);
            true
        } else {
            false
        }
    }

    /// Redoes the previously undone edit command and syncs dirty caches
    pub fn redo(&mut self, world: &mut WorldGrid) -> bool {
        if let Some(mut command) = self.redo_stack.pop() {
            command.apply(world, &mut self.dirty_chunks);
            self.undo_stack.push(command);
            self.sync_dirty_chunks(world);
            true
        } else {
            false
        }
    }

    /// Synchronizes meshes and path caches for all chunks marked dirty
    pub fn sync_dirty_chunks(&mut self, world: &WorldGrid) {
        let dirty_list = self.dirty_chunks.drain_dirty();
        for (face, cx, cy) in dirty_list {
            self.mesh_cache.rebuild_chunk(face, cx, cy, world);
            self.path_cost_cache.update_chunk(face, cx, cy, world);
        }
    }
}
