// Genesis Cogniverse - High-Performance WorldBox God Admin Console
let ws = null;
let currentTelemetry = null;
let selectedAgentId = 1;
let favoriteAgentId = null;
let isFollowActive = false;
const terraformedTiles = new Map(); // key: "x,y" => BiomeType string

// Canvas & Viewport Setup
const canvas = document.getElementById('world-canvas');
const ctx = canvas.getContext('2d');
const viewport = document.getElementById('canvas-viewport');
const tooltip = document.getElementById('hover-tooltip');
const toolToast = document.getElementById('tool-toast');

const WORLD_SIZE = 64;
const TILE_BASE_PX = 16; // Base tile size in virtual bitmap pixels (1024x1024)
canvas.width = WORLD_SIZE * TILE_BASE_PX; // 1024
canvas.height = WORLD_SIZE * TILE_BASE_PX; // 1024

// Camera Matrix
const camera = {
  x: 0,
  y: 0,
  zoom: 1.0,
  minZoom: 0.5,
  maxZoom: 4.0,
  isDragging: false,
  dragStartX: 0,
  dragStartY: 0,
  panStartX: 0,
  panStartY: 0,
};

function initCamera() {
  camera.zoom = 1.0;
  camera.x = 0;
  camera.y = 0;
}

// Active God Power Tool & Brush State
let activeCategory = 'nature';
let activeTool = 'brush-plains';
let brushRadius = 1; // 1 = 1x1, 2 = 3x3, 3 = 5x5
let mouseWorldTile = { x: 32, y: 32 };
let isMouseDownOnMap = false;

// Biome Colors (WorldBox Style Palette)
const BIOME_COLORS = {
  SunlitPlains: '#3f6212',
  WhisperingForest: '#14532d',
  CrystalRiverlands: '#0284c7',
  IroncragMountains: '#52525b',
  AncientLeylineRuins: '#581c87',
};

// Weather Particle System
const weatherParticles = [];
for (let i = 0; i < 120; i++) {
  weatherParticles.push({
    x: Math.random() * canvas.width,
    y: Math.random() * canvas.height,
    speed: 4 + Math.random() * 6,
    len: 8 + Math.random() * 12,
    sway: Math.random() * Math.PI * 2,
  });
}

// -------------------------------------------------------------
// WEBSOCKET COMMUNICATION
// -------------------------------------------------------------
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
      console.error('Error in telemetry handler:', e);
    }
  };

  ws.onclose = () => {
    console.warn('[!] WebSocket closed. Retrying in 2 seconds...');
    setTimeout(connectWebSocket, 2000);
  };
}

function sendMsg(obj) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify(obj));
  }
}

// -------------------------------------------------------------
// TIME DILATION CONTROLS
// -------------------------------------------------------------
function setSpeed(spd) {
  sendMsg({ type: 'SetSpeedMultiplier', speed: spd });
  document.querySelectorAll('.btn-time').forEach(b => b.classList.remove('active'));
  if (spd === 0) document.getElementById('btn-pause').classList.add('active');
  else if (spd === 1) document.getElementById('btn-1x').classList.add('active');
  else if (spd === 10) document.getElementById('btn-10x').classList.add('active');
  else if (spd === 100) document.getElementById('btn-100x').classList.add('active');
  else if (spd >= 1000) document.getElementById('btn-turbo').classList.add('active');
}

function stepTick() {
  sendMsg({ type: 'StepSingleTick' });
}

// -------------------------------------------------------------
// WORLDBOX GOD BAR & TOOL MANAGEMENT
// -------------------------------------------------------------
function setGodCategory(cat) {
  activeCategory = cat;
  document.querySelectorAll('.cat-tab').forEach(t => t.classList.remove('active'));
  document.querySelectorAll('.tools-tray').forEach(tray => tray.classList.remove('active'));

  const tabEl = document.getElementById(`cat-tab-${cat}`);
  const trayEl = document.getElementById(`tray-${cat}`);
  if (tabEl) tabEl.classList.add('active');
  if (trayEl) trayEl.classList.add('active');

  if (cat === 'nature') selectTool('brush-plains', 'Plains Brush');
  else if (cat === 'civilization') selectTool('spawn-solar', 'Spawn Solar Terran');
  else if (cat === 'powers') selectTool('smite-lightning', 'Wrathful Smite');
  else if (cat === 'camera') selectTool('tool-inspect', 'Character Inspect');
}

function selectTool(toolName, displayName) {
  activeTool = toolName;
  document.querySelectorAll('.btn-god-power').forEach(b => {
    if (b.dataset.tool === toolName) b.classList.add('active');
    else b.classList.remove('active');
  });

  if (displayName) {
    showToast(`Tool: ${displayName} (${getBrushDimensionsString()})`);
  }
}

function setBrushRadius(rad) {
  brushRadius = rad;
  document.querySelectorAll('.btn-brush').forEach(b => b.classList.remove('active'));
  const btn = document.getElementById(`brush-r${rad}`);
  if (btn) btn.classList.add('active');
  showToast(`Brush Size: ${getBrushDimensionsString()}`);
}

function getBrushDimensionsString() {
  if (brushRadius === 1) return '1x1';
  if (brushRadius === 2) return '3x3';
  return '5x5';
}

