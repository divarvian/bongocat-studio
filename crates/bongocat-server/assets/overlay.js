import { DeskRenderer } from '/assets/flat_desk.js';
import { SlantedDeskRenderer } from '/assets/slanted_desk.js';

const rootEl = document.getElementById('root');
const catWrap = document.getElementById('cat-wrap');
const deskWrap = document.getElementById('desk-wrap');
const hudStrip = document.getElementById('hud-strip');
const kpsBadge = document.getElementById('kps-badge');
const kpsValue = document.getElementById('kps-value');
const kpsFill = document.getElementById('kps-fill');
const keyHud = document.getElementById('key-hud');
const classicFrames = [...document.querySelectorAll('.cat-frame')];

const query = new URLSearchParams(window.location.search);

let liveConfig = {
    mode: 'slanted',
    pointer: 'trackpad',
    desk_theme: 'white',
    eye_style: 'classic',
    eye_dir: 'screen',
    ptr_dir: 'natural',
    right_paw: 'auto',
    privacy_mode: 'full',
    eyes_enabled: true,
    show_hud: true,
    show_kps: true,
    bounce: true,
    key_hold_ms: 140,
    idle_raise_sec: 4,
    sleep_sec: 60,
    kps_happy_threshold: 5,
};

let privacyPaused = false;
let renderer = null;
let lastRendererKey = '';
let sleepTimer = 0;
let classicTimer = 0;
let classicNextHand = 1;
let bounceTimer = 0;
let bounceDir = 1;
const hitTimestamps = [];

function mergedConfig() {
    const cfg = { ...liveConfig };
    if (query.has('mode')) cfg.mode = query.get('mode');
    if (query.has('pointer')) cfg.pointer = query.get('pointer');
    if (query.has('theme')) cfg.desk_theme = query.get('theme');
    if (query.has('eyes')) cfg.eyes_enabled = query.get('eyes') !== '0';
    if (query.has('kps')) cfg.show_kps = query.get('kps') !== '0';
    if (query.has('hud')) cfg.show_hud = query.get('hud') !== '0';
    if (query.has('bounce')) cfg.bounce = query.get('bounce') !== '0';
    if (query.has('keys')) cfg.privacy_mode = query.get('keys');
    return cfg;
}

function renderHudPills(pills) {
    if (!mergedConfig().show_hud || privacyPaused) {
        keyHud.replaceChildren();
        return;
    }
    const nodes = pills.map((p) => {
        const el = document.createElement('span');
        el.className = `hud-pill kind-${p.kind}`;
        el.textContent = p.text;
        return el;
    });
    keyHud.replaceChildren(...nodes);
}

function setClassicFrame(frameIndex) {
    for (let i = 0; i < classicFrames.length; i++) {
        classicFrames[i].classList.toggle('active', i === frameIndex);
    }
}

function applyConfig(newCfg) {
    liveConfig = { ...liveConfig, ...newCfg };
    const cfg = mergedConfig();

    rootEl.classList.toggle('desk-dark', cfg.desk_theme === 'dark');
    catWrap.classList.toggle('no-bounce', !cfg.bounce);
    deskWrap.classList.toggle('no-bounce', !cfg.bounce);

    const isClassic = cfg.mode === 'classic';
    catWrap.classList.toggle('hidden', !isClassic);
    deskWrap.classList.toggle('hidden', isClassic);
    deskWrap.classList.toggle('slanted-ratio', cfg.mode === 'slanted');

    kpsBadge.classList.toggle('hidden', !cfg.show_kps);
    keyHud.classList.toggle('hidden', !cfg.show_hud || isClassic);
    hudStrip.classList.toggle('hidden', !cfg.show_kps && (!cfg.show_hud || isClassic));

    const rendererKey = JSON.stringify([
        cfg.mode,
        cfg.pointer,
        cfg.desk_theme,
        cfg.eye_style,
        cfg.eye_dir,
        cfg.ptr_dir,
        cfg.right_paw,
        cfg.privacy_mode,
        cfg.eyes_enabled,
        cfg.key_hold_ms,
        cfg.idle_raise_sec,
    ]);

    if (rendererKey !== lastRendererKey) {
        lastRendererKey = rendererKey;
        if (renderer && typeof renderer.destroy === 'function') {
            renderer.destroy();
        }
        renderer = null;

        if (!isClassic) {
            const opts = {
                layout: cfg.mode === 'slanted'
                    ? 'slanted'
                    : (cfg.pointer === 'mouse' ? 'side-mouse' : 'laptop'),
                pointer: cfg.pointer,
                pov: 'cat',
                deskTheme: cfg.desk_theme,
                eyeStyle: cfg.eye_style,
                eyeDir: cfg.eye_dir,
                ptrDir: cfg.ptr_dir,
                rightPaw: cfg.right_paw,
                keys: cfg.privacy_mode,
                eyes: cfg.eyes_enabled,
                duration: cfg.key_hold_ms,
                idleRaiseSec: cfg.idle_raise_sec,
            };
            const RendererClass = cfg.mode === 'slanted' ? SlantedDeskRenderer : DeskRenderer;
            renderer = new RendererClass(deskWrap, opts, { onHudChange: renderHudPills });
        } else {
            deskWrap.replaceChildren();
            setClassicFrame(0);
        }
    }

    resetSleepTimer();
}

