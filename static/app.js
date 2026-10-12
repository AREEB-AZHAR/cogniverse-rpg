// Genesis Cogniverse - High-Performance Web Console Client
let ws = null;
let currentTelemetry = null;
let selectedAgentId = 1;
const canvas = document.getElementById('world-canvas');
const ctx = canvas.getContext('2d');
const tooltip = document.getElementById('hover-tooltip');

const WORLD_SIZE = 64;
const TILE_PX = canvas.width / WORLD_SIZE; // 10px per tile

// Biome color map
const BIOME_COLORS = {
  SunlitPlains: '#3f6212',
  WhisperingForest: '#14532d',
  CrystalRiverlands: '#0284c7',
  IroncragMountains: '#71717a',
  AncientLeylineRuins: '#581c87',
};

// Connect WebSocket
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
      console.error('Error parsing telemetry JSON:', e);
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

// Time Dilation Commands
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

// Tab Switching
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

// Covenants
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
}

// Divine Interventions
function promptDivineWhisper() {
  const msg = prompt('Enter the divine revelation or thought to whisper:', 'Gather iron and build the Great Cabin before the blizzard strikes.');
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
  }
}

function triggerManna() {
  const x = prompt('Target X coordinate for Manna (0-63):', '24');
  const y = prompt('Target Y coordinate for Manna (0-63):', '36');
  if (x !== null && y !== null) {
    sendMsg({
      type: 'TriggerIntervention',
      intervention: {
        MiracleManna: {
          x: parseInt(x, 10),
          y: parseInt(y, 10),
        }
      }
    });
  }
}

function triggerSmite() {
  const x = prompt('Target X coordinate for Lightning Smite (0-63):', '48');
  const y = prompt('Target Y coordinate for Lightning Smite (0-63):', '16');
  if (x !== null && y !== null) {
    sendMsg({
      type: 'TriggerIntervention',
      intervention: {
        SmiteLightning: {
          x: parseInt(x, 10),
          y: parseInt(y, 10),
        }
      }
    });
  }
}

function triggerBabel() {
  if (confirm('Initiate Tower of Babel? This will scramble all emergent dialect trees.')) {
    sendMsg({
      type: 'TriggerIntervention',
      intervention: 'TowerOfBabel'
    });
  }
}

function triggerGenesisSpark() {
  if (confirm('Ignite Genesis Evolution Spark? Forces an immediate mutation epoch across all agents.')) {
    sendMsg({
      type: 'TriggerIntervention',
      intervention: 'GenesisEvolutionSpark'
    });
  }
}

// Altar Actions
function blessOffering(altarId, offId) {
  sendMsg({ type: 'BlessOffering', altar_id: altarId, offering_id: offId });
}

function smiteOffering(altarId, offId) {
  sendMsg({ type: 'SmiteOffering', altar_id: altarId, offering_id: offId });
}

// Canvas Interaction (Click to Inspect)
canvas.addEventListener('click', (e) => {
  const rect = canvas.getBoundingClientRect();
  const clickX = Math.floor((e.clientX - rect.left) / TILE_PX);
  const clickY = Math.floor((e.clientY - rect.top) / TILE_PX);

  if (currentTelemetry && currentTelemetry.agents) {
    // Find closest agent within 3 tiles
    let closest = null;
    let minD = 4;
    for (const a of currentTelemetry.agents) {
      const d = Math.hypot(a.x - clickX, a.y - clickY);
      if (d < minD) {
        minD = d;
        closest = a;
      }
    }

    if (closest) {
      selectedAgentId = closest.id;
      sendMsg({ type: 'SelectAgent', agent_id: closest.id });
    }
  }
});

// Canvas Hover Tooltip
canvas.addEventListener('mousemove', (e) => {
  const rect = canvas.getBoundingClientRect();
  const tx = Math.floor((e.clientX - rect.left) / TILE_PX);
  const ty = Math.floor((e.clientY - rect.top) / TILE_PX);

  if (tx >= 0 && tx < WORLD_SIZE && ty >= 0 && ty < WORLD_SIZE) {
    let text = `Coords: (${tx}, ${ty})`;
    if (currentTelemetry && currentTelemetry.agents) {
      const onTile = currentTelemetry.agents.find(a => a.x === tx && a.y === ty);
      if (onTile) {
        text += ` | Agent: ${onTile.name} (${onTile.race})`;
      }
    }
    tooltip.textContent = text;
    tooltip.style.left = `${e.clientX - rect.left + 15}px`;
    tooltip.style.top = `${e.clientY - rect.top + 15}px`;
    tooltip.classList.remove('hidden');
  } else {
    tooltip.classList.add('hidden');
  }
});