function showToast(msg) {
  if (!toolToast) return;
  toolToast.textContent = msg;
  toolToast.classList.remove('hidden');
  clearTimeout(toolToast._timeout);
  toolToast._timeout = setTimeout(() => {
    toolToast.classList.add('hidden');
  }, 1800);
}

// -------------------------------------------------------------
// CAMERA COORDINATE MATHEMATICS & PAN / ZOOM
// -------------------------------------------------------------
function screenToWorld(clientX, clientY) {
  const rect = canvas.getBoundingClientRect();
  const screenScaleX = canvas.width / rect.width;
  const screenScaleY = canvas.height / rect.height;

  const canvasX = (clientX - rect.left) * screenScaleX;
  const canvasY = (clientY - rect.top) * screenScaleY;

  const tilePx = TILE_BASE_PX * camera.zoom;
  const wx = Math.floor((canvasX - camera.x) / tilePx);
  const wy = Math.floor((canvasY - camera.y) / tilePx);

  return { x: wx, y: wy, rawX: canvasX, rawY: canvasY };
}

function zoomIn() {
  applyZoom(1.25, canvas.width / 2, canvas.height / 2);
}

function zoomOut() {
  applyZoom(0.8, canvas.width / 2, canvas.height / 2);
}

function resetCamera() {
  camera.zoom = 1.0;
  camera.x = 0;
  camera.y = 0;
  isFollowActive = false;
  updateFollowButton();
  updateCoordsLabel();
}

function applyZoom(factor, focusX, focusY) {
  const oldZoom = camera.zoom;
  let newZoom = Math.min(camera.maxZoom, Math.max(camera.minZoom, oldZoom * factor));
  if (newZoom === oldZoom) return;

  camera.x = focusX - (focusX - camera.x) * (newZoom / oldZoom);
  camera.y = focusY - (focusY - camera.y) * (newZoom / oldZoom);
  camera.zoom = newZoom;
  updateCoordsLabel();
}

function updateCoordsLabel() {
  const label = document.getElementById('camera-coords-label');
  if (label) {
    label.textContent = `X: ${mouseWorldTile.x} Y: ${mouseWorldTile.y} | Zoom: ${camera.zoom.toFixed(1)}x`;
  }
}

// Wheel zoom targeting mouse position
viewport.addEventListener('wheel', (e) => {
  e.preventDefault();
  const rect = canvas.getBoundingClientRect();
  const screenScaleX = canvas.width / rect.width;
  const screenScaleY = canvas.height / rect.height;
  const focusX = (e.clientX - rect.left) * screenScaleX;
  const focusY = (e.clientY - rect.top) * screenScaleY;

  const factor = e.deltaY < 0 ? 1.15 : 0.87;
  applyZoom(factor, focusX, focusY);
}, { passive: false });

// Pan & Drag Handlers
viewport.addEventListener('mousedown', (e) => {
  const isPan = e.button === 1 || e.button === 2 || activeTool === 'tool-pan' || e.spaceKey;
  
  if (isPan) {
    camera.isDragging = true;
    camera.dragStartX = e.clientX;
    camera.dragStartY = e.clientY;
    camera.panStartX = camera.x;
    camera.panStartY = camera.y;
    viewport.classList.add('panning');
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
    const dx = (e.clientX - camera.dragStartX) * (canvas.width / viewport.clientWidth);
    const dy = (e.clientY - camera.dragStartY) * (canvas.height / viewport.clientHeight);
    camera.x = camera.panStartX + dx;
    camera.y = camera.panStartY + dy;
    updateCoordsLabel();
    return;
  }

  const { x, y } = screenToWorld(e.clientX, e.clientY);
  mouseWorldTile.x = Math.max(0, Math.min(WORLD_SIZE - 1, x));
  mouseWorldTile.y = Math.max(0, Math.min(WORLD_SIZE - 1, y));
  updateCoordsLabel();

  if (isMouseDownOnMap && activeTool.startsWith('brush-')) {
    handleMapAction(x, y);
  }

  if (x >= 0 && x < WORLD_SIZE && y >= 0 && y < WORLD_SIZE) {
    let text = `Tile (${x}, ${y})`;
    if (currentTelemetry && currentTelemetry.agents) {
      const onTile = currentTelemetry.agents.find(a => a.x === x && a.y === y);
      if (onTile) {
        text += ` | ${onTile.name} [${onTile.race === 'SolarTerran' ? 'Solar' : 'Lunar'}]`;
      }
    }
    tooltip.textContent = text;
    const vpRect = viewport.getBoundingClientRect();
    tooltip.style.left = `${e.clientX - vpRect.left + 14}px`;
    tooltip.style.top = `${e.clientY - vpRect.top + 14}px`;
    tooltip.classList.remove('hidden');
  } else {
    tooltip.classList.add('hidden');
  }
});

window.addEventListener('mouseup', () => {
  if (camera.isDragging) {
    camera.isDragging = false;
    viewport.classList.remove('panning');
  }
  isMouseDownOnMap = false;
});

viewport.addEventListener('mouseleave', () => {
  tooltip.classList.add('hidden');
});

viewport.addEventListener('contextmenu', (e) => e.preventDefault());