function triggerBounce() {
    const cfg = mergedConfig();
    if (!cfg.bounce) return;
    bounceDir = -bounceDir;
    document.documentElement.style.setProperty('--bounce-scale-x', '1.035');
    document.documentElement.style.setProperty('--bounce-scale-y', '0.955');
    document.documentElement.style.setProperty('--bounce-y', '3.5px');
    document.documentElement.style.setProperty('--bounce-rot', `${(bounceDir * 1.1).toFixed(1)}deg`);
    clearTimeout(bounceTimer);
    bounceTimer = setTimeout(() => {
        document.documentElement.style.setProperty('--bounce-scale-x', '1');
        document.documentElement.style.setProperty('--bounce-scale-y', '1');
        document.documentElement.style.setProperty('--bounce-y', '0px');
        document.documentElement.style.setProperty('--bounce-rot', '0deg');
    }, 90);
}

function recordHit() {
    const now = performance.now();
    hitTimestamps.push(now);
    refreshKps();
    triggerBounce();

    const cfg = mergedConfig();
    if (cfg.mode === 'classic') {
        const fast = hitTimestamps.length >= cfg.kps_happy_threshold;
        if (fast) {
            setClassicFrame(3);
        } else {
            setClassicFrame(classicNextHand);
            classicNextHand = classicNextHand === 1 ? 2 : 1;
        }
        clearTimeout(classicTimer);
        classicTimer = setTimeout(() => setClassicFrame(0), cfg.key_hold_ms);
    }
}

function refreshKps() {
    const now = performance.now();
    const cutoff = now - 1000;
    while (hitTimestamps.length && hitTimestamps[0] < cutoff) {
        hitTimestamps.shift();
    }
    const kps = hitTimestamps.length;
    kpsValue.textContent = privacyPaused ? 'PAUSED' : `${kps} KPS`;
    const pct = Math.min(100, Math.round((kps / 12) * 100));
    kpsFill.style.width = `${pct}%`;

    const cfg = mergedConfig();
    if (renderer && typeof renderer.setFastTyping === 'function') {
        renderer.setFastTyping(kps >= cfg.kps_happy_threshold);
    }
}

function wakeUp() {
    if (renderer && typeof renderer.setSleeping === 'function') {
        renderer.setSleeping(false);
    }
    resetSleepTimer();
}

function resetSleepTimer() {
    clearTimeout(sleepTimer);
    const sec = mergedConfig().sleep_sec;
    if (!sec || sec <= 0) return;
    sleepTimer = setTimeout(() => {
        if (renderer && typeof renderer.setSleeping === 'function') {
            renderer.setSleeping(true);
        } else {
            setClassicFrame(4);
        }
    }, sec * 1000);
}

function handleMessage(msg) {
    switch (msg.t) {
        case 'config':
            applyConfig(msg.config);
            break;
        case 'privacy':
            privacyPaused = Boolean(msg.paused);
            kpsBadge.classList.toggle('privacy', privacyPaused);
            if (privacyPaused) keyHud.replaceChildren();
            refreshKps();
            break;
        case 'k':
            if (privacyPaused) break;
            wakeUp();
            if (msg.d) recordHit();
            if (renderer) renderer.handleKey(msg.c, Boolean(msg.d));
            break;
        case 'm':
            wakeUp();
            if (renderer) renderer.handleMove(msg.x, msg.y, Boolean(msg.tc), msg.f || 0);
            break;
        case 'b':
            wakeUp();
            if (msg.d) recordHit();
            if (renderer) renderer.handleButton(msg.b, Boolean(msg.d));
            break;
        case 'w':
            wakeUp();
            if (renderer) renderer.handleWheel(msg.d);
            break;
        case 'c':
            if (renderer && typeof renderer.handleCursor === 'function') {
                renderer.handleCursor(msg.x, msg.y);
            }
            break;
    }
}

function connectSse() {
    const es = new EventSource('/events');
    es.onopen = () => {
        kpsBadge.classList.remove('offline');
    };
    es.onerror = () => {
        kpsBadge.classList.add('offline');
    };
    es.onmessage = (ev) => {
        try {
            handleMessage(JSON.parse(ev.data));
        } catch {
            // Ignore malformed keepalive frames
        }
    };
}

applyConfig(liveConfig);
setInterval(refreshKps, 120);
connectSse();
