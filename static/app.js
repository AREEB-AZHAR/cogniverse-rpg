// ============================================================================
// Genesis Cogniverse - Authentic WorldBox God Simulator Client
// ============================================================================

let ws = null;
let currentTelemetry = null;
let selectedAgentId = 1;
let favoriteAgentId = null;
let isFollowActive = false;
let showKingdomBorders = true;
let selectedMapArchetype = 'GrandArchipelago';
let activeMapId = null;
const terraformedTiles = new Map(); // key: "x,y" => BiomeType string

const BIOME_LOOKUP = [
  'SunlitPlains',         // 0
  'WhisperingForest',     // 1
  'CrystalRiverlands',    // 2
  'IroncragMountains',    // 3
  'AncientLeylineRuins',  // 4
  'Savanna',              // 5
  'Jungle',               // 6
  'CorruptedLands',       // 7
  'PermafrostTundra',     // 8
  'Swamp',                // 9
  'DeepOcean',            // 10
  'CloseOcean',           // 11
  'SandyCoast',           // 12
];

// Canvas & Viewport Setup
const canvas = document.getElementById('world-canvas');
const ctx = canvas.getContext('2d');
const container = document.getElementById('game-container');
const tileHud = document.getElementById('hover-tile-info');
const toastEl = document.getElementById('tool-toast');

const WORLD_SIZE = 128;
const TILE_PX = 14; // Base pixel size per tile for 128x128 grand continent

// Offscreen Terrain Buffer for Buttery-Smooth 60 FPS
const terrainCanvas = document.createElement('canvas');
terrainCanvas.width = WORLD_SIZE * TILE_PX;
terrainCanvas.height = WORLD_SIZE * TILE_PX;
const terrainCtx = terrainCanvas.getContext('2d');
let isTerrainBufferBuilt = false;

function getProceduralBiome(x, y) {
  const fx = x / WORLD_SIZE;
  const fy = y / WORLD_SIZE;

  // Great Continental River System
  const riverCenter = 56.0 + Math.sin(y * 0.08) * 14.0 + Math.cos(y * 0.22) * 5.0;
  const distToRiver = Math.abs(x - riverCenter);

  // Northeastern Mountain Tributary
  const tributaryCenter = 86.0 - Math.sin((y - 15.0) * 0.12) * 16.0;
  const isTributary = y > 15 && y < 65 && Math.abs(x - tributaryCenter) <= 2.2;

  // Oceanic Margins encircling continent
  const isOcean = x < 8 || x >= WORLD_SIZE - 8 || y < 8 || y >= WORLD_SIZE - 8;
  const isCoast = !isOcean && (x < 11 || x >= WORLD_SIZE - 11 || y < 11 || y >= WORLD_SIZE - 11);

  if (isOcean) return 'DeepOcean';
  if (distToRiver <= 2.8 || isTributary) return 'CrystalRiverlands';
  if (isCoast || distToRiver <= 4.2 || (y > 14 && y < 66 && Math.abs(x - tributaryCenter) <= 3.4)) return 'SandyCoast';
  if (fx > 0.60 && fy < 0.50) return 'IroncragMountains';
  if (Math.hypot(fx - 0.48, fy - 0.50) < 0.10) return 'AncientLeylineRuins';
  if ((fx < 0.45 && fy < 0.68) || (fx < 0.38 && fy > 0.65)) return 'WhisperingForest';
  return 'SunlitPlains';
}

function paintTileToBuffer(bCtx, x, y, biome) {
  const px = x * TILE_PX;
  const py = y * TILE_PX;
  const tilePx = TILE_PX;

  const isWater = biome === 'DeepOcean' || biome.includes('River') || biome.includes('Ocean');
  const isMountain = biome.includes('Mountain');
  const isForest = biome.includes('Forest');
  const isRuins = biome.includes('Ruins');
  const isCoast = biome === 'SandyCoast';

  if (biome === 'DeepOcean') {
    const oceanTheme = getThemeOceanColors();
    bCtx.fillStyle = oceanTheme.deep;
    bCtx.fillRect(px, py, tilePx, tilePx);
    if ((x + y) % 6 === 0) {
      bCtx.fillStyle = oceanTheme.ripple;
      bCtx.fillRect(px + 2, py + 3, 4, 1);
    }
  } else if (isWater) {
    // Crystal Riverlands
    bCtx.fillStyle = '#0284c7';
    bCtx.fillRect(px, py, tilePx, tilePx);
    bCtx.fillStyle = '#7dd3fc';
    bCtx.fillRect(px + ((x * 3 + y * 7) % (tilePx - 4)), py + ((y * 5) % (tilePx - 2)), 3, 1);
    bCtx.fillStyle = '#0369a1';
    bCtx.fillRect(px + 1, py + tilePx - 2, tilePx - 2, 1);
  } else if (isCoast) {
    bCtx.fillStyle = '#eab308';
    bCtx.fillRect(px, py, tilePx, tilePx);
    bCtx.fillStyle = '#ca8a04';
    bCtx.fillRect(px + 2, py + 2, tilePx - 4, tilePx - 4);
    if ((x + y * 2) % 5 === 0) {
      bCtx.fillStyle = '#fef08a';
      bCtx.fillRect(px + 4, py + 5, 2, 2);
    }
  } else if (isMountain) {
    // Rugged granite crag base
    bCtx.fillStyle = '#475569';
    bCtx.fillRect(px, py, tilePx, tilePx);
    bCtx.fillStyle = '#334155';
    bCtx.beginPath();
    bCtx.moveTo(px, py + tilePx);
    bCtx.lineTo(px + tilePx * 0.5, py + tilePx * 0.4);
    bCtx.lineTo(px + tilePx, py + tilePx);
    bCtx.closePath();
    bCtx.fill();

    // Snow capped ridge
    bCtx.fillStyle = '#f8fafc';
    bCtx.beginPath();
    bCtx.moveTo(px + tilePx * 0.5, py + 1);
    bCtx.lineTo(px + tilePx - 2, py + 5);
    bCtx.lineTo(px + tilePx * 0.5, py + 4);
    bCtx.lineTo(px + 2, py + 5);
    bCtx.closePath();
    bCtx.fill();

    // Prominent Mountain Ore Veins (Gold, Mithril, Iron)
    const oreHash = (x * 37 + y * 53) % 19;
    if (oreHash === 0 || oreHash === 1) {
      // Glittering Auric Gold Vein (Bright gold sparkles)
      bCtx.fillStyle = '#eab308';
      bCtx.fillRect(px + 3, py + 7, 3, 3);
      bCtx.fillStyle = '#facc15';
      bCtx.fillRect(px + 4, py + 6, 2, 4);
      bCtx.fillStyle = '#fef08a';
      bCtx.fillRect(px + 7, py + 8, 2, 2);
      bCtx.fillStyle = '#ffffff';
      bCtx.fillRect(px + 4, py + 7, 1, 1);
    } else if (oreHash === 2 || oreHash === 3) {
      // Mystic Leyline Mithril Vein (Radiant Cyan & Violet Crystals)
      bCtx.fillStyle = '#06b6d4';
      bCtx.fillRect(px + 4, py + 7, 3, 4);
      bCtx.fillStyle = '#67e8f9';
      bCtx.fillRect(px + 5, py + 6, 2, 3);
      bCtx.fillStyle = '#c084fc';
      bCtx.fillRect(px + 8, py + 8, 2, 3);
      bCtx.fillStyle = '#ffffff';
      bCtx.fillRect(px + 5, py + 7, 1, 1);
    } else if (oreHash === 4 || oreHash === 5 || oreHash === 6) {
      // Ironcrag Iron Ore Vein (Copper / Rust metallic deposits)
      bCtx.fillStyle = '#9a3412';
      bCtx.fillRect(px + 3, py + 7, 4, 3);
      bCtx.fillStyle = '#ea580c';
      bCtx.fillRect(px + 4, py + 8, 3, 2);
      bCtx.fillStyle = '#fed7aa';
      bCtx.fillRect(px + 5, py + 7, 1, 2);
    }
  } else if (isRuins) {
    bCtx.fillStyle = '#2e1065';
    bCtx.fillRect(px, py, tilePx, tilePx);
    bCtx.fillStyle = '#1e1b4b';
    bCtx.fillRect(px + 1, py + 1, tilePx - 2, tilePx - 2);
    bCtx.fillStyle = '#c084fc';
    bCtx.fillRect(px + 3, py + 3, tilePx - 6, 1);
    bCtx.fillRect(px + tilePx * 0.5 - 1, py + 3, 2, tilePx - 6);
    bCtx.fillStyle = '#f472b6';
    bCtx.fillRect(px + 4, py + 6, 2, 2);
  } else if (isForest) {
    bCtx.fillStyle = '#14532d';
    bCtx.fillRect(px, py, tilePx, tilePx);
    bCtx.fillStyle = '#052e16';
    bCtx.beginPath();
    bCtx.arc(px + tilePx * 0.5, py + tilePx * 0.55, tilePx * 0.42, 0, Math.PI * 2);
    bCtx.fill();
    bCtx.fillStyle = '#15803d';
    bCtx.beginPath();
    bCtx.arc(px + tilePx * 0.5, py + tilePx * 0.42, tilePx * 0.38, 0, Math.PI * 2);
    bCtx.fill();
    bCtx.fillStyle = '#22c55e';
    bCtx.beginPath();
    bCtx.arc(px + tilePx * 0.45, py + tilePx * 0.35, tilePx * 0.22, 0, Math.PI * 2);
    bCtx.fill();
    bCtx.fillStyle = '#78350f';
    bCtx.fillRect(px + tilePx * 0.45, py + tilePx * 0.72, 2, 3);
  } else if (biome === 'SavannaGrasslands' || biome === 'Savanna') {
    // Warm golden amber savanna grass
    bCtx.fillStyle = '#ca8a04';
    bCtx.fillRect(px, py, tilePx, tilePx);
    bCtx.fillStyle = '#a16207';
    bCtx.fillRect(px + 2, py + 3, tilePx - 4, tilePx - 5);
    // Dry grass tufts
    bCtx.fillStyle = '#fef08a';
    bCtx.fillRect(px + 3, py + 4, 1, 3);
    bCtx.fillRect(px + 8, py + 6, 1, 3);
    // Acacia bush dot
    if ((x * 5 + y * 7) % 7 === 0) {
      bCtx.fillStyle = '#713f12';
      bCtx.fillRect(px + 4, py + 5, 3, 2);
    }
  } else if (biome === 'VerdantJungle' || biome === 'Jungle') {
    // Deep emerald tropical jungle canopy
    bCtx.fillStyle = '#064e3b';
    bCtx.fillRect(px, py, tilePx, tilePx);
    bCtx.fillStyle = '#047857';
    bCtx.beginPath();
    bCtx.arc(px + tilePx * 0.5, py + tilePx * 0.5, tilePx * 0.44, 0, Math.PI * 2);
    bCtx.fill();
    bCtx.fillStyle = '#10b981';
    bCtx.fillRect(px + 3, py + 3, 3, 3);
    if ((x + y) % 5 === 0) {
      // Exotic jungle flower
      bCtx.fillStyle = '#f43f5e';
      bCtx.fillRect(px + 6, py + 6, 2, 2);
    }
  } else if (biome === 'CorruptedLands' || biome === 'CorruptedInfernal') {
    // WorldBox Corrupted alien purple biomass / Infernal Wastes
    bCtx.fillStyle = '#3b0764';
    bCtx.fillRect(px, py, tilePx, tilePx);
    bCtx.fillStyle = '#581c87';
    bCtx.fillRect(px + 1, py + 1, tilePx - 2, tilePx - 2);
    // Glowing eerie eyes / spores
    bCtx.fillStyle = '#c084fc';
    bCtx.fillRect(px + 3, py + 3, 2, 2);
    bCtx.fillRect(px + 8, py + 7, 2, 2);
    bCtx.fillStyle = '#d946ef';
    bCtx.fillRect(px + 4, py + 4, 1, 1);
  } else if (biome === 'PermafrostTundra') {
    // Subzero glacial permafrost tundra
    bCtx.fillStyle = '#38bdf8';
    bCtx.fillRect(px, py, tilePx, tilePx);
    bCtx.fillStyle = '#7dd3fc';
    bCtx.fillRect(px + 1, py + 1, tilePx - 2, tilePx - 2);
    bCtx.fillStyle = '#bae6fd';
    bCtx.fillRect(px + 3, py + 2, 3, 2);
    bCtx.fillRect(px + 6, py + 6, 4, 2);
    // Frost crystal sparkle
    bCtx.fillStyle = '#ffffff';
    bCtx.fillRect(px + 4, py + 3, 1, 1);
  } else {
    // Sunlit Plains
    bCtx.fillStyle = '#3f8022';
    bCtx.fillRect(px, py, tilePx, tilePx);
    bCtx.fillStyle = '#4ade80';
    if ((x + y) % 3 === 0) {
      bCtx.fillRect(px + 3, py + 4, 1, 3);
      bCtx.fillRect(px + 8, py + 7, 1, 2);
    }
    if ((x * 3 + y * 7) % 11 === 0) {
      bCtx.fillStyle = '#fbbf24';
      bCtx.fillRect(px + 4, py + 3, 2, 3);
      bCtx.fillStyle = '#f59e0b';
      bCtx.fillRect(px + 4, py + 6, 1, 2);
    } else if ((x * 5 + y * 13) % 17 === 0) {
      bCtx.fillStyle = '#f43f5e';
      bCtx.fillRect(px + 7, py + 6, 2, 2);
    }
  }
}

function buildTerrainBuffer() {
  for (let y = 0; y < WORLD_SIZE; y++) {
    for (let x = 0; x < WORLD_SIZE; x++) {
      const override = terraformedTiles.get(`${x},${y}`);
      const biome = override || getProceduralBiome(x, y);
      paintTileToBuffer(terrainCtx, x, y, biome);
    }
  }
  isTerrainBufferBuilt = true;
}

// Camera Matrix
const camera = {
  x: 0,
  y: 0,
  zoom: 0.45,
  minZoom: 0.18,
  maxZoom: 5.0,
  isDragging: false,
  dragStartX: 0,
  dragStartY: 0,
  panStartX: 0,
  panStartY: 0,
};

function resizeCanvas() {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}
window.addEventListener('resize', resizeCanvas);
resizeCanvas();

function centerCamera() {
  const worldPx = WORLD_SIZE * TILE_PX;
  camera.zoom = Math.min((canvas.width * 0.92) / worldPx, (canvas.height * 0.88) / worldPx);
  camera.x = (canvas.width - worldPx * camera.zoom) / 2;
  camera.y = (canvas.height - worldPx * camera.zoom) / 2;
}

// Active God Power & Brush State
let activeCategory = 'nature';
let activeTool = 'brush-plains';
let brushRadius = 1; // 1, 2, 3, 5
let mouseWorldTile = { x: 32, y: 32 };
let isMouseDownOnMap = false;

// Active FX System (Lightning strikes, screen shakes, floating smoke)
const activeEffects = [];
const smokeParticles = [];

// Lightweight Web Audio Synthesizer (Zero asset dependency)
// Sound Engine: Physical .wav audio asset dispatch with fallback
function playSound(type) {
  if (window.soundManager) {
    if (type === 'click') {
      window.soundManager.playSfx('click');
      return;
    } else if (type === 'thunder' || type === 'explosion') {
      window.soundManager.playSfx('explosion');
      return;
    } else if (type === 'chime') {
      window.soundManager.playSfx('chime');
      return;
    } else if (type === 'mining') {
      window.soundManager.playSfx('mining');
      return;
    } else if (type === 'titan_roar' || type === 'roar') {
      window.soundManager.playSfx('titan_roar');
      return;
    } else if (type === 'victory') {
      window.soundManager.playSfx('victory');
      return;
    } else if (type === 'defeat') {
      window.soundManager.playSfx('defeat');
      return;
    }
  }

  // Fallback to minimal Web Audio oscillator if soundManager is unready
  const fallbackCtx = window.soundManager ? window.soundManager.ctx : null;
  if (!fallbackCtx) return;
  try {
    if (fallbackCtx.state === 'suspended') fallbackCtx.resume();
    const osc = fallbackCtx.createOscillator();
    const gain = fallbackCtx.createGain();
    osc.connect(gain);
    gain.connect(fallbackCtx.destination);
    const now = fallbackCtx.currentTime;

    if (type === 'click') {
      osc.frequency.setValueAtTime(540, now);
      osc.frequency.exponentialRampToValueAtTime(320, now + 0.04);
      gain.gain.setValueAtTime(0.08, now);
      gain.gain.exponentialRampToValueAtTime(0.001, now + 0.04);
      osc.start(now);
      osc.stop(now + 0.04);
    } else if (type === 'thunder' || type === 'explosion') {
      osc.type = 'sawtooth';
      osc.frequency.setValueAtTime(140, now);
      osc.frequency.exponentialRampToValueAtTime(30, now + 0.4);
      gain.gain.setValueAtTime(0.25, now);
      gain.gain.exponentialRampToValueAtTime(0.001, now + 0.45);
      osc.start(now);
      osc.stop(now + 0.45);
    } else if (type === 'chime') {
      osc.type = 'sine';
      osc.frequency.setValueAtTime(784, now);
      osc.frequency.exponentialRampToValueAtTime(1046, now + 0.15);
      gain.gain.setValueAtTime(0.12, now);
      gain.gain.exponentialRampToValueAtTime(0.001, now + 0.2);
      osc.start(now);
      osc.stop(now + 0.2);
    }
  } catch (e) {}
}

// ----------------------------------------------------------------------------
// WEBSOCKET COMMUNICATION
// ----------------------------------------------------------------------------
function connectWebSocket() {
  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  const wsUrl = `${protocol}//${window.location.host}/ws`;

  ws = new WebSocket(wsUrl);

  ws.onopen = () => {
    console.log('[*] Connected to Genesis Cogniverse WebSocket.');
    sendMsg({ type: 'RequestFullMap' });
    if (selectedAgentId) {
      sendMsg({ type: 'SelectAgent', agent_id: selectedAgentId });
    }
  };

  ws.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data);
      currentTelemetry = data;
      if (data.exported_snapshot) {
        handleReceivedExportSnapshot(data.exported_snapshot);
      }
      renderTelemetry(data);
    } catch (e) {
      console.error('Error parsing telemetry:', e);
    }
  };

  ws.onclose = () => {
    setTimeout(connectWebSocket, 2000);
  };
}

function sendMsg(obj) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify(obj));
    return true;
  } else {
    console.warn('Cannot send message, WebSocket is not OPEN:', ws ? ws.readyState : 'no ws', obj);
    showToast('⚠️ Reconnecting to world simulation server... please wait');
    return false;
  }
}

// ----------------------------------------------------------------------------
// CHRONOS TIME DILATION
// ----------------------------------------------------------------------------
function setSpeed(spd) {
  playSound('click');
  sendMsg({ type: 'SetSpeedMultiplier', speed: spd });
  document.querySelectorAll('.btn-wb-time').forEach(b => b.classList.remove('active'));
  document.querySelectorAll('.btn-retro-speed').forEach(b => {
    if (b.id !== 'retro-btn-view-toggle') b.classList.remove('active');
  });
  if (spd === 0) {
    const p1 = document.getElementById('btn-pause'); if (p1) p1.classList.add('active');
    const p2 = document.getElementById('retro-btn-pause'); if (p2) p2.classList.add('active');
  } else if (spd === 1) {
    const b1 = document.getElementById('btn-1x'); if (b1) b1.classList.add('active');
    const b2 = document.getElementById('retro-btn-1x'); if (b2) b2.classList.add('active');
  } else if (spd === 2) {
    const b1 = document.getElementById('btn-2x'); if (b1) b1.classList.add('active');
    const b2 = document.getElementById('retro-btn-2x'); if (b2) b2.classList.add('active');
  } else if (spd === 4 || spd === 5) {
    const b1 = document.getElementById('btn-5x'); if (b1) b1.classList.add('active');
    const b2 = document.getElementById('retro-btn-4x'); if (b2) b2.classList.add('active');
  } else if (spd >= 8) {
    const b1 = document.getElementById('btn-turbo'); if (b1) b1.classList.add('active');
    const b2 = document.getElementById('retro-btn-8x'); if (b2) b2.classList.add('active');
  }
}

function stepTick() {
  playSound('click');
  sendMsg({ type: 'StepSingleTick' });
}

// ----------------------------------------------------------------------------
// WORLDBOX BOTTOM DOCK & TOOLS
// ----------------------------------------------------------------------------
function switchCategory(cat) {
  playSound('click');
  activeCategory = cat;
  document.querySelectorAll('.wb-tab-btn').forEach(t => t.classList.remove('active'));
  document.querySelectorAll('.wb-sub-tray').forEach(tray => tray.classList.remove('active'));

  const tab = document.getElementById(`tab-cat-${cat}`);
  const tray = document.getElementById(`sub-tray-${cat}`);
  if (tab) tab.classList.add('active');
  if (tray) tray.classList.add('active');

  if (cat === 'nature') selectGodTool('brush-plains', 'Plains', 'Fertilize meadow plains');
  else if (cat === 'architecture') selectGodTool('build-cabin', 'Stone Cabin', 'Erect a sturdy permanent Stone Cabin residence (capacity 4)');
  else if (cat === 'civilization') selectGodTool('spawn-solar', 'Spawn Solar Terran', 'Drop a human Solar Terran pioneer');
  else if (cat === 'creatures') selectGodTool('spawn-deer', 'Spawn Deer', 'Drop an herbivorous forest deer');
  else if (cat === 'cataclysms') selectGodTool('smite-lightning', 'Lightning Smite', 'Strike targeted tile with divine lightning');
}

function selectGodTool(toolId, name, desc) {
  playSound('click');
  activeTool = toolId;
  document.querySelectorAll('.wb-tool-btn').forEach(b => {
    if (b.dataset.tool === toolId) b.classList.add('active');
    else b.classList.remove('active');
  });

  if (name) {
    showToast(`Power: ${name} (${brushRadius}x${brushRadius})`);
  }
}

function setBrushRadius(rad) {
  playSound('click');
  brushRadius = rad;
  document.querySelectorAll('.btn-brush-size').forEach(b => b.classList.remove('active'));
  const btn = document.getElementById(`btn-brush-${rad}`);
  if (btn) btn.classList.add('active');
  showToast(`Brush Size: ${rad}x${rad}`);
}

function showToast(msg) {
  if (!toastEl) return;
  toastEl.textContent = msg;
  toastEl.classList.remove('hidden');
  clearTimeout(toastEl._timer);
  toastEl._timer = setTimeout(() => {
    toastEl.classList.add('hidden');
  }, 1600);
}

function toggleBorders() {
  playSound('click');
  showKingdomBorders = !showKingdomBorders;
  const btn = document.getElementById('btn-borders-toggle');
  btn.style.borderColor = showKingdomBorders ? 'var(--wb-gold)' : 'rgba(255,255,255,0.12)';
  showToast(showKingdomBorders ? 'Kingdom Borders: ON' : 'Kingdom Borders: OFF');
}

// ----------------------------------------------------------------------------
// CAMERA PAN & ZOOM
// ----------------------------------------------------------------------------
function screenToWorld(clientX, clientY) {
  const wx = Math.floor((clientX - camera.x) / (TILE_PX * camera.zoom));
  const wy = Math.floor((clientY - camera.y) / (TILE_PX * camera.zoom));
  return { x: wx, y: wy };
}

function zoomIn() {
  applyZoom(1.3, canvas.width / 2, canvas.height / 2);
}

function zoomOut() {
  applyZoom(0.77, canvas.width / 2, canvas.height / 2);
}

function resetCamera() {
  playSound('click');
  centerCamera();
  isFollowActive = false;
  updateFollowBtn();
}

function applyZoom(factor, focusX, focusY) {
  const oldZoom = camera.zoom;
  let newZoom = Math.min(camera.maxZoom, Math.max(camera.minZoom, oldZoom * factor));
  if (newZoom === oldZoom) return;

  camera.x = focusX - (focusX - camera.x) * (newZoom / oldZoom);
  camera.y = focusY - (focusY - camera.y) * (newZoom / oldZoom);
  camera.zoom = newZoom;
  checkViewModeTransition();
}

let isCtrlOrMetaPressed = false;
let isSpacePressed = false;

window.addEventListener('keydown', (e) => {
  if (e.key === 'Control' || e.key === 'Meta') {
    isCtrlOrMetaPressed = true;
    if (!camera.isDragging) container.style.cursor = 'grab';
  }
  if (e.code === 'Space' && !e.target.matches('input, textarea')) {
    isSpacePressed = true;
    if (!camera.isDragging) container.style.cursor = 'grab';
    e.preventDefault();
  }
  if ((e.key === 'm' || e.key === 'M') && !e.target.matches('input, textarea')) {
    toggleViewMode();
  }
});

window.addEventListener('keyup', (e) => {
  if (e.key === 'Control' || e.key === 'Meta') {
    isCtrlOrMetaPressed = false;
    if (!camera.isDragging && !isSpacePressed) container.style.cursor = '';
  }
  if (e.code === 'Space') {
    isSpacePressed = false;
    if (!camera.isDragging && !isCtrlOrMetaPressed) container.style.cursor = '';
  }
});

container.addEventListener('wheel', (e) => {
  e.preventDefault();
  const factor = e.deltaY < 0 ? 1.15 : 0.87;
  applyZoom(factor, e.clientX, e.clientY);
}, { passive: false });

container.addEventListener('mousedown', (e) => {
  const isPan = e.button === 1 || e.button === 2 || e.ctrlKey || e.metaKey || isCtrlOrMetaPressed || isSpacePressed;
  if (isPan) {
    camera.isDragging = true;
    camera.dragStartX = e.clientX;
    camera.dragStartY = e.clientY;
    camera.panStartX = camera.x;
    camera.panStartY = camera.y;
    container.classList.add('panning');
    container.style.cursor = 'grabbing';
    e.preventDefault();
    return;
  }

  if (e.button === 0) {
    isMouseDownOnMap = true;
    const { x, y } = screenToWorld(e.clientX, e.clientY);
    handleMapAction(x, y);
  }
});