// -------------------------------------------------------------
// MAP ACTION DISPATCHER (TERRAFORMING / SPAWN / SMITE)
// -------------------------------------------------------------
function handleMapAction(tx, ty) {
  if (tx < 0 || tx >= WORLD_SIZE || ty < 0 || ty >= WORLD_SIZE) return;

  // 1. Terraforming Brushes
  if (activeTool.startsWith('brush-')) {
    let brushType = 'FertilizePlains';
    let biomeKey = 'SunlitPlains';

    if (activeTool === 'brush-forest') {
      brushType = 'PlantForest';
      biomeKey = 'WhisperingForest';
    } else if (activeTool === 'brush-river') {
      brushType = 'CarveRiver';
      biomeKey = 'CrystalRiverlands';
    } else if (activeTool === 'brush-mountains') {
      brushType = 'RaiseMountain';
      biomeKey = 'IroncragMountains';
    } else if (activeTool === 'brush-ruins') {
      brushType = 'InfuseLeylineRuins';
      biomeKey = 'AncientLeylineRuins';
    } else if (activeTool === 'brush-iron') {
      brushType = 'EnrichIronVein';
      biomeKey = 'IroncragMountains';
    } else if (activeTool === 'brush-clear') {
      brushType = 'ClearLand';
      biomeKey = 'SunlitPlains';
    }

    // Immediately update local terraform map for zero-latency response
    const half = Math.floor(brushRadius / 2);
    for (let dy = -half; dy <= half; dy++) {
      for (let dx = -half; dx <= half; dx++) {
        const px = tx + dx;
        const py = ty + dy;
        if (px >= 0 && px < WORLD_SIZE && py >= 0 && py < WORLD_SIZE) {
          terraformedTiles.set(`${px},${py}`, biomeKey);
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

  // 2. Spawn Agents
  if (activeTool === 'spawn-solar') {
    sendMsg({ type: 'SpawnAgentAt', race: 'SolarTerran', x: tx, y: ty });
    showToast(`☀️ Spawned Solar Terran at (${tx}, ${ty})`);
    return;
  }
  if (activeTool === 'spawn-lunar') {
    sendMsg({ type: 'SpawnAgentAt', race: 'ChthonicLunarian', x: tx, y: ty });
    showToast(`🌙 Spawned Lunari at (${tx}, ${ty})`);
    return;
  }

  // 3. Cataclysms & Miracles
  if (activeTool === 'smite-lightning') {
    sendMsg({
      type: 'TriggerIntervention',
      intervention: { SmiteLightning: { x: tx, y: ty } }
    });
    showToast(`⚡ Wrathful Smite cast on (${tx}, ${ty})`);
    return;
  }
  if (activeTool === 'manna-drop') {
    sendMsg({
      type: 'TriggerIntervention',
      intervention: { MiracleManna: { x: tx, y: ty } }
    });
    showToast(`🍞 Rain of Manna dropped on (${tx}, ${ty})`);
    return;
  }

  // 4. Character Inspection
  if (activeTool === 'tool-inspect' || activeTool === 'inspect') {
    if (currentTelemetry && currentTelemetry.agents) {
      let closest = null;
      let minD = 4;
      for (const a of currentTelemetry.agents) {
        const d = Math.hypot(a.x - tx, a.y - ty);
        if (d < minD) {
          minD = d;
          closest = a;
        }
      }

      if (closest) {
        selectedAgentId = closest.id;
        sendMsg({ type: 'SelectAgent', agent_id: closest.id });
        showToast(`Inspecting: ${closest.name}`);
      }
    }
  }
}

// -------------------------------------------------------------
// CAMERA FOLLOW & FAVORITES
// -------------------------------------------------------------
function toggleFavoriteCurrentAgent() {
  const btn = document.getElementById('btn-favorite-agent');
  if (favoriteAgentId === selectedAgentId) {
    favoriteAgentId = null;
    btn.classList.remove('favorited');
    showToast('Unfavorited Character');
  } else {
    favoriteAgentId = selectedAgentId;
    btn.classList.add('favorited');
    showToast('⭐ Favorited Character!');
  }
}

function toggleFollowSelected() {
  isFollowActive = !isFollowActive;
  updateFollowButton();
}

function updateFollowButton() {
  const btn = document.getElementById('btn-follow-toggle');
  if (isFollowActive) {
    btn.classList.add('active-follow');
    btn.textContent = '🔭 Following';
  } else {
    btn.classList.remove('active-follow');
    btn.textContent = '🔭 Follow Off';
  }
}

function centerOnLeader() {
  if (!currentTelemetry || !currentTelemetry.agents || currentTelemetry.agents.length === 0) return;
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
    showToast(`👑 Centered on Civilizational Leader: ${leader.name}`);
  }
}

function centerCameraOn(worldX, worldY) {
  const tilePx = TILE_BASE_PX * camera.zoom;
  camera.x = canvas.width / 2 - (worldX + 0.5) * tilePx;
  camera.y = canvas.height / 2 - (worldY + 0.5) * tilePx;
  updateCoordsLabel();
}

// -------------------------------------------------------------
// TELEMETRY RENDER PIPELINE
// -------------------------------------------------------------
function renderTelemetry(data) {
  try {
    // Sync backend modified tiles
    if (data.modified_tiles && Array.isArray(data.modified_tiles)) {
      for (const [x, y, biome] of data.modified_tiles) {
        terraformedTiles.set(`${x},${y}`, biome);
      }
    }

    // 1. Update Macro Demographics Bar (WorldBox Style)
    const d = data.demographics;
    if (d) {
      document.getElementById('metric-pop-total').textContent = d.total_population ?? 0;
      const tCount = d.terran_count ?? (data.agents ? data.agents.filter(a => a.race === 'SolarTerran').length : 0);
      const lCount = d.lunarian_count ?? (data.agents ? data.agents.filter(a => a.race === 'ChthonicLunarian').length : 0);
      document.getElementById('metric-factions').innerHTML = 
        `<span class="text-gold">☀️ ${tCount}</span> / <span class="text-violet">🌙 ${lCount}</span>`;
      document.getElementById('metric-wildlife').textContent = `🦌 ${d.wildlife_count ?? 0}`;
      document.getElementById('val-births').textContent = d.births_total ?? 0;
      document.getElementById('val-deaths').textContent = d.deaths_total ?? 0;
      document.getElementById('metric-max-gen').textContent = `🧬 Gen ${d.max_generation ?? 1}`;
      
      const avgHp = d.avg_health ?? 100;
      document.getElementById('val-avg-hp').textContent = `${Number(avgHp).toFixed(0)}%`;
      
      const avgHunger = d.avg_hunger ?? 0.2;
      document.getElementById('val-avg-hunger').textContent = `${(Number(avgHunger) * 100).toFixed(0)}%`;
      
      const avgPiety = d.avg_piety ?? 0.6;
      document.getElementById('val-avg-piety').textContent = `${(Number(avgPiety) * 100).toFixed(0)}%`;
      
      const wealth = d.total_circulating_wealth ?? 0;
      document.getElementById('val-wealth').textContent = Math.round(Number(wealth));
    }

    // Environment badge
    const dayNightStr = data.is_daytime ? '☀️ Day' : '🌙 Night';
    document.getElementById('metric-environment').textContent = `${dayNightStr} | ${data.weather}`;

    // 2. Auto-follow tracked agent if enabled
    if (isFollowActive && data.agents) {
      const tracked = data.agents.find(a => a.id === (favoriteAgentId || selectedAgentId));
      if (tracked) {
        centerCameraOn(tracked.x, tracked.y);
      }
    }

    // 3. Draw 2D Procedural World Canvas
    drawWorldMap(data);

    // 4. Render WorldBox Character Sheet Details
    renderAgentDetails(data.selected_agent);

    // 5. Render Covenants, Altars & Logs
    renderCovenants(data.active_covenants);
    renderAltars(data.altars);
    renderEventLog(data.event_log);
  } catch (err) {
    console.error('Error rendering telemetry:', err);
  }
}

// -------------------------------------------------------------
// 2D PROCEDURAL WORLD MAP & CANVASES
// -------------------------------------------------------------
function drawWorldMap(data) {
  ctx.save();
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  // Apply Camera Translation & Zoom
  ctx.translate(camera.x, camera.y);
  ctx.scale(camera.zoom, camera.zoom);

  const tilePx = TILE_BASE_PX;

  // 1. Terrain & Biomes (Procedural + Dynamic Terraforming)
  for (let y = 0; y < WORLD_SIZE; y++) {
    for (let x = 0; x < WORLD_SIZE; x++) {
      let biome = 'SunlitPlains';
      let biomeType = 'plains';

      // Check if dynamically terraformed
      const override = terraformedTiles.get(`${x},${y}`);
      if (override) {
        biome = override;
        if (biome.includes('Forest')) biomeType = 'forest';
        else if (biome.includes('River')) biomeType = 'river';
        else if (biome.includes('Mountain')) biomeType = 'mountains';
        else if (biome.includes('Ruins')) biomeType = 'ruins';
        else biomeType = 'plains';
      } else {
        const fx = x / WORLD_SIZE;
        const fy = y / WORLD_SIZE;
        const riverCenter = 28 + Math.sin(y * 0.15) * 7;
        const distToRiver = Math.abs(x - riverCenter);

        if (distToRiver <= 2.2) {
          biome = 'CrystalRiverlands';
          biomeType = 'river';
        } else if (fx > 0.65 && fy < 0.55) {
          biome = 'IroncragMountains';
          biomeType = 'mountains';
        } else if (Math.hypot(fx - 0.5, fy - 0.45) < 0.12) {
          biome = 'AncientLeylineRuins';
          biomeType = 'ruins';
        } else if (fx < 0.45 && fy < 0.70) {
          biome = 'WhisperingForest';
          biomeType = 'forest';
        }
      }

      const color = BIOME_COLORS[biome] || BIOME_COLORS.SunlitPlains;
      const px = x * tilePx;
      const py = y * tilePx;

      ctx.fillStyle = color;
      ctx.fillRect(px, py, tilePx, tilePx);

      // Micro-texture details (WorldBox aesthetic)
      if (biomeType === 'forest') {
        ctx.fillStyle = '#0f3a1f';
        ctx.beginPath();
        ctx.arc(px + tilePx * 0.5, py + tilePx * 0.5, tilePx * 0.35, 0, Math.PI * 2);
        ctx.fill();
      } else if (biomeType === 'mountains') {
        ctx.fillStyle = '#3f3f46';
        ctx.beginPath();
        ctx.moveTo(px + tilePx * 0.5, py + 2);
        ctx.lineTo(px + tilePx - 2, py + tilePx - 2);
        ctx.lineTo(px + 2, py + tilePx - 2);
        ctx.closePath();
        ctx.fill();
      } else if (biomeType === 'river') {
        const shimmer = Math.sin(Date.now() * 0.003 + x + y);
        if (shimmer > 0.6) {
          ctx.fillStyle = 'rgba(255, 255, 255, 0.25)';
          ctx.fillRect(px + 3, py + 3, tilePx - 6, 2);
        }
      } else if (biomeType === 'ruins') {
        ctx.fillStyle = 'rgba(192, 132, 252, 0.4)';
        ctx.fillRect(px + 4, py + 4, tilePx - 8, tilePx - 8);
      }
    }
  }

  // 2. Altars (Sun & Moon Holy Sites)
  if (data.altars) {
    for (const altar of data.altars) {
      const px = altar.x * tilePx;
      const py = altar.y * tilePx;

      if (altar.deity === 'SolarPlayerGod') {
        ctx.save();
        ctx.fillStyle = 'rgba(251, 191, 36, 0.35)';
        ctx.beginPath();
        ctx.arc(px + tilePx / 2, py + tilePx / 2, tilePx * 2.2, 0, Math.PI * 2);
        ctx.fill();

        ctx.fillStyle = '#fef08a';
        ctx.fillRect(px - 1, py - 1, tilePx + 2, tilePx + 2);
        ctx.fillStyle = '#eab308';
        ctx.fillRect(px + 2, py + 2, tilePx - 4, tilePx - 4);
        ctx.restore();
      } else {
        ctx.save();
        ctx.fillStyle = 'rgba(192, 132, 252, 0.35)';
        ctx.beginPath();
        ctx.arc(px + tilePx / 2, py + tilePx / 2, tilePx * 2.2, 0, Math.PI * 2);
        ctx.fill();

        ctx.fillStyle = '#c084fc';
        ctx.fillRect(px - 1, py - 1, tilePx + 2, tilePx + 2);
        ctx.fillStyle = '#7e22ce';
        ctx.fillRect(px + 2, py + 2, tilePx - 4, tilePx - 4);
        ctx.restore();
      }
    }
  }

  // 3. Placed Structures (Cabins, Lean-tos, Granaries, Campfires)
  if (data.structures) {
    for (const s of data.structures) {
      const px = s.x * tilePx;
      const py = s.y * tilePx;

      if (s.kind === 'Campfire') {
        ctx.save();
        ctx.fillStyle = '#78350f';
        ctx.fillRect(px + 2, py + 8, tilePx - 4, 4);

        const flicker = Math.sin(Date.now() * 0.02 + s.x) * 2;
        ctx.fillStyle = '#f97316';
        ctx.beginPath();
        ctx.arc(px + tilePx / 2, py + tilePx / 2 - 1, 4 + flicker, 0, Math.PI * 2);
        ctx.fill();

        ctx.fillStyle = '#fef08a';
        ctx.beginPath();
        ctx.arc(px + tilePx / 2, py + tilePx / 2 - 2, 2, 0, Math.PI * 2);
        ctx.fill();
        ctx.restore();
      } else if (s.kind === 'StoneCabin') {
        ctx.save();
        ctx.fillStyle = '#475569';
        ctx.fillRect(px + 1, py + 2, tilePx - 2, tilePx - 3);
        ctx.fillStyle = '#991b1b';
        ctx.beginPath();
        ctx.moveTo(px + tilePx / 2, py);
        ctx.lineTo(px + tilePx, py + 5);
        ctx.lineTo(px, py + 5);
        ctx.closePath();
        ctx.fill();
        ctx.fillStyle = '#fef08a';
        ctx.fillRect(px + 6, py + 7, 4, 6);
        ctx.restore();
      } else if (s.kind === 'LeanTo') {
        ctx.save();
        ctx.fillStyle = '#92400e';
        ctx.beginPath();
        ctx.moveTo(px + 2, py + tilePx - 2);
        ctx.lineTo(px + tilePx - 2, py + 3);
        ctx.lineTo(px + tilePx - 2, py + tilePx - 2);
        ctx.closePath();
        ctx.fill();
        ctx.restore();
      } else if (s.kind === 'Granary') {
        ctx.save();
        ctx.fillStyle = '#b45309';
        ctx.fillRect(px + 2, py + 4, tilePx - 4, tilePx - 6);
        ctx.fillStyle = '#fbbf24';
        ctx.fillRect(px + 4, py + 2, tilePx - 8, 3);
        ctx.restore();
      }
    }
  }

  // 4. Wildlife
  if (data.wildlife) {
    for (const w of data.wildlife) {
      const px = w.x * tilePx + tilePx / 2;
      const py = w.y * tilePx + tilePx / 2;

      ctx.save();
      ctx.fillStyle = '#38bdf8';
      ctx.beginPath();
      ctx.arc(px, py, 2.5, 0, Math.PI * 2);
      ctx.fill();
      ctx.restore();
    }
  }

  // 5. Living Agents (Animated WorldBox Characters)
  if (data.agents) {
    const time = Date.now();
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
        ctx.arc(px, py + bounce, tilePx * 0.9, 0, Math.PI * 2);
        ctx.stroke();

        ctx.fillStyle = '#38bdf8';
        ctx.fillRect(px - 1, py + bounce - tilePx * 1.1, 2, 4);
      }

      // Favorite Star Indicator
      if (isFavorite) {
        ctx.fillStyle = '#fbbf24';
        ctx.font = '10px sans-serif';
        ctx.fillText('⭐', px - 5, py + bounce - tilePx * 0.8);
      }

      // Leader Crown
      if (a.generation >= 2 || a.piety > 0.85) {
        ctx.fillStyle = '#eab308';
        ctx.font = '9px sans-serif';
        ctx.fillText('👑', px - 4, py + bounce - tilePx * 0.7);
      }

      // Agent Body
      const isTerran = a.race === 'SolarTerran';
      ctx.fillStyle = isTerran ? '#fbbf24' : '#c084fc';
      ctx.shadowColor = isTerran ? '#fbbf24' : '#c084fc';
      ctx.shadowBlur = 6;

      ctx.beginPath();
      ctx.arc(px, py + bounce, tilePx * 0.42, 0, Math.PI * 2);
      ctx.fill();

      // Health bar when zoomed in
      if (camera.zoom >= 1.4) {
        ctx.fillStyle = 'rgba(0, 0, 0, 0.6)';
        ctx.fillRect(px - 8, py + bounce + 7, 16, 2.5);
        ctx.fillStyle = '#4ade80';
        ctx.fillRect(px - 8, py + bounce + 7, 16 * (a.health / 100), 2.5);
      }

      ctx.restore();
    }
  }

  // 6. Dynamic Day/Night Lighting Ambient Overlay
  if (!data.is_daytime) {
    ctx.save();
    ctx.fillStyle = 'rgba(5, 8, 22, 0.72)';
    ctx.fillRect(0, 0, WORLD_SIZE * tilePx, WORLD_SIZE * tilePx);

    if (data.structures) {
      for (const s of data.structures) {
        if (s.kind === 'Campfire' || s.kind === 'StoneCabin') {
          const px = s.x * tilePx + tilePx / 2;
          const py = s.y * tilePx + tilePx / 2;
          const radGrad = ctx.createRadialGradient(px, py, 2, px, py, tilePx * 2.5);
          radGrad.addColorStop(0, 'rgba(251, 191, 36, 0.45)');
          radGrad.addColorStop(1, 'rgba(251, 191, 36, 0)');
          ctx.fillStyle = radGrad;
          ctx.beginPath();
          ctx.arc(px, py, tilePx * 2.5, 0, Math.PI * 2);
          ctx.fill();
        }
      }
    }
    ctx.restore();
  }

  // 7. Atmospheric Weather Particles (Rain / Storm / Snow)
  if (data.weather === 'GentleRain' || data.weather === 'Thunderstorm') {
    ctx.save();
    ctx.strokeStyle = data.weather === 'Thunderstorm' ? 'rgba(186, 230, 253, 0.7)' : 'rgba(125, 211, 252, 0.45)';
    ctx.lineWidth = 1.2;

    for (const p of weatherParticles) {
      p.y += p.speed;
      p.x += Math.sin(p.sway) * 0.8;
      if (p.y > canvas.height) {
        p.y = 0;
        p.x = Math.random() * canvas.width;
      }
      ctx.beginPath();
      ctx.moveTo(p.x, p.y);
      ctx.lineTo(p.x - 3, p.y + p.len);
      ctx.stroke();
    }

    if (data.weather === 'Thunderstorm' && Math.random() < 0.006) {
      ctx.fillStyle = 'rgba(255, 255, 255, 0.4)';
      ctx.fillRect(0, 0, WORLD_SIZE * tilePx, WORLD_SIZE * tilePx);
    }
    ctx.restore();
  } else if (data.weather === 'FrostBreeze') {
    ctx.save();
    ctx.fillStyle = 'rgba(255, 255, 255, 0.75)';
    for (const p of weatherParticles) {
      p.y += p.speed * 0.35;
      p.x += Math.sin(p.sway) * 1.5;
      if (p.y > canvas.height) {
        p.y = 0;
        p.x = Math.random() * canvas.width;
      }
      ctx.beginPath();
      ctx.arc(p.x, p.y, 1.8, 0, Math.PI * 2);
      ctx.fill();
    }
    ctx.restore();
  }

  // 8. Brush Hover Outline Reticle
  if (activeTool.startsWith('brush-')) {
    ctx.save();
    const half = Math.floor(brushRadius / 2);
    const startX = Math.max(0, mouseWorldTile.x - half);
    const startY = Math.max(0, mouseWorldTile.y - half);
    const size = brushRadius;

    ctx.strokeStyle = 'rgba(251, 191, 36, 0.85)';
    ctx.lineWidth = 2;
    ctx.setLineDash([4, 4]);
    ctx.strokeRect(startX * tilePx, startY * tilePx, size * tilePx, size * tilePx);
    ctx.restore();
  }

  ctx.restore();
}

// -------------------------------------------------------------
// WORLDBOX CHARACTER SHEET INSPECTOR
// -------------------------------------------------------------
function renderAgentDetails(agent) {
  const body = document.getElementById('inspect-body');
  const archBadge = document.getElementById('inspect-archetype-badge');
  const factBadge = document.getElementById('inspect-faction-badge');
  const nameEl = document.getElementById('inspect-name');
  const subEl = document.getElementById('inspect-sub');
  const starBtn = document.getElementById('btn-favorite-agent');

  if (!agent) {
    body.classList.add('hidden');
    archBadge.classList.add('hidden');
    factBadge.classList.add('hidden');
    nameEl.textContent = 'Select an Agent to Inspect';
    subEl.textContent = 'Click any character on the map';
    starBtn.classList.remove('favorited');
    return;
  }

  body.classList.remove('hidden');
  archBadge.classList.remove('hidden');
  factBadge.classList.remove('hidden');

  archBadge.textContent = agent.archetype;
  const isSolar = agent.race === 'SolarTerran';
  factBadge.textContent = isSolar ? '☀️ Solar Terran' : '🌙 Chthonic Lunarian';
  factBadge.style.color = isSolar ? '#fef08a' : '#c084fc';
  factBadge.style.borderColor = isSolar ? '#eab308' : '#a855f7';

  const agentGen = agent.genome ? agent.genome.generation : 1;
  nameEl.textContent = agent.name;
  subEl.textContent = `Gen: ${agentGen} | HP: ${agent.health.toFixed(0)} | At (${agent.x}, ${agent.y}) | Piety: ${(agent.piety * 100).toFixed(0)}%`;

  if (favoriteAgentId === agent.id) {
    starBtn.classList.add('favorited');
  } else {
    starBtn.classList.remove('favorited');
  }

  // WorldBox Traits Grid
  const traitsContainer = document.getElementById('inspect-traits-container');
  traitsContainer.innerHTML = '';
  const traitsList = agent.traits && agent.traits.length > 0 
    ? agent.traits 
    : ['🌟 Genius', '⚔️ Strong', '🕊️ Pacifist'];

  for (const t of traitsList) {
    const badge = document.createElement('span');
    badge.className = 'trait-badge';
    badge.textContent = t;
    traitsContainer.appendChild(badge);
  }

  // Inner Monologue
  document.getElementById('inspect-monologue').textContent = agent.current_thought || 'Observing the cosmos...';

  // Vitals Bars & Text
  document.getElementById('bar-health').style.width = `${agent.health}%`;
  document.getElementById('txt-val-health').textContent = `${agent.health.toFixed(0)}%`;

  document.getElementById('bar-energy').style.width = `${agent.energy * 100}%`;
  document.getElementById('txt-val-energy').textContent = `${(agent.energy * 100).toFixed(0)}%`;

  document.getElementById('bar-hunger').style.width = `${agent.hunger * 100}%`;
  document.getElementById('txt-val-hunger').textContent = `${(agent.hunger * 100).toFixed(0)}%`;

  document.getElementById('bar-piety').style.width = `${agent.piety * 100}%`;
  document.getElementById('txt-val-piety').textContent = `${(agent.piety * 100).toFixed(0)}%`;

  // Neuromodulation
  if (agent.neuromod) {
    document.getElementById('bar-dopamine').style.width = `${agent.neuromod.dopamine * 100}%`;
    document.getElementById('val-dopamine').textContent = agent.neuromod.dopamine.toFixed(2);

    document.getElementById('bar-serotonin').style.width = `${agent.neuromod.serotonin * 100}%`;
    document.getElementById('val-serotonin').textContent = agent.neuromod.serotonin.toFixed(2);

    document.getElementById('bar-noradrenaline').style.width = `${agent.neuromod.noradrenaline * 100}%`;
    document.getElementById('val-noradrenaline').textContent = agent.neuromod.noradrenaline.toFixed(2);
  }

  // Inventory & Lineage
  const invEl = document.getElementById('inspect-inventory');
  invEl.innerHTML = '';
  if (agent.inventory_items && agent.inventory_items.length > 0) {
    for (const [itemName, qty] of agent.inventory_items) {
      const tag = document.createElement('span');
      tag.className = 'inv-tag';
      tag.textContent = `${itemName} x${qty}`;
      invEl.appendChild(tag);
    }
  } else {
    invEl.innerHTML = '<span class="text-dim" style="font-size:0.6rem">Empty Pack</span>';
  }

  document.getElementById('inspect-echo').textContent = agent.ancestral_echo || 'First Generation Pioneer';

  // AST Genome
  const genNum = agent.genome ? agent.genome.generation : agent.generation;
  const mutNum = agent.genome ? agent.genome.mutation_count : 0;
  document.getElementById('inspect-gen').textContent = `GEN: ${genNum} | MUTATIONS: ${mutNum}`;

  const astStr = agent.genome 
    ? `(ast\n  (condition (> hunger 0.65))\n  (then (seek-food))\n  (else (explore-world)))` 
    : `(ast (action WanderRandomly))`;
  document.getElementById('inspect-ast').textContent = astStr;
}

// -------------------------------------------------------------
// DIRECT CHARACTER ACTIONS
// -------------------------------------------------------------
function whisperCurrentAgent() {
  if (!selectedAgentId) return;
  const msg = prompt('Enter divine telepathic whisper for this character:', 'Erect a great stone cabin before the frost deepens.');
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
    showToast(`🕊️ Whispered divine command to Agent #${selectedAgentId}`);
  }
}