canvas.addEventListener('mouseleave', () => {
  tooltip.classList.add('hidden');
});

// Main Telemetry Renderer
function renderTelemetry(data) {
  // Update Top Metrics
  document.getElementById('metric-tick').textContent = data.world_tick.toLocaleString();
  document.getElementById('metric-weather').textContent = data.weather;
  const tempSign = data.global_temperature_offset >= 0 ? '+' : '';
  document.getElementById('metric-temp').textContent = `${tempSign}${data.global_temperature_offset.toFixed(1)}°C`;

  const terranCount = data.agents.filter(a => a.race === 'SolarTerran').length;
  const lunarCount = data.agents.filter(a => a.race === 'ChthonicLunarian').length;
  document.getElementById('metric-pop').textContent = `${terranCount} / ${lunarCount}`;

  // Draw Canvas
  drawWorldMap(data);

  // Render Selected Agent Details
  renderAgentDetails(data.selected_agent);

  // Render Covenants
  renderCovenants(data.active_covenants);

  // Render Altars
  renderAltars(data.altars);

  // Render Event Log
  renderEventLog(data.event_log);
}

// 2D World Map Canvas Rendering
function drawWorldMap(data) {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  // 1. Procedural terrain background
  for (let y = 0; y < WORLD_SIZE; y++) {
    for (let x = 0; x < WORLD_SIZE; x++) {
      const fx = x / WORLD_SIZE;
      const fy = y / WORLD_SIZE;
      const riverCenter = 28 + Math.sin(y * 0.15) * 7;
      const distToRiver = Math.abs(x - riverCenter);

      let color = BIOME_COLORS.SunlitPlains;
      if (distToRiver <= 2.2) {
        color = BIOME_COLORS.CrystalRiverlands;
      } else if (fx > 0.65 && fy < 0.55) {
        color = BIOME_COLORS.IroncragMountains;
      } else if (Math.hypot(fx - 0.5, fy - 0.45) < 0.12) {
        color = BIOME_COLORS.AncientLeylineRuins;
      } else if (fx < 0.45 && fy < 0.70) {
        color = BIOME_COLORS.WhisperingForest;
      }

      ctx.fillStyle = color;
      ctx.fillRect(x * TILE_PX, y * TILE_PX, TILE_PX, TILE_PX);
    }
  }

  // 2. Altars
  if (data.altars) {
    for (const altar of data.altars) {
      const px = altar.x * TILE_PX;
      const py = altar.y * TILE_PX;

      ctx.save();
      if (altar.deity === 'SolarPlayerGod') {
        // Glowing Golden Sun Altar
        ctx.fillStyle = 'rgba(234, 179, 8, 0.4)';
        ctx.beginPath();
        ctx.arc(px + TILE_PX / 2, py + TILE_PX / 2, TILE_PX * 2.5, 0, Math.PI * 2);
        ctx.fill();

        ctx.fillStyle = '#fef08a';
        ctx.fillRect(px - 2, py - 2, TILE_PX + 4, TILE_PX + 4);
      } else {
        // Glowing Obsidian Moon Altar
        ctx.fillStyle = 'rgba(168, 85, 247, 0.4)';
        ctx.beginPath();
        ctx.arc(px + TILE_PX / 2, py + TILE_PX / 2, TILE_PX * 2.5, 0, Math.PI * 2);
        ctx.fill();

        ctx.fillStyle = '#c084fc';
        ctx.fillRect(px - 2, py - 2, TILE_PX + 4, TILE_PX + 4);
      }
      ctx.restore();
    }
  }

  // 3. Wildlife
  // We can render simple animal blips
  // (Represented as subtle green/gray indicators)

  // 4. Agents
  if (data.agents) {
    for (const a of data.agents) {
      const px = a.x * TILE_PX + TILE_PX / 2;
      const py = a.y * TILE_PX + TILE_PX / 2;

      ctx.save();
      const isSelected = selectedAgentId === a.id;

      if (isSelected) {
        // Selection reticle
        ctx.strokeStyle = '#06b6d4';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.arc(px, py, TILE_PX * 1.6, 0, Math.PI * 2);
        ctx.stroke();
      }

      // Agent marker
      const isTerran = a.race === 'SolarTerran';
      ctx.fillStyle = isTerran ? '#eab308' : '#a855f7';
      ctx.shadowColor = isTerran ? '#eab308' : '#a855f7';
      ctx.shadowBlur = 6;

      ctx.beginPath();
      ctx.arc(px, py, TILE_PX * 0.7, 0, Math.PI * 2);
      ctx.fill();

      ctx.restore();
    }
  }
}