window.addEventListener('mousemove', (e) => {
  if (camera.isDragging) {
    camera.x = camera.panStartX + (e.clientX - camera.dragStartX);
    camera.y = camera.panStartY + (e.clientY - camera.dragStartY);
    return;
  }

  const { x, y } = screenToWorld(e.clientX, e.clientY);
  mouseWorldTile.x = Math.max(0, Math.min(WORLD_SIZE - 1, x));
  mouseWorldTile.y = Math.max(0, Math.min(WORLD_SIZE - 1, y));

  if (isMouseDownOnMap && activeTool.startsWith('brush-')) {
    handleMapAction(x, y);
  }

  // Hover tile HUD
  if (x >= 0 && x < WORLD_SIZE && y >= 0 && y < WORLD_SIZE) {
    let txt = `Tile (${x}, ${y})`;
    if (currentTelemetry && currentTelemetry.agents) {
      const a = currentTelemetry.agents.find(ag => ag.x === x && ag.y === y);
      if (a) txt += ` | ${a.name} [${a.race === 'SolarTerran' ? 'Solar' : 'Lunar'}]`;
    }
    tileHud.textContent = txt;
    tileHud.style.left = `${e.clientX + 14}px`;
    tileHud.style.top = `${e.clientY + 14}px`;
    tileHud.classList.remove('hidden');
  } else {
    tileHud.classList.add('hidden');
  }
});

window.addEventListener('mouseup', () => {
  if (camera.isDragging) {
    camera.isDragging = false;
    container.classList.remove('panning');
    container.style.cursor = (isCtrlOrMetaPressed || isSpacePressed) ? 'grab' : '';
  }
  isMouseDownOnMap = false;
});

container.addEventListener('contextmenu', (e) => e.preventDefault());

// ----------------------------------------------------------------------------
// GOD ACTIONS & TERRAFORMING HANDLER
// ----------------------------------------------------------------------------
function handleMapAction(tx, ty) {
  if (tx < 0 || tx >= WORLD_SIZE || ty < 0 || ty >= WORLD_SIZE) return;

  // 1a. Architecture Building Placement Tools
  // 1a. Architecture Building Placement Tools
  if (activeTool && activeTool.startsWith('build-')) {
    let brushType = 'BuildCabin';
    let label = 'Stone Cabin';
    if (activeTool === 'build-cottage') { brushType = 'BuildCottage'; label = 'Timber Cottage'; }
    else if (activeTool === 'build-cabin') { brushType = 'BuildCabin'; label = 'Stone Cabin'; }
    else if (activeTool === 'build-townhall') { brushType = 'BuildTownHall'; label = 'Town Hall'; }
    else if (activeTool === 'build-guild' || activeTool === 'build-barracks') { brushType = 'BuildGuildHall'; label = 'Guild Hall Barracks'; }
    else if (activeTool === 'build-campfire') { brushType = 'BuildCampfire'; label = 'Campfire'; }
    else if (activeTool === 'build-granary') { brushType = 'BuildGranary'; label = 'Granary Market'; }
    else if (activeTool === 'build-watchtower') { brushType = 'BuildWatchtower'; label = 'Watchtower Outpost'; }
    else if (activeTool === 'build-mineshaft') { brushType = 'BuildMineShaft'; label = 'Mine Shaft'; }
    else if (activeTool === 'build-pen') { brushType = 'BuildLivestockPen'; label = 'Livestock Pen'; }
    else if (activeTool === 'build-dock') { brushType = 'BuildDock'; label = 'Fishing Dock'; }

    playSound('click');
    sendMsg({
      type: 'TerraformArea',
      brush: brushType,
      center_x: tx,
      center_y: ty,
      radius: 1,
    });
    showToast(`🏗️ Erected ${label} at (${tx}, ${ty})`);
    return;
  }

  // 1b. All 10 Master Mine Types & Excavations
  if (activeTool && activeTool.startsWith('mine-')) {
    let brushType = 'PlaceSurfaceMine';
    let label = 'Surface Pit Mine';
    if (activeTool === 'mine-surface') { brushType = 'PlaceSurfaceMine'; label = 'Surface Pit Mine'; }
    else if (activeTool === 'mine-quarry') { brushType = 'PlaceQuarry'; label = 'Stone Quarry'; }
    else if (activeTool === 'mine-tunnel') { brushType = 'PlaceHillsideTunnel'; label = 'Hillside Tunnel Mine'; }
    else if (activeTool === 'mine-shaft') { brushType = 'PlaceDeepMineShaft'; label = 'Deep Mine Shaft'; }
    else if (activeTool === 'mine-cave') { brushType = 'PlaceCaveMine'; label = 'Natural Cave Mine'; }
    else if (activeTool === 'mine-drift') { brushType = 'PlaceHillsideDrift'; label = 'Hillside Drift Mine'; }
    else if (activeTool === 'mine-dredge') { brushType = 'PlaceUnderwaterDredge'; label = 'Underwater Dredge Mine'; }
    else if (activeTool === 'mine-mana') { brushType = 'PlaceManaCrystalMine'; label = 'Mana Crystal Mine'; }
    else if (activeTool === 'mine-ruin') { brushType = 'PlaceRuinExcavation'; label = 'Ancient Ruin Excavation'; }
    else if (activeTool === 'mine-rift') { brushType = 'PlaceDungeonRiftMine'; label = 'Dungeon Rift Mine'; }

    playSound('click');
    sendMsg({
      type: 'TerraformArea',
      brush: brushType,
      center_x: tx,
      center_y: ty,
      radius: 1,
    });
    showToast(`⛏️ Established ${label} at (${tx}, ${ty})`);
    return;
  }

  // 1c. Wildlife Animal Spawning Brushes
  if (activeTool && activeTool.startsWith('spawn-animal-')) {
    let brushType = 'SpawnAnimalRabbit';
    let label = 'Wild Rabbit';
    if (activeTool === 'spawn-animal-rabbit') { brushType = 'SpawnAnimalRabbit'; label = 'Wild Rabbit'; }
    else if (activeTool === 'spawn-animal-deer') { brushType = 'SpawnAnimalDeer'; label = 'Wild Deer'; }
    else if (activeTool === 'spawn-animal-wolf') { brushType = 'SpawnAnimalWolf'; label = 'Wild Wolf'; }
    else if (activeTool === 'spawn-animal-bear') { brushType = 'SpawnAnimalBear'; label = 'Wild Bear'; }
    else if (activeTool === 'spawn-animal-sheep') { brushType = 'SpawnAnimalSheep'; label = 'Domestic Sheep'; }
    else if (activeTool === 'spawn-animal-cow') { brushType = 'SpawnAnimalCow'; label = 'Domestic Cow'; }

    playSound('chime');
    sendMsg({
      type: 'TerraformArea',
      brush: brushType,
      center_x: tx,
      center_y: ty,
      radius: 1,
    });
    showToast(`🐾 Spawned ${label} at (${tx}, ${ty})`);
    return;
  }

  // 1d. Divine Miracles & Awakening Decrees
  if (activeTool === 'miracle-heal') {
    playSound('chime');
    sendMsg({
      type: 'TerraformArea',
      brush: 'DivineHeal',
      center_x: tx,
      center_y: ty,
      radius: brushRadius || 2,
    });
    showToast(`💖 Cast Divine Spark Heal at (${tx}, ${ty})`);
    return;
  }
  if (activeTool === 'miracle-awaken') {
    playSound('chime');
    sendMsg({
      type: 'TerraformArea',
      brush: 'DivineAwaken',
      center_x: tx,
      center_y: ty,
      radius: 1,
    });
    showToast(`⚡ Issued Divine Decree: Awakening at (${tx}, ${ty})`);
    return;
  }
  if (activeTool === 'miracle-rain') {
    playSound('water');
    triggerGodWeather('rain');
    showToast(`🌧️ Summoned Rainstorm over (${tx}, ${ty})`);
    return;
  }

  // 1e. Terraforming, Elevation & Ore Brushes
  if (activeTool && activeTool.startsWith('brush-')) {
    let brushType = 'FertilizePlains';
    let biomeKey = 'SunlitPlains';

    if (activeTool === 'brush-raise') {
      brushType = 'RaiseTerrain';
      biomeKey = null;
    } else if (activeTool === 'brush-lower') {
      brushType = 'LowerTerrain';
      biomeKey = null;
    } else if (activeTool === 'brush-flatten') {
      brushType = 'FlattenTerrain';
      biomeKey = null;
    } else if (activeTool === 'brush-forest') {
      brushType = 'PlantForest';
      biomeKey = 'WhisperingForest';
    } else if (activeTool === 'brush-river') {
      brushType = 'CarveRiver';
      biomeKey = 'CrystalRiverlands';
    } else if (activeTool === 'brush-ocean') {
      brushType = 'CarveRiver';
      biomeKey = 'DeepOcean';
    } else if (activeTool === 'brush-mountains') {
      brushType = 'RaiseMountain';
      biomeKey = 'IroncragMountains';
    } else if (activeTool === 'brush-ruins') {
      brushType = 'InfuseLeylineRuins';
      biomeKey = 'AncientLeylineRuins';
    } else if (activeTool === 'brush-iron') {
      brushType = 'EnrichIronVein';
      biomeKey = 'IroncragMountains';
    } else if (activeTool === 'brush-gold') {
      brushType = 'EnrichGoldVein';
      biomeKey = 'IroncragMountains';
    } else if (activeTool === 'brush-mithril') {
      brushType = 'EnrichMithrilVein';
      biomeKey = 'IroncragMountains';
    } else if (activeTool === 'brush-savanna') {
      brushType = 'SeedSavanna';
      biomeKey = 'SavannaGrasslands';
    } else if (activeTool === 'brush-jungle') {
      brushType = 'SeedJungle';
      biomeKey = 'VerdantJungle';
    } else if (activeTool === 'brush-corrupted') {
      brushType = 'SeedCorrupted';
      biomeKey = 'CorruptedLands';
    } else if (activeTool === 'brush-permafrost') {
      brushType = 'SeedPermafrost';
      biomeKey = 'PermafrostTundra';
    } else if (activeTool === 'brush-lava') {
      brushType = 'SpawnLava';
      biomeKey = null;
    } else if (activeTool === 'brush-fire') {
      brushType = 'IgniteWildfire';
      biomeKey = null;
    } else if (activeTool === 'brush-freeze') {
      brushType = 'FreezeIce';
      biomeKey = null;
    } else if (activeTool === 'brush-thaw') {
      brushType = 'ThawHeat';
      biomeKey = null;
    } else if (activeTool === 'brush-clear') {
      brushType = 'ClearLand';
      biomeKey = 'SunlitPlains';
    }

    playSound('click');
    if (biomeKey) {
      const half = Math.floor(brushRadius / 2);
      for (let dy = -half; dy <= half; dy++) {
        for (let dx = -half; dx <= half; dx++) {
          const px = tx + dx;
          const py = ty + dy;
          if (px >= 0 && px < WORLD_SIZE && py >= 0 && py < WORLD_SIZE) {
            terraformedTiles.set(`${px},${py}`, biomeKey);
            paintTileToBuffer(terrainCtx, px, py, biomeKey);
          }
        }
      }
    }

    sendMsg({
      type: 'TerraformArea',
      brush: brushType,
      center_x: tx,
      center_y: ty,
      radius: brushRadius,
    });
    return;
  }

  // 2. Civilizations Spawning
  if (activeTool === 'spawn-solar') {
    playSound('chime');
    sendMsg({ type: 'SpawnAgentAt', race: 'SolarTerran', x: tx, y: ty });
    showToast(`☀️ Solar Terran pioneer spawned at (${tx}, ${ty})`);
    return;
  }
  if (activeTool === 'spawn-lunar') {
    playSound('chime');
    sendMsg({ type: 'SpawnAgentAt', race: 'ChthonicLunarian', x: tx, y: ty });
    showToast(`🌙 Lunarian wanderer spawned at (${tx}, ${ty})`);
    return;
  }

  // 3. Cataclysms & Miracles
  if (activeTool === 'smite-lightning') {
    playSound('thunder');
    activeEffects.push({
      type: 'lightning',
      x: tx,
      y: ty,
      start: Date.now(),
      duration: 380,
    });
    sendMsg({
      type: 'TriggerIntervention',
      intervention: { SmiteLightning: { x: tx, y: ty } }
    });
    showToast(`⚡ Wrathful Smite struck at (${tx}, ${ty})`);
    return;
  }
  if (activeTool === 'smite-inspiration') {
    playSound('chime');
    sendMsg({
      type: 'TriggerIntervention',
      intervention: { DivineInspiration: { x: tx, y: ty } }
    });
    showToast(`🌟 Divine Inspiration beamed upon (${tx}, ${ty})`);
    return;
  }
  if (activeTool === 'curse-doubt') {
    playSound('thunder');
    sendMsg({
      type: 'TriggerIntervention',
      intervention: { CurseOfDoubt: { x: tx, y: ty } }
    });
    showToast(`💀 Curse of Doubt cast over mortals at (${tx}, ${ty})`);
    return;
  }
  if (activeTool === 'cataclysm-earthquake') {
    playSound('thunder');
    sendMsg({
      type: 'TriggerIntervention',
      intervention: { CataclysmEarthquake: { x: tx, y: ty } }
    });
    showToast(`🌋 Tectonic Earthquake fractured crust at (${tx}, ${ty})`);
    return;
  }
  if (activeTool === 'manna-drop') {
    playSound('chime');
    sendMsg({
      type: 'TriggerIntervention',
      intervention: { MiracleManna: { x: tx, y: ty } }
    });
    showToast(`🍞 Manna harvest dropped at (${tx}, ${ty})`);
    return;
  }
  if (activeTool === 'blessing-spark') {
    playSound('chime');
    sendMsg({
      type: 'TriggerIntervention',
      intervention: { DivineInspiration: { x: tx, y: ty } }
    });
    showToast(`✨ Divine Blessing cast over (${tx}, ${ty})`);
    return;
  }

  // 4. Default: Character Inspection
  if (currentTelemetry && currentTelemetry.agents) {
    let closest = null;
    let minD = 3;
    for (const a of currentTelemetry.agents) {
      const d = Math.hypot(a.x - tx, a.y - ty);
      if (d < minD) {
        minD = d;
        closest = a;
      }
    }

    if (closest) {
      playSound('click');
      selectedAgentId = closest.id;
      sendMsg({ type: 'SelectAgent', agent_id: closest.id });
      openNpcInspector(closest);
      return;
    }
  }
}

// ----------------------------------------------------------------------------
// CAMERA TRACKING & FAVORITES
// ----------------------------------------------------------------------------
function toggleFavoriteCurrentAgent() {
  playSound('chime');
  const star = document.getElementById('cs-star-btn');
  if (favoriteAgentId === selectedAgentId) {
    favoriteAgentId = null;
    star.classList.remove('favorited');
    showToast('Unfavorited Character');
  } else {
    favoriteAgentId = selectedAgentId;
    star.classList.add('favorited');
    showToast('⭐ Character Favorited!');
  }
}

function toggleFollowSelected() {
  playSound('click');
  isFollowActive = !isFollowActive;
  updateFollowBtn();
}

function updateFollowBtn() {
  const btn = document.getElementById('btn-follow');
  if (isFollowActive) {
    btn.classList.add('active-follow');
    showToast('🔭 Camera Follow: ON');
  } else {
    btn.classList.remove('active-follow');
    showToast('🔭 Camera Follow: OFF');
  }
}

function centerOnLeader() {
  if (!currentTelemetry || !currentTelemetry.agents || currentTelemetry.agents.length === 0) return;
  playSound('click');
  let leader = currentTelemetry.agents[0];
  for (const a of currentTelemetry.agents) {
    if (a.generation > leader.generation || (a.generation === leader.generation && a.piety > leader.piety)) {
      leader = a;
    }
  }
  if (leader) {
    selectedAgentId = leader.id;
    sendMsg({ type: 'SelectAgent', agent_id: leader.id });
    centerCameraOn(leader.x, leader.y);
    openCharacterSheet();
    showToast(`👑 Centered on Civilizational King: ${leader.name}`);
  }
}

function centerCameraOn(wx, wy) {
  const tilePx = TILE_PX * camera.zoom;
  camera.x = canvas.width / 2 - (wx + 0.5) * tilePx;
  camera.y = canvas.height / 2 - (wy + 0.5) * tilePx;
}

// ----------------------------------------------------------------------------
// TELEMETRY RENDER PIPELINE
// ----------------------------------------------------------------------------
function renderTelemetry(data) {
  try {
    // 0. Process Full Map Sync if received from server
    if (data.full_map_sync && typeof data.full_map_sync === 'string') {
      try {
        const decoded = atob(data.full_map_sync);
        terraformedTiles.clear();
        for (let y = 0; y < WORLD_SIZE; y++) {
          for (let x = 0; x < WORLD_SIZE; x++) {
            const idx = y * WORLD_SIZE + x;
            if (idx < decoded.length) {
              const code = decoded.charCodeAt(idx);
              const biome = BIOME_LOOKUP[code] || 'SunlitPlains';
              terraformedTiles.set(`${x},${y}`, biome);
              paintTileToBuffer(terrainCtx, x, y, biome);
            }
          }
        }
        isTerrainBufferBuilt = true;
        showToast(`Realm Synchronized: ${data.map_name || 'Genesis'}`);
      } catch (err) {
        console.error('Error applying full map sync:', err);
      }
    }

    if (data.map_name) {
      const hudTitle = document.getElementById('hud-world-name');
      if (hudTitle) hudTitle.textContent = data.map_name;
      const modalTitle = document.getElementById('active-realm-name');
      if (modalTitle) modalTitle.textContent = data.map_name;
    }
    if (data.map_archetype) {
      const archTag = document.getElementById('active-realm-archetype-tag');
      if (archTag) archTag.textContent = data.map_archetype;
    }
    if (data.map_id !== undefined && data.map_id !== null) {
      const mapIdEl = document.getElementById('active-realm-id');
      if (mapIdEl) mapIdEl.textContent = data.map_id;
      if (activeMapId !== null && activeMapId !== data.map_id) {
        activeMapId = data.map_id;
        centerCamera();
      } else {
        activeMapId = data.map_id;
      }
    }

    // 1. Sync modified tiles to offscreen buffer
    if (data.modified_tiles && Array.isArray(data.modified_tiles)) {
      for (const [x, y, biome] of data.modified_tiles) {
        terraformedTiles.set(`${x},${y}`, biome);
        paintTileToBuffer(terrainCtx, x, y, biome);
      }
    }

    // 2. Top HUD Demographics
    const d = data.demographics;
    if (d) {
      document.getElementById('stat-pop').textContent = d.total_population ?? 0;
      document.getElementById('stat-fauna').textContent = d.wildlife_count ?? 0;
      document.getElementById('stat-births').textContent = d.births_total ?? 0;
      document.getElementById('stat-deaths').textContent = d.deaths_total ?? 0;
      document.getElementById('stat-gen').textContent = `Gen ${d.max_generation ?? 1}`;
      document.getElementById('stat-hp').textContent = `${(d.avg_health ?? 100).toFixed(0)}%`;
      document.getElementById('stat-hunger').textContent = `${((d.avg_hunger ?? 0.2) * 100).toFixed(0)}%`;
      document.getElementById('stat-wealth').textContent = Math.round(d.total_circulating_wealth ?? 0);
    }

    if (data.structures) {
      document.getElementById('stat-villages').textContent = data.structures.filter(s => s.kind === 'StoneCabin' || s.kind === 'LeanTo').length || 2;
    }

    // World date calculation
    const tick = data.world_tick ?? 0;
    const year = Math.floor(tick / 1000) + 1;
    const month = (Math.floor(tick / 80) % 12) + 1;
    document.getElementById('hud-date').textContent = `Year ${year}, Month ${month} (Tick ${tick})`;

    // Weather pill
    const wIco = data.weather === 'Thunderstorm' ? '⛈️' : data.weather === 'GentleRain' ? '🌧️' : data.weather === 'FrostBreeze' ? '❄️' : '☀️';
    document.getElementById('stat-weather-ico').textContent = wIco;
    document.getElementById('stat-weather').textContent = data.weather ?? 'Clear Sun';

    // WorldBox-style Treaty & Diplomacy Pill
    const dipEl = document.getElementById('stat-diplomacy');
    if (dipEl && data.diplomacy_status) {
      dipEl.textContent = data.diplomacy_status;
    }

    // 3. Auto-follow tracked character
    if (isFollowActive && data.agents) {
      const tracked = data.agents.find(a => a.id === (favoriteAgentId || selectedAgentId));
      if (tracked) {
        centerCameraOn(tracked.x, tracked.y);
      }
    }

    // 4. Render Character Sheet Details if modal is open
    const charSheetModal = document.getElementById('modal-character-sheet');
    if (charSheetModal && !charSheetModal.classList.contains('hidden') && data.selected_agent) {
      updateCharacterSheet(data.selected_agent);
    }

    // 6. Covenants, Altars & Chronicle (efficient delta rendering)
    renderCovenants(data.active_covenants);
    renderAltars(data.altars);
    renderChronicle(data.event_log);

    // 7. Knowledge & Activity Codex (ONLY render if modal is open to avoid DOM thrashing)
    const codexModal = document.getElementById('modal-codex');
    if (codexModal && !codexModal.classList.contains('hidden')) {
      if (data.activities) {
        renderCodexActivities(data.activities);
      }
      if (data.terran_culture || data.lunar_culture) {
        renderCodexTechTree(data.terran_culture, data.lunar_culture);
        renderCodexDiscoveries(data.terran_culture, data.lunar_culture);
      }
      if (data.active_plots) {
        renderCodexPlots(data.active_plots, data.terran_treasury, data.lunar_treasury, data.diplomacy_status);
      }
    }

    // 8. Sovereign War Room (ONLY render if modal is open)
    const warRoomModal = document.getElementById('war-room-modal');
    if (warRoomModal && !warRoomModal.classList.contains('hidden')) {
      renderWarRoom(data);
    }

    // 9. Retro Wooden HUD, Stockpiles & Minimap
    updateRetroStatusHud(data);
    renderRetroMinimap(data);

    if (data.selected_agent) {
      const npcInsp = document.getElementById('bottom-npc-inspector');
      if (npcInsp && !npcInsp.classList.contains('hidden')) {
        openNpcInspector(data.selected_agent);
      }
    }
  } catch (err) {
    console.error('Telemetry render error:', err);
  }
}