function blessCurrentAgent() {
  if (!selectedAgentId) return;
  sendMsg({
    type: 'TriggerIntervention',
    intervention: {
      DivineWhisper: {
        target_agent_id: selectedAgentId,
        message: 'The Golden Sun fills your soul with divine ecstasy.',
      }
    }
  });
  showToast(`✨ Blessed Agent #${selectedAgentId} with divine inspiration!`);
}

function healCurrentAgent() {
  if (!selectedAgentId) return;
  sendMsg({
    type: 'TriggerIntervention',
    intervention: {
      MiracleManna: {
        x: mouseWorldTile.x,
        y: mouseWorldTile.y,
      }
    }
  });
  showToast(`💚 Restored vitals for Agent #${selectedAgentId}`);
}

function smiteCurrentAgent() {
  if (!selectedAgentId) return;
  if (confirm(`Strike Agent #${selectedAgentId} with wrathful divine lightning?`)) {
    sendMsg({
      type: 'TriggerIntervention',
      intervention: {
        SmiteLightning: {
          x: mouseWorldTile.x,
          y: mouseWorldTile.y,
        }
      }
    });
    showToast(`⚡ Wrathful Smite struck Agent #${selectedAgentId}`);
  }
}

// -------------------------------------------------------------
// COVENANTS, ALTARS & EVENT LOG RENDERING
// -------------------------------------------------------------
function switchTab(tab) {
  document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
  document.querySelectorAll('.tab-pane').forEach(p => p.classList.remove('active'));

  if (tab === 'covenants') {
    document.getElementById('tab-cov-btn').classList.add('active');
    document.getElementById('tab-covenants').classList.add('active');
  } else if (tab === 'altars') {
    document.getElementById('tab-alt-btn').classList.add('active');
    document.getElementById('tab-altars').classList.add('active');
  } else if (tab === 'events') {
    document.getElementById('tab-log-btn').classList.add('active');
    document.getElementById('tab-events').classList.add('active');
  }
}