// Render Inspector Panel
function renderAgentDetails(agent) {
  const body = document.getElementById('inspect-body');
  const badge = document.getElementById('inspect-archetype-badge');
  const nameEl = document.getElementById('inspect-name');
  const subEl = document.getElementById('inspect-sub');

  if (!agent) {
    body.classList.add('hidden');
    badge.classList.add('hidden');
    nameEl.textContent = 'Select an Agent to Inspect';
    subEl.textContent = 'Click any gold or violet dot on the canvas';
    return;
  }

  body.classList.remove('hidden');
  badge.classList.remove('hidden');
  badge.textContent = agent.archetype;
  nameEl.textContent = `${agent.name} (${agent.race === 'SolarTerran' ? 'Solar Terran' : 'Chthonic Lunarian'})`;
  subEl.textContent = `Age: ${agent.health.toFixed(0)} HP | At (${agent.x}, ${agent.y}) | Piety: ${(agent.piety * 100).toFixed(0)}%`;

  // Monologue
  document.getElementById('inspect-monologue').textContent = agent.current_thought;

  // Vitals Bars
  document.getElementById('bar-health').style.width = `${agent.health}%`;
  document.getElementById('bar-energy').style.width = `${agent.energy * 100}%`;
  document.getElementById('bar-hunger').style.width = `${agent.hunger * 100}%`;
  document.getElementById('bar-thirst').style.width = `${agent.thirst * 100}%`;

  // Neuromodulators
  document.getElementById('bar-dopamine').style.width = `${agent.neuromod.dopamine * 100}%`;
  document.getElementById('val-dopamine').textContent = agent.neuromod.dopamine.toFixed(2);

  document.getElementById('bar-serotonin').style.width = `${agent.neuromod.serotonin * 100}%`;
  document.getElementById('val-serotonin').textContent = agent.neuromod.serotonin.toFixed(2);

  document.getElementById('bar-noradrenaline').style.width = `${agent.neuromod.noradrenaline * 100}%`;
  document.getElementById('val-noradrenaline').textContent = agent.neuromod.noradrenaline.toFixed(2);

  // Inventory
  const invContainer = document.getElementById('inspect-inventory');
  invContainer.innerHTML = '';
  if (agent.inventory_items.length === 0) {
    invContainer.innerHTML = '<span class="text-dim">Empty</span>';
  } else {
    for (const [item, count] of agent.inventory_items) {
      const tag = document.createElement('span');
      tag.className = 'inv-tag';
      tag.textContent = `${item} x${count}`;
      invContainer.appendChild(tag);
    }
  }

  // Heritage & Echo
  document.getElementById('inspect-echo').textContent = agent.ancestral_echo || 'None (Ordinary Mortal)';

  // AST Genome
  document.getElementById('inspect-gen').textContent = `GEN: ${agent.genome.generation} | MUTATIONS: ${agent.genome.mutations_count}`;
  document.getElementById('inspect-ast').textContent = formatAST(agent.genome.root, 0);
}