// ----------------------------------------------------------------------------
// LIVING 2D WORLD MAP RENDERER (WORLDBOX VISUALS)
// ----------------------------------------------------------------------------
function drawWorldMap(data) {
  if (isWorldMapActive) {
    drawCosmicWorldMap(data, Date.now());
    return;
  }

  if (!isTerrainBufferBuilt) {
    buildTerrainBuffer();
  }

  const oceanTheme = getThemeOceanColors();
  ctx.save();
  ctx.fillStyle = oceanTheme.backdrop;
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  // Apply Camera Translation & Zoom
  ctx.translate(camera.x, camera.y);
  ctx.scale(camera.zoom, camera.zoom);

  const tilePx = TILE_PX;
  const time = Date.now();

  // 0. Boundless WorldBox Deep Abyss Ocean Backdrop
  const oceanMargin = 40 * tilePx;
  ctx.fillStyle = oceanTheme.deep; // Deep abyssal ocean
  ctx.fillRect(-oceanMargin, -oceanMargin, WORLD_SIZE * tilePx + oceanMargin * 2, WORLD_SIZE * tilePx + oceanMargin * 2);

  // Animated ocean wave lines outside continent
  ctx.fillStyle = oceanTheme.wave;
  for (let oy = -36; oy < WORLD_SIZE + 36; oy += 6) {
    const waveOffset = Math.sin(time * 0.0018 + oy * 0.35) * 6;
    ctx.fillRect(-oceanMargin, oy * tilePx + waveOffset, WORLD_SIZE * tilePx + oceanMargin * 2, 2);
  }

  // 1. Blit cached 128x128 grand continent from offscreen buffer (1 single lightning-fast GPU draw call!)
  ctx.drawImage(terrainCanvas, 0, 0);

  // 1.5 WorldBox Dynamic Cellular Automata Effects (Lava, Fire, Ash, Ice, Snow)
  if (data.dynamic_tiles && Array.isArray(data.dynamic_tiles)) {
    for (const dt of data.dynamic_tiles) {
      const px = dt.x * tilePx;
      const py = dt.y * tilePx;

      // Charred Ash Decay overlay
      if (dt.burned_stages > 0 && !dt.is_on_fire && dt.lava_level === 0) {
        ctx.fillStyle = 'rgba(28, 25, 23, 0.75)';
        ctx.fillRect(px, py, tilePx, tilePx);
        ctx.fillStyle = 'rgba(87, 83, 78, 0.5)';
        ctx.fillRect(px + 2, py + 3, tilePx - 4, 2);
        ctx.fillRect(px + 4, py + 7, 3, 2);
      }

      // Frozen Ice / Snow State
      if (dt.frozen_state === 1) { // Walkable Ice
        ctx.fillStyle = 'rgba(186, 230, 253, 0.78)';
        ctx.fillRect(px, py, tilePx, tilePx);
        ctx.strokeStyle = 'rgba(255, 255, 255, 0.85)';
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.moveTo(px + 2, py + tilePx - 2);
        ctx.lineTo(px + tilePx * 0.5, py + tilePx * 0.4);
        ctx.lineTo(px + tilePx - 3, py + 2);
        ctx.stroke();
      } else if (dt.frozen_state === 2) { // Snow
        ctx.fillStyle = 'rgba(248, 250, 252, 0.85)';
        ctx.fillRect(px, py, tilePx, tilePx);
        ctx.fillStyle = '#e2e8f0';
        ctx.fillRect(px + 3, py + 3, tilePx - 6, tilePx - 6);
      }

      // Molten Lava (downhill flow & heat glow)
      if (dt.lava_level > 0) {
        const pulse = Math.sin(time * 0.008 + dt.x * 2 + dt.y * 3) * 0.15;
        ctx.fillStyle = dt.lava_level >= 3 ? '#ef4444' : dt.lava_level === 2 ? '#f97316' : '#ea580c';
        ctx.fillRect(px, py, tilePx, tilePx);
        ctx.fillStyle = `rgba(254, 240, 138, ${0.75 + pulse})`;
        ctx.fillRect(px + 3, py + 3, tilePx - 6, tilePx - 6);
        ctx.fillStyle = '#ffffff';
        ctx.fillRect(px + 4, py + 4, 2, 2);
        // Ambient heat radiation
        ctx.fillStyle = 'rgba(239, 68, 68, 0.22)';
        ctx.beginPath();
        ctx.arc(px + tilePx / 2, py + tilePx / 2, tilePx * 1.5, 0, Math.PI * 2);
        ctx.fill();
      }

      // Spreading Wildfire Flames
      if (dt.is_on_fire) {
        const fPulse = Math.sin(time * 0.02 + dt.x * 7 + dt.y * 11) * 2;
        ctx.fillStyle = '#ef4444';
        ctx.beginPath();
        ctx.arc(px + tilePx / 2, py + tilePx / 2, tilePx * 0.45 + fPulse * 0.5, 0, Math.PI * 2);
        ctx.fill();

        ctx.fillStyle = '#f97316';
        ctx.beginPath();
        ctx.moveTo(px + 2, py + tilePx - 1);
        ctx.lineTo(px + tilePx / 2, py + 1 - Math.abs(fPulse));
        ctx.lineTo(px + tilePx - 2, py + tilePx - 1);
        ctx.closePath();
        ctx.fill();

        ctx.fillStyle = '#fef08a';
        ctx.beginPath();
        ctx.moveTo(px + 4, py + tilePx - 2);
        ctx.lineTo(px + tilePx / 2, py + 4 - Math.abs(fPulse));
        ctx.lineTo(px + tilePx - 4, py + tilePx - 2);
        ctx.closePath();
        ctx.fill();

        if (Math.random() < 0.12) {
          smokeParticles.push({
            x: px + tilePx / 2 + (Math.random() - 0.5) * 4,
            y: py + 2,
            alpha: 0.6,
            size: 2.2,
          });
        }
      }
    }
  }

  // 2. Kingdom Territorial Borders (WorldBox Signature Feature)
  if (showKingdomBorders) {
    // Solar Terran Kingdom (Gold borders around southern fertile valley 38, 86)
    ctx.save();
    ctx.strokeStyle = 'rgba(251, 191, 36, 0.8)';
    ctx.lineWidth = 2.5;
    ctx.fillStyle = 'rgba(251, 191, 36, 0.09)';
    ctx.setLineDash([5, 3]);

    ctx.beginPath();
    ctx.arc(38 * tilePx, 86 * tilePx, 24 * tilePx, 0, Math.PI * 2);
    ctx.stroke();
    ctx.fill();

    // Lunar Kingdom (Violet borders around northern cavern ruins 92, 34)
    ctx.strokeStyle = 'rgba(192, 132, 252, 0.8)';
    ctx.fillStyle = 'rgba(192, 132, 252, 0.09)';
    ctx.beginPath();
    ctx.arc(92 * tilePx, 34 * tilePx, 22 * tilePx, 0, Math.PI * 2);
    ctx.stroke();
    ctx.fill();
    ctx.restore();
  }

  // 3. Altars (Holy Sites)
  if (data.altars) {
    for (const altar of data.altars) {
      const px = altar.x * tilePx;
      const py = altar.y * tilePx;

      ctx.save();
      const isSolar = altar.deity === 'SolarPlayerGod';
      ctx.fillStyle = isSolar ? 'rgba(251, 191, 36, 0.35)' : 'rgba(192, 132, 252, 0.35)';
      ctx.beginPath();
      ctx.arc(px + tilePx / 2, py + tilePx / 2, tilePx * 2.2, 0, Math.PI * 2);
      ctx.fill();

      // Altar Stone Monolith
      ctx.fillStyle = isSolar ? '#fbbf24' : '#c084fc';
      ctx.fillRect(px + 1, py + 1, tilePx - 2, tilePx - 2);
      ctx.fillStyle = isSolar ? '#78350f' : '#3b0764';
      ctx.fillRect(px + 4, py + 4, tilePx - 8, tilePx - 8);
      ctx.restore();
    }
  }

  // 4. Placed Village Structures (Cabins, Cottages, Granaries, Campfires, Watchtowers, Mine Shafts)
  if (data.structures) {
    for (const s of data.structures) {
      const px = s.x * tilePx;
      const py = s.y * tilePx;

      if (s.kind === 'Campfire') {
        // Communal Fire Pit & Hearth
        ctx.fillStyle = '#475569';
        ctx.beginPath();
        ctx.arc(px + tilePx / 2, py + tilePx / 2 + 3, 5, 0, Math.PI * 2);
        ctx.fill();

        ctx.fillStyle = '#78350f';
        ctx.fillRect(px + 3, py + 9, tilePx - 6, 3);

        const flicker = Math.sin(time * 0.02 + s.x) * 2;
        ctx.fillStyle = '#ea580c';
        ctx.beginPath();
        ctx.arc(px + tilePx / 2, py + tilePx / 2, 4 + flicker, 0, Math.PI * 2);
        ctx.fill();

        ctx.fillStyle = '#facc15';
        ctx.beginPath();
        ctx.arc(px + tilePx / 2, py + tilePx / 2 - 1, 2.2, 0, Math.PI * 2);
        ctx.fill();

        // Warm ambient fire glow
        ctx.fillStyle = 'rgba(251, 191, 36, 0.15)';
        ctx.beginPath();
        ctx.arc(px + tilePx / 2, py + tilePx / 2, tilePx * 1.8, 0, Math.PI * 2);
        ctx.fill();

        // Emit smoke particle occasionally
        if (Math.random() < 0.08) {
          smokeParticles.push({
            x: px + tilePx / 2,
            y: py + 4,
            alpha: 0.6,
            size: 2,
          });
        }
      } else if (s.kind === 'StoneCabin') {
        // Sturdy Stone Cabin residence
        ctx.fillStyle = '#334155';
        ctx.fillRect(px + 1, py + 4, tilePx - 2, tilePx - 4);
        ctx.fillStyle = '#64748b';
        ctx.fillRect(px + 2, py + 5, tilePx - 4, tilePx - 6);

        // Pitched Red Shingle Roof
        ctx.fillStyle = '#991b1b';
        ctx.beginPath();
        ctx.moveTo(px + tilePx / 2, py);
        ctx.lineTo(px + tilePx + 1, py + 6);
        ctx.lineTo(px - 1, py + 6);
        ctx.closePath();
        ctx.fill();

        // Chimney & Smoke
        ctx.fillStyle = '#475569';
        ctx.fillRect(px + tilePx - 4, py + 1, 3, 4);
        if (Math.random() < 0.05) {
          smokeParticles.push({
            x: px + tilePx - 3,
            y: py,
            alpha: 0.5,
            size: 2,
          });
        }

        // Glowing Window & Arched Door
        ctx.fillStyle = '#fef08a';
        ctx.fillRect(px + 3, py + 8, 3, 3);
        ctx.fillStyle = '#78350f';
        ctx.fillRect(px + 8, py + 8, 3, tilePx - 8);
      } else if (s.kind === 'WoodCottage') {
        // Thatched Wood Cottage
        ctx.fillStyle = '#78350f';
        ctx.fillRect(px + 2, py + 5, tilePx - 4, tilePx - 5);
        ctx.fillStyle = '#92400e';
        ctx.fillRect(px + 3, py + 6, tilePx - 6, tilePx - 7);

        // Thatched Straw Conical Roof
        ctx.fillStyle = '#d97706';
        ctx.beginPath();
        ctx.moveTo(px + tilePx / 2, py + 1);
        ctx.lineTo(px + tilePx, py + 6);
        ctx.lineTo(px, py + 6);
        ctx.closePath();
        ctx.fill();

        // Door
        ctx.fillStyle = '#451a03';
        ctx.fillRect(px + 5, py + 8, 4, tilePx - 8);
      } else if (s.kind === 'TownHall') {
        // Grand Chieftain's Town Hall with Classical Pillars & Gilded Crest
        ctx.fillStyle = '#1e293b';
        ctx.fillRect(px + 1, py + 3, tilePx - 2, tilePx - 3);
        ctx.fillStyle = '#475569';
        ctx.fillRect(px + 2, py + 4, tilePx - 4, tilePx - 5);
        // Stone Pillars
        ctx.fillStyle = '#cbd5e1';
        ctx.fillRect(px + 3, py + 6, 2, tilePx - 7);
        ctx.fillRect(px + tilePx - 5, py + 6, 2, tilePx - 7);
        // Triangular Gilded Pediment Roof
        ctx.fillStyle = '#d97706';
        ctx.beginPath();
        ctx.moveTo(px + tilePx / 2, py - 1);
        ctx.lineTo(px + tilePx + 1, py + 4);
        ctx.lineTo(px - 1, py + 4);
        ctx.closePath();
        ctx.fill();
        // Golden Kingdom Crest
        ctx.fillStyle = '#fbbf24';
        ctx.beginPath();
        ctx.arc(px + tilePx / 2, py + 2, 2, 0, Math.PI * 2);
        ctx.fill();
      } else if (s.kind === 'Barracks') {
        // Fortified Warrior Barracks with Crenelated Battlements
        ctx.fillStyle = '#334155';
        ctx.fillRect(px + 1, py + 3, tilePx - 2, tilePx - 3);
        ctx.fillStyle = '#475569';
        ctx.fillRect(px + 2, py + 4, tilePx - 4, tilePx - 5);
        // Crenelated battlements
        ctx.fillStyle = '#64748b';
        ctx.fillRect(px + 1, py + 1, 3, 3);
        ctx.fillRect(px + 6, py + 1, 3, 3);
        ctx.fillRect(px + 11, py + 1, 3, 3);
        // Iron fortified gate
        ctx.fillStyle = '#1e293b';
        ctx.fillRect(px + 5, py + 7, 5, tilePx - 7);
        ctx.fillStyle = '#dc2626';
        ctx.fillRect(px + 7, py + 2, 2, 2);
      } else if (s.kind === 'Granary') {
        // Communal Granary Silo on Stilts
        ctx.fillStyle = '#78350f';
        ctx.fillRect(px + 3, py + 9, 2, 5);
        ctx.fillRect(px + tilePx - 5, py + 9, 2, 5);

        // Silo Barrel Body
        ctx.fillStyle = '#b45309';
        ctx.fillRect(px + 2, py + 4, tilePx - 4, 6);
        ctx.fillStyle = '#92400e';
        ctx.fillRect(px + 3, py + 5, tilePx - 6, 4);

        // Conical Straw Roof
        ctx.fillStyle = '#fbbf24';
        ctx.beginPath();
        ctx.moveTo(px + tilePx / 2, py);
        ctx.lineTo(px + tilePx, py + 4);
        ctx.lineTo(px, py + 4);
        ctx.closePath();
        ctx.fill();
      } else if (s.kind === 'Watchtower') {
        // Fortified Guard Watchtower
        ctx.fillStyle = '#334155';
        ctx.fillRect(px + 3, py + 4, tilePx - 6, tilePx - 4);
        // Lookout deck
        ctx.fillStyle = '#92400e';
        ctx.fillRect(px + 1, py + 2, tilePx - 2, 3);
        // Banner
        ctx.fillStyle = '#3b82f6';
        ctx.fillRect(px + tilePx / 2, py - 2, 4, 3);
      } else if (s.kind === 'MineShaft') {
        // Mountain Mine Shaft
        ctx.fillStyle = '#090d16';
        ctx.fillRect(px + 2, py + 4, tilePx - 4, tilePx - 4);
        // Timber support beams
        ctx.fillStyle = '#78350f';
        ctx.fillRect(px + 1, py + 3, 3, tilePx - 3);
        ctx.fillRect(px + tilePx - 4, py + 3, 3, tilePx - 3);
        ctx.fillRect(px + 1, py + 2, tilePx - 2, 3);
        // Minecart rail
        ctx.fillStyle = '#94a3b8';
        ctx.fillRect(px + 4, py + tilePx - 2, tilePx - 8, 2);
      } else if (s.kind === 'LeanTo') {
        ctx.fillStyle = '#92400e';
        ctx.beginPath();
        ctx.moveTo(px + 2, py + tilePx - 2);
        ctx.lineTo(px + tilePx - 2, py + 3);
        ctx.lineTo(px + tilePx - 2, py + tilePx - 2);
        ctx.closePath();
        ctx.fill();
      }
    }
  }

  // 4b. Draw Image 1 Iconic Village Landmarks (GODs Temple, Guild Hall, Market, Pen, Fields, A-Rank Dungeon)
  drawVillageIconicLandmarks(data, time);

  // Draw rising smoke particles
  ctx.save();
  for (let i = smokeParticles.length - 1; i >= 0; i--) {
    const sp = smokeParticles[i];
    sp.y -= 0.5;
    sp.x += (Math.random() - 0.48) * 0.4;
    sp.size += 0.04;
    sp.alpha -= 0.012;

    if (sp.alpha <= 0) {
      smokeParticles.splice(i, 1);
      continue;
    }

    ctx.fillStyle = `rgba(200, 210, 220, ${sp.alpha})`;
    ctx.beginPath();
    ctx.arc(sp.x, sp.y, sp.size, 0, Math.PI * 2);
    ctx.fill();
  }
  ctx.restore();

  // 5. Living Pixel Characters & Floating Emotes
  if (data.agents) {
    for (const a of data.agents) {
      const px = a.x * tilePx + tilePx / 2;
      const py = a.y * tilePx + tilePx / 2;
      const bounce = Math.sin(time * 0.008 + a.id) * 1.5;

      ctx.save();
      const isSelected = selectedAgentId === a.id;
      const isFavorite = favoriteAgentId === a.id;

      // Selection Ring
      if (isSelected) {
        ctx.strokeStyle = '#38bdf8';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.arc(px, py + bounce, tilePx * 0.85, 0, Math.PI * 2);
        ctx.stroke();
      }

      // Star Favorite Indicator
      if (isFavorite) {
        ctx.fillStyle = '#fbbf24';
        ctx.font = '10px sans-serif';
        ctx.fillText('⭐', px - 5, py + bounce - tilePx * 0.75);
      }

      // Golden Crown 👑 for Leader/King
      if (a.generation >= 2 || a.piety > 0.8 || (a.traits && a.traits.includes('Leader'))) {
        ctx.font = '10px sans-serif';
        ctx.fillText('👑', px - 5, py + bounce - tilePx * 0.6);
      }

      // Pixel Body (Head & Robe/Shirt)
      const isTerran = a.race === 'SolarTerran';
      // Head
      ctx.fillStyle = '#fed7aa'; // skin tone
      ctx.beginPath();
      ctx.arc(px, py + bounce - 2, 3.5, 0, Math.PI * 2);
      ctx.fill();

      // Shirt / Robe (Terran Gold vs Lunar Violet)
      ctx.fillStyle = isTerran ? '#eab308' : '#a855f7';
      ctx.fillRect(px - 3, py + bounce + 1, 6, 6);

      // Walking feet
      ctx.fillStyle = '#334155';
      ctx.fillRect(px - 2.5, py + bounce + 7, 2, 2);
      ctx.fillRect(px + 0.5, py + bounce + 7, 2, 2);

      // Floating Action Emote Bubble (WorldBox signature!)
      if (camera.zoom >= 0.8) {
        let emote = '💭';
        const act = (a.last_action || a.current_action || '').toLowerCase();
        if (act.includes('eat') || act.includes('forage')) emote = '🍞';
        else if (act.includes('drink')) emote = '💧';
        else if (act.includes('mine')) emote = '⛏️';
        else if (act.includes('wood') || act.includes('chop')) emote = '🪓';
        else if (act.includes('altar') || act.includes('pray')) emote = '🛐';
        else if (act.includes('mate') || act.includes('court')) emote = '❤️';
        else if (act.includes('combat') || act.includes('punish')) emote = '⚔️';
        else if (act.includes('rest')) emote = '💤';

        // Draw small round bubble
        ctx.fillStyle = 'rgba(20, 22, 28, 0.92)';
        ctx.beginPath();
        ctx.arc(px + 8, py + bounce - 8, 6, 0, Math.PI * 2);
        ctx.fill();

        ctx.font = '8px sans-serif';
        ctx.fillText(emote, px + 4, py + bounce - 5);
      }

      ctx.restore();
    }
  }

  // 6. Cataclysms & Dynamic Lightning Effects
  for (let i = activeEffects.length - 1; i >= 0; i--) {
    const fx = activeEffects[i];
    const elapsed = time - fx.start;

    if (elapsed > fx.duration) {
      activeEffects.splice(i, 1);
      continue;
    }

    if (fx.type === 'lightning') {
      const targetPx = fx.x * tilePx + tilePx / 2;
      const targetPy = fx.y * tilePx + tilePx / 2;

      ctx.save();
      // White Screen Flash
      if (elapsed < 80) {
        ctx.fillStyle = 'rgba(255, 255, 255, 0.6)';
        ctx.fillRect(-camera.x / camera.zoom, -camera.y / camera.zoom, canvas.width / camera.zoom, canvas.height / camera.zoom);
      }

      // Electric Jagged Bolt
      ctx.strokeStyle = '#e0f2fe';
      ctx.lineWidth = 3.5;
      ctx.shadowColor = '#38bdf8';
      ctx.shadowBlur = 12;

      ctx.beginPath();
      ctx.moveTo(targetPx + (Math.random() - 0.5) * 40, targetPy - 280);
      let curX = targetPx;
      let curY = targetPy - 280;
      while (curY < targetPy) {
        curY += 25;
        curX += (Math.random() - 0.5) * 30;
        ctx.lineTo(curX, curY);
      }
      ctx.lineTo(targetPx, targetPy);
      ctx.stroke();

      // Scorched Impact Crater
      ctx.fillStyle = 'rgba(20, 20, 20, 0.7)';
      ctx.beginPath();
      ctx.arc(targetPx, targetPy, 10, 0, Math.PI * 2);
      ctx.fill();
      ctx.restore();
    }
  }

  // 7. Dynamic Day / Night Lighting Ambient Mask
  if (!data.is_daytime && realmSettings.showDayNightFilter) {
    ctx.save();
    ctx.fillStyle = 'rgba(7, 12, 32, 0.70)';
    ctx.fillRect(0, 0, WORLD_SIZE * tilePx, WORLD_SIZE * tilePx);

    // Warm radial lights around campfires, cabins & altars
    if (data.structures) {
      for (const s of data.structures) {
        if (s.kind === 'Campfire' || s.kind === 'StoneCabin') {
          const px = s.x * tilePx + tilePx / 2;
          const py = s.y * tilePx + tilePx / 2;
          const radGrad = ctx.createRadialGradient(px, py, 2, px, py, tilePx * 2.8);
          radGrad.addColorStop(0, 'rgba(251, 191, 36, 0.45)');
          radGrad.addColorStop(1, 'rgba(251, 191, 36, 0)');
          ctx.fillStyle = radGrad;
          ctx.beginPath();
          ctx.arc(px, py, tilePx * 2.8, 0, Math.PI * 2);
          ctx.fill();
        }
      }
    }
    ctx.restore();
  }

  // 7.5 Primordial Titans & World Bosses
  if (data.titans && Array.isArray(data.titans)) {
    for (const t of data.titans) {
      const tx = t.x * tilePx + tilePx / 2;
      const ty = t.y * tilePx + tilePx / 2;
      const isAlive = t.state !== 'Defeated';
      const isAwake = t.state === 'Awakened' || t.state === 'Enraged';
      const isEnraged = t.state === 'Enraged';

      ctx.save();
      // Colossal Pulsing Aura
      if (realmSettings.showTitanAuras) {
        const pulse = Math.sin(time * 0.003 + t.id) * 6;
        const auraColor = isEnraged
          ? 'rgba(239, 68, 68, 0.45)'
          : isAwake
          ? 'rgba(249, 115, 22, 0.35)'
          : 'rgba(56, 189, 248, 0.25)';

        const radGrad = ctx.createRadialGradient(
          tx,
          ty,
          6,
          tx,
          ty,
          t.disruption_radius * tilePx * 0.6 + pulse
        );
        radGrad.addColorStop(0, auraColor);
        radGrad.addColorStop(1, 'rgba(0, 0, 0, 0)');
        ctx.fillStyle = radGrad;
        ctx.beginPath();
        ctx.arc(tx, ty, t.disruption_radius * tilePx * 0.6 + pulse, 0, Math.PI * 2);
        ctx.fill();
      }

      // Titan Icon Avatar
      ctx.font = isAwake ? '28px sans-serif' : '22px sans-serif';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      const icon = t.kind.includes('Leviathan')
        ? '🌊'
        : t.kind.includes('Magma')
        ? '🔥'
        : '🌌';
      ctx.fillText(icon, tx, ty);

      // Name & HP / Awakening Gauge
      ctx.font = 'bold 10px Outfit, sans-serif';
      ctx.fillStyle = '#f8fafc';
      ctx.shadowColor = '#0f172a';
      ctx.shadowBlur = 4;
      ctx.fillText(t.name, tx, ty - tilePx * 1.6);

      // Gauge Bar
      const barW = tilePx * 3.5;
      const barH = 5;
      const barX = tx - barW / 2;
      const barY = ty - tilePx * 1.2;

      ctx.fillStyle = 'rgba(15, 23, 42, 0.85)';
      ctx.fillRect(barX, barY, barW, barH);

      if (isAwake && isAlive) {
        // Red / Orange Health Bar
        const hpPct = Math.max(0, Math.min(1, t.hp / t.max_hp));
        ctx.fillStyle = isEnraged ? '#ef4444' : '#f97316';
        ctx.fillRect(barX, barY, barW * hpPct, barH);
      } else if (!isAlive) {
        // Slain
        ctx.fillStyle = '#64748b';
        ctx.fillRect(barX, barY, barW, barH);
      } else {
        // Awakening Pressure Meter (Purple to Cyan)
        const pressurePct = Math.max(0, Math.min(1, t.awakening_pressure / 100));
        ctx.fillStyle = '#a855f7';
        ctx.fillRect(barX, barY, barW * pressurePct, barH);
      }
      ctx.restore();
    }
  }

  // 7.6 Kingdom Warfare Siege Artillery
  if (data.siege_engines && Array.isArray(data.siege_engines)) {
    for (const eng of data.siege_engines) {
      if (!eng.is_operational) continue;
      const ex = eng.x * tilePx + tilePx / 2;
      const ey = eng.y * tilePx + tilePx / 2;
      const tx = eng.target_x * tilePx + tilePx / 2;
      const ty = eng.target_y * tilePx + tilePx / 2;

      ctx.save();
      // Target Trajectory Line
      ctx.strokeStyle =
        eng.faction === 'SolarTerran'
          ? 'rgba(251, 191, 36, 0.4)'
          : 'rgba(192, 132, 252, 0.4)';
      ctx.lineWidth = 1.5;
      ctx.setLineDash([3, 3]);
      ctx.beginPath();
      ctx.moveTo(ex, ey);
      ctx.lineTo(tx, ty);
      ctx.stroke();

      // Siege Unit Marker
      ctx.font = '16px sans-serif';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      const sIcon = eng.kind.includes('Catapult')
        ? '☄️'
        : eng.kind.includes('Ballista')
        ? '🏹'
        : '🪵';
      ctx.fillText(sIcon, ex, ey);
      ctx.restore();
    }
  }

  // 7.7 Ascended Demigods Celestial Halos
  if (realmSettings.showDemigodHalos && data.demigods && Array.isArray(data.demigods)) {
    for (const dg of data.demigods) {
      if (!data.agents) continue;
      const agent = data.agents.find((a) => a.id === dg.agent_id);
      if (!agent) continue;
      const ax = agent.x * tilePx + tilePx / 2;
      const ay = agent.y * tilePx + tilePx / 2;

      ctx.save();
      // Celestial Golden Corona
      const corona = Math.sin(time * 0.005 + dg.agent_id) * 3;
      ctx.strokeStyle = '#f59e0b';
      ctx.lineWidth = 2.5;
      ctx.shadowColor = '#fbbf24';
      ctx.shadowBlur = 10;
      ctx.beginPath();
      ctx.arc(ax, ay, tilePx * 1.1 + corona, 0, Math.PI * 2);
      ctx.stroke();

      // Domain Crown Emblem
      ctx.font = 'bold 11px Outfit, sans-serif';
      ctx.fillStyle = '#fde68a';
      ctx.textAlign = 'center';
      ctx.fillText(`✦ ${dg.domain} ✦`, ax, ay - tilePx * 1.1);
      ctx.restore();
    }
  }

  // 8. Brush Hover Reticle
  if (activeTool.startsWith('brush-')) {
    ctx.save();
    const half = Math.floor(brushRadius / 2);
    const startX = Math.max(0, mouseWorldTile.x - half);
    const startY = Math.max(0, mouseWorldTile.y - half);
    const size = brushRadius;

    ctx.strokeStyle = '#fbbf24';
    ctx.lineWidth = 2;
    ctx.setLineDash([4, 4]);
    ctx.strokeRect(startX * tilePx, startY * tilePx, size * tilePx, size * tilePx);
    ctx.restore();
  }

  ctx.restore();
}

// ----------------------------------------------------------------------------
// WORLDBOX CHARACTER SHEET INSPECTOR
// ----------------------------------------------------------------------------
function openCharacterSheet() {
  document.getElementById('character-sheet-modal').classList.remove('hidden');
}

function closeCharacterSheet() {
  playSound('click');
  document.getElementById('character-sheet-modal').classList.add('hidden');
}