function issueCovenant() {
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
    list.innerHTML = '<span class="text-dim" style="font-size: 0.65rem">No active covenants. Forge one above!</span>';
    return;
  }

  for (const c of covs) {
    const el = document.createElement('div');
    el.className = 'covenant-item';
    el.innerHTML = `
      <strong>${c.title}</strong><br>
      <span class="text-dim">Progress: ${c.current_progress} / ${c.target_progress || c.target_amount || 5} | Reward: ${c.promised_reward || c.reward}</span>
    `;
    list.appendChild(el);
  }
}

function renderAltars(altars) {
  const solarEl = document.getElementById('solar-offerings');
  const lunarEl = document.getElementById('lunar-offerings');
  if (!solarEl || !lunarEl) return;

  solarEl.innerHTML = '';
  lunarEl.innerHTML = '';

  if (!altars) return;

  for (const altar of altars) {
    const isSolar = altar.deity === 'SolarPlayerGod';
    const container = isSolar ? solarEl : lunarEl;

    if (!altar.active_offerings || altar.active_offerings.length === 0) {
      container.innerHTML = '<span class="text-dim" style="font-size:0.6rem">No offerings currently presented.</span>';
      continue;
    }

    for (const off of altar.active_offerings) {
      const row = document.createElement('div');
      row.className = 'offering-item';
      row.innerHTML = `
        <div>
          <strong>${off.item}</strong> by Agent #${off.offerer_id}<br>
          <span class="text-dim">Sincerity: ${(off.sincerity * 100).toFixed(0)}%</span>
        </div>
        <div class="off-actions">
          <button class="btn-mini text-green" onclick="blessOffering(${altar.id}, ${off.id})">Bless</button>
          <button class="btn-mini text-red" onclick="smiteOffering(${altar.id}, ${off.id})">Smite</button>
        </div>
      `;
      container.appendChild(row);
    }
  }
}

