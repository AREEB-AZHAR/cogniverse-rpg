// ============================================================================
// Genesis Cogniverse - Authentic WorldBox God Simulator Client
// ============================================================================

let ws = null;
let currentTelemetry = null;
let selectedAgentId = 1;
let favoriteAgentId = null;
let isFollowActive = false;
let showKingdomBorders = true;
const terraformedTiles = new Map(); // key: "x,y" => BiomeType string

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
    bCtx.fillStyle = '#08203e';
    bCtx.fillRect(px, py, tilePx, tilePx);
    if ((x + y) % 6 === 0) {
      bCtx.fillStyle = '#0f3869';
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
  } else if (biome === 'SavannaGrasslands') {
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
  } else if (biome === 'VerdantJungle') {
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
  } else if (biome === 'CorruptedLands') {
    // WorldBox Corrupted alien purple biomass
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
const audioCtx = (window.AudioContext || window.webkitAudioContext) ? new (window.AudioContext || window.webkitAudioContext)() : null;
function playSound(type) {
  if (!audioCtx) return;
  try {
    if (audioCtx.state === 'suspended') audioCtx.resume();
    const osc = audioCtx.createOscillator();
    const gain = audioCtx.createGain();
    osc.connect(gain);
    gain.connect(audioCtx.destination);
    const now = audioCtx.currentTime;

    if (type === 'click') {
      osc.frequency.setValueAtTime(540, now);
      osc.frequency.exponentialRampToValueAtTime(320, now + 0.04);
      gain.gain.setValueAtTime(0.08, now);
      gain.gain.exponentialRampToValueAtTime(0.001, now + 0.04);
      osc.start(now);
      osc.stop(now + 0.04);
    } else if (type === 'thunder') {
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
    if (selectedAgentId) {
      sendMsg({ type: 'SelectAgent', agent_id: selectedAgentId });
    }
  };

  ws.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data);
      currentTelemetry = data;
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
  }
}

// ----------------------------------------------------------------------------
// CHRONOS TIME DILATION
// ----------------------------------------------------------------------------
function setSpeed(spd) {
  playSound('click');
  sendMsg({ type: 'SetSpeedMultiplier', speed: spd });
  document.querySelectorAll('.btn-wb-time').forEach(b => b.classList.remove('active'));
  if (spd === 0) document.getElementById('btn-pause').classList.add('active');
  else if (spd === 1) document.getElementById('btn-1x').classList.add('active');
  else if (spd === 2) document.getElementById('btn-2x').classList.add('active');
  else if (spd === 5) document.getElementById('btn-5x').classList.add('active');
  else if (spd >= 1000) document.getElementById('btn-turbo').classList.add('active');
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
  if (activeTool.startsWith('build-')) {
    let brushType = 'BuildCabin';
    if (activeTool === 'build-cottage') brushType = 'BuildCottage';
    else if (activeTool === 'build-townhall') brushType = 'BuildTownHall';
    else if (activeTool === 'build-barracks') brushType = 'BuildBarracks';
    else if (activeTool === 'build-campfire') brushType = 'BuildCampfire';
    else if (activeTool === 'build-granary') brushType = 'BuildGranary';
    else if (activeTool === 'build-watchtower') brushType = 'BuildWatchtower';
    else if (activeTool === 'build-mineshaft') brushType = 'BuildMineShaft';

    playSound('click');
    sendMsg({
      type: 'TerraformArea',
      brush: brushType,
      center_x: tx,
      center_y: ty,
      radius: 1,
    });
    showToast(`🏗️ Erected ${brushType} at (${tx}, ${ty})`);
    return;
  }

  // 1b. Terraforming & Ore Brushes
  if (activeTool.startsWith('brush-')) {
    let brushType = 'FertilizePlains';
    let biomeKey = 'SunlitPlains';

    if (activeTool === 'brush-forest') {
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
      openCharacterSheet();
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

    // 4. Render Character Sheet Details if open
    if (data.selected_agent) {
      updateCharacterSheet(data.selected_agent);
    }

    // 6. Covenants & Altars
    renderCovenants(data.active_covenants);
    renderAltars(data.altars);
    renderChronicle(data.event_log);

    // 7. Knowledge & Activity Codex
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
  } catch (err) {
    console.error('Telemetry render error:', err);
  }
}

// ----------------------------------------------------------------------------
// LIVING 2D WORLD MAP RENDERER (WORLDBOX VISUALS)
// ----------------------------------------------------------------------------
function drawWorldMap(data) {
  if (!isTerrainBufferBuilt) {
    buildTerrainBuffer();
  }

  ctx.save();
  ctx.fillStyle = '#061c33';
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  // Apply Camera Translation & Zoom
  ctx.translate(camera.x, camera.y);
  ctx.scale(camera.zoom, camera.zoom);

  const tilePx = TILE_PX;
  const time = Date.now();

  // 0. Boundless WorldBox Deep Abyss Ocean Backdrop
  const oceanMargin = 40 * tilePx;
  ctx.fillStyle = '#061c33'; // Deep abyssal ocean
  ctx.fillRect(-oceanMargin, -oceanMargin, WORLD_SIZE * tilePx + oceanMargin * 2, WORLD_SIZE * tilePx + oceanMargin * 2);

  // Animated ocean wave lines outside continent
  ctx.fillStyle = 'rgba(16, 82, 142, 0.28)';
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
        ctx.fillStyle = 'rgba(15, 23, 42, 0.85)';
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
  if (!data.is_daytime) {
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
  if (agent.inventory_items && agent.inventory_items.length > 0) {
    for (const [name, qty] of agent.inventory_items) {
      const tag = document.createElement('span');
      tag.className = 'inv-item';
      tag.textContent = `${name} x${qty}`;
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

function renderCovenants(covs) {
  const list = document.getElementById('covenants-list');
  if (!list) return;
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

function renderAltars(altars) {
  const sBox = document.getElementById('solar-offerings');
  const lBox = document.getElementById('lunar-offerings');
  if (!sBox || !lBox || !altars) return;
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

function renderChronicle(logs) {
  const stream = document.getElementById('event-log-stream');
  if (!stream || !logs) return;
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
// CONTINUOUS 60 FPS GAME & RENDER LOOP
// ----------------------------------------------------------------------------
function gameLoop() {
  if (currentTelemetry) {
    drawWorldMap(currentTelemetry);
  }
  requestAnimationFrame(gameLoop);
}

// ----------------------------------------------------------------------------
// INITIALIZATION
// ----------------------------------------------------------------------------
window.addEventListener('load', () => {
  resizeCanvas();
  buildTerrainBuffer();
  centerCamera();
  connectWebSocket();
  requestAnimationFrame(gameLoop);
});