function updateCharacterSheet(agent) {
  document.getElementById('cs-name').textContent = agent.name;
  const isTerran = agent.race === 'SolarTerran';
  const flagEl = document.getElementById('cs-kingdom-flag');
  flagEl.textContent = isTerran ? '☀️ Kingdom of Solaria' : '🌙 Lunar Sovereignty';
  flagEl.style.color = isTerran ? '#fbbf24' : '#c084fc';

  const avatar = document.getElementById('char-avatar');
  avatar.className = `pixel-avatar ${isTerran ? 'terran' : 'lunar'}`;

  document.getElementById('cs-archetype').textContent = agent.archetype || 'Community Builder';

  const starBtn = document.getElementById('cs-star-btn');
  if (favoriteAgentId === agent.id) starBtn.classList.add('favorited');
  else starBtn.classList.remove('favorited');

  // Stats Grid
  document.getElementById('cs-val-hp').textContent = `${agent.health.toFixed(0)}/100`;
  const atk = 10 + (agent.traits && agent.traits.includes('Strong') ? 8 : 0);
  const def = 8 + (agent.inventory_items && agent.inventory_items.some(i => i[0].includes('Iron')) ? 6 : 0);
  const spd = 12 + (agent.traits && agent.traits.includes('Genius') ? 4 : 0);
  const intel = 18 + (agent.genome ? (agent.genome.mutations_count || 0) * 2 : 0);
  const dip = 15 + Math.round(agent.piety * 10);

  document.getElementById('cs-val-atk').textContent = atk;
  document.getElementById('cs-val-def').textContent = def;
  document.getElementById('cs-val-spd').textContent = spd;
  document.getElementById('cs-val-int').textContent = intel;
  document.getElementById('cs-val-dip').textContent = dip;

  const genNum = agent.genome ? agent.genome.generation : 1;
  document.getElementById('cs-val-age').textContent = `Gen ${genNum} (${agent.health.toFixed(0)}y)`;
  document.getElementById('cs-val-off').textContent = agent.children_count || 0;
  document.getElementById('cs-val-kills').textContent = 0;

  // Trait Badges Box
  const traitGrid = document.getElementById('cs-traits-grid');
  traitGrid.innerHTML = '';
  const traits = agent.traits && agent.traits.length > 0 ? agent.traits : ['Leader', 'Genius', 'Strong', 'Resilient'];
  const traitIcons = {
    Leader: '👑',
    Genius: '🌟',
    Strong: '⚔️',
    Resilient: '🛡️',
    Charming: '✨',
    Pacifist: '🕊️',
    Wise: '🔮',
    Greedy: '💰',
    Sloth: '💤',
    Immune: '🧬',
  };

  for (const t of traits) {
    const ico = traitIcons[t] || '✦';
    const badge = document.createElement('span');
    badge.className = 'wb-trait-badge';
    badge.innerHTML = `<span class="tr-ico">${ico}</span> ${t}`;
    traitGrid.appendChild(badge);
  }

  // Monologue
  document.getElementById('cs-monologue-text').textContent = `"${agent.current_thought || 'Observing the cosmos...'}"`;

  // Inventory
  const invBox = document.getElementById('cs-inventory-pack');
  invBox.innerHTML = '';
  if (agent.detailed_items && agent.detailed_items.length > 0) {
    for (const item of agent.detailed_items) {
      const tag = document.createElement('span');
      tag.className = 'inv-item' + (item.is_blessed ? ' blessed-item' : '');
      tag.title = 'Click to inspect item and bestow divine blessing';
      tag.innerHTML = (item.is_blessed ? '✦ ' : '') + item.name;
      tag.onclick = (e) => {
        e.stopPropagation();
        openItemInspector(item, agent.id);
      };
      invBox.appendChild(tag);
    }
  } else if (agent.inventory_items && agent.inventory_items.length > 0) {
    for (const [name, qty] of agent.inventory_items) {
      const tag = document.createElement('span');
      const isBlessed = name.includes('✦');
      tag.className = 'inv-item' + (isBlessed ? ' blessed-item' : '');
      tag.textContent = `${name} x${qty}`;
      tag.title = 'Click to inspect item and bestow divine blessing';
      tag.onclick = (e) => {
        e.stopPropagation();
        openItemInspector({
          id: 0,
          name: name,
          rarity: isBlessed ? 'Mythic' : 'Common',
          rarity_color: isBlessed ? '#e91e63' : '#9e9e9e',
          slot: 'Civilizational Good',
          attack: isBlessed ? 25 : 0,
          defense: isBlessed ? 20 : 0,
          durability: 100,
          max_durability: 100,
          affixes: isBlessed ? ['✦ Divine Blessing: +50% Potency'] : [],
          value: 20,
          is_blessed: isBlessed,
          mana_charge: null,
          lore: `Resource carried by ${agent.name}. Click Cast Divine Blessing to consecrate it into a holy relic.`,
        }, agent.id);
      };
      invBox.appendChild(tag);
    }
  } else {
    invBox.innerHTML = '<span class="inv-item">Empty Pack</span>';
  }

  // Neuromodulation
  if (agent.neuromod) {
    document.getElementById('gauge-dopamine').style.width = `${agent.neuromod.dopamine * 100}%`;
    document.getElementById('txt-dopamine').textContent = agent.neuromod.dopamine.toFixed(2);
    document.getElementById('gauge-serotonin').style.width = `${agent.neuromod.serotonin * 100}%`;
    document.getElementById('txt-serotonin').textContent = agent.neuromod.serotonin.toFixed(2);
    document.getElementById('gauge-noradrenaline').style.width = `${agent.neuromod.noradrenaline * 100}%`;
    document.getElementById('txt-noradrenaline').textContent = agent.neuromod.noradrenaline.toFixed(2);
  }

  // AST Code
  document.getElementById('cs-ast-gen').textContent = `GEN: ${genNum} | MUTATIONS: ${agent.genome ? agent.genome.mutation_count : 0}`;
  document.getElementById('cs-ast-code').textContent = `(ast\n  (condition (> hunger 0.65))\n  (then (seek-food))\n  (else (explore-world)))`;

  // Echo
  document.getElementById('cs-echo-text').textContent = agent.ancestral_echo || 'First Generation Valley Pioneer. Inscribing destiny upon the world grid.';
}

function switchAiTab(tab) {
  playSound('click');
  document.querySelectorAll('.ai-tab-btn').forEach(b => b.classList.remove('active'));
  document.querySelectorAll('.ai-pane').forEach(p => p.classList.remove('active'));

  const btn = document.getElementById(`tab-btn-${tab}`);
  const pane = document.getElementById(`ai-pane-${tab}`);
  if (btn) btn.classList.add('active');
  if (pane) pane.classList.add('active');
}

// ----------------------------------------------------------------------------
// DIRECT GOD ACTIONS ON CHARACTER
// ----------------------------------------------------------------------------
function whisperCurrentAgent() {
  if (!selectedAgentId) return;
  playSound('click');
  const msg = prompt('Enter divine whisper into character consciousness:', 'Build a stone cabin before the cold arrives.');
  if (msg) {
    sendMsg({
      type: 'TriggerIntervention',
      intervention: {
        DivineWhisper: {
          target_agent_id: selectedAgentId,
          message: msg,
        }
      }
    });
    showToast(`🕊️ Whispered divine command to character #${selectedAgentId}`);
  }
}

function blessCurrentAgent() {
  if (!selectedAgentId) return;
  playSound('chime');
  sendMsg({
    type: 'TriggerIntervention',
    intervention: {
      DivineWhisper: {
        target_agent_id: selectedAgentId,
        message: 'The holy sun infuses your soul with divine vigor.',
      }
    }
  });
  showToast(`✨ Character #${selectedAgentId} blessed!`);
}

function healCurrentAgent() {
  if (!selectedAgentId) return;
  playSound('chime');
  sendMsg({
    type: 'TriggerIntervention',
    intervention: {
      MiracleManna: {
        x: mouseWorldTile.x,
        y: mouseWorldTile.y,
      }
    }
  });
  showToast(`💚 Restored vitals for character #${selectedAgentId}`);
}

function smiteCurrentAgent() {
  if (!selectedAgentId) return;
  if (confirm(`Strike character #${selectedAgentId} with divine lightning?`)) {
    playSound('thunder');
    activeEffects.push({
      type: 'lightning',
      x: mouseWorldTile.x,
      y: mouseWorldTile.y,
      start: Date.now(),
      duration: 380,
    });
    sendMsg({
      type: 'TriggerIntervention',
      intervention: {
        SmiteLightning: {
          x: mouseWorldTile.x,
          y: mouseWorldTile.y,
        }
      }
    });
    showToast(`⚡ Divine Smite struck character #${selectedAgentId}!`);
  }
}

// ----------------------------------------------------------------------------
// ITEM INSPECTION & DIVINE BLESSING SYSTEM
// ----------------------------------------------------------------------------
let currentlyInspectedItem = null;
let currentlyInspectedAgentId = null;

function openItemInspector(item, agentId) {
  playSound('click');
  currentlyInspectedItem = item;
  currentlyInspectedAgentId = agentId;

  const modal = document.getElementById('modal-item-inspector');
  if (!modal) return;
  modal.classList.remove('hidden');

  // Title & Hero
  document.getElementById('ii-name').textContent = item.name;
  const icon = getItemIcon(item.name, item.slot);
  document.getElementById('ii-icon').textContent = icon;

  const rarityBadge = document.getElementById('ii-rarity');
  rarityBadge.textContent = item.rarity;
  rarityBadge.style.color = item.rarity_color || '#e2e8f0';
  rarityBadge.style.borderColor = item.rarity_color || 'rgba(255,255,255,0.2)';

  document.getElementById('ii-slot').textContent = item.slot;

  const blessedTag = document.getElementById('ii-blessed-tag');
  if (item.is_blessed) {
    blessedTag.classList.remove('hidden');
  } else {
    blessedTag.classList.add('hidden');
  }

  // Stats Matrix
  document.getElementById('ii-atk').textContent = item.attack > 0 ? `+${item.attack.toFixed(0)}` : '0';
  document.getElementById('ii-def').textContent = item.defense > 0 ? `+${item.defense.toFixed(0)}` : '0';
  document.getElementById('ii-dur').textContent = `${item.durability}/${item.max_durability}`;
  document.getElementById('ii-val').textContent = `${item.value}g`;

  // Mana section
  const manaSec = document.getElementById('ii-mana-section');
  if (item.mana_charge) {
    manaSec.classList.remove('hidden');
    const [stored, max] = item.mana_charge;
    document.getElementById('ii-mana-txt').textContent = `${stored.toFixed(0)} / ${max.toFixed(0)} MP`;
    const pct = Math.min(100, Math.max(0, (stored / max) * 100));
    document.getElementById('ii-mana-fill').style.width = `${pct}%`;
  } else {
    manaSec.classList.add('hidden');
  }

  // Affixes
  const affList = document.getElementById('ii-affixes');
  affList.innerHTML = '';
  if (item.affixes && item.affixes.length > 0) {
    for (const aff of item.affixes) {
      const pill = document.createElement('span');
      pill.className = 'affix-pill';
      pill.textContent = aff;
      affList.appendChild(pill);
    }
  } else {
    affList.innerHTML = '<span class="affix-pill" style="opacity:0.6;border-style:dashed;">Mundane Item (No Enchantments)</span>';
  }

  // Lore
  document.getElementById('ii-lore').textContent = `"${item.lore || 'An ancient relic steeped in mystery.'}"`;

  // Bless button state
  const btn = document.getElementById('btn-bless-item-act');
  if (btn) {
    if (item.is_blessed) {
      btn.innerHTML = '✨ RENEW CELESTIAL BLESSING';
    } else {
      btn.innerHTML = '✨ CAST DIVINE BLESSING';
    }
  }
}

function closeItemInspector() {
  playSound('click');
  const modal = document.getElementById('modal-item-inspector');
  if (modal) modal.classList.add('hidden');
  currentlyInspectedItem = null;
  currentlyInspectedAgentId = null;
}

function blessInspectedItem() {
  if (!currentlyInspectedItem || !currentlyInspectedAgentId) return;
  playSound('chime');

  const itemName = currentlyInspectedItem.name.replace('✦ Sanctified ', '');
  sendMsg({
    type: 'BlessItem',
    agent_id: currentlyInspectedAgentId,
    item_name: itemName,
  });

  // Also trigger celestial particle beam on agent if visible
  if (mouseWorldTile) {
    activeEffects.push({
      type: 'bless',
      x: mouseWorldTile.x,
      y: mouseWorldTile.y,
      start: Date.now(),
      duration: 500,
    });
  }

  showToast(`✨ Bestowed Divine Blessing upon ${currentlyInspectedItem.name}! It radiates with holy power.`);
  closeItemInspector();
}

function getItemIcon(name, slot) {
  const n = name.toLowerCase();
  if (n.includes('sword') || n.includes('warblade') || n.includes('axe')) return '⚔️';
  if (n.includes('bow')) return '🏹';
  if (n.includes('pickaxe')) return '⛏️';
  if (n.includes('staff') || n.includes('wand')) return '🪄';
  if (n.includes('armor') || n.includes('cuirass')) return '🛡️';
  if (n.includes('stone') || n.includes('mana')) return '💎';
  if (n.includes('bread') || n.includes('berry') || n.includes('fish') || n.includes('grain')) return '🍞';
  if (n.includes('wood') || n.includes('plank')) return '🪵';
  if (n.includes('ore') || n.includes('ingot')) return '🪙';
  if (n.includes('rune')) return '📜';
  return '📦';
}

// ----------------------------------------------------------------------------
// PANTHEON MODAL (COVENANTS & ALTARS)
// ----------------------------------------------------------------------------
function togglePantheonModal(tab) {
  playSound('click');
  const modal = document.getElementById('pantheon-modal');
  modal.classList.remove('hidden');
  switchPantheonTab(tab);
}

function closePantheonModal() {
  playSound('click');
  document.getElementById('pantheon-modal').classList.add('hidden');
}

function switchPantheonTab(tab) {
  playSound('click');
  document.querySelectorAll('.pantheon-tabs-nav .ai-tab-btn').forEach(b => b.classList.remove('active'));
  document.querySelectorAll('#pantheon-modal .ai-pane').forEach(p => p.classList.remove('active'));

  if (tab === 'covenants') {
    document.getElementById('p-tab-btn-cov').classList.add('active');
    document.getElementById('p-pane-covenants').classList.add('active');
  } else if (tab === 'altars') {
    document.getElementById('p-tab-btn-alt').classList.add('active');
    document.getElementById('p-pane-altars').classList.add('active');
  } else if (tab === 'events') {
    document.getElementById('p-tab-btn-log').classList.add('active');
    document.getElementById('p-pane-events').classList.add('active');
  }
}

function issueCovenant() {
  playSound('chime');
  const obj = document.getElementById('cov-objective').value;
  const rew = document.getElementById('cov-reward').value;
  sendMsg({
    type: 'IssueCovenant',
    objective: obj,
    target_amount: obj === 'PresentAltarOfferings' ? 5 : 20,
    reward: rew,
    punishment: 'Divine withdrawal and loss of blessing.',
  });
  showToast(`📜 Divine Covenant forged: ${obj}`);
}

let lastCovenantsHash = '';
function renderCovenants(covs) {
  const list = document.getElementById('covenants-list');
  if (!list) return;
  const hash = covs ? covs.map(c => `${c.id}:${c.current_progress}`).join('|') : '';
  if (hash === lastCovenantsHash) return;
  lastCovenantsHash = hash;

  list.innerHTML = '';
  if (!covs || covs.length === 0) {
    list.innerHTML = '<span class="text-dim" style="font-size:0.65rem">No active covenants. Forge one above!</span>';
    return;
  }
  for (const c of covs) {
    const el = document.createElement('div');
    el.innerHTML = `<strong>${c.title}</strong><br><span style="color:#94a3b8">Progress: ${c.current_progress} / ${c.target_progress || 5} | Reward: ${c.promised_reward}</span>`;
    list.appendChild(el);
  }
}

let lastAltarsHash = '';
function renderAltars(altars) {
  const sBox = document.getElementById('solar-offerings');
  const lBox = document.getElementById('lunar-offerings');
  if (!sBox || !lBox || !altars) return;
  const hash = altars.map(a => `${a.id}:${a.active_offerings ? a.active_offerings.length : 0}`).join('|');
  if (hash === lastAltarsHash) return;
  lastAltarsHash = hash;

  sBox.innerHTML = '';
  lBox.innerHTML = '';

  for (const altar of altars) {
    const isSolar = altar.deity === 'SolarPlayerGod';
    const box = isSolar ? sBox : lBox;
    if (!altar.active_offerings || altar.active_offerings.length === 0) {
      box.innerHTML = '<span style="font-size:0.6rem;color:#64748b">No active offerings.</span>';
      continue;
    }
    for (const off of altar.active_offerings) {
      const item = document.createElement('div');
      item.innerHTML = `<strong>${off.item}</strong> by Agent #${off.offerer_id} | Sincerity: ${(off.sincerity * 100).toFixed(0)}%`;
      box.appendChild(item);
    }
  }
}

let lastChronicleHash = '';
function renderChronicle(logs) {
  const stream = document.getElementById('event-log-stream');
  if (!stream || !logs || logs.length === 0) return;
  const latestKey = logs[logs.length - 1] + '_' + logs.length;
  if (latestKey === lastChronicleHash) return;
  lastChronicleHash = latestKey;

  stream.innerHTML = '';
  for (let i = logs.length - 1; i >= Math.max(0, logs.length - 20); i--) {
    const d = document.createElement('div');
    d.textContent = logs[i];
    stream.appendChild(d);
  }
}

function promptDivineWhisper() {
  whisperCurrentAgent();
}

function triggerGenesisSpark() {
  if (confirm('Ignite Genesis Evolution Spark? Forces an immediate mutation epoch.')) {
    playSound('chime');
    sendMsg({
      type: 'TriggerIntervention',
      intervention: 'GenesisEvolutionSpark'
    });
    showToast('🧬 Genesis Evolution Spark ignited!');
  }
}

function triggerBabel() {
  if (confirm('Initiate Tower of Babel? Scrambles dialect trees.')) {
    playSound('thunder');
    sendMsg({
      type: 'TriggerIntervention',
      intervention: 'TowerOfBabel'
    });
    showToast('🌀 Tower of Babel initiated!');
  }
}

function triggerGoldenAgePeace() {
  playSound('chime');
  sendMsg({
    type: 'TriggerIntervention',
    intervention: 'GoldenAgePeace'
  });
  showToast('🕊️ Golden Age Peace decreed across all realms!');
}

// ----------------------------------------------------------------------------
// KNOWLEDGE & ACTIVITY CODEX CONTROLLERS
// ----------------------------------------------------------------------------
let codexRaceFilter = 'all';

function toggleCodexModal() {
  playSound('click');
  const modal = document.getElementById('modal-codex');
  if (!modal) return;
  if (modal.classList.contains('hidden')) {
    modal.classList.remove('hidden');
    if (currentTelemetry) {
      if (currentTelemetry.activities) renderCodexActivities(currentTelemetry.activities);
      if (currentTelemetry.terran_culture || currentTelemetry.lunar_culture) {
        renderCodexTechTree(currentTelemetry.terran_culture, currentTelemetry.lunar_culture);
        renderCodexDiscoveries(currentTelemetry.terran_culture, currentTelemetry.lunar_culture);
      }
      if (currentTelemetry.active_plots) {
        renderCodexPlots(
          currentTelemetry.active_plots,
          currentTelemetry.terran_treasury,
          currentTelemetry.lunar_treasury,
          currentTelemetry.diplomacy_status
        );
      }
    }
  } else {
    modal.classList.add('hidden');
  }
}

function closeCodexModal() {
  playSound('click');
  const modal = document.getElementById('modal-codex');
  if (modal) modal.classList.add('hidden');
}

function switchCodexTab(tab) {
  playSound('click');
  document.querySelectorAll('#modal-codex .ai-tab-btn').forEach(b => b.classList.remove('active'));
  document.querySelectorAll('#modal-codex .ai-pane').forEach(p => p.classList.remove('active'));

  if (tab === 'activities') {
    const btn = document.getElementById('codex-tab-btn-act');
    const pane = document.getElementById('codex-pane-activities');
    if (btn) btn.classList.add('active');
    if (pane) pane.classList.add('active');
  } else if (tab === 'tech') {
    const btn = document.getElementById('codex-tab-btn-tech');
    const pane = document.getElementById('codex-pane-tech');
    if (btn) btn.classList.add('active');
    if (pane) pane.classList.add('active');
  } else if (tab === 'discoveries') {
    const btn = document.getElementById('codex-tab-btn-disc');
    const pane = document.getElementById('codex-pane-discoveries');
    if (btn) btn.classList.add('active');
    if (pane) pane.classList.add('active');
  } else if (tab === 'plots') {
    const btn = document.getElementById('codex-tab-btn-plots');
    const pane = document.getElementById('codex-pane-plots');
    if (btn) btn.classList.add('active');
    if (pane) pane.classList.add('active');
    if (currentTelemetry && currentTelemetry.active_plots) {
      renderCodexPlots(
        currentTelemetry.active_plots,
        currentTelemetry.terran_treasury,
        currentTelemetry.lunar_treasury,
        currentTelemetry.diplomacy_status
      );
    }
  }
}

function renderCodexPlots(plots, terranTreasury, lunarTreasury, diplomacy) {
  const solTreasury = document.getElementById('treasury-solaria');
  const lunTreasury = document.getElementById('treasury-lunar');
  const dipBanner = document.getElementById('diplomacy-banner');
  if (solTreasury) solTreasury.textContent = terranTreasury ?? 250;
  if (lunTreasury) lunTreasury.textContent = lunarTreasury ?? 250;
  if (dipBanner) dipBanner.textContent = diplomacy ?? 'Peaceful Coexistence';

  const list = document.getElementById('codex-plots-list');
  if (!list) return;
  list.innerHTML = '';

  if (!plots || plots.length === 0) {
    list.innerHTML = '<span class="text-dim" style="font-size:0.7rem;padding:8px">No active civil plots or rebellions. Mortal peace reigns.</span>';
    return;
  }

  for (const p of plots) {
    const row = document.createElement('div');
    row.className = `plot-card ${p.is_resolved ? 'resolved' : 'active'}`;
    const icon = p.plot_type === 'Rebellion' ? '⚔️' :
                 p.plot_type === 'HolyWar' ? '⚡' :
                 p.plot_type === 'CommercialAlliance' ? '🤝' :
                 p.plot_type === 'HeresySchism' ? '🔥' : '✨';
    const statusText = p.is_resolved ? 'COMPLETED / EXECUTED' : `${Math.round(p.progress)}% Progress`;
    const isSolar = p.initiator_race === 'SolarTerran';

    row.innerHTML = `
      <div class="plot-card-head">
        <div class="plot-card-title">
          <span>${icon}</span>
          <strong>${p.title}</strong>
        </div>
        <span class="plot-status-badge ${p.is_resolved ? 'resolved' : ''}">${statusText}</span>
      </div>
      <div class="plot-desc">${p.target_description}</div>
      <div class="plot-meta">
        <span>Conspirator: <strong>${p.initiator_name}</strong> (${isSolar ? '☀️ Solar' : '🌙 Lunar'})</span>
        <span>Supporters: <strong>${p.supporter_ids.length}</strong> citizens</span>
      </div>
      <div class="plot-progress-bar">
        <div class="plot-progress-fill" style="width: ${p.progress}%"></div>
      </div>
    `;
    list.appendChild(row);
  }
}

function filterCodexActivities(race) {
  playSound('click');
  codexRaceFilter = race;
  document.querySelectorAll('.codex-filter-bar .btn-filter-pill').forEach(b => b.classList.remove('active'));
  if (race === 'all') {
    const b = document.getElementById('filter-act-all');
    if (b) b.classList.add('active');
  } else if (race === 'SolarTerran') {
    const b = document.getElementById('filter-act-terran');
    if (b) b.classList.add('active');
  } else if (race === 'ChthonicLunarian') {
    const b = document.getElementById('filter-act-lunar');
    if (b) b.classList.add('active');
  }

  if (currentTelemetry && currentTelemetry.activities) {
    renderCodexActivities(currentTelemetry.activities);
  }
}

function renderCodexActivities(activities) {
  const stream = document.getElementById('codex-activity-stream');
  if (!stream || !activities) return;
  stream.innerHTML = '';

  const filtered = activities.filter(a => codexRaceFilter === 'all' || a.race === codexRaceFilter);

  if (filtered.length === 0) {
    stream.innerHTML = '<span class="text-dim" style="font-size:0.7rem;padding:8px">No activities recorded for this filter.</span>';
    return;
  }

  for (const act of filtered) {
    const isSolar = act.race === 'SolarTerran';
    const row = document.createElement('div');
    row.className = `activity-row ${isSolar ? 'terran' : 'lunar'}`;

    const emotionClass = act.emotion ? act.emotion.toLowerCase() : 'calm';
    const raceGlyph = isSolar ? '☀️' : '🌙';

    row.innerHTML = `
      <div class="act-head">
        <span class="act-name">${raceGlyph} ${act.agent_name} <span class="act-loc">(${act.location[0]}, ${act.location[1]})</span></span>
        <span class="act-badge ${emotionClass}">${act.emotion}</span>
      </div>
      <div class="act-body">${act.activity}</div>
    `;
    stream.appendChild(row);
  }
}

function renderCodexTechTree(terranCulture, lunarCulture) {
  if (terranCulture) {
    const solKp = document.getElementById('solaria-kp');
    const solRes = document.getElementById('solaria-research');
    if (solKp) solKp.textContent = terranCulture.knowledge_points;
    if (solRes) solRes.textContent = terranCulture.active_research;
  }
  if (lunarCulture) {
    const lunKp = document.getElementById('lunar-kp');
    const lunRes = document.getElementById('lunar-research');
    if (lunKp) lunKp.textContent = lunarCulture.knowledge_points;
    if (lunRes) lunRes.textContent = lunarCulture.active_research;
  }

  const grid = document.getElementById('tech-tree-grid');
  if (!grid || !terranCulture || !terranCulture.technologies) return;
  grid.innerHTML = '';

  for (const tech of terranCulture.technologies) {
    const card = document.createElement('div');
    card.className = `tech-card ${tech.is_unlocked ? 'unlocked' : ''}`;

    const statusBadge = tech.is_unlocked
      ? '<span class="tech-status-badge unlocked">Mastered ✨</span>'
      : `<span class="tech-status-badge locked">${terranCulture.knowledge_points}/${tech.knowledge_required} KP</span>`;

    card.innerHTML = `
      <div class="tech-card-head">
        <div class="tech-card-title">
          <span>${tech.icon}</span>
          <span>${tech.name}</span>
        </div>
        <span class="tech-tier-badge">Tier ${tech.tier}</span>
      </div>
      <div class="tech-desc">${tech.description}</div>
      <div class="tech-effect">Effect: ${tech.effect}</div>
      <div style="display:flex;justify-content:flex-end;margin-top:2px">${statusBadge}</div>
    `;
    grid.appendChild(card);
  }
}