// AST Tree Formatter
function formatAST(stmt, indent) {
  const pad = '  '.repeat(indent);
  if (!stmt) return `${pad}(None)`;

  if (stmt.IfElse) {
    const condStr = formatExpr(stmt.IfElse[0]);
    const thenStr = formatAST(stmt.IfElse[1], indent + 1);
    const elseStr = formatAST(stmt.IfElse[2], indent + 1);
    return `${pad}IF (${condStr}) THEN:\n${thenStr}\n${pad}ELSE:\n${elseStr}`;
  } else if (stmt.PriorityBranch) {
    let out = `${pad}PRIORITY SELECTOR:`;
    for (const [expr, s] of stmt.PriorityBranch) {
      out += `\n${pad}  [When ${formatExpr(expr)}] =>\n${formatAST(s, indent + 2)}`;
    }
    return out;
  } else if (stmt.Act) {
    return `${pad}ACT: ${JSON.stringify(stmt.Act)}`;
  } else if (stmt.Sequence) {
    return `${pad}SEQ: [\n` + stmt.Sequence.map(s => formatAST(s, indent + 1)).join('\n') + `\n${pad}]`;
  }
  return `${pad}${JSON.stringify(stmt)}`;
}

function formatExpr(expr) {
  if (typeof expr === 'string') return expr;
  if (expr.Vital) return `Vital(${expr.Vital})`;
  if (expr.Constant !== undefined) return expr.Constant.toFixed(2);
  if (expr.DistanceTo) return `DistTo(${expr.DistanceTo})`;
  if (expr.BinaryOp) {
    return `${formatExpr(expr.BinaryOp[0])} ${expr.BinaryOp[1]} ${formatExpr(expr.BinaryOp[2])}`;
  }
  return JSON.stringify(expr);
}

// Render Covenants
function renderCovenants(covenants) {
  const container = document.getElementById('covenants-list');
  container.innerHTML = '';

  if (!covenants || covenants.length === 0) {
    container.innerHTML = '<span class="text-dim">No active divine covenants.</span>';
    return;
  }

  for (const c of covenants) {
    const el = document.createElement('div');
    el.className = 'cov-item';
    const pct = Math.min(100, (c.current_progress / c.target_progress) * 100);
    const statusText = c.is_completed ? '✅ FULFILLED' : `${c.current_progress}/${c.target_progress}`;

    el.innerHTML = `
      <div style="display:flex; justify-content:space-between;">
        <strong>${c.title}</strong>
        <span>${statusText}</span>
      </div>
      <div style="color:#94a3b8; font-size:0.7rem;">Reward: ${c.promised_reward}</div>
      <div class="cov-progress-bar">
        <div class="cov-progress-fill" style="width:${pct}%"></div>
      </div>
    `;
    container.appendChild(el);
  }
}

// Render Altars & Sacrifices
function renderAltars(altars) {
  const solarDiv = document.getElementById('solar-offerings');
  const lunarDiv = document.getElementById('lunar-offerings');
  solarDiv.innerHTML = '';
  lunarDiv.innerHTML = '';

  if (!altars) return;

  for (const altar of altars) {
    const targetDiv = altar.deity === 'SolarPlayerGod' ? solarDiv : lunarDiv;
    if (altar.recent_offerings.length === 0) {
      targetDiv.innerHTML = '<span class="text-dim">No recent offerings.</span>';
      continue;
    }

    for (const off of altar.recent_offerings.slice(-4).reverse()) {
      const row = document.createElement('div');
      row.className = 'offering-entry';
      const outcome = off.outcome ? (off.outcome.Blessed ? ' (Blessed)' : ' (Smitten)') : '';

      row.innerHTML = `
        <div>
          <strong>${off.agent_name}</strong>: ${off.amount}x ${off.item} ${outcome}
          <div style="font-size:0.6rem; color:#94a3b8;">Sincerity: ${(off.sincerity_score * 100).toFixed(0)}%</div>
        </div>
        ${!off.resolved && altar.deity === 'SolarPlayerGod' ? `
          <div style="display:flex; gap:3px;">
            <button class="btn-sm-bless" onclick="blessOffering(${altar.id}, ${off.offering_id})">Bless</button>
            <button class="btn-sm-smite" onclick="smiteOffering(${altar.id}, ${off.offering_id})">Smite</button>
          </div>
        ` : ''}
      `;
      targetDiv.appendChild(row);
    }
  }
}

// Render Event Log
function renderEventLog(logs) {
  const container = document.getElementById('event-log-stream');
  container.innerHTML = '';
  if (!logs) return;

  for (const log of logs.slice(-20)) {
    const el = document.createElement('div');
    el.className = 'log-entry';
    el.textContent = `> ${log}`;
    container.appendChild(el);
  }
}

// Start
window.addEventListener('DOMContentLoaded', () => {
  connectWebSocket();
});