function blessOffering(altarId, offId) {
  sendMsg({ type: 'BlessOffering', altar_id: altarId, offering_id: offId });
  showToast('✨ Sacred offering blessed!');
}

function smiteOffering(altarId, offId) {
  sendMsg({ type: 'SmiteOffering', altar_id: altarId, offering_id: offId });
  showToast('⚡ Unworthy offering rejected with lightning!');
}

function renderEventLog(logs) {
  const stream = document.getElementById('event-log-stream');
  if (!stream || !logs) return;

  stream.innerHTML = '';
  for (let i = logs.length - 1; i >= Math.max(0, logs.length - 25); i--) {
    const line = document.createElement('div');
    line.className = 'event-line';
    line.textContent = logs[i];
    stream.appendChild(line);
  }
}

function promptDivineWhisper() {
  whisperCurrentAgent();
}

function triggerGenesisSpark() {
  if (confirm('Ignite Genesis Evolution Spark? Forces an immediate mutation epoch across all agents.')) {
    sendMsg({
      type: 'TriggerIntervention',
      intervention: 'GenesisEvolutionSpark'
    });
    showToast('🧬 Genesis Evolution Spark ignited!');
  }
}

function triggerBabel() {
  if (confirm('Initiate Tower of Babel? Scrambles dialect trees.')) {
    sendMsg({
      type: 'TriggerIntervention',
      intervention: 'TowerOfBabel'
    });
    showToast('🌀 Tower of Babel initiated!');
  }
}

// -------------------------------------------------------------
// INITIALIZATION
// -------------------------------------------------------------
window.addEventListener('load', () => {
  initCamera();
  connectWebSocket();
});