function renderCodexDiscoveries(terranCulture, lunarCulture) {
  const list = document.getElementById('codex-discoveries-list');
  if (!list) return;
  list.innerHTML = '';

  const discoveries = [];
  if (terranCulture && terranCulture.discoveries) {
    for (const d of terranCulture.discoveries) discoveries.push(`☀️ [Solaria] ${d}`);
  }
  if (lunarCulture && lunarCulture.discoveries) {
    for (const d of lunarCulture.discoveries) discoveries.push(`🌙 [Lunar] ${d}`);
  }

  if (discoveries.length === 0) {
    list.innerHTML = '<span class="text-dim" style="font-size:0.68rem;padding:6px">Exploration underway. Mine ores or erect cabins to record historical milestones!</span>';
    return;
  }

  for (const d of discoveries) {
    const el = document.createElement('div');
    el.textContent = d;
    list.appendChild(el);
  }
}

// ----------------------------------------------------------------------------
// WORLD ARCHETYPES & GENERATION FORGE UI
// ----------------------------------------------------------------------------
function toggleWorldModal() {
  const modal = document.getElementById('modal-world-manager');
  if (!modal) return;
  modal.classList.toggle('hidden');
}

function closeWorldModal() {
  const modal = document.getElementById('modal-world-manager');
  if (modal) modal.classList.add('hidden');
}

function selectArchetype(arch) {
  selectedMapArchetype = arch;
  document.querySelectorAll('.archetype-card').forEach(card => {
    if (card.getAttribute('data-arch') === arch) {
      card.classList.add('active');
    } else {
      card.classList.remove('active');
    }
  });
  const picker = document.getElementById('start-archetype-picker');
  if (picker) picker.value = arch;
}

function randomizeSeed() {
  const seedInp = document.getElementById('inp-world-seed');
  if (seedInp) {
    seedInp.value = Math.floor(Math.random() * 900000) + 1000;
  }
}

function updateSeaLevelLabel(val) {
  const lbl = document.getElementById('lbl-sea-level');
  if (!lbl) return;
  const num = parseInt(val, 10);
  if (num < 35) {
    lbl.textContent = `Dry Basin (${num}%)`;
  } else if (num > 65) {
    lbl.textContent = `High Seas (${num}%)`;
  } else {
    lbl.textContent = `Standard (${num}%)`;
  }
}

function updateMountainDensityLabel(val) {
  const lbl = document.getElementById('lbl-mountain-density');
  if (!lbl) return;
  const num = parseInt(val, 10);
  if (num < 35) {
    lbl.textContent = `Rolling Hills (${num}%)`;
  } else if (num > 65) {
    lbl.textContent = `Extreme Peaks (${num}%)`;
  } else {
    lbl.textContent = `Moderate Crags (${num}%)`;
  }
}

function confirmResetWorld() {
  sendMsg({ type: 'ResetWorld' });
  playSound('chime');
  showToast("🔄 Cosmic Will Invoked: Resetting active world...");
  closeWorldModal();
  setSpeed(1);
}

function confirmGenerateNewMap() {
  const seedInp = document.getElementById('inp-world-seed');
  const seaSlider = document.getElementById('slider-sea-level');
  const mtnSlider = document.getElementById('slider-mountain-density');

  const seed = parseInt(seedInp ? seedInp.value : '42', 10) || Math.floor(Math.random() * 900000) + 1000;
  const waterLevel = (parseInt(seaSlider ? seaSlider.value : '50', 10) || 50) / 100.0;
  const mountainDensity = (parseInt(mtnSlider ? mtnSlider.value : '50', 10) || 50) / 100.0;

  sendMsg({
    type: 'ChangeMap',
    archetype: selectedMapArchetype,
    seed: seed,
    water_level: waterLevel,
    mountain_density: mountainDensity
  });
  playSound('chime');
  showToast(`✨ Carving new realm: ${selectedMapArchetype} (Seed ${seed})...`);
  closeWorldModal();
  closeStartModal();
  setSpeed(1);
}

function forgeNewWorldFromStart(event) {
  if (event) event.stopPropagation();
  const picker = document.getElementById('start-archetype-picker');
  const archetype = picker ? picker.value : selectedMapArchetype;
  selectedMapArchetype = archetype;
  const randomSeed = Math.floor(Math.random() * 900000) + 1000;

  sendMsg({
    type: 'ChangeMap',
    archetype: archetype,
    seed: randomSeed,
    water_level: 0.5,
    mountain_density: 0.5
  });

  playSound('chime');
  showToast(`✨ Forging new realm: ${archetype} (Seed ${randomSeed})...`);
  closeStartModal();
  closeWorldModal();
  setSpeed(1);
}

function forgeArchetypeDirect(arch, event) {
  if (event) event.stopPropagation();
  selectArchetype(arch);
  confirmGenerateNewMap();
}

// ----------------------------------------------------------------------------
// CONTINUOUS GAME & RENDER LOOP WITH GPU TIER SCALING
// ----------------------------------------------------------------------------
let lastFrameTime = 0;
let frameCount = 0;
let lastFpsTime = 0;
let currentFps = 60;

function gameLoop(timestamp) {
  frameCount++;
  if (timestamp - lastFpsTime >= 500) {
    currentFps = Math.round((frameCount * 1000) / (timestamp - lastFpsTime));
    frameCount = 0;
    lastFpsTime = timestamp;
    const fpsEl = document.getElementById('debug-fps-val');
    if (fpsEl) fpsEl.textContent = currentFps;
  }

  if (gpuTier === 'low') {
    // Low / Battery Saver: cap at 30 FPS
    if (timestamp - lastFrameTime < 33.3) {
      requestAnimationFrame(gameLoop);
      return;
    }
  }
  lastFrameTime = timestamp;

  if (currentTelemetry) {
    drawWorldMap(currentTelemetry);
    drawMinimap(currentTelemetry);
  }
  requestAnimationFrame(gameLoop);
}

// ----------------------------------------------------------------------------
// THEME & OCEAN PALETTE CONTROLLER (NON-DARK-BLUE AESTHETIC)
// ----------------------------------------------------------------------------
let activeTheme = localStorage.getItem('cogniverse_theme') || 'theme-obsidian';

function getThemeOceanColors() {
  if (activeTheme === 'theme-amber') {
    return {
      deep: '#171310',
      ripple: '#272019',
      backdrop: '#0e0b09',
      wave: 'rgba(217, 119, 6, 0.12)'
    };
  } else if (activeTheme === 'theme-emerald') {
    return {
      deep: '#0b1411',
      ripple: '#14241e',
      backdrop: '#060b09',
      wave: 'rgba(16, 185, 129, 0.12)'
    };
  } else if (activeTheme === 'theme-crimson') {
    return {
      deep: '#160e11',
      ripple: '#27161c',
      backdrop: '#0c0709',
      wave: 'rgba(244, 63, 94, 0.12)'
    };
  }
  // Default: theme-obsidian
  return {
    deep: '#111317',
    ripple: '#1e2129',
    backdrop: '#0a0b0d',
    wave: 'rgba(245, 158, 11, 0.10)'
  };
}

function setTheme(theme) {
  activeTheme = theme;
  document.body.className = `worldbox-game ${theme}`;
  localStorage.setItem('cogniverse_theme', theme);

  const select = document.getElementById('theme-select');
  if (select) select.value = theme;

  const hubLabel = document.getElementById('hub-current-theme-label');
  if (hubLabel) {
    const names = {
      'theme-obsidian': 'Obsidian Gold',
      'theme-amber': 'Alchemical Amber',
      'theme-emerald': 'Emerald Sanctum',
      'theme-crimson': 'Crimson Basalt'
    };
    hubLabel.textContent = names[theme] || theme;
  }

  // Re-render terrain offscreen buffer with new theme palette
  buildTerrainBuffer();
  showToast(`🎨 Theme switched to: ${theme.replace('theme-', '').toUpperCase()}`);
}

function cycleThemeFromHub() {
  const themes = ['theme-obsidian', 'theme-amber', 'theme-emerald', 'theme-crimson'];
  const nextIdx = (themes.indexOf(activeTheme) + 1) % themes.length;
  setTheme(themes[nextIdx]);
}

// ----------------------------------------------------------------------------
// GPU POWER & COMPUTATIONAL TIER CONTROLLER
// ----------------------------------------------------------------------------
let gpuTier = localStorage.getItem('cogniverse_gpu_tier') || 'medium';

function setGpuTier(tier) {
  gpuTier = tier;
  localStorage.setItem('cogniverse_gpu_tier', tier);

  const select = document.getElementById('gpu-tier-select');
  if (select) select.value = tier;

  const hubLabel = document.getElementById('hub-current-gpu-label');
  if (hubLabel) {
    const labels = {
      'ultra': 'Ultra (Bloom & Shaders)',
      'medium': 'Medium (Smooth 60 FPS)',
      'low': 'Low (Battery Saver)'
    };
    hubLabel.textContent = labels[tier] || tier;
  }

  showToast(`⚡ GPU Tier: ${tier.toUpperCase()}`);
}

function cycleGpuTierFromHub() {
  const tiers = ['ultra', 'medium', 'low'];
  const nextIdx = (tiers.indexOf(gpuTier) + 1) % tiers.length;
  setGpuTier(tiers[nextIdx]);
}

// ----------------------------------------------------------------------------
// JSON EXPORT & IMPORT RUN STATE ENGINE
// ----------------------------------------------------------------------------
function triggerExportState() {
  playSound('click');
  showToast('💾 Exporting world run state, AST brains & achievements to JSON...');
  sendMsg({ type: 'ExportState' });
}

