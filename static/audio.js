/**
 * Genesis Cogniverse - Audio Asset Manager
 * Preloads, manages, and plays disk-based WAV sound assets with gain stage control.
 */

class SoundManager {
  constructor() {
    this.ctx = null;
    this.buffers = new Map();
    this.isMuted = false;
    this.masterVolume = 0.8;
    this.sfxVolume = 0.85;
    this.ambientVolume = 0.4;

    this.masterGain = null;
    this.sfxGain = null;
    this.ambientGain = null;
    this.ambientSource = null;
    this.isAmbientPlaying = false;

    this.assetList = [
      'click',
      'mining',
      'explosion',
      'titan_roar',
      'chime',
      'victory',
      'defeat',
      'ambient_breeze',
    ];

    // Load saved settings if any
    try {
      const savedMaster = localStorage.getItem('cogniverse_master_volume');
      if (savedMaster !== null) this.masterVolume = parseFloat(savedMaster);
      const savedSfx = localStorage.getItem('cogniverse_sfx_volume');
      if (savedSfx !== null) this.sfxVolume = parseFloat(savedSfx);
      const savedAmbient = localStorage.getItem('cogniverse_ambient_volume');
      if (savedAmbient !== null) this.ambientVolume = parseFloat(savedAmbient);
      const savedMute = localStorage.getItem('cogniverse_audio_muted');
      if (savedMute !== null) this.isMuted = savedMute === 'true';
    } catch (_) {}
  }

  ensureContext() {
    if (!this.ctx) {
      const AudioCtx = window.AudioContext || window.webkitAudioContext;
      if (AudioCtx) {
        this.ctx = new AudioCtx();
        this.masterGain = this.ctx.createGain();
        this.sfxGain = this.ctx.createGain();
        this.ambientGain = this.ctx.createGain();

        this.sfxGain.connect(this.masterGain);
        this.ambientGain.connect(this.masterGain);
        this.masterGain.connect(this.ctx.destination);

        this.updateGains();
      }
    }
    if (this.ctx && this.ctx.state === 'suspended') {
      this.ctx.resume();
    }
  }

  updateGains() {
    if (!this.masterGain || !this.sfxGain || !this.ambientGain) return;
    const effectiveMaster = this.isMuted ? 0.0 : this.masterVolume;
    this.masterGain.gain.setValueAtTime(effectiveMaster, this.ctx.currentTime);
    this.sfxGain.gain.setValueAtTime(this.sfxVolume, this.ctx.currentTime);
    this.ambientGain.gain.setValueAtTime(this.ambientVolume, this.ctx.currentTime);
  }

  async preloadAll() {
    this.ensureContext();
    if (!this.ctx) return;

    for (const name of this.assetList) {
      try {
        const response = await fetch(`/audio/${name}.wav`);
        if (response.ok) {
          const arrayBuffer = await response.arrayBuffer();
          const audioBuffer = await this.ctx.decodeAudioData(arrayBuffer);
          this.buffers.set(name, audioBuffer);
        }
      } catch (e) {
        console.warn(`Could not preload audio asset: ${name}.wav`, e);
      }
    }
  }

  playSfx(name, pitchVariation = 0.05) {
    if (this.isMuted) return;
    this.ensureContext();
    if (!this.ctx) return;

    const buffer = this.buffers.get(name);
    if (!buffer) {
      // If buffer not preloaded yet, fetch on-the-fly
      fetch(`/audio/${name}.wav`)
        .then((r) => r.arrayBuffer())
        .then((ab) => this.ctx.decodeAudioData(ab))
        .then((decoded) => {
          this.buffers.set(name, decoded);
          this.playSfx(name, pitchVariation);
        })
        .catch(() => {});
      return;
    }

    try {
      const source = this.ctx.createBufferSource();
      source.buffer = buffer;

      // Slight natural pitch jitter
      if (pitchVariation > 0) {
        source.playbackRate.value = 1.0 + (Math.random() * 2 - 1) * pitchVariation;
      }

      source.connect(this.sfxGain);
      source.start();
    } catch (e) {
      console.warn(`Error playing SFX ${name}:`, e);
    }
  }

  startAmbient() {
    if (this.isAmbientPlaying || this.isMuted) return;
    this.ensureContext();
    if (!this.ctx) return;

    const buffer = this.buffers.get('ambient_breeze');
    if (!buffer) {
      fetch('/audio/ambient_breeze.wav')
        .then((r) => r.arrayBuffer())
        .then((ab) => this.ctx.decodeAudioData(ab))
        .then((decoded) => {
          this.buffers.set('ambient_breeze', decoded);
          this.startAmbient();
        })
        .catch(() => {});
      return;
    }

    try {
      this.ambientSource = this.ctx.createBufferSource();
      this.ambientSource.buffer = buffer;
      this.ambientSource.loop = true;
      this.ambientSource.connect(this.ambientGain);
      this.ambientSource.start();
      this.isAmbientPlaying = true;
    } catch (e) {
      console.warn('Error starting ambient audio:', e);
    }
  }

  stopAmbient() {
    if (this.ambientSource && this.isAmbientPlaying) {
      try {
        this.ambientSource.stop();
        this.ambientSource.disconnect();
      } catch (_) {}
      this.ambientSource = null;
      this.isAmbientPlaying = false;
    }
  }

  setMasterVolume(val) {
    this.masterVolume = Math.max(0, Math.min(1, val));
    this.updateGains();
    try {
      localStorage.setItem('cogniverse_master_volume', this.masterVolume.toString());
    } catch (_) {}
  }

  setSfxVolume(val) {
    this.sfxVolume = Math.max(0, Math.min(1, val));
    this.updateGains();
    try {
      localStorage.setItem('cogniverse_sfx_volume', this.sfxVolume.toString());
    } catch (_) {}
  }

  setAmbientVolume(val) {
    this.ambientVolume = Math.max(0, Math.min(1, val));
    this.updateGains();
    try {
      localStorage.setItem('cogniverse_ambient_volume', this.ambientVolume.toString());
    } catch (_) {}
  }

  toggleMute() {
    this.isMuted = !this.isMuted;
    this.updateGains();
    if (this.isMuted) {
      this.stopAmbient();
    } else {
      this.startAmbient();
    }
    try {
      localStorage.setItem('cogniverse_audio_muted', this.isMuted.toString());
    } catch (_) {}
    return this.isMuted;
  }
}

// Global instance
window.soundManager = new SoundManager();

// Preload assets upon first user interaction
['click', 'keydown', 'touchstart'].forEach((evt) => {
  window.addEventListener(
    evt,
    () => {
      if (window.soundManager) {
        window.soundManager.ensureContext();
        window.soundManager.preloadAll();
      }
    },
    { once: true }
  );
});