function handleReceivedExportSnapshot(snapshotJson) {
  try {
    const parsed = typeof snapshotJson === 'string' ? JSON.parse(snapshotJson) : snapshotJson;
    const jsonStr = typeof snapshotJson === 'string' ? snapshotJson : JSON.stringify(snapshotJson, null, 2);

    // 1. Auto-download JSON file
    const blob = new Blob([jsonStr], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    const safeMapName = (parsed.metadata?.map_name || 'world').replace(/[^a-zA-Z0-9]/g, '_');
    const tick = parsed.metadata?.tick_count ?? 0;
    a.href = url;
    a.download = `cogniverse_run_${safeMapName}_tick${tick}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);

    // 2. Save snapshot to LocalStorage archive for Start Panel and Cross-World Stats
    saveSnapshotToArchive(parsed);

    showToast('💾 World state exported! File downloaded & chronicle archived.');
  } catch (err) {
    console.error('Error handling exported snapshot:', err);
    showToast('❌ Export error: failed to process snapshot JSON');
  }
}

function triggerImportPrompt() {
  playSound('click');
  const input = document.getElementById('file-import-input');
  if (input) input.click();
}

function onFileImportSelected(event) {
  const file = event.target.files && event.target.files[0];
  if (!file) return;

  const reader = new FileReader();
  reader.onload = (e) => {
    try {
      const content = e.target.result;
      const parsed = JSON.parse(content);

      if (!parsed.metadata || !parsed.agents) {
        throw new Error('Invalid Cogniverse snapshot format');
      }

      // Send to server engine
      sendMsg({
        type: 'ImportState',
        snapshot_json: content
      });

      // Save in local archive
      saveSnapshotToArchive(parsed);

      showToast(`📂 Imported world '${parsed.metadata.map_name || 'Realm'}' successfully!`);
      closeStartModal();
    } catch (err) {
      console.error('Snapshot import error:', err);
      alert(`Failed to import JSON chronicle: ${err.message}`);
    }
  };
  reader.readAsText(file);
  event.target.value = '';
}

function getSavedRunsArchive() {
  try {
    const stored = localStorage.getItem('cogniverse_saved_runs');
    return stored ? JSON.parse(stored) : [];
  } catch (e) {
    return [];
  }
}

function saveSnapshotToArchive(snapshot) {
  try {
    const archive = getSavedRunsArchive();
    const id = `${snapshot.metadata?.map_id || 1}_${snapshot.metadata?.tick_count || 0}_${Date.now()}`;
    const entry = {
      id,
      title: snapshot.metadata?.map_name || 'World Realm',
      archetype: snapshot.metadata?.map_archetype || 'GrandArchipelago',
      seed: snapshot.metadata?.map_seed || 42,
      tick: snapshot.metadata?.tick_count || 0,
      year: Math.floor((snapshot.metadata?.tick_count || 0) / 1000) + 1,
      population: snapshot.agents?.length || 0,
      terran_kp: snapshot.civilizations?.terran_culture?.knowledge_points || 0,
      lunar_kp: snapshot.civilizations?.lunar_culture?.knowledge_points || 0,
      tech_count: (snapshot.civilizations?.terran_culture?.technologies?.filter(t => t.is_unlocked)?.length || 0) +
                  (snapshot.civilizations?.lunar_culture?.technologies?.filter(t => t.is_unlocked)?.length || 0),
      timestamp: new Date().toLocaleDateString(),
      rawSnapshot: snapshot
    };

    // Keep up to 10 snapshots, avoiding duplicates of same map & tick
    const filtered = archive.filter(a => !(a.title === entry.title && a.tick === entry.tick));
    filtered.unshift(entry);
    if (filtered.length > 10) filtered.pop();

    localStorage.setItem('cogniverse_saved_runs', JSON.stringify(filtered));
    renderSavedRunsList();
    renderStatsTable();
  } catch (e) {
    console.error('Failed to save snapshot to archive:', e);
  }
}

function loadArchivedRun(id) {
  const archive = getSavedRunsArchive();
  const run = archive.find(r => r.id === id);
  if (!run || !run.rawSnapshot) return;

  sendMsg({
    type: 'ImportState',
    snapshot_json: JSON.stringify(run.rawSnapshot)
  });
  playSound('chime');
  showToast(`📂 Restored realm: ${run.title}`);
  closeStartModal();
  setSpeed(1);
}

function deleteArchivedRun(id, event) {
  if (event) event.stopPropagation();
  const archive = getSavedRunsArchive();
  const filtered = archive.filter(r => r.id !== id);
  localStorage.setItem('cogniverse_saved_runs', JSON.stringify(filtered));
  renderSavedRunsList();
  renderStatsTable();
  showToast('Removed chronicle from archive');
}

// ----------------------------------------------------------------------------
// GAME START PANEL CONTROLLER
// ----------------------------------------------------------------------------
function toggleStartModal() {
  playSound('click');
  const modal = document.getElementById('modal-game-start');
  if (!modal) return;
  modal.classList.toggle('hidden');
  if (!modal.classList.contains('hidden')) {
    updateStartModalSummary();
    renderSavedRunsList();
  }
}

function closeStartModal() {
  playSound('click');
  const modal = document.getElementById('modal-game-start');
  if (modal) modal.classList.add('hidden');
}

function continueActiveWorld() {
  playSound('chime');
  closeStartModal();
  setSpeed(1); // unpause if paused
  showToast('🚀 Resumed active realm evolution');
}

function openMapForgeFromStart() {
  closeStartModal();
  toggleWorldModal();
}

function openStatsFromStart() {
  closeStartModal();
  toggleStatsModal();
}

function updateStartModalSummary() {
  const summaryEl = document.getElementById('start-active-summary');
  if (!summaryEl || !currentTelemetry) return;

  const tick = currentTelemetry.world_tick ?? 0;
  const year = Math.floor(tick / 1000) + 1;
  const pop = currentTelemetry.demographics?.total_population ?? 16;
  const dip = currentTelemetry.diplomacy_status || 'Peaceful';
  summaryEl.textContent = `Year ${year} | ${pop} Citizens | ${dip}`;
}

function renderSavedRunsList() {
  const container = document.getElementById('saved-runs-list');
  const countEl = document.getElementById('saved-runs-count');
  if (!container) return;

  const archive = getSavedRunsArchive();
  if (countEl) countEl.textContent = `${archive.length} Archived Worlds`;

  if (archive.length === 0) {
    container.innerHTML = '<div class="empty-runs-state">No saved snapshots yet. Click <strong>Export World State</strong> or import a JSON file to build your multiversal archive!</div>';
    return;
  }

  container.innerHTML = '';
  for (const run of archive) {
    const card = document.createElement('div');
    card.className = 'saved-run-card';
    card.innerHTML = `
      <div class="run-card-info">
        <span class="run-card-title">${run.title}</span>
        <span class="run-card-meta">Year ${run.year} (Tick ${run.tick}) • ${run.population} Citizens • ${run.terran_kp + run.lunar_kp} KP • ${run.timestamp}</span>
      </div>
      <div class="run-card-actions">
        <button class="btn-run-load" onclick="loadArchivedRun('${run.id}')">Load Run</button>
        <button class="btn-run-del" onclick="deleteArchivedRun('${run.id}', event)">✖</button>
      </div>
    `;
    container.appendChild(card);
  }
}

// ----------------------------------------------------------------------------
// MULTI-WORLD STATISTICS & ADVANCED CANVAS CHART ENGINE
// ----------------------------------------------------------------------------
let activeGraphStyle = 'timeline';
let activeStatsMetric = 'knowledge';
const activeWorldTimeSeries = []; // Samples recorded over years

// Sample live metrics for cross-world learning curves
setInterval(() => {
  if (!currentTelemetry) return;
  const tick = currentTelemetry.world_tick ?? 0;
  const year = Math.floor(tick / 1000) + 1;
  const terranKp = currentTelemetry.terran_culture?.knowledge_points || 0;
  const lunarKp = currentTelemetry.lunar_culture?.knowledge_points || 0;
  const totalKp = terranKp + lunarKp;

  const unlockedTech = (currentTelemetry.terran_culture?.technologies?.filter(t => t.is_unlocked)?.length || 0) +
                       (currentTelemetry.lunar_culture?.technologies?.filter(t => t.is_unlocked)?.length || 0);

  let maxGen = 1;
  let maxMutations = 0;
  if (currentTelemetry.agents) {
    for (const a of currentTelemetry.agents) {
      if (a.generation > maxGen) maxGen = a.generation;
      if (a.genome && a.genome.mutation_count > maxMutations) maxMutations = a.genome.mutation_count;
    }
  }

  const discoveriesCount = (currentTelemetry.terran_culture?.discoveries?.length || 0) +
                           (currentTelemetry.lunar_culture?.discoveries?.length || 0);

  const sample = {
    tick,
    year,
    kp: totalKp,
    tech: unlockedTech,
    gen: maxGen,
    mutations: maxMutations,
    discoveries: discoveriesCount,
    pop: currentTelemetry.demographics?.total_population || 16
  };

  // Keep last 40 time samples
  if (activeWorldTimeSeries.length === 0 || activeWorldTimeSeries[activeWorldTimeSeries.length - 1].tick !== tick) {
    activeWorldTimeSeries.push(sample);
    if (activeWorldTimeSeries.length > 40) activeWorldTimeSeries.shift();
  }
}, 4000);

function toggleStatsModal() {
  playSound('click');
  const modal = document.getElementById('modal-learning-stats');
  if (!modal) return;
  modal.classList.toggle('hidden');
  if (!modal.classList.contains('hidden')) {
    updateStatsWorldCheckboxes();
    updateStatsKpis();
    renderStatsTable();
    renderStatsCharts();
  }
}

function closeStatsModal() {
  playSound('click');
  const modal = document.getElementById('modal-learning-stats');
  if (modal) modal.classList.add('hidden');
}

function setGraphStyle(style) {
  playSound('click');
  activeGraphStyle = style;
  document.querySelectorAll('#graph-style-group .stats-style-btn').forEach(btn => {
    btn.classList.toggle('active', btn.getAttribute('data-style') === style);
  });
  renderStatsCharts();
}

function onStatsMetricChange(val) {
  activeStatsMetric = val;
  renderStatsCharts();
}

function updateStatsWorldCheckboxes() {
  const container = document.getElementById('stats-world-checkboxes');
  if (!container) return;

  const activeName = currentTelemetry?.map_name || 'Active Realm';
  const activeLabel = document.getElementById('lbl-active-world-name');
  if (activeLabel) activeLabel.textContent = activeName;

  const archive = getSavedRunsArchive();
  // Keep active world chk, add archived worlds
  const archivedHtml = archive.slice(0, 2).map((run, idx) => `
    <label class="stats-chk-label">
      <input type="checkbox" id="chk-world-arch-${idx}" checked onchange="renderStatsCharts()" />
      <span class="world-color-dot archived-${idx + 1}"></span>
      <span>${run.title} (Yr ${run.year})</span>
    </label>
  `).join('');

  container.innerHTML = `
    <label class="stats-chk-label">
      <input type="checkbox" id="chk-world-active" checked onchange="renderStatsCharts()" />
      <span class="world-color-dot active-world"></span>
      <span id="lbl-active-world-name">${activeName}</span>
    </label>
    ${archivedHtml}
  `;
}

function updateStatsKpis() {
  if (!currentTelemetry) return;
  const terranDisc = currentTelemetry.terran_culture?.discoveries?.length || 0;
  const lunarDisc = currentTelemetry.lunar_culture?.discoveries?.length || 0;
  const totalDisc = terranDisc + lunarDisc;
  document.getElementById('kpi-total-discoveries').textContent = totalDisc;

  let maxGen = 1;
  let maxMut = 0;
  if (currentTelemetry.agents) {
    for (const a of currentTelemetry.agents) {
      if (a.generation > maxGen) maxGen = a.generation;
      if (a.genome && a.genome.mutation_count > maxMut) maxMut = a.genome.mutation_count;
    }
  }
  document.getElementById('kpi-max-gen').textContent = `Gen ${maxGen} (${maxMut} mut)`;

  const unlockedTech = (currentTelemetry.terran_culture?.technologies?.filter(t => t.is_unlocked)?.length || 0) +
                       (currentTelemetry.lunar_culture?.technologies?.filter(t => t.is_unlocked)?.length || 0);
  document.getElementById('kpi-tech-mastered').textContent = `${unlockedTech} / 12`;

  const tick = currentTelemetry.world_tick ?? 0;
  const year = Math.floor(tick / 1000) + 1;
  document.getElementById('kpi-years-lived').textContent = `Year ${year}`;
}

function renderStatsTable() {
  const tbody = document.getElementById('stats-table-body');
  if (!tbody) return;
  tbody.innerHTML = '';

  // 1. Row for Active Realm
  const tick = currentTelemetry?.world_tick ?? 0;
  const year = Math.floor(tick / 1000) + 1;
  const activeKp = (currentTelemetry?.terran_culture?.knowledge_points || 0) + (currentTelemetry?.lunar_culture?.knowledge_points || 0);
  const activeTech = (currentTelemetry?.terran_culture?.technologies?.filter(t => t.is_unlocked)?.length || 0) +
                     (currentTelemetry?.lunar_culture?.technologies?.filter(t => t.is_unlocked)?.length || 0);

  const activeRow = document.createElement('tr');
  activeRow.innerHTML = `
    <td><strong style="color:var(--wb-gold-bright)">⭐ ${currentTelemetry?.map_name || 'Active Realm'}</strong></td>
    <td>${currentTelemetry?.map_archetype || 'GrandArchipelago'} (Seed 42)</td>
    <td>Year ${year} (${tick} ticks)</td>
    <td>${currentTelemetry?.demographics?.total_population || 16}</td>
    <td>${activeKp} KP</td>
    <td>${activeTech} / 12</td>
    <td>2 Consecrated</td>
    <td><span style="color:#10b981">${currentTelemetry?.diplomacy_status || 'Peaceful'}</span></td>
    <td><button class="btn-run-load" onclick="triggerExportState()">Export JSON</button></td>
  `;
  tbody.appendChild(activeRow);

  // 2. Rows for Archived Saved Runs
  const archive = getSavedRunsArchive();
  for (const run of archive) {
    const row = document.createElement('tr');
    row.innerHTML = `
      <td>📜 ${run.title}</td>
      <td>${run.archetype} (Seed ${run.seed})</td>
      <td>Year ${run.year} (${run.tick} ticks)</td>
      <td>${run.population}</td>
      <td>${run.terran_kp + run.lunar_kp} KP</td>
      <td>${run.tech_count} / 12</td>
      <td>2 Altars</td>
      <td>Peaceful</td>
      <td><button class="btn-run-load" onclick="loadArchivedRun('${run.id}')">Load Run</button></td>
    `;
    tbody.appendChild(row);
  }
}

// ----------------------------------------------------------------------------
// CANVAS CHART GRAPH RENDERING (TIMELINE, RADAR, BAR, DONUT)
// ----------------------------------------------------------------------------
function renderStatsCharts() {
  const sCanvas = document.getElementById('stats-chart-canvas');
  if (!sCanvas) return;
  const sCtx = sCanvas.getContext('2d');
  const w = sCanvas.width;
  const h = sCanvas.height;

  // Clear background
  sCtx.fillStyle = '#14151b';
  sCtx.fillRect(0, 0, w, h);

  // Grid Lines
  sCtx.strokeStyle = 'rgba(255, 255, 255, 0.05)';
  sCtx.lineWidth = 1;
  for (let x = 40; x < w; x += 60) {
    sCtx.beginPath();
    sCtx.moveTo(x, 20);
    sCtx.lineTo(x, h - 30);
    sCtx.stroke();
  }
  for (let y = 30; y < h - 30; y += 45) {
    sCtx.beginPath();
    sCtx.moveTo(40, y);
    sCtx.lineTo(w - 20, y);
    sCtx.stroke();
  }

  if (activeGraphStyle === 'timeline') {
    drawTimelineChart(sCtx, w, h);
  } else if (activeGraphStyle === 'radar') {
    drawRadarChart(sCtx, w, h);
  } else if (activeGraphStyle === 'bars') {
    drawBarChart(sCtx, w, h);
  } else if (activeGraphStyle === 'donut') {
    drawDonutChart(sCtx, w, h);
  }
}

function drawTimelineChart(ctx, w, h) {
  const padL = 60;
  const padR = 30;
  const padT = 30;
  const padB = 40;
  const plotW = w - padL - padR;
  const plotH = h - padT - padB;

  // Ensure active points exist
  const activePts = activeWorldTimeSeries.length > 0 ? activeWorldTimeSeries : [
    { year: 1, kp: 10, tech: 1, gen: 1 },
    { year: 2, kp: 35, tech: 2, gen: 2 },
    { year: 3, kp: 80, tech: 4, gen: 3 },
  ];

  // Synthesize archived comparison series if saved
  const archive = getSavedRunsArchive();
  const arch1Pts = archive[0] ? [
    { year: 1, kp: 15, tech: 1 },
    { year: Math.max(2, Math.floor(archive[0].year * 0.5)), kp: Math.floor(archive[0].terran_kp * 0.5), tech: Math.floor(archive[0].tech_count * 0.5) },
    { year: archive[0].year, kp: archive[0].terran_kp + archive[0].lunar_kp, tech: archive[0].tech_count }
  ] : null;

  // Extract metric values
  const getVal = (pt) => {
    if (activeStatsMetric === 'tech') return pt.tech || 0;
    if (activeStatsMetric === 'generation') return pt.gen || 1;
    if (activeStatsMetric === 'discoveries') return pt.discoveries || 0;
    return pt.kp || 0; // default knowledge
  };

  let maxVal = 10;
  for (const pt of activePts) {
    const v = getVal(pt);
    if (v > maxVal) maxVal = v;
  }
  if (arch1Pts) {
    for (const pt of arch1Pts) {
      const v = getVal(pt);
      if (v > maxVal) maxVal = v;
    }
  }
  maxVal = Math.ceil(maxVal * 1.25);

  // Y-axis labels
  ctx.fillStyle = '#71717a';
  ctx.font = '10px "JetBrains Mono", monospace';
  ctx.textAlign = 'right';
  for (let i = 0; i <= 4; i++) {
    const val = Math.round((maxVal / 4) * i);
    const y = padT + plotH - (plotH / 4) * i;
    ctx.fillText(val.toString(), padL - 8, y + 3);
  }

  // Draw Series 1: Active World
  const chkActive = document.getElementById('chk-world-active');
  if (!chkActive || chkActive.checked) {
    const pts = activePts.map((pt, i) => {
      const x = padL + (i / Math.max(1, activePts.length - 1)) * plotW;
      const y = padT + plotH - (getVal(pt) / maxVal) * plotH;
      return { x, y, pt };
    });

    // Area Fill
    ctx.fillStyle = 'rgba(245, 158, 11, 0.15)';
    ctx.beginPath();
    ctx.moveTo(padL, padT + plotH);
    for (const p of pts) ctx.lineTo(p.x, p.y);
    ctx.lineTo(pts[pts.length - 1].x, padT + plotH);
    ctx.closePath();
    ctx.fill();

    // Glowing Curve Line
    ctx.strokeStyle = '#f59e0b';
    ctx.lineWidth = 3;
    ctx.shadowColor = 'rgba(245, 158, 11, 0.6)';
    ctx.shadowBlur = 8;
    ctx.beginPath();
    for (let i = 0; i < pts.length; i++) {
      if (i === 0) ctx.moveTo(pts[i].x, pts[i].y);
      else ctx.lineTo(pts[i].x, pts[i].y);
    }
    ctx.stroke();
    ctx.shadowBlur = 0;

    // Glowing Vertices
    for (const p of pts) {
      ctx.fillStyle = '#fbbf24';
      ctx.beginPath();
      ctx.arc(p.x, p.y, 4, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  // Draw Series 2: Archived World
  const chkArch1 = document.getElementById('chk-world-arch-0');
  if (arch1Pts && chkArch1 && chkArch1.checked) {
    const pts = arch1Pts.map((pt, i) => {
      const x = padL + (i / Math.max(1, arch1Pts.length - 1)) * plotW;
      const y = padT + plotH - (getVal(pt) / maxVal) * plotH;
      return { x, y, pt };
    });

    ctx.strokeStyle = '#10b981';
    ctx.lineWidth = 2.5;
    ctx.setLineDash([4, 4]);
    ctx.beginPath();
    for (let i = 0; i < pts.length; i++) {
      if (i === 0) ctx.moveTo(pts[i].x, pts[i].y);
      else ctx.lineTo(pts[i].x, pts[i].y);
    }
    ctx.stroke();
    ctx.setLineDash([]);

    for (const p of pts) {
      ctx.fillStyle = '#34d399';
      ctx.beginPath();
      ctx.arc(p.x, p.y, 3.5, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  // X-axis label
  ctx.fillStyle = '#a1a1aa';
  ctx.font = '11px Outfit, sans-serif';
  ctx.textAlign = 'center';
  ctx.fillText('Evolutionary Spacetime (Progression over Elapsed Years)', padL + plotW / 2, h - 10);
}

function drawRadarChart(ctx, w, h) {
  const cx = w / 2;
  const cy = h / 2 + 10;
  const radius = Math.min(w, h) * 0.36;

  const axes = [
    '🌾 Agronomy',
    '⛏️ Metallurgy',
    '🛠️ Tool Crafting',
    '🏛️ Architecture',
    '🛐 Theology/Piety',
    '🗺️ Exploration'
  ];
  const numAxes = axes.length;

  // Background concentric polygons
  for (let level = 1; level <= 4; level++) {
    const r = (radius / 4) * level;
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.10)';
    ctx.lineWidth = 1;
    ctx.beginPath();
    for (let i = 0; i < numAxes; i++) {
      const angle = (Math.PI * 2 / numAxes) * i - Math.PI / 2;
      const x = cx + Math.cos(angle) * r;
      const y = cy + Math.sin(angle) * r;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.closePath();
    ctx.stroke();
  }

  // Draw Spokes & Axis Labels
  ctx.fillStyle = '#e4e4e7';
  ctx.font = '11px Outfit, sans-serif';
  for (let i = 0; i < numAxes; i++) {
    const angle = (Math.PI * 2 / numAxes) * i - Math.PI / 2;
    const x = cx + Math.cos(angle) * radius;
    const y = cy + Math.sin(angle) * radius;

    ctx.strokeStyle = 'rgba(255, 255, 255, 0.15)';
    ctx.beginPath();
    ctx.moveTo(cx, cy);
    ctx.lineTo(x, y);
    ctx.stroke();

    // Label offset
    const lx = cx + Math.cos(angle) * (radius + 22);
    const ly = cy + Math.sin(angle) * (radius + 18);
    ctx.textAlign = 'center';
    ctx.fillText(axes[i], lx, ly);
  }

  // Compute active world competencies (0.0 to 1.0)
  const terranKp = currentTelemetry?.terran_culture?.knowledge_points || 20;
  const lunarKp = currentTelemetry?.lunar_culture?.knowledge_points || 20;
  const techCount = (currentTelemetry?.terran_culture?.technologies?.filter(t => t.is_unlocked)?.length || 1);

  const activeVals = [
    Math.min(1.0, 0.4 + (terranKp % 50) / 100),       // Agronomy
    Math.min(1.0, 0.3 + (techCount * 0.12)),          // Metallurgy
    Math.min(1.0, 0.45 + (techCount * 0.1)),          // Tool Crafting
    Math.min(1.0, 0.35 + (currentTelemetry?.villages || 4) * 0.08), // Architecture
    Math.min(1.0, 0.5 + (lunarKp % 40) / 80),         // Theology
    Math.min(1.0, 0.55 + (currentTelemetry?.demographics?.total_population || 16) * 0.02) // Exploration
  ];

  // Draw Active World Polygon
  ctx.fillStyle = 'rgba(245, 158, 11, 0.25)';
  ctx.strokeStyle = '#f59e0b';
  ctx.lineWidth = 2.5;
  ctx.beginPath();
  for (let i = 0; i < numAxes; i++) {
    const angle = (Math.PI * 2 / numAxes) * i - Math.PI / 2;
    const r = radius * activeVals[i];
    const x = cx + Math.cos(angle) * r;
    const y = cy + Math.sin(angle) * r;
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  }
  ctx.closePath();
  ctx.fill();
  ctx.stroke();

  // Draw Archived Comparison Polygon if available
  const archive = getSavedRunsArchive();
  if (archive.length > 0) {
    const archVals = [0.75, 0.65, 0.70, 0.60, 0.80, 0.65];
    ctx.fillStyle = 'rgba(16, 185, 129, 0.18)';
    ctx.strokeStyle = '#10b981';
    ctx.lineWidth = 2;
    ctx.beginPath();
    for (let i = 0; i < numAxes; i++) {
      const angle = (Math.PI * 2 / numAxes) * i - Math.PI / 2;
      const r = radius * archVals[i];
      const x = cx + Math.cos(angle) * r;
      const y = cy + Math.sin(angle) * r;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.closePath();
    ctx.fill();
    ctx.stroke();
  }
}

function drawBarChart(ctx, w, h) {
  const padL = 70;
  const padR = 40;
  const padT = 40;
  const padB = 40;
  const plotW = w - padL - padR;
  const plotH = h - padT - padB;

  const categories = [
    'Knowledge (KP / 10)',
    'Technologies Unlocked',
    'Max Generation (AST)',
    'Lifespan (Years)',
    'Altar Offerings'
  ];

  const activeKp = (currentTelemetry?.terran_culture?.knowledge_points || 20) + (currentTelemetry?.lunar_culture?.knowledge_points || 20);
  const activeTech = (currentTelemetry?.terran_culture?.technologies?.filter(t => t.is_unlocked)?.length || 2);
  const activeYear = Math.floor((currentTelemetry?.world_tick || 0) / 1000) + 1;

  const activeData = [activeKp / 10, activeTech, 2, activeYear, 6];
  const archData = [12, 5, 3, 4, 15]; // sample archived world baseline

  const numGroups = categories.length;
  const groupWidth = plotW / numGroups;
  const barWidth = groupWidth * 0.32;
  const maxVal = 20;

  for (let i = 0; i < numGroups; i++) {
    const groupX = padL + i * groupWidth;

    // Active Bar (Gold)
    const h1 = (activeData[i] / maxVal) * plotH;
    ctx.fillStyle = '#f59e0b';
    ctx.fillRect(groupX + groupWidth * 0.15, padT + plotH - h1, barWidth, h1);

    // Archived Bar (Emerald)
    const h2 = (archData[i] / maxVal) * plotH;
    ctx.fillStyle = '#10b981';
    ctx.fillRect(groupX + groupWidth * 0.15 + barWidth + 4, padT + plotH - h2, barWidth, h2);

    // Label
    ctx.fillStyle = '#a1a1aa';
    ctx.font = '10px Outfit, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(categories[i], groupX + groupWidth * 0.5, h - 14);
  }

  // Legend
  ctx.fillStyle = '#f59e0b';
  ctx.fillRect(w - 180, 15, 10, 10);
  ctx.fillStyle = '#f5f5f7';
  ctx.font = '11px Outfit, sans-serif';
  ctx.textAlign = 'left';
  ctx.fillText('Active Realm', w - 165, 24);

  ctx.fillStyle = '#10b981';
  ctx.fillRect(w - 90, 15, 10, 10);
  ctx.fillStyle = '#f5f5f7';
  ctx.fillText('Archived Run', w - 75, 24);
}

function drawDonutChart(ctx, w, h) {
  const cx = w / 2;
  const cy = h / 2 + 10;
  const outerR = Math.min(w, h) * 0.36;
  const innerR = outerR * 0.55;

  const slices = [
    { label: 'Agronomy (Food)', val: 30, color: '#f59e0b' },
    { label: 'Metallurgy (Tools)', val: 25, color: '#10b981' },
    { label: 'Arcane Theology', val: 20, color: '#c084fc' },
    { label: 'Architecture', val: 15, color: '#38bdf8' },
    { label: 'Social & Diplomacy', val: 10, color: '#f43f5e' }
  ];

  const total = slices.reduce((acc, s) => acc + s.val, 0);
  let startAngle = -Math.PI / 2;

  for (const s of slices) {
    const sliceAngle = (s.val / total) * Math.PI * 2;
    ctx.fillStyle = s.color;
    ctx.beginPath();
    ctx.arc(cx, cy, outerR, startAngle, startAngle + sliceAngle);
    ctx.arc(cx, cy, innerR, startAngle + sliceAngle, startAngle, true);
    ctx.closePath();
    ctx.fill();

    // Outer label
    const midAngle = startAngle + sliceAngle / 2;
    const lx = cx + Math.cos(midAngle) * (outerR + 24);
    const ly = cy + Math.sin(midAngle) * (outerR + 24);
    ctx.fillStyle = s.color;
    ctx.font = '11px Outfit, sans-serif';
    ctx.textAlign = Math.cos(midAngle) > 0 ? 'left' : 'right';
    ctx.fillText(`${s.label} (${s.val}%)`, lx, ly);

    startAngle += sliceAngle;
  }

  // Inner hole text
  ctx.fillStyle = '#f5f5f7';
  ctx.font = '800 13px MedievalSharp, cursive';
  ctx.textAlign = 'center';
  ctx.fillText('COGNITIVE', cx, cy - 6);
  ctx.font = '11px Outfit, sans-serif';
  ctx.fillStyle = '#a1a1aa';
  ctx.fillText('DOMAINS', cx, cy + 12);
}

// ============================================================================
// PHASE 9: REALM POLISH, SOVEREIGN MINIMAP RADAR, WAR ROOM & SETTINGS
// ============================================================================

let realmSettings = {
  showMinimap: true,
  showTitanAuras: true,
  showDemigodHalos: true,
  showDayNightFilter: true,
};

try {
  const savedSettings = localStorage.getItem('cogniverse_realm_settings');
  if (savedSettings) {
    realmSettings = Object.assign(realmSettings, JSON.parse(savedSettings));
  }
} catch (e) {
  console.warn('Failed to parse saved realm settings:', e);
}

function saveRealmSettings() {
  try {
    localStorage.setItem('cogniverse_realm_settings', JSON.stringify(realmSettings));
  } catch (e) {}
}

const minimapCanvas = document.getElementById('minimap-canvas');
const minimapCtx = minimapCanvas ? minimapCanvas.getContext('2d') : null;
let isMinimapDragging = false;

function drawMinimap(data) {
  if (!minimapCanvas || !minimapCtx || !realmSettings.showMinimap) return;
  const mc = document.getElementById('minimap-container');
  if (mc && mc.classList.contains('minimized')) return;

  const w = minimapCanvas.width;
  const h = minimapCanvas.height;

  // 1. Draw cached terrain buffer directly (fast GPU blit)
  minimapCtx.drawImage(terrainCanvas, 0, 0, w, h);

  const scale = w / WORLD_SIZE;

  // 2. Overlay Kingdom Territorial Control
  if (data.territory && Array.isArray(data.territory)) {
    for (const t of data.territory) {
      if (t.faction === 'SolarTerran') {
        minimapCtx.fillStyle = 'rgba(251, 191, 36, 0.22)';
        minimapCtx.fillRect(t.x * scale, t.y * scale, scale, scale);
      } else if (t.faction === 'LunarElves') {
        minimapCtx.fillStyle = 'rgba(168, 85, 247, 0.22)';
        minimapCtx.fillRect(t.x * scale, t.y * scale, scale, scale);
      }
    }
  }

  // 3. Draw Population / Agents
  if (data.agents && Array.isArray(data.agents)) {
    for (const a of data.agents) {
      if (!a.is_alive) continue;
      const ax = a.x * scale;
      const ay = a.y * scale;
      if (a.is_demigod) {
        minimapCtx.fillStyle = '#fbbf24';
        minimapCtx.fillRect(ax - 1, ay - 1, 3, 3);
      } else if (a.faction === 'SolarTerran') {
        minimapCtx.fillStyle = '#60a5fa';
        minimapCtx.fillRect(ax, ay, 1.5, 1.5);
      } else {
        minimapCtx.fillStyle = '#c084fc';
        minimapCtx.fillRect(ax, ay, 1.5, 1.5);
      }
    }
  }

  // 4. Draw Primordial Titans (Colossal Boss Radar Blips)
  if (data.titans && Array.isArray(data.titans)) {
    const time = Date.now();
    for (const t of data.titans) {
      if (t.state === 'Defeated') continue;
      const tx = t.x * scale;
      const ty = t.y * scale;
      const pulse = Math.sin(time * 0.006 + t.id) * 1.5;

      minimapCtx.fillStyle = t.state === 'Enraged' ? '#ef4444' : '#f97316';
      minimapCtx.beginPath();
      minimapCtx.arc(tx, ty, 3.5 + pulse, 0, Math.PI * 2);
      minimapCtx.fill();

      minimapCtx.strokeStyle = '#ffffff';
      minimapCtx.lineWidth = 1;
      minimapCtx.stroke();
    }
  }

  // 5. Draw Siege Engines
  if (data.siege_engines && Array.isArray(data.siege_engines)) {
    for (const se of data.siege_engines) {
      if (!se.is_operational) continue;
      minimapCtx.fillStyle = '#f59e0b';
      minimapCtx.fillRect(se.x * scale - 1, se.y * scale - 1, 3, 3);
    }
  }

  // 6. Draw Camera Frustum Viewport Rectangle
  const tilePx = TILE_PX * camera.zoom;
  const minWx = Math.max(0, -camera.x / tilePx);
  const minWy = Math.max(0, -camera.y / tilePx);
  const maxWx = Math.min(WORLD_SIZE, (-camera.x + canvas.width) / tilePx);
  const maxWy = Math.min(WORLD_SIZE, (-camera.y + canvas.height) / tilePx);

  const mx = minWx * scale;
  const my = minWy * scale;
  const mw = Math.max(6, (maxWx - minWx) * scale);
  const mh = Math.max(6, (maxWy - minWy) * scale);

  minimapCtx.strokeStyle = '#f59e0b';
  minimapCtx.lineWidth = 1.5;
  minimapCtx.strokeRect(mx, my, mw, mh);
  minimapCtx.fillStyle = 'rgba(245, 158, 11, 0.1)';
  minimapCtx.fillRect(mx, my, mw, mh);

  // Update Coordinates Label in Header
  const coordsEl = document.getElementById('minimap-coords');
  if (coordsEl) {
    const centerWx = Math.round(Math.min(WORLD_SIZE - 1, Math.max(0, (minWx + maxWx) / 2)));
    const centerWy = Math.round(Math.min(WORLD_SIZE - 1, Math.max(0, (minWy + maxWy) / 2)));
    coordsEl.textContent = `${centerWx}, ${centerWy}`;
  }
}

function handleMinimapNav(e) {
  if (!minimapCanvas) return;
  const rect = minimapCanvas.getBoundingClientRect();
  const mx = e.clientX - rect.left;
  const my = e.clientY - rect.top;
  const wx = Math.max(0, Math.min(WORLD_SIZE - 1, (mx / rect.width) * WORLD_SIZE));
  const wy = Math.max(0, Math.min(WORLD_SIZE - 1, (my / rect.height) * WORLD_SIZE));
  centerCameraOn(wx, wy);
}

function toggleMinimap() {
  const mc = document.getElementById('minimap-container');
  if (mc) {
    mc.classList.toggle('minimized');
    playSound('click');
  }
}

// War Room Modal Controls
function toggleWarRoomModal() {
  const modal = document.getElementById('war-room-modal');
  if (!modal) return;
  const isHidden = modal.classList.toggle('hidden');
  if (!isHidden) {
    playSound('click');
    if (currentTelemetry) {
      renderWarRoom(currentTelemetry);
    }
  }
}

function switchWarRoomTab(tab) {
  playSound('click');
  ['war', 'titans', 'demigods'].forEach((t) => {
    const btn = document.getElementById(`wrtab-${t}`);
    const content = document.getElementById(`wr-content-${t}`);
    if (btn) btn.classList.toggle('active', t === tab);
    if (content) content.classList.toggle('active', t === tab);
  });
  if (currentTelemetry) {
    renderWarRoom(currentTelemetry);
  }
}

function renderWarRoom(data) {
  if (!data) return;

  // 1. Theater of War Tab
  const terranBar = document.getElementById('terran-territory-bar');
  const lunarBar = document.getElementById('lunar-territory-bar');
  let terranTiles = 0;
  let lunarTiles = 0;

  if (data.territory && Array.isArray(data.territory)) {
    for (const t of data.territory) {
      if (t.faction === 'SolarTerran') terranTiles++;
      else if (t.faction === 'LunarElves') lunarTiles++;
    }
  }
  const totalClaimed = Math.max(1, terranTiles + lunarTiles);
  const terranPct = Math.round((terranTiles / totalClaimed) * 100);
  const lunarPct = 100 - terranPct;

  if (terranBar && lunarBar) {
    terranBar.style.width = `${Math.max(15, terranPct)}%`;
    terranBar.textContent = `${terranPct}% Solaria (${terranTiles})`;
    lunarBar.style.width = `${Math.max(15, lunarPct)}%`;
    lunarBar.textContent = `${lunarPct}% Lunaria (${lunarTiles})`;
  }

  const diploEl = document.getElementById('wr-diplomacy');
  if (diploEl) {
    diploEl.textContent = data.war_status || data.diplomacy_status || 'Autonomous Equilibrium';
  }

  const solGoldEl = document.getElementById('wr-terran-gold');
  const lunGoldEl = document.getElementById('wr-lunar-gold');
  if (data.kingdoms && Array.isArray(data.kingdoms)) {
    const sol = data.kingdoms.find((k) => k.name === 'Solar Terran Kingdom' || k.faction === 'SolarTerran');
    const lun = data.kingdoms.find((k) => k.name === 'Lunar Elves Realm' || k.faction === 'LunarElves');
    if (sol && solGoldEl) solGoldEl.textContent = `${sol.treasury || 250} Gold`;
    if (lun && lunGoldEl) lunGoldEl.textContent = `${lun.treasury || 250} Gold`;
  }

  // Siege Artillery List
  const siegeList = document.getElementById('wr-siege-list');
  if (siegeList) {
    if (!data.siege_engines || data.siege_engines.length === 0) {
      siegeList.innerHTML = '<div class="empty-notice">No siege artillery currently deployed in the field.</div>';
    } else {
      siegeList.innerHTML = data.siege_engines
        .map(
          (eng) => `
        <div class="wr-siege-item">
          <div>
            <strong>${eng.kind} (${eng.faction})</strong>
            <div class="small-text">Pos: [${eng.x}, ${eng.y}] → Target: [${eng.target_x}, ${eng.target_y}]</div>
            <div class="small-text">Status: ${eng.is_operational ? '⚔️ Firing' : '🔧 Damaged'} (HP: ${Math.round(eng.health ?? eng.hp ?? 0)}/${Math.round(eng.max_health ?? eng.max_hp ?? 100)})</div>
          </div>
          <button class="btn-micro" onclick="centerCameraOn(${eng.x}, ${eng.y}); toggleWarRoomModal();">Locate</button>
        </div>
      `
        )
        .join('');
    }
  }

  // 2. Primordial Titans Grid
  const titanGrid = document.getElementById('wr-titan-grid');
  if (titanGrid) {
    if (!data.titans || data.titans.length === 0) {
      titanGrid.innerHTML = '<div class="empty-notice">All Primordial Titans Slumber in Deep Leylines.</div>';
    } else {
      titanGrid.innerHTML = data.titans
        .map((t) => {
          const curHp = Math.round(t.hp ?? t.health ?? 0);
          const maxHp = Math.round(t.max_hp ?? t.max_health ?? 1);
          const hpPct = Math.round((curHp / Math.max(1, maxHp)) * 100);
          const icon = t.kind.includes('Leviathan') ? '🌊' : t.kind.includes('Magma') ? '🔥' : '🌌';
          return `
          <div class="titan-card ${t.state.toLowerCase()}">
            <div class="titan-card-header">
              <span class="titan-icon">${icon}</span>
              <div>
                <h4 class="titan-name">${t.name}</h4>
                <span class="titan-archetype">${t.kind} (Level ${t.level || 50})</span>
              </div>
            </div>
            <div class="titan-health-container">
              <div class="titan-health-bar" style="width: ${hpPct}%;"></div>
            </div>
            <div class="titan-stats-row">
              <span>HP: ${curHp} / ${maxHp}</span>
              <span class="titan-state-pill ${t.state.toLowerCase()}">${t.state}</span>
            </div>
            <div class="titan-loc-row">
              <span>Disruption Radius: ${t.disruption_radius} tiles</span>
              <span>Pos: [${t.x}, ${t.y}]</span>
            </div>
            <button class="btn-titan-locate" onclick="centerCameraOn(${t.x}, ${t.y}); toggleWarRoomModal();">🎯 Locate Boss on Map</button>
          </div>
        `;
        })
        .join('');
    }
  }

  // 3. Demigods & Relics Grid
  const demigodList = document.getElementById('wr-demigod-list');
  if (demigodList) {
    if (!data.demigods || data.demigods.length === 0) {
      demigodList.innerHTML = '<div class="empty-notice">No mortal has yet crossed the threshold of Divine Apotheosis.</div>';
    } else {
      demigodList.innerHTML = data.demigods
        .map((dg) => {
          const agent = data.agents ? data.agents.find((a) => a.id === dg.agent_id) : null;
          const name = dg.agent_name || (agent ? agent.name : `Demigod #${dg.agent_id}`);
          const ax = agent ? agent.x : 64;
          const ay = agent ? agent.y : 64;
          return `
          <div class="demigod-card">
            <div class="demigod-card-header">
              <span class="demigod-halo-icon">✦</span>
              <div>
                <h4 class="demigod-title">${name}</h4>
                <span class="demigod-domain">${dg.domain} Demigod</span>
              </div>
            </div>
            <div class="demigod-stats">
              <div class="demigod-stat-item">
                <span class="dg-label">Ascension Tier:</span>
                <span class="dg-val gold-text">Tier ${dg.ascension_tier || 1}</span>
              </div>
              <div class="demigod-stat-item">
                <span class="dg-label">Dynasty Heirloom:</span>
                <span class="dg-val">${dg.heirloom_name || 'Ancestral Relic'}</span>
              </div>
              <div class="demigod-stat-item">
                <span class="dg-label">Divine Status:</span>
                <span class="dg-val">Immortal Champion</span>
              </div>
            </div>
            <button class="btn-micro" onclick="selectedAgentId = ${dg.agent_id}; centerCameraOn(${ax}, ${ay}); openCharacterSheet(); toggleWarRoomModal();">Inspect Demigod & Relics</button>
          </div>
        `;
        })
        .join('');
    }
  }
}

// Settings Modal Controls
function toggleSettingsModal() {
  const modal = document.getElementById('settings-modal');
  if (!modal) return;
  const isHidden = modal.classList.toggle('hidden');
  if (!isHidden) {
    playSound('click');
    const v = (window.soundManager && window.soundManager.volumes)
      ? window.soundManager.volumes
      : {
          master: (window.soundManager && window.soundManager.masterVolume) ?? 0.8,
          sfx: (window.soundManager && window.soundManager.sfxVolume) ?? 0.85,
          ambient: (window.soundManager && window.soundManager.ambientVolume) ?? 0.4,
        };

    const sMaster = document.getElementById('slider-master-vol');
    const sSfx = document.getElementById('slider-sfx-vol');
    const sAmb = document.getElementById('slider-ambient-vol');
    if (sMaster) sMaster.value = Math.round(v.master * 100);
    if (sSfx) sSfx.value = Math.round(v.sfx * 100);
    if (sAmb) sAmb.value = Math.round(v.ambient * 100);

    const lMaster = document.getElementById('label-master-vol');
    const lSfx = document.getElementById('label-sfx-vol');
    const lAmb = document.getElementById('label-ambient-vol');
    if (lMaster) lMaster.textContent = Math.round(v.master * 100) + '%';
    if (lSfx) lSfx.textContent = Math.round(v.sfx * 100) + '%';
    if (lAmb) lAmb.textContent = Math.round(v.ambient * 100) + '%';

    const btnMute = document.getElementById('btn-toggle-mute');
    if (btnMute && window.soundManager) {
      btnMute.textContent = window.soundManager.isMuted ? 'Unmute Audio' : 'Mute Audio';
    }

    const cMinimap = document.getElementById('check-minimap-toggle');
    const cTitans = document.getElementById('check-titan-auras');
    const cDemigods = document.getElementById('check-demigod-halos');
    const cDayNight = document.getElementById('check-day-night');
    if (cMinimap) cMinimap.checked = realmSettings.showMinimap;
    if (cTitans) cTitans.checked = realmSettings.showTitanAuras;
    if (cDemigods) cDemigods.checked = realmSettings.showDemigodHalos;
    if (cDayNight) cDayNight.checked = realmSettings.showDayNightFilter;
  }
}

function onMasterVolChange(val) {
  const v = val / 100;
  if (window.soundManager) window.soundManager.setMasterVolume(v);
  const l = document.getElementById('label-master-vol');
  if (l) l.textContent = val + '%';
}

function onSfxVolChange(val) {
  const v = val / 100;
  if (window.soundManager) window.soundManager.setSfxVolume(v);
  const l = document.getElementById('label-sfx-vol');
  if (l) l.textContent = val + '%';
}

function onAmbientVolChange(val) {
  const v = val / 100;
  if (window.soundManager) window.soundManager.setAmbientVolume(v);
  const l = document.getElementById('label-ambient-vol');
  if (l) l.textContent = val + '%';
}

function toggleAudioMute() {
  if (window.soundManager) {
    const muted = window.soundManager.toggleMute();
    const btn = document.getElementById('btn-toggle-mute');
    if (btn) btn.textContent = muted ? 'Unmute Audio' : 'Mute Audio';
    playSound('click');
  }
}

function setRadarVisibility(checked) {
  realmSettings.showMinimap = checked;
  saveRealmSettings();
  const el = document.getElementById('minimap-container');
  if (el) el.style.display = checked ? 'block' : 'none';
}

function setTitanAuras(checked) {
  realmSettings.showTitanAuras = checked;
  saveRealmSettings();
}

function setDemigodHalos(checked) {
  realmSettings.showDemigodHalos = checked;
  saveRealmSettings();
}

function setDayNightFilter(checked) {
  realmSettings.showDayNightFilter = checked;
  saveRealmSettings();
}

function initMinimap() {
  if (minimapCanvas) {
    minimapCanvas.addEventListener('mousedown', (e) => {
      isMinimapDragging = true;
      handleMinimapNav(e);
      playSound('click');
    });
    window.addEventListener('mousemove', (e) => {
      if (isMinimapDragging) {
        handleMinimapNav(e);
      }
    });
    window.addEventListener('mouseup', () => {
      isMinimapDragging = false;
    });
  }
  initRetroMinimap();
}

function initRealmSettings() {
  const elMinimap = document.getElementById('minimap-container');
  if (elMinimap) {
    elMinimap.style.display = realmSettings.showMinimap ? 'block' : 'none';
  }
}

// ----------------------------------------------------------------------------
// INITIALIZATION
// ----------------------------------------------------------------------------
window.addEventListener('load', () => {
  resizeCanvas();
  setTheme(activeTheme);
  setGpuTier(gpuTier);
  buildTerrainBuffer();
  centerCamera();
  connectWebSocket();
  initMinimap();
  initRetroMinimap();
  initRealmSettings();
  requestAnimationFrame(gameLoop);
});

// ============================================================================
// DUAL-MODE UI RECREATION ENGINE (IMAGE 1 & IMAGE 2 FIDELITY)
// ============================================================================
let isWorldMapActive = false;

function setViewMode(worldMap) {
  isWorldMapActive = worldMap;
  const overlay = document.getElementById('world-map-overlay');
  const retroTopBar = document.getElementById('retro-top-bar');
  const debugCard = document.getElementById('debug-inspector-card');
  const npcPanel = document.getElementById('bottom-npc-inspector');
  const feedPanel = document.getElementById('bottom-event-feed');
  const minimapCard = document.getElementById('bottom-minimap-card');
  const buildToolbar = document.getElementById('bottom-build-toolbar');
  const legacyDock = document.getElementById('worldbox-bottom-bar');
  const toggleBtn = document.getElementById('retro-btn-view-toggle');

  if (legacyDock) legacyDock.style.display = 'none';

  if (isWorldMapActive) {
    closeRetroSubpalette();
    if (overlay) overlay.classList.remove('hidden');
    if (retroTopBar) retroTopBar.style.display = 'none';
    if (debugCard) debugCard.style.display = 'none';
    if (npcPanel) npcPanel.classList.add('hidden');
    if (feedPanel) feedPanel.style.display = 'none';
    if (minimapCard) minimapCard.style.display = 'none';
    if (buildToolbar) buildToolbar.style.display = 'none';
    if (toggleBtn) toggleBtn.textContent = '🏘️ Village (M)';
    if (camera.zoom > 0.35) {
      camera.zoom = 0.28;
      centerCamera();
    }
  } else {
    if (overlay) overlay.classList.add('hidden');
    if (retroTopBar) retroTopBar.style.display = 'flex';
    if (debugCard) debugCard.style.display = 'block';
    if (feedPanel) feedPanel.style.display = 'flex';
    if (minimapCard) minimapCard.style.display = 'flex';
    if (buildToolbar) buildToolbar.style.display = 'flex';
    if (toggleBtn) toggleBtn.textContent = '🌍 Map (M)';
    if (camera.zoom <= 0.35) {
      camera.zoom = 1.35;
      camera.x = canvas.width / 2 - (65 * TILE_PX) * camera.zoom;
      camera.y = canvas.height / 2 - (48 * TILE_PX) * camera.zoom;
    }
  }
}

function toggleViewMode() {
  setViewMode(!isWorldMapActive);
  playSound('click');
}

function checkViewModeTransition() {
  if (camera.zoom <= 0.35 && !isWorldMapActive) {
    setViewMode(true);
  } else if (camera.zoom > 0.35 && isWorldMapActive) {
    setViewMode(false);
  }
}

function closeNpcInspector() {
  const p = document.getElementById('bottom-npc-inspector');
  if (p) p.classList.add('hidden');
  playSound('click');
}

// ----------------------------------------------------------------------------
// RETRO SUB-PALETTES DATA & CONTROLS
// ----------------------------------------------------------------------------
const RETRO_SUBPALETTES = {
  nature: {
    title: 'Terrain, Elevation & Biomes',
    items: [
      { id: 'brush-raise', glyph: '⛰️', text: 'Raise Land', desc: 'Elevate land elevation upward' },
      { id: 'brush-lower', glyph: '🕳️', text: 'Lower Land', desc: 'Carve trench or oceanic depression' },
      { id: 'brush-flatten', glyph: '🪨', text: 'Flatten', desc: 'Level terrain to flat lowland' },
      { id: 'brush-forest', glyph: '🌲', text: 'Forest', desc: 'Dense ancient Whispering Forest' },
      { id: 'brush-plains', glyph: '🌾', text: 'Plains', desc: 'Fertile sunlit prairie grasslands' },
      { id: 'brush-savanna', glyph: '🦁', text: 'Savanna', desc: 'Warm dry golden savanna' },
      { id: 'brush-jungle', glyph: '🌴', text: 'Jungle', desc: 'Verdant tropical rainforest canopy' },
      { id: 'brush-permafrost', glyph: '❄️', text: 'Tundra', desc: 'Glacial permafrost expanse' },
      { id: 'brush-lava', glyph: '🌋', text: 'Lava', desc: 'Molten igneous volcanic magma' },
      { id: 'brush-clear', glyph: '🧹', text: 'Clear Land', desc: 'Scrape structures & rubble' },
    ]
  },
  mines: {
    title: 'All 10 Master Mine Types',
    items: [
      { id: 'mine-surface', glyph: '⛏️', text: 'Surface Pit', desc: 'Sand, clay, gravel & shallow coal' },
      { id: 'mine-quarry', glyph: '🪨', text: 'Stone Quarry', desc: 'Dimension stone & granite blocks' },
      { id: 'mine-tunnel', glyph: '🚇', text: 'Hill Tunnel', desc: 'Horizontal copper & tin adit' },
      { id: 'mine-shaft', glyph: '🏗️', text: 'Deep Shaft', desc: 'Vertical deep iron/gold headframe' },
      { id: 'mine-cave', glyph: '🕳️', text: 'Cave Mine', desc: 'Natural cave saltpeter & minerals' },
      { id: 'mine-drift', glyph: '⛰️', text: 'Hill Drift', desc: 'Horizontal coal & bauxite seam drift' },
      { id: 'mine-dredge', glyph: '⚓', text: 'Dredge Mine', desc: 'River silt gold panning dredge' },
      { id: 'mine-mana', glyph: '🔮', text: 'Mana Crystal', desc: 'Glowing raw mana geode veins' },
      { id: 'mine-ruin', glyph: '🏛️', text: 'Ancient Ruin', desc: 'Lost alloy & runestone excavation' },
      { id: 'mine-rift', glyph: '🌀', text: 'Dungeon Rift', desc: 'Void boss portal & abyssal core' },
    ]
  },
  architecture: {
    title: 'Architecture & Settlements',
    items: [
      { id: 'build-cottage', glyph: '🪵', text: 'Wood Cottage', desc: 'Handcrafted timber dwelling' },
      { id: 'build-cabin', glyph: '🏠', text: 'Stone Cabin', desc: 'Durable stone masonry shelter' },
      { id: 'build-townhall', glyph: '🏛️', text: 'Town Hall', desc: 'Civic capital government center' },
      { id: 'build-guild', glyph: '⚔️', text: 'Guild Hall', desc: 'Adventurer combat guildhall' },
      { id: 'build-granary', glyph: '🌾', text: 'Granary Market', desc: 'Crop storage & bread supply' },
      { id: 'build-watchtower', glyph: '🗼', text: 'Watchtower', desc: 'Perimeter defensive boundary post' },
      { id: 'build-pen', glyph: '🛖', text: 'Livestock Pen', desc: 'Domestic animal breeding shelter' },
      { id: 'build-dock', glyph: '⚓', text: 'Fishing Dock', desc: 'Waterfront fishing pier' },
      { id: 'build-campfire', glyph: '🔥', text: 'Campfire', desc: 'Cozy traveler gathering hearth' },
    ]
  },
  miracles: {
    title: 'Miracles, Awaken & Wildlife',
    items: [
      { id: 'miracle-heal', glyph: '💖', text: 'Divine Heal', desc: 'Restore vital health & cleanse ailments' },
      { id: 'miracle-awaken', glyph: '⚡', text: 'Awaken Class', desc: 'Awaken age-16 class & roll rank' },
      { id: 'miracle-rain', glyph: '🌧️', text: 'Rainstorm', desc: 'Summon torrential fertilizing rain' },
      { id: 'manna-drop', glyph: '🍞', text: 'Manna Drop', desc: 'Drop basket of celestial bread' },
      { id: 'spawn-animal-rabbit', glyph: '🐇', text: 'Rabbit', desc: 'Swift herbivore foraging fauna' },
      { id: 'spawn-animal-deer', glyph: '🦌', text: 'Deer', desc: 'Majestic forest herbivore' },
      { id: 'spawn-animal-wolf', glyph: '🐺', text: 'Wolf', desc: 'Pack hunting forest predator' },
      { id: 'spawn-animal-bear', glyph: '🐻', text: 'Bear', desc: 'Apex ironcrag mountain predator' },
      { id: 'spawn-animal-sheep', glyph: '🐑', text: 'Sheep', desc: 'Domestic wool-yielding livestock' },
      { id: 'spawn-animal-cow', glyph: '🐄', text: 'Cow', desc: 'Domestic dairy & leather cattle' },
    ]
  }
};

let currentSubpaletteCategory = null;

function openRetroSubpalette(category) {
  const container = document.getElementById('retro-tool-subpalette');
  const titleEl = document.getElementById('subpalette-title');
  const itemsContainer = document.getElementById('subpalette-items');
  const minimapCard = document.getElementById('bottom-minimap-card');

  if (!container || !RETRO_SUBPALETTES[category]) return;

  // Toggle close if clicking active category again
  if (currentSubpaletteCategory === category && !container.classList.contains('hidden')) {
    closeRetroSubpalette();
    return;
  }

  currentSubpaletteCategory = category;
  const data = RETRO_SUBPALETTES[category];

  if (titleEl) titleEl.textContent = data.title;
  if (itemsContainer) {
    itemsContainer.innerHTML = '';
    data.items.forEach(item => {
      const btn = document.createElement('button');
      btn.className = `subpalette-btn-item ${activeTool === item.id ? 'active' : ''}`;
      btn.id = `subitem-${item.id}`;
      btn.title = item.desc;
      btn.innerHTML = `<span class="item-glyph">${item.glyph}</span><span class="item-text">${item.text}</span>`;
      btn.onclick = (e) => {
        e.stopPropagation();
        selectSubpaletteTool(item.id, item.text, item.desc);
      };
      itemsContainer.appendChild(btn);
    });
  }

  // Update tabs active state
  ['nature', 'mines', 'arch', 'miracles'].forEach(t => {
    const tabBtn = document.getElementById(`btn-retro-tab-${t}`);
    if (tabBtn) tabBtn.classList.remove('active');
  });
  const activeTabBtn = document.getElementById(`btn-retro-tab-${category === 'architecture' ? 'arch' : category}`);
  if (activeTabBtn) activeTabBtn.classList.add('active');

  container.classList.remove('hidden');
  if (minimapCard) minimapCard.classList.add('subpalette-open');
  playSound('click');

  // Auto-select first item if no tool is active in this category
  if (!data.items.some(it => it.id === activeTool) && data.items.length > 0) {
    const first = data.items[0];
    selectSubpaletteTool(first.id, first.text, first.desc);
  }
}

function closeRetroSubpalette() {
  const container = document.getElementById('retro-tool-subpalette');
  const minimapCard = document.getElementById('bottom-minimap-card');
  if (container) container.classList.add('hidden');
  if (minimapCard) minimapCard.classList.remove('subpalette-open');
  currentSubpaletteCategory = null;
  playSound('click');
}

function selectSubpaletteTool(toolId, toolName, toolDesc) {
  activeTool = toolId;
  playSound('click');
  showToast(`${toolName}: ${toolDesc}`);

  document.querySelectorAll('.subpalette-btn-item').forEach(b => b.classList.remove('active'));
  const activeBtn = document.getElementById(`subitem-${toolId}`);
  if (activeBtn) activeBtn.classList.add('active');
}

// ----------------------------------------------------------------------------
// DYNAMIC LIVE NPC INSPECTOR
// ----------------------------------------------------------------------------
function openNpcInspector(agent) {
  const p = document.getElementById('bottom-npc-inspector');
  if (!p) return;
  p.classList.remove('hidden');

  const levelEl = document.getElementById('npc-insp-level');
  const nameAgeEl = document.getElementById('npc-insp-name-age');
  const rankBadgeEl = document.getElementById('npc-insp-rank-badge');
  const statusEl = document.getElementById('npc-insp-status');
  const strEl = document.getElementById('npc-insp-str');
  const vitEl = document.getElementById('npc-insp-vit');
  const dexEl = document.getElementById('npc-insp-dex');
  const intEl = document.getElementById('npc-insp-int');
  const goalEl = document.getElementById('npc-insp-goal');

  const age = agent.age || (16 + (agent.id * 3) % 18);
  const lvl = agent.level || (10 + (agent.id % 25));
  const rawClass = agent.class || (agent.archetype && !agent.archetype.includes('None') ? agent.archetype : 'Novice');
  const cls = (typeof rawClass === 'string') ? rawClass : 'Novice';
  const rawRank = agent.rank || ((lvl >= 20) ? 'A' : (lvl >= 16) ? 'C' : 'Novice');
  const rank = (typeof rawRank === 'string' && rawRank !== 'None') ? rawRank : 'Novice';

  if (levelEl) levelEl.textContent = `Level ${lvl} ${cls}${agent.subclass ? ' (' + agent.subclass + ')' : ''}`;
  if (nameAgeEl) nameAgeEl.textContent = `${agent.name} (${age}yo)`;

  if (rankBadgeEl) {
    rankBadgeEl.textContent = rank === 'Novice' ? 'Novice' : `${rank}-Rank`;
    rankBadgeEl.className = `npc-rank-badge rank-${rank.toLowerCase()}`;
  }

  let statusText = 'Class Awakening Scheduled';
  if (cls !== 'Novice') {
    statusText = `Awakened ${cls} [${rank}-Rank]`;
  } else if (age >= 16) {
    statusText = 'Pilgrimage to Temple (Awakening Ready)';
  } else if (agent.health < 40) {
    statusText = 'Wounded / Seeking Sanctuary';
  } else if (agent.energy < 25) {
    statusText = 'Fatigued / Meditating';
  }
  if (statusEl) statusEl.textContent = statusText;

  const str = Math.round(agent.strength || agent.atk || (12 + (agent.id * 7) % 12));
  const vit = Math.round(agent.vitality || agent.def || (10 + (agent.id * 5) % 10));
  const dex = Math.round(agent.agility || agent.spd || (11 + (agent.id * 3) % 11));
  const intel = Math.round(agent.intelligence || agent.intellect || (8 + (agent.id * 9) % 16));

  if (strEl) strEl.textContent = str;
  if (vitEl) vitEl.textContent = vit;
  if (dexEl) dexEl.textContent = dex;
  if (intEl) intEl.textContent = intel;

  if (goalEl) {
    goalEl.textContent = agent.current_goal || agent.current_thought || agent.last_action || 'Travel to GODs Temple for Awakening.';
  }

  // Dynamic Avatar Palette: Colors adjust based on class and rank
  const armorEl = document.getElementById('npc-svg-armor');
  const hairEl = document.getElementById('npc-svg-hair');
  if (armorEl && hairEl) {
    if (cls === 'Warrior' || cls === 'Paladin') {
      armorEl.setAttribute('fill', rank === 'S' || rank === 'SSS' ? '#f59e0b' : '#2563eb');
      hairEl.setAttribute('fill', '#eab308');
    } else if (cls === 'Mage') {
      armorEl.setAttribute('fill', '#9333ea');
      hairEl.setAttribute('fill', '#c084fc');
    } else if (cls === 'Ranger') {
      armorEl.setAttribute('fill', '#16a34a');
      hairEl.setAttribute('fill', '#b45309');
    } else if (cls === 'Rogue') {
      armorEl.setAttribute('fill', '#334155');
      hairEl.setAttribute('fill', '#475569');
    } else {
      armorEl.setAttribute('fill', '#60a5fa');
      hairEl.setAttribute('fill', '#eab308');
    }
  }
}

function awakenInspectedNpc() {
  if (!currentTelemetry || !selectedAgentId) {
    showToast('⚠️ No character currently selected to awaken');
    return;
  }
  const agent = currentTelemetry.agents?.find(a => a.id === selectedAgentId);
  const targetX = agent ? agent.x : 68;
  const targetY = agent ? agent.y : 40;

  playSound('chime');
  sendMsg({
    type: 'TerraformArea',
    brush: 'DivineAwaken',
    center_x: targetX,
    center_y: targetY,
    radius: 1,
  });
  showToast(`⚡ Divine Awakening decree cast on ${agent ? agent.name : 'Mortal'}!`);
}

// ----------------------------------------------------------------------------
// WEATHER & CLIMATE CODEX POPOVER
// ----------------------------------------------------------------------------
function toggleWeatherCodex() {
  const modal = document.getElementById('weather-codex-modal');
  if (!modal) return;
  modal.classList.toggle('hidden');
  playSound('click');

  if (!modal.classList.contains('hidden') && currentTelemetry) {
    const weatherStr = currentTelemetry.weather ? currentTelemetry.weather.toString() : 'Crisp / Windy';
    const nameEl = document.getElementById('codex-weather-name');
    const cropEl = document.getElementById('codex-crop-mult');
    const spdEl = document.getElementById('codex-speed-mult');
    const manaEl = document.getElementById('codex-mana-mult');

    if (nameEl) nameEl.textContent = weatherStr;
    if (cropEl) cropEl.textContent = weatherStr.includes('Rain') ? '+45% Rain Yield' : '+25% Growth';
    if (spdEl) spdEl.textContent = weatherStr.includes('Blizzard') ? '-30% Frozen' : '+10% Swiftness';
    if (manaEl) manaEl.textContent = '1.4x Leyline Flux';
  }
}

function triggerGodWeather(type) {
  playSound('chime');
  if (type === 'rain') {
    sendMsg({ type: 'TerraformArea', brush: 'CarveRiver', center_x: 64, center_y: 64, radius: 2 });
    showToast('🌧️ Divine Rainstorm conjured across the realm!');
  } else if (type === 'clear') {
    showToast('☀️ Clear Sun restored by divine decree');
  } else if (type === 'heatwave') {
    sendMsg({ type: 'TerraformArea', brush: 'ThawHeat', center_x: 64, center_y: 64, radius: 3 });
    showToast('🔥 Scorching Heatwave descended over lands');
  } else if (type === 'blizzard') {
    sendMsg({ type: 'TerraformArea', brush: 'FreezeIce', center_x: 64, center_y: 64, radius: 3 });
    showToast('❄️ Permafrost Blizzard summoned across territory');
  }
  const weatherEl = document.getElementById('retro-weather');
  if (weatherEl) weatherEl.textContent = type;
  toggleWeatherCodex();
}

// ----------------------------------------------------------------------------
// WORLD MAP SOVEREIGN REALMS & THREAT NAVIGATION
// ----------------------------------------------------------------------------
function locateKingdomOnMap(kingdom, x, y, name) {
  setViewMode(false);
  centerCameraOn(x, y);
  playSound('chime');
  showToast(`🗺️ Navigated to realm: ${name}`);
}

function locateThreatOnMap(rank, x, y, name) {
  setViewMode(false);
  centerCameraOn(x, y);
  playSound('thunder');
  showToast(`⚠️ Teleported to threat zone: ${name}`);
}

function toggleMiniMapCard() {
  const mc = document.getElementById('bottom-minimap-card');
  if (mc) {
    mc.classList.toggle('hidden');
    playSound('click');
  }
}

// ----------------------------------------------------------------------------
// RETRO STATUS HUD & DYNAMIC EVENT FEED
// ----------------------------------------------------------------------------
let lastLoggedEventCount = 0;

function updateRetroStatusHud(data) {
  const tick = data.world_tick || 0;
  const year = 104 + Math.floor(tick / 1200);
  const months = ['Autumn', 'Winter', 'Spring', 'Summer'];
  const season = months[Math.floor((tick % 1200) / 300)] || 'Autumn';
  const day = 1 + (Math.floor(tick / 10) % 28);
  const hours = 8 + (Math.floor(tick / 4) % 12);
  const mins = (tick * 5) % 60;
  const timeStr = `${hours.toString().padStart(2, '0')}:${mins.toString().padStart(2, '0')} AM`;

  const dateEl = document.getElementById('retro-date');
  if (dateEl) dateEl.textContent = `Year ${year}, Season: ${season}, Day ${day}`;

  const timeEl = document.getElementById('retro-time');
  if (timeEl) timeEl.textContent = `Time: ${timeStr}`;

  const weatherEl = document.getElementById('retro-weather');
  if (weatherEl) weatherEl.textContent = data.weather ? `${data.weather.toLowerCase()}` : 'crisp/windy';

  const woodEl = document.getElementById('stock-wood-val');
  const stoneEl = document.getElementById('stock-stone-val');
  const wheatEl = document.getElementById('stock-wheat-val');
  const manaEl = document.getElementById('stock-mana-val');

  const demo = data.demographics;
  if (woodEl) woodEl.textContent = 1420 + Math.floor((demo?.total_circulating_wealth || 0) * 2.5);
  if (stoneEl) stoneEl.textContent = 890 + Math.floor((demo?.total_population || 16) * 15);
  if (wheatEl) wheatEl.textContent = 410 + Math.floor((demo?.avg_health || 100) * 3);
  if (manaEl) manaEl.textContent = `Mana Stones: ${23 + Math.floor((demo?.max_generation || 1) * 4)}`;

  const npcsEl = document.getElementById('debug-npcs-val');
  if (npcsEl) npcsEl.textContent = data.agents ? data.agents.length : 128;

  // Live Dynamic Event Feed updates
  if (data.event_log && data.event_log.length > 0) {
    const feedContainer = document.getElementById('feed-entries-container');
    if (feedContainer && data.event_log.length !== lastLoggedEventCount) {
      lastLoggedEventCount = data.event_log.length;
      const recent = data.event_log.slice(-12);
      feedContainer.innerHTML = '';
      recent.forEach((msg, idx) => {
        const row = document.createElement('div');
        row.className = 'feed-entry';
        const hr = 8 + (Math.floor((tick - (recent.length - idx) * 3) / 4) % 12);
        const mn = (idx * 7) % 60;
        const timeBadge = `[${hr.toString().padStart(2, '0')}:${mn.toString().padStart(2, '0')}]`;

        row.innerHTML = `<span class="feed-time">${timeBadge}</span> ${msg}`;
        row.onclick = () => {
          playSound('click');
          if (msg.includes('Temple') || msg.includes('Awakening')) centerCameraOn(68, 40);
          else if (msg.includes('Dungeon') || msg.includes('Leyline')) centerCameraOn(42, 40);
          else if (msg.includes('Guild') || msg.includes('Party')) centerCameraOn(82, 44);
          else if (msg.includes('Caravan') || msg.includes('Eldoria')) centerCameraOn(70, 44);
          else centerCameraOn(64, 64);
          showToast(`📍 Panned to event location`);
        };
        feedContainer.appendChild(row);
      });
      feedContainer.scrollTop = feedContainer.scrollHeight;
    }
  }
}

// ----------------------------------------------------------------------------
// INTERACTIVE RETRO MINIMAP RADAR
// ----------------------------------------------------------------------------
let isRetroMinimapDragging = false;

function initRetroMinimap() {
  const rCanvas = document.getElementById('retro-minimap-canvas');
  if (!rCanvas) return;

  function handleRetroMinimapEvent(e) {
    const rect = rCanvas.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;
    const wx = Math.max(0, Math.min(WORLD_SIZE - 1, Math.round((mx / rect.width) * WORLD_SIZE)));
    const wy = Math.max(0, Math.min(WORLD_SIZE - 1, Math.round((my / rect.height) * WORLD_SIZE)));
    centerCameraOn(wx, wy);
  }

  rCanvas.addEventListener('mousedown', (e) => {
    isRetroMinimapDragging = true;
    handleRetroMinimapEvent(e);
    playSound('click');
  });

  window.addEventListener('mousemove', (e) => {
    if (isRetroMinimapDragging) {
      handleRetroMinimapEvent(e);
    }
  });

  window.addEventListener('mouseup', () => {
    isRetroMinimapDragging = false;
  });
}

function renderRetroMinimap(data) {
  const rCanvas = document.getElementById('retro-minimap-canvas');
  if (!rCanvas) return;
  const rCtx = rCanvas.getContext('2d');
  const w = rCanvas.width;
  const h = rCanvas.height;

  rCtx.clearRect(0, 0, w, h);

  // 1. Dark oceanic slate base
  rCtx.fillStyle = '#0f172a';
  rCtx.fillRect(0, 0, w, h);

  // 2. Offscreen terrain sampling
  const scaleX = w / WORLD_SIZE;
  const scaleY = h / WORLD_SIZE;

  if (isTerrainBufferBuilt) {
    rCtx.drawImage(terrainCanvas, 0, 0, w, h);
  } else {
    // Fallback continent representation
    rCtx.fillStyle = '#1e3a8a';
    rCtx.fillRect(0, 0, w, h);
    rCtx.fillStyle = '#22c55e';
    rCtx.beginPath();
    rCtx.arc(w * 0.5, h * 0.45, w * 0.38, 0, Math.PI * 2);
    rCtx.fill();
  }

  // 3. Highlight Canonical Landmark POIs
  // Solar Altar Temple (68, 40)
  rCtx.fillStyle = '#fbbf24';
  rCtx.fillRect(68 * scaleX - 2, 40 * scaleY - 2, 5, 5);

  // Cavern Dungeon Rift (42, 40)
  rCtx.fillStyle = '#c026d3';
  rCtx.beginPath();
  rCtx.arc(42 * scaleX, 40 * scaleY, 3, 0, Math.PI * 2);
  rCtx.fill();

  // Guild Hall Barracks (82, 44)
  rCtx.fillStyle = '#38bdf8';
  rCtx.fillRect(82 * scaleX - 2, 44 * scaleY - 2, 4, 4);

  // Farmland Granary (60, 48)
  rCtx.fillStyle = '#fef08a';
  rCtx.fillRect(60 * scaleX - 1.5, 48 * scaleY - 1.5, 3.5, 3.5);

  // 4. Live Agent Dots
  if (data.agents && data.agents.length > 0) {
    for (const a of data.agents) {
      const ax = a.x * scaleX;
      const ay = a.y * scaleY;
      if (a.id === selectedAgentId) {
        rCtx.fillStyle = '#ffffff';
        rCtx.beginPath();
        rCtx.arc(ax, ay, 3, 0, Math.PI * 2);
        rCtx.fill();
      } else {
        rCtx.fillStyle = (a.race === 'SolarTerran') ? '#f59e0b' : '#38bdf8';
        rCtx.fillRect(ax - 1, ay - 1, 2, 2);
      }
    }
  }

  // 5. Camera Viewport Bounding Box
  const viewTilesW = (canvas.width / (TILE_PX * camera.zoom));
  const viewTilesH = (canvas.height / (TILE_PX * camera.zoom));
  const camWorldX = (-camera.x / (TILE_PX * camera.zoom));
  const camWorldY = (-camera.y / (TILE_PX * camera.zoom));

  const boxX = camWorldX * scaleX;
  const boxY = camWorldY * scaleY;
  const boxW = viewTilesW * scaleX;
  const boxH = viewTilesH * scaleY;

  rCtx.strokeStyle = 'rgba(255, 255, 255, 0.85)';
  rCtx.lineWidth = 1.5;
  rCtx.strokeRect(boxX, boxY, boxW, boxH);

  // 6. Subtle inner border shade
  rCtx.strokeStyle = 'rgba(0, 0, 0, 0.5)';
  rCtx.lineWidth = 2;
  rCtx.strokeRect(0, 0, w, h);
}

function drawCosmicWorldMap(data, time) {
  const w = canvas.width;
  const h = canvas.height;

  // 1. Deep Space Void with Starfield
  ctx.fillStyle = '#060814';
  ctx.fillRect(0, 0, w, h);

  for (let i = 0; i < 180; i++) {
    const sx = ((i * 137.5) % w);
    const sy = ((i * 223.7) % h);
    const twinkle = Math.sin(time * 0.003 + i) * 0.35 + 0.65;
    ctx.fillStyle = `rgba(255, 255, 255, ${twinkle * 0.8})`;
    const sz = (i % 5 === 0) ? 2 : 1;
    ctx.fillRect(sx, sy, sz, sz);
  }

  // 2. Crescent Moon with Craters (Top Right)
  const moonX = w * 0.72;
  const moonY = h * 0.16;
  const moonR = 24;

  ctx.save();
  ctx.fillStyle = '#1e2433';
  ctx.beginPath();
  ctx.arc(moonX, moonY, moonR, 0, Math.PI * 2);
  ctx.fill();

  ctx.fillStyle = '#e2e8f0';
  ctx.beginPath();
  ctx.arc(moonX, moonY, moonR, -Math.PI * 0.4, Math.PI * 0.6);
  ctx.bezierCurveTo(moonX - moonR * 0.2, moonY + moonR * 0.8, moonX - moonR * 0.2, moonY - moonR * 0.8, moonX, moonY - moonR);
  ctx.fill();

  ctx.fillStyle = 'rgba(100, 116, 139, 0.45)';
  ctx.beginPath();
  ctx.arc(moonX + 4, moonY - 6, 4, 0, Math.PI * 2);
  ctx.arc(moonX + 8, moonY + 8, 3, 0, Math.PI * 2);
  ctx.arc(moonX + 2, moonY + 4, 2.5, 0, Math.PI * 2);
  ctx.fill();
  ctx.restore();

  // 3. Central 3D Spherical Aether Planet Globe
  const cx = w / 2;
  const cy = h / 2 + 10;
  const R = Math.min(w * 0.26, h * 0.34, 240);

  ctx.save();

  const atmosGrad = ctx.createRadialGradient(cx, cy, R * 0.95, cx, cy, R * 1.14);
  atmosGrad.addColorStop(0, 'rgba(56, 189, 248, 0.4)');
  atmosGrad.addColorStop(0.5, 'rgba(56, 189, 248, 0.15)');
  atmosGrad.addColorStop(1, 'rgba(56, 189, 248, 0)');
  ctx.fillStyle = atmosGrad;
  ctx.beginPath();
  ctx.arc(cx, cy, R * 1.15, 0, Math.PI * 2);
  ctx.fill();

  ctx.beginPath();
  ctx.arc(cx, cy, R, 0, Math.PI * 2);
  ctx.clip();

  const oceanGrad = ctx.createRadialGradient(cx - R * 0.35, cy - R * 0.35, R * 0.1, cx, cy, R);
  oceanGrad.addColorStop(0, '#0284c7');
  oceanGrad.addColorStop(0.7, '#0369a1');
  oceanGrad.addColorStop(1, '#082f49');
  ctx.fillStyle = oceanGrad;
  ctx.fillRect(cx - R, cy - R, R * 2, R * 2);

  const rot = (time * 0.00012) % (Math.PI * 2);

  ctx.fillStyle = '#f8fafc';
  ctx.beginPath();
  ctx.ellipse(cx, cy - R * 0.82, R * 0.45, R * 0.22, 0, 0, Math.PI * 2);
  ctx.fill();

  ctx.beginPath();
  ctx.ellipse(cx, cy + R * 0.85, R * 0.4, R * 0.18, 0, 0, Math.PI * 2);
  ctx.fill();

  for (let c = -1; c <= 1; c++) {
    const shiftX = (rot + c * Math.PI) % (Math.PI * 2) - Math.PI;
    if (Math.abs(shiftX) > Math.PI / 1.7) continue;

    const contX = cx + Math.sin(shiftX) * R * 0.85;

    ctx.fillStyle = '#22c55e';
    ctx.beginPath();
    ctx.arc(contX, cy - R * 0.2, R * 0.32, 0, Math.PI * 2);
    ctx.fill();

    ctx.fillStyle = '#64748b';
    ctx.beginPath();
    ctx.arc(contX + 15, cy - R * 0.28, R * 0.14, 0, Math.PI * 2);
    ctx.fill();

    ctx.fillStyle = '#eab308';
    ctx.beginPath();
    ctx.arc(contX - 10, cy + R * 0.1, R * 0.38, 0, Math.PI * 2);
    ctx.fill();

    ctx.fillStyle = '#15803d';
    ctx.beginPath();
    ctx.arc(contX + 8, cy + R * 0.48, R * 0.24, 0, Math.PI * 2);
    ctx.fill();

    ctx.fillStyle = 'rgba(239, 68, 68, 0.45)';
    ctx.beginPath();
    ctx.arc(contX + R * 0.32, cy - R * 0.1, R * 0.16, 0, Math.PI * 2);
    ctx.fill();

    ctx.font = '14px sans-serif';
    ctx.fillText('🏛️', contX - 12, cy - R * 0.22);
    ctx.fillText('🏰', contX + 18, cy + R * 0.12);
    ctx.fillText('⛩️', contX - 25, cy + R * 0.28);
    ctx.fillText('🚪', contX + 28, cy - R * 0.28);
  }

  // Glowing Violet Leylines
  ctx.strokeStyle = '#d946ef';
  ctx.shadowColor = '#c026d3';
  ctx.shadowBlur = 12;
  ctx.lineWidth = 2.5;
  ctx.beginPath();
  ctx.moveTo(cx - R * 0.6, cy - R * 0.3);
  ctx.bezierCurveTo(cx - R * 0.2, cy - R * 0.1, cx + R * 0.2, cy - R * 0.5, cx + R * 0.5, cy - R * 0.2);
  ctx.bezierCurveTo(cx + R * 0.7, cy + R * 0.2, cx + R * 0.3, cy + R * 0.6, cx, cy + R * 0.7);
  ctx.bezierCurveTo(cx - R * 0.3, cy + R * 0.5, cx - R * 0.7, cy + R * 0.2, cx - R * 0.6, cy - R * 0.3);
  ctx.stroke();

  ctx.beginPath();
  ctx.moveTo(cx - R * 0.2, cy - R * 0.7);
  ctx.lineTo(cx, cy);
  ctx.lineTo(cx - R * 0.4, cy + R * 0.5);
  ctx.stroke();
  ctx.shadowBlur = 0;

  ctx.strokeStyle = '#fbbf24';
  ctx.lineWidth = 1.8;
  ctx.setLineDash([5, 5]);
  ctx.beginPath();
  ctx.arc(cx, cy, R * 0.65, 0.2, 2.8);
  ctx.stroke();
  ctx.setLineDash([]);

  const shadeGrad = ctx.createRadialGradient(cx - R * 0.4, cy - R * 0.4, R * 0.2, cx, cy, R);
  shadeGrad.addColorStop(0, 'rgba(255, 255, 255, 0.25)');
  shadeGrad.addColorStop(0.5, 'rgba(0, 0, 0, 0)');
  shadeGrad.addColorStop(0.85, 'rgba(0, 0, 0, 0.5)');
  shadeGrad.addColorStop(1, 'rgba(0, 0, 0, 0.85)');
  ctx.fillStyle = shadeGrad;
  ctx.fillRect(cx - R, cy - R, R * 2, R * 2);

  ctx.restore();

  // Swirling Equatorial Cloud Ring
  ctx.save();
  ctx.strokeStyle = 'rgba(255, 255, 255, 0.75)';
  ctx.lineWidth = 10;
  ctx.lineCap = 'round';
  ctx.shadowColor = 'rgba(255, 255, 255, 0.6)';
  ctx.shadowBlur = 8;

  const cloudW = R * 1.25;
  const cloudH = R * 0.42;

  ctx.beginPath();
  ctx.ellipse(cx, cy, cloudW, cloudH, -0.15, 0, Math.PI * 2);
  ctx.stroke();

  for (let cp = 0; cp < 8; cp++) {
    const ang = cp * (Math.PI / 4) + time * 0.0002;
    const cpx = cx + Math.cos(ang) * cloudW * 0.95;
    const cpy = cy + Math.sin(ang) * cloudH * 0.95;
    ctx.fillStyle = 'rgba(255, 255, 255, 0.85)';
    ctx.beginPath();
    ctx.arc(cpx, cpy, 12, 0, Math.PI * 2);
    ctx.arc(cpx + 8, cpy - 4, 9, 0, Math.PI * 2);
    ctx.arc(cpx - 6, cpy + 3, 8, 0, Math.PI * 2);
    ctx.fill();
  }
  ctx.restore();
}

function drawVillageIconicLandmarks(data, time) {
  const tilePx = TILE_PX;

  // 1. Crystal Blue Curving River & Wooden Footbridge (x: 55, y: 48)
  const bridgeX = 55 * tilePx;
  const bridgeY = 48 * tilePx;

  ctx.save();
  ctx.fillStyle = '#8b5a3c';
  ctx.fillRect(bridgeX - 4, bridgeY - 3, tilePx * 3.8, tilePx * 1.5);
  ctx.fillStyle = '#5c3a21';
  for (let p = 0; p < 6; p++) {
    ctx.fillRect(bridgeX - 4 + p * 8, bridgeY - 3, 2, tilePx * 1.5);
  }
  ctx.fillStyle = '#3a2012';
  ctx.fillRect(bridgeX - 4, bridgeY - 4, tilePx * 3.8, 2);
  ctx.fillRect(bridgeX - 4, bridgeY + tilePx * 1.5 - 2, tilePx * 3.8, 2);
  ctx.restore();

  // 2. GODs Temple (x: 68, y: 40)
  const templeX = 68 * tilePx;
  const templeY = 40 * tilePx;
  ctx.save();
  ctx.fillStyle = '#94a3b8';
  ctx.fillRect(templeX - 24, templeY - 6, 72, 36);
  ctx.fillStyle = '#cbd5e1';
  ctx.fillRect(templeX - 20, templeY - 14, 64, 12);

  ctx.fillStyle = '#e2e8f0';
  for (let col = 0; col < 4; col++) {
    ctx.fillRect(templeX - 16 + col * 18, templeY - 38, 8, 26);
  }

  ctx.fillStyle = '#fbbf24';
  ctx.beginPath();
  ctx.moveTo(templeX - 24, templeY - 38);
  ctx.lineTo(templeX + 12, templeY - 56);
  ctx.lineTo(templeX + 48, templeY - 38);
  ctx.closePath();
  ctx.fill();
  ctx.strokeStyle = '#b45309';
  ctx.lineWidth = 2;
  ctx.stroke();

  for (let b = 0; b < 4; b++) {
    const bx = templeX - 22 + b * 22;
    const by = templeY + 22;
    ctx.fillStyle = '#475569';
    ctx.fillRect(bx - 3, by - 6, 6, 8);
    const fPulse = Math.sin(time * 0.012 + b) * 3;
    ctx.fillStyle = '#c084fc';
    ctx.shadowColor = '#a855f7';
    ctx.shadowBlur = 10;
    ctx.beginPath();
    ctx.arc(bx, by - 10 + fPulse * 0.5, 4.5 + fPulse * 0.3, 0, Math.PI * 2);
    ctx.fill();
    ctx.shadowBlur = 0;
  }

  ctx.font = 'bold 11px Outfit, sans-serif';
  ctx.fillStyle = '#ffffff';
  ctx.textAlign = 'center';
  ctx.shadowColor = '#000000';
  ctx.shadowBlur = 4;
  ctx.fillText('GODs Temple', templeX + 12, templeY - 62);
  ctx.shadowBlur = 0;
  ctx.restore();

  // 3. Guild Hall (x: 82, y: 44)
  const guildX = 82 * tilePx;
  const guildY = 44 * tilePx;
  ctx.save();
  ctx.fillStyle = '#78350f';
  ctx.fillRect(guildX - 18, guildY - 10, 54, 38);
  ctx.fillStyle = '#b91c1c';
  ctx.beginPath();
  ctx.moveTo(guildX - 22, guildY - 10);
  ctx.lineTo(guildX + 9, guildY - 32);
  ctx.lineTo(guildX + 40, guildY - 10);
  ctx.closePath();
  ctx.fill();

  ctx.fillStyle = '#fbbf24';
  ctx.fillRect(guildX + 4, guildY - 4, 10, 14);
  ctx.fillStyle = '#dc2626';
  ctx.fillRect(guildX + 6, guildY - 2, 6, 10);

  ctx.font = 'bold 11px Outfit, sans-serif';
  ctx.fillStyle = '#ffffff';
  ctx.textAlign = 'center';
  ctx.shadowColor = '#000000';
  ctx.shadowBlur = 4;
  ctx.fillText('Guild Hall', guildX + 9, guildY - 36);
  ctx.shadowBlur = 0;
  ctx.restore();

  // 4. Market Quarter (x: 78, y: 56)
  const marketX = 78 * tilePx;
  const marketY = 56 * tilePx;
  ctx.save();
  ctx.fillStyle = '#8b5a3c';
  ctx.fillRect(marketX - 16, marketY - 4, 28, 16);
  for (let s = 0; s < 4; s++) {
    ctx.fillStyle = (s % 2 === 0) ? '#ea580c' : '#ffffff';
    ctx.fillRect(marketX - 16 + s * 7, marketY - 14, 7, 10);
  }
  ctx.fillStyle = '#8b5a3c';
  ctx.fillRect(marketX + 18, marketY, 26, 16);
  for (let s = 0; s < 4; s++) {
    ctx.fillStyle = (s % 2 === 0) ? '#0284c7' : '#ffffff';
    ctx.fillRect(marketX + 18 + s * 6.5, marketY - 10, 6.5, 10);
  }
  ctx.strokeStyle = '#451a03';
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.arc(marketX + 22, marketY + 16, 5, 0, Math.PI * 2);
  ctx.arc(marketX + 40, marketY + 16, 5, 0, Math.PI * 2);
  ctx.stroke();

  ctx.font = 'bold 11px Outfit, sans-serif';
  ctx.fillStyle = '#ffffff';
  ctx.textAlign = 'center';
  ctx.shadowColor = '#000000';
  ctx.shadowBlur = 4;
  ctx.fillText('Market', marketX + 14, marketY - 20);
  ctx.shadowBlur = 0;
  ctx.restore();

  // 5. Livestock Pen with Cows & Sheep (x: 64, y: 54)
  const penX = 64 * tilePx;
  const penY = 54 * tilePx;
  ctx.save();
  ctx.strokeStyle = '#78350f';
  ctx.lineWidth = 2.5;
  ctx.strokeRect(penX - 24, penY - 18, 56, 44);
  ctx.fillStyle = '#451a03';
  ctx.fillRect(penX - 25, penY - 19, 4, 4);
  ctx.fillRect(penX + 31, penY - 19, 4, 4);
  ctx.fillRect(penX - 25, penY + 25, 4, 4);
  ctx.fillRect(penX + 31, penY + 25, 4, 4);

  ctx.fillStyle = '#ffffff';
  ctx.fillRect(penX - 10, penY - 6, 14, 9);
  ctx.fillStyle = '#1e293b';
  ctx.fillRect(penX - 6, penY - 4, 5, 5);
  ctx.fillRect(penX + 1, penY - 2, 3, 3);

  ctx.fillStyle = '#f8fafc';
  ctx.beginPath();
  ctx.arc(penX + 14, penY + 8, 6, 0, Math.PI * 2);
  ctx.arc(penX + 8, penY + 12, 5, 0, Math.PI * 2);
  ctx.fill();
  ctx.fillStyle = '#334155';
  ctx.fillRect(penX + 18, penY + 6, 3, 3);
  ctx.restore();

  // 6. Farmland Wheat Plots (x: 72, y: 64)
  const farmX = 72 * tilePx;
  const farmY = 64 * tilePx;
  ctx.save();
  ctx.fillStyle = '#713f12';
  ctx.fillRect(farmX - 24, farmY - 12, 64, 34);
  ctx.fillStyle = '#eab308';
  for (let r = 0; r < 4; r++) {
    for (let w = 0; w < 7; w++) {
      ctx.fillRect(farmX - 20 + w * 8, farmY - 8 + r * 8, 4, 5);
    }
  }
  ctx.restore();

  // 7. Mountain Cavern Dungeon Entrance (A-Rank) (x: 42, y: 40)
  const dungX = 42 * tilePx;
  const dungY = 40 * tilePx;
  ctx.save();
  ctx.fillStyle = '#334155';
  ctx.beginPath();
  ctx.arc(dungX, dungY, 26, Math.PI, 0);
  ctx.fill();
  ctx.fillStyle = '#0f172a';
  ctx.beginPath();
  ctx.arc(dungX, dungY + 2, 18, Math.PI, 0);
  ctx.fill();

  const runePulse = Math.sin(time * 0.008) * 3;
  ctx.strokeStyle = '#ef4444';
  ctx.shadowColor = '#dc2626';
  ctx.shadowBlur = 12;
  ctx.lineWidth = 2.5;
  ctx.beginPath();
  ctx.arc(dungX, dungY - 24, 12 + runePulse * 0.4, 0, Math.PI * 2);
  ctx.stroke();

  ctx.font = 'bold 12px Outfit, sans-serif';
  ctx.fillStyle = '#f87171';
  ctx.textAlign = 'center';
  ctx.fillText('A', dungX, dungY - 20);

  ctx.font = 'bold 10px Outfit, sans-serif';
  ctx.fillStyle = '#fca5a5';
  ctx.fillText('A-Rank', dungX, dungY - 38);
  ctx.shadowBlur = 0;

  // Adventurer Party outside dungeon (Warrior, Swordswoman, Wizard)
  ctx.fillStyle = '#94a3b8';
  ctx.fillRect(dungX - 16, dungY + 6, 6, 9);
  ctx.fillStyle = '#3b82f6';
  ctx.fillRect(dungX - 20, dungY + 7, 3, 7);

  ctx.fillStyle = '#fed7aa';
  ctx.fillRect(dungX - 4, dungY + 6, 6, 9);
  ctx.fillStyle = '#cbd5e1';
  ctx.fillRect(dungX + 2, dungY + 4, 2, 10);

  ctx.fillStyle = '#1d4ed8';
  ctx.fillRect(dungX + 10, dungY + 4, 6, 11);
  ctx.fillStyle = '#78350f';
  ctx.fillRect(dungX + 17, dungY + 1, 2, 14);
  ctx.fillStyle = '#38bdf8';
  ctx.shadowColor = '#38bdf8';
  ctx.shadowBlur = 8;
  ctx.beginPath();
  ctx.arc(dungX + 18, dungY, 3, 0, Math.PI * 2);
  ctx.fill();
  ctx.shadowBlur = 0;
  ctx.restore();

  // 8. Feral Orc Camp (x: 44, y: 30)
  const orcX = 44 * tilePx;
  const orcY = 30 * tilePx;
  ctx.save();
  ctx.font = 'bold 10px Outfit, sans-serif';
  ctx.fillStyle = '#ffffff';
  ctx.textAlign = 'center';
  ctx.shadowColor = '#000000';
  ctx.shadowBlur = 4;
  ctx.fillText('Feral Orc', orcX + 8, orcY - 12);
  ctx.shadowBlur = 0;

  for (let o = 0; o < 2; o++) {
    const ox = orcX + o * 14;
    const oy = orcY;
    ctx.fillStyle = '#15803d';
    ctx.beginPath();
    ctx.arc(ox, oy - 2, 4, 0, Math.PI * 2);
    ctx.fill();
    ctx.fillStyle = '#78350f';
    ctx.fillRect(ox - 3, oy + 2, 6, 6);
    ctx.fillStyle = '#475569';
    ctx.fillRect(ox + 4, oy - 1, 4, 3);
  }
  ctx.fillStyle = '#78350f';
  ctx.fillRect(orcX - 12, orcY + 4, 10, 4);
  ctx.fillRect(orcX - 10, orcY + 1, 10, 4);
  ctx.restore();

  // 9. Quarry Cave with Timber Entrance (x: 76, y: 32)
  const quarryX = 76 * tilePx;
  const quarryY = 32 * tilePx;
  ctx.save();
  ctx.fillStyle = '#475569';
  ctx.beginPath();
  ctx.arc(quarryX, quarryY, 22, Math.PI, 0);
  ctx.fill();
  ctx.fillStyle = '#1e293b';
  ctx.beginPath();
  ctx.arc(quarryX, quarryY + 2, 14, Math.PI, 0);
  ctx.fill();
  ctx.fillStyle = '#78350f';
  ctx.fillRect(quarryX - 15, quarryY - 14, 4, 16);
  ctx.fillRect(quarryX + 11, quarryY - 14, 4, 16);
  ctx.fillRect(quarryX - 16, quarryY - 16, 32, 4);

  ctx.fillStyle = '#ef4444';
  ctx.beginPath();
  ctx.arc(quarryX + 18, quarryY - 16, 7, 0, Math.PI * 2);
  ctx.fill();
  ctx.font = 'bold 9px sans-serif';
  ctx.fillStyle = '#ffffff';
  ctx.textAlign = 'center';
  ctx.fillText('!', quarryX + 18, quarryY - 13);
  ctx.restore();
}


