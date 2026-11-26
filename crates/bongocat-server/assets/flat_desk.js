// Anatomically-accurate & ergonomically human Streamer Desk SVG Renderer:
// - Supports Cat POV (facing cat: Space & Trackpad closest to cat, anatomically
//   mirrored hands!) and Viewer POV (readable left-to-right QWERTY)
// - 3 Desk Layouts: Laptop Trackpad (centered!), Side Trackpad, Side Mouse
// - True 10-finger human touch-typing hand split + Smart Space thumb alternation
// - Smoothly interpolated paws (home-row standby -> keycap tap -> relaxed beans)
// - Full click differentiation: Left (accent), Right (coral + 2-finger), Middle
//   (violet), Side mouse buttons, and 2-finger/wheel scroll indicators

const SVG_NS = 'http://www.w3.org/2000/svg';

// Standard 10-finger human touch-typing split (each row sums to 15.0u).
// hand: 'L' = left hand, 'R' = right hand, 'S' = smart thumb (Spacebar).
const KEY_ROWS = [
    [
        {code: 1, label: 'Esc', hud: 'Esc', w: 1, hand: 'L'},
        {code: 2, label: '1', hud: '1', w: 1, hand: 'L'},
        {code: 3, label: '2', hud: '2', w: 1, hand: 'L'},
        {code: 4, label: '3', hud: '3', w: 1, hand: 'L'},
        {code: 5, label: '4', hud: '4', w: 1, hand: 'L'},
        {code: 6, label: '5', hud: '5', w: 1, hand: 'L'},
        {code: 7, label: '6', hud: '6', w: 1, hand: 'R'},
        {code: 8, label: '7', hud: '7', w: 1, hand: 'R'},
        {code: 9, label: '8', hud: '8', w: 1, hand: 'R'},
        {code: 10, label: '9', hud: '9', w: 1, hand: 'R'},
        {code: 11, label: '0', hud: '0', w: 1, hand: 'R'},
        {code: 12, label: '-', hud: '-', w: 1, hand: 'R'},
        {code: 13, label: '=', hud: '=', w: 1, hand: 'R'},
        {code: 14, label: '⌫', hud: 'Bksp', w: 2, hand: 'R'},
    ],
    [
        {code: 15, label: 'Tab', hud: 'Tab', w: 1.5, hand: 'L'},
        {code: 16, label: 'Q', hud: 'Q', w: 1, hand: 'L'},
        {code: 17, label: 'W', hud: 'W', w: 1, hand: 'L'},
        {code: 18, label: 'E', hud: 'E', w: 1, hand: 'L'},
        {code: 19, label: 'R', hud: 'R', w: 1, hand: 'L'},
        {code: 20, label: 'T', hud: 'T', w: 1, hand: 'L'},
        {code: 21, label: 'Y', hud: 'Y', w: 1, hand: 'R'},
        {code: 22, label: 'U', hud: 'U', w: 1, hand: 'R'},
        {code: 23, label: 'I', hud: 'I', w: 1, hand: 'R'},
        {code: 24, label: 'O', hud: 'O', w: 1, hand: 'R'},
        {code: 25, label: 'P', hud: 'P', w: 1, hand: 'R'},
        {code: 26, label: '[', hud: '[', w: 1, hand: 'R'},
        {code: 27, label: ']', hud: ']', w: 1, hand: 'R'},
        {code: 43, label: '\\', hud: '\\', w: 1.5, hand: 'R'},
    ],
    [
        {code: 58, label: 'Caps', hud: 'Caps', w: 1.75, hand: 'L'},
        {code: 30, label: 'A', hud: 'A', w: 1, hand: 'L'},
        {code: 31, label: 'S', hud: 'S', w: 1, hand: 'L'},
        {code: 32, label: 'D', hud: 'D', w: 1, hand: 'L'},
        {code: 33, label: 'F', hud: 'F', w: 1, hand: 'L'},
        {code: 34, label: 'G', hud: 'G', w: 1, hand: 'L'},
        {code: 35, label: 'H', hud: 'H', w: 1, hand: 'R'},
        {code: 36, label: 'J', hud: 'J', w: 1, hand: 'R'},
        {code: 37, label: 'K', hud: 'K', w: 1, hand: 'R'},
        {code: 38, label: 'L', hud: 'L', w: 1, hand: 'R'},
        {code: 39, label: ';', hud: ';', w: 1, hand: 'R'},
        {code: 40, label: "'", hud: "'", w: 1, hand: 'R'},
        {code: 28, label: '↵', hud: 'Enter', w: 2.25, hand: 'R'},
    ],
    [
        {code: 42, label: 'Shift', hud: 'Shift', w: 2.25, hand: 'L'},
        {code: 44, label: 'Z', hud: 'Z', w: 1, hand: 'L'},
        {code: 45, label: 'X', hud: 'X', w: 1, hand: 'L'},
        {code: 46, label: 'C', hud: 'C', w: 1, hand: 'L'},
        {code: 47, label: 'V', hud: 'V', w: 1, hand: 'L'},
        {code: 48, label: 'B', hud: 'B', w: 1, hand: 'L'},
        {code: 49, label: 'N', hud: 'N', w: 1, hand: 'R'},
        {code: 50, label: 'M', hud: 'M', w: 1, hand: 'R'},
        {code: 51, label: ',', hud: ',', w: 1, hand: 'R'},
        {code: 52, label: '.', hud: '.', w: 1, hand: 'R'},
        {code: 53, label: '/', hud: '/', w: 1, hand: 'R'},
        {code: 103, label: '↑', hud: '↑', w: 1, hand: 'R'},
        {code: 54, label: 'Shift', hud: 'R-Shift', w: 1.75, hand: 'R'},
    ],
    [
        {code: 29, label: 'Ctrl', hud: 'Ctrl', w: 1.5, hand: 'L'},
        {code: 125, label: '⌘', hud: 'Super', w: 1.25, hand: 'L'},
        {code: 56, label: 'Alt', hud: 'Alt', w: 1.25, hand: 'L'},
        {code: 57, label: 'Space', hud: 'Space', w: 6.0, hand: 'S'},
        {code: 100, label: 'Alt', hud: 'R-Alt', w: 1.0, hand: 'R'},
        {code: 105, label: '←', hud: '←', w: 1.0, hand: 'R'},
        {code: 108, label: '↓', hud: '↓', w: 1.0, hand: 'R'},
        {code: 106, label: '→', hud: '→', w: 1.0, hand: 'R'},
        {code: 97, label: 'Ctrl', hud: 'R-Ctrl', w: 1.0, hand: 'R'},
    ],
];

const KEY_ALIASES = new Map([
    [41, 1],    // KEY_GRAVE -> Esc
    [111, 14],  // KEY_DELETE -> Backspace
    [126, 125], // KEY_RIGHTMETA -> Super
    [96, 28],   // KEY_KPENTER -> Enter
    [82, 11],
    [79, 2], [80, 3], [81, 4], [75, 5], [76, 6], [77, 7], [71, 8], [72, 9], [73, 10],
]);

const GAMING_KEYS = new Set([
    1, 2, 3, 4, 5, 6, 15, 16, 17, 18, 19, 30, 31, 32, 33,
    42, 54, 29, 97, 56, 100, 57, 103, 105, 106, 108,
]);

function svgEl(tag, attrs = {}, text = '') {
    const el = document.createElementNS(SVG_NS, tag);
    for (const [k, v] of Object.entries(attrs))
        el.setAttribute(k, String(v));
    if (text)
        el.textContent = text;
    return el;
}

// Builds a soft curved cartoon cat paw reaching from shoulder (sx, sy) to (tx, ty).
function buildPawPath(sx, sy, tx, ty, screenSide, pressed) {
    const dx = tx - sx;
    const dy = ty - sy;
    const dist = Math.hypot(dx, dy) || 1;
    const ux = dx / dist;
    const uy = dy / dist;
    const nx = -uy;
    const ny = ux;

    const baseHalf = 21;
    const tipHalf = pressed ? 16.5 : 15.5;

    // Gentle outward cartoon elbow curve
    const bend = screenSide === 'L' ? -15 : 15;
    const mx = (sx + tx) * 0.5 + nx * bend * 0.3;
    const my = (sy + ty) * 0.5 - 10;

    const s1x = sx + nx * baseHalf;
    const s1y = sy + ny * baseHalf;
    const s2x = sx - nx * baseHalf;
    const s2y = sy - ny * baseHalf;

    const t1x = tx + nx * tipHalf;
    const t1y = ty + ny * tipHalf;
    const t2x = tx - nx * tipHalf;
    const t2y = ty - ny * tipHalf;

    const c1x = mx + nx * ((baseHalf + tipHalf) * 0.5);
    const c1y = my + ny * ((baseHalf + tipHalf) * 0.5);
    const c2x = mx - nx * ((baseHalf + tipHalf) * 0.5);
    const c2y = my - ny * ((baseHalf + tipHalf) * 0.5);

    const tipBulgeX = tx + ux * (tipHalf * 1.15);
    const tipBulgeY = ty + uy * (tipHalf * 1.15);

    return (
        `M ${s1x.toFixed(1)} ${s1y.toFixed(1)} ` +
        `Q ${c1x.toFixed(1)} ${c1y.toFixed(1)} ${t1x.toFixed(1)} ${t1y.toFixed(1)} ` +
        `Q ${tipBulgeX.toFixed(1)} ${tipBulgeY.toFixed(1)} ${t2x.toFixed(1)} ${t2y.toFixed(1)} ` +
        `Q ${c2x.toFixed(1)} ${c2y.toFixed(1)} ${s2x.toFixed(1)} ${s2y.toFixed(1)} Z`
    );
}

export class DeskRenderer {
    constructor(container, opts, callbacks = {}) {
        this._container = container;
        this._opts = opts;
        this._onHudChange = callbacks.onHudChange || null;
        this._isCatPov = opts.pov !== 'viewer';
        this._layout = opts.layout || 'laptop'; // laptop | side-trackpad | side-mouse

        // Anatomical shoulder mapping:
        // In Cat POV (cat faces camera), cat's LEFT hand is on screen-RIGHT,
        // and cat's RIGHT hand is on screen-LEFT!
        this._shoulder = this._isCatPov
            ? {L: {x: 432, y: 204, screenSide: 'R'}, R: {x: 208, y: 204, screenSide: 'L'}}
            : {L: {x: 208, y: 204, screenSide: 'L'}, R: {x: 432, y: 204, screenSide: 'R'}};

        // Key tracking
        this._pressedRawToVisual = new Map();
        this._visualKeyRefCount = new Map();
        this._keyInfo = new Map();
        this._lastKeyHand = 'L';
        this._idleRaiseMs = Math.max(1000, (opts.idleRaiseSec ?? 4) * 1000);
        this._lastLeftAt = -Infinity;
        this._lastRightKeyAt = -Infinity;

        // Pointer tracking
        this._ptrX = 0.5;
        this._ptrY = 0.5;
        this._ptrFingers = 0;
        this._lastPointerAt = -Infinity;
        this._buttonsDown = new Set(); // 1=L, 2=R, 3=M, 4=Back, 5=Fwd
        this._wheelBadge = '';
        this._wheelTimer = 0;

        // Animation states
        this._sleeping = false;
        this._fastTyping = false;
        this._leftActive = null;   // {x, y, pressed}
        this._rightActive = null;  // {x, y, pressed}
        this._leftReleaseTimer = 0;
        this._rightReleaseTimer = 0;
        this._idleRelaxTimer = 0;
        this._blinkTimer = 0;

        // Smooth interpolation coordinates
        this._curPtrX = 0.5;
        this._curPtrY = 0.5;
        this._curEyeX = 0;
        this._curEyeY = 0;
        this._eyeTargetX = 0;
        this._eyeTargetY = 0;
        this._rafId = 0;

        this._buildSvg();

        // Initialize smoothed paw coordinates at their raised idle spots
        const restL = this._raisedPosition('L');
        const restR = this._raisedPosition('R');
        this._curLX = restL.x;
        this._curLY = restL.y;
        this._curRX = restR.x;
        this._curRY = restR.y;

        this._scheduleBlink();
        this._renderFrame();
    }

    _buildSvg() {
        const svg = svgEl('svg', {
            id: 'desk-svg',
            viewBox: '0 0 640 436',
            width: '100%',
            height: '100%',
            'aria-label': 'Bongo Cat Desk Overlay',
        });

        const isDarkDesk = this._opts.deskTheme === 'dark';
        const isPastelDesk = this._opts.deskTheme === 'pastel';

        const defs = svgEl('defs');
        const deskGrad = svgEl('linearGradient', {id: 'desk-grad', x1: '0', y1: '0', x2: '0', y2: '1'});
        deskGrad.append(
            svgEl('stop', {offset: '0%', 'stop-color': isDarkDesk ? '#273043' : (isPastelDesk ? '#fce7f3' : '#f8fafc')}),
            svgEl('stop', {offset: '100%', 'stop-color': isDarkDesk ? '#1b2230' : (isPastelDesk ? '#f5d0fe' : '#e2e8f0')})
        );

        const chassisGrad = svgEl('linearGradient', {id: 'chassis-grad', x1: '0', y1: '0', x2: '0', y2: '1'});
        chassisGrad.append(
            svgEl('stop', {offset: '0%', 'stop-color': isDarkDesk ? '#334155' : (isPastelDesk ? '#fdf4ff' : '#e2e8f0')}),
            svgEl('stop', {offset: '100%', 'stop-color': isDarkDesk ? '#1e293b' : (isPastelDesk ? '#f3e8ff' : '#cbd5e1')})
        );

        const padGrad = svgEl('linearGradient', {id: 'pad-grad', x1: '0', y1: '0', x2: '1', y2: '1'});
        padGrad.append(
            svgEl('stop', {offset: '0%', 'stop-color': isDarkDesk ? '#3b4a62' : '#eef2f7'}),
            svgEl('stop', {offset: '100%', 'stop-color': isDarkDesk ? '#232f42' : '#d5dde8'})
        );

        defs.append(deskGrad, chassisGrad, padGrad);
        svg.append(defs);

        // 1. Cat Body & Head behind desk
        svg.append(this._buildCatHead());

        // 2. Desk Tabletop
        const deskGroup = svgEl('g', {id: 'desk-surface'});
        deskGroup.append(
            svgEl('rect', {
                x: 16, y: 202, width: 608, height: 222, rx: 20,
                fill: 'url(#desk-grad)',
                stroke: '#111827',
                'stroke-width': '4.5',
            }),
            svgEl('path', {
                d: 'M 28 412 Q 320 420 612 412',
                fill: 'none',
                stroke: isDarkDesk ? 'rgba(255,255,255,0.08)' : 'rgba(15,23,42,0.08)',
                'stroke-width': '3',
                'stroke-linecap': 'round',
            })
        );
        svg.append(deskGroup);

        // 3. Layout-specific Laptop / Keyboard + Trackpad / Mouse
        if (this._layout === 'laptop') {
            svg.append(this._buildLaptopDeck(isDarkDesk));
        } else if (this._layout === 'side-mouse') {
            svg.append(this._buildSideDeck(isDarkDesk, 'mouse'));
        } else {
            svg.append(this._buildSideDeck(isDarkDesk, 'trackpad'));
        }

        // 4. Dynamic Animated Paws (top layer!)
        svg.append(this._buildPawsLayer());

        this._svg = svg;
        this._container.replaceChildren(svg);
    }

    _buildCatHead() {
        const g = svgEl('g', {id: 'desk-cat-group'});

        const bodyPath = svgEl('path', {
            d: 'M 158 214 ' +
               'C 154 156, 178 104, 210 86 ' +
               'C 212 52, 228 28, 244 26 ' +
               'C 262 30, 278 48, 288 66 ' +
               'C 308 60, 332 60, 352 66 ' +
               'C 362 48, 378 30, 396 26 ' +
               'C 412 28, 428 52, 430 86 ' +
               'C 462 104, 486 156, 482 214 Z',
            fill: '#ffffff',
            stroke: '#111827',
            'stroke-width': '4.6',
            'stroke-linecap': 'round',
            'stroke-linejoin': 'round',
        });

        const earLeft = svgEl('path', {
            d: 'M 227 78 C 227 59, 236 44, 245 42 C 256 47, 265 58, 270 69 Z',
            fill: '#fbcfe8',
            opacity: '0.78',
        });
        const earRight = svgEl('path', {
            d: 'M 413 78 C 413 59, 404 44, 395 42 C 384 47, 375 58, 370 69 Z',
            fill: '#fbcfe8',
            opacity: '0.78',
        });

        const blushL = svgEl('ellipse', {cx: 224, cy: 152, rx: 16, ry: 8.5, fill: '#f9a8d4', opacity: '0.55'});
        const blushR = svgEl('ellipse', {cx: 416, cy: 152, rx: 16, ry: 8.5, fill: '#f9a8d4', opacity: '0.55'});

        this._eyesAwake = svgEl('g', {id: 'desk-eyes-awake'});
        this._eyeLeftGroup = this._createEye(262, 128);
        this._eyeRightGroup = this._createEye(378, 128);
        this._eyesAwake.append(this._eyeLeftGroup, this._eyeRightGroup);

        this._eyesSleep = svgEl('g', {id: 'desk-eyes-sleep', style: 'display:none'});
        this._eyesSleep.append(
            svgEl('path', {
                d: 'M 248 130 Q 262 142 276 130',
                fill: 'none', stroke: '#111827', 'stroke-width': '4.2', 'stroke-linecap': 'round',
            }),
            svgEl('path', {
                d: 'M 364 130 Q 378 142 392 130',
                fill: 'none', stroke: '#111827', 'stroke-width': '4.2', 'stroke-linecap': 'round',
            }),
            svgEl('text', {
                x: 448, y: 74, fill: '#38bdf8', 'font-size': '25', 'font-weight': '800',
                class: 'zzz-float',
            }, 'Z'),
            svgEl('text', {
                x: 472, y: 54, fill: '#a78bfa', 'font-size': '18', 'font-weight': '800',
                class: 'zzz-float zzz-2',
            }, 'z')
        );

        this._mouthCalm = svgEl('path', {
            d: 'M 301 146 C 304 157, 317 155, 320 146 C 323 155, 336 157, 339 146',
            fill: 'none',
            stroke: '#111827',
            'stroke-width': '4.1',
            'stroke-linecap': 'round',
            'stroke-linejoin': 'round',
        });
        this._mouthHappy = svgEl('path', {
            d: 'M 311 151 Q 320 164 329 151 Z',
            fill: '#fb7185',
            stroke: '#111827',
            'stroke-width': '2.6',
            'stroke-linejoin': 'round',
            style: 'display:none',
        });

        g.append(
            bodyPath, earLeft, earRight, blushL, blushR,
            this._eyesAwake, this._eyesSleep, this._mouthCalm, this._mouthHappy
        );
        return g;
    }

    _createEye(cx, cy) {
        const outer = svgEl('g', {transform: `translate(${cx}, ${cy})`});
        const blinkWrap = svgEl('g', {class: 'eye-blink'});
        const pupil = svgEl('g', {class: 'eye-pupil'});
        pupil.append(
            svgEl('ellipse', {cx: 0, cy: 0, rx: 10.5, ry: 12.2, fill: '#111827'}),
            svgEl('circle', {cx: -3.2, cy: -4.0, r: 3.3, fill: '#ffffff'}),
            svgEl('circle', {cx: 3.2, cy: 3.4, r: 1.5, fill: '#ffffff', opacity: '0.85'})
        );
        blinkWrap.append(pupil);
        outer.append(blinkWrap);
        outer._pupil = pupil;
        outer._blinkWrap = blinkWrap;
        return outer;
    }

    _buildLaptopDeck(isDarkDesk) {
        const g = svgEl('g', {id: 'desk-laptop'});

        // Sleek aluminum laptop body centered on the desk
        const bx = 76;
        const by = 212;
        const bw = 488;
        const bh = 202;

        g.append(
            svgEl('rect', {
                x: bx, y: by, width: bw, height: bh, rx: 14,
                fill: 'url(#chassis-grad)',
                stroke: '#111827',
                'stroke-width': '3.6',
            }),
            // Subtle hinge bar at the monitor-facing edge of the laptop
            svgEl('rect', {
                x: bx + 68,
                y: this._isCatPov ? (by + bh - 6) : (by - 1),
                width: bw - 136,
                height: 7,
                rx: 3,
                fill: '#1e293b',
            })
        );

        if (this._isCatPov) {
            // Anatomically facing cat:
            // Trackpad is CLOSEST to cat's belly (top of laptop base),
            // Keyboard is BELOW trackpad (facing cat: Space at top, 1234 at bottom)!
            g.append(this._buildTrackpad(238, 220, 164, 62, isDarkDesk, true));
            g.append(this._buildKeyboard(96, 288, 448, 114, isDarkDesk));
        } else {
            // Viewer POV:
            // Keyboard at top (1234 top -> Space bottom), Trackpad centered below!
            g.append(this._buildKeyboard(96, 222, 448, 114, isDarkDesk));
            g.append(this._buildTrackpad(238, 342, 164, 62, isDarkDesk, false));
        }
        return g;
    }

    _buildSideDeck(isDarkDesk, pointerKind) {
        const g = svgEl('g', {id: 'desk-side'});

        // In Cat POV, cat's RIGHT hand is on screen-LEFT -> mouse/trackpad on LEFT, keyboard on RIGHT!
        // In Viewer POV, keyboard on LEFT, mouse/trackpad on RIGHT!
        const ptrOnLeft = this._isCatPov;
        const kbX = ptrOnLeft ? 210 : 34;
        const ptrX = ptrOnLeft ? 34 : 444;
        const y = 238;
        const h = 152;

        g.append(this._buildKeyboard(kbX, y, 396, h, isDarkDesk));
        if (pointerKind === 'mouse')
            g.append(this._buildMousepad(ptrX, y, 162, h, isDarkDesk));
        else
            g.append(this._buildTrackpad(ptrX, y, 162, h, isDarkDesk, this._isCatPov));

        return g;
    }

    _buildKeyboard(kbX, kbY, kbW, kbH, isDarkDesk) {
        const g = svgEl('g', {id: 'desk-keyboard'});

        g.append(
            svgEl('rect', {
                x: kbX, y: kbY, width: kbW, height: kbH, rx: 10,
                fill: isDarkDesk ? '#0f172a' : '#1e293b',
                stroke: '#111827',
                'stroke-width': '3.2',
            })
        );

        const padX = 6.5;
        const padY = 6.5;
        const gap = 2.2;
        const unitW = (kbW - padX * 2 - gap * 14) / 15;
        const rowH = (kbH - padY * 2 - gap * 4) / 5;
        const hideLabels = this._opts.keys === 'zones';

        // In Cat POV, rows go Space -> 1234 (top to bottom) and columns mirror
        // left-to-right so A/S/D/F sit directly under cat's LEFT hand (screen-right)
        // and J/K/L/Enter sit directly under cat's RIGHT hand (screen-left)!
        const rows = this._isCatPov ? [...KEY_ROWS].reverse() : KEY_ROWS;

        for (let r = 0; r < rows.length; r++) {
            const row = this._isCatPov ? [...rows[r]].reverse() : rows[r];
            let cursorX = kbX + padX;
            const y = kbY + padY + r * (rowH + gap);

            for (const key of row) {
                const w = key.w * unitW + (key.w - 1) * gap;
                const h = rowH;
                const cx = cursorX + w / 2;
                const cy = y + h / 2;

                const keyGroup = svgEl('g', {
                    class: 'kb-key',
                    'data-code': key.code,
                });

                keyGroup.append(svgEl('rect', {
                    x: cursorX.toFixed(2),
                    y: y.toFixed(2),
                    width: w.toFixed(2),
                    height: h.toFixed(2),
                    rx: '3.4',
                    class: 'kb-cap',
                }));

                if (!hideLabels) {
                    const fontSize = key.label.length > 2 ? 7.0 : (key.label.length === 2 ? 8.0 : 9.2);
                    const textAttrs = {
                        x: cx.toFixed(2),
                        y: cy.toFixed(2),
                        'text-anchor': 'middle',
                        'dominant-baseline': 'central',
                        'font-size': fontSize,
                        class: 'kb-label',
                    };
                    if (this._isCatPov)
                        textAttrs.transform = `rotate(180 ${cx.toFixed(2)} ${cy.toFixed(2)})`;
                    keyGroup.append(svgEl('text', textAttrs, key.label));
                }

                g.append(keyGroup);

                // For Spacebar, also store left-thumb & right-thumb target X!
                const leftThumbX = this._isCatPov ? (cursorX + w * 0.68) : (cursorX + w * 0.32);
                const rightThumbX = this._isCatPov ? (cursorX + w * 0.32) : (cursorX + w * 0.68);

                this._keyInfo.set(key.code, {
                    el: keyGroup,
                    cx,
                    cy,
                    leftThumbX,
                    rightThumbX,
                    hand: key.hand,
                    label: key.label,
                    hud: key.hud,
                });

                cursorX += w + gap;
            }
        }

        return g;
    }

    _buildTrackpad(x, y, w, h, isDarkDesk, buttonsAtTop) {
        const g = svgEl('g', {id: 'desk-trackpad'});
        const btnH = Math.min(22, Math.round(h * 0.3));
        const btnY = buttonsAtTop ? y + 4 : y + h - btnH - 4;
        const padMarginY = 14;

        this._ptrBounds = {
            x: x + 18,
            y: buttonsAtTop ? y + btnH + 8 : y + padMarginY,
            w: w - 36,
            h: Math.max(18, h - btnH - padMarginY - 12),
        };
        this._ptrCenter = {x: x + w / 2, y: y + h / 2};

        g.append(
            svgEl('rect', {
                x, y, width: w, height: h, rx: 11,
                fill: 'url(#pad-grad)',
                stroke: '#111827',
                'stroke-width': '3.2',
            }),
            svgEl('rect', {
                x: x + 4, y: y + 4, width: w - 8, height: h - 8, rx: 8,
                fill: 'none',
                stroke: isDarkDesk ? 'rgba(255,255,255,0.12)' : 'rgba(255,255,255,0.68)',
                'stroke-width': '1.4',
            })
        );

        // Two click zones: left half & right half of screen.
        // In Cat POV: screen-RIGHT half = cat's LEFT click, screen-LEFT half = cat's RIGHT click!
        const halfW = w / 2 - 5;
        const leftHalfRect = svgEl('rect', {
            x: x + 4, y: btnY, width: halfW, height: btnH, rx: 5,
            class: this._isCatPov ? 'ptr-btn ptr-btn-right' : 'ptr-btn ptr-btn-left',
        });
        const rightHalfRect = svgEl('rect', {
            x: x + w / 2 + 1, y: btnY, width: halfW, height: btnH, rx: 5,
            class: this._isCatPov ? 'ptr-btn ptr-btn-left' : 'ptr-btn ptr-btn-right',
        });

        this._ptrLeftBtn = this._isCatPov ? rightHalfRect : leftHalfRect;
        this._ptrRightBtn = this._isCatPov ? leftHalfRect : rightHalfRect;

        // Tiny crisp L / R labels in click zones (rotated 180° in Cat POV to face cat!)
        const labelY = btnY + btnH / 2;
        const lLabelX = this._isCatPov ? (x + w * 0.75) : (x + w * 0.25);
        const rLabelX = this._isCatPov ? (x + w * 0.25) : (x + w * 0.75);
        const labelFill = isDarkDesk ? 'rgba(226,232,240,0.45)' : 'rgba(30,41,59,0.42)';
        const lTextAttrs = {
            x: lLabelX, y: labelY, 'text-anchor': 'middle', 'dominant-baseline': 'central',
            'font-size': '8.5', 'font-weight': '700', fill: labelFill,
        };
        const rTextAttrs = {
            x: rLabelX, y: labelY, 'text-anchor': 'middle', 'dominant-baseline': 'central',
            'font-size': '8.5', 'font-weight': '700', fill: labelFill,
        };
        if (this._isCatPov) {
            lTextAttrs.transform = `rotate(180 ${lLabelX} ${labelY})`;
            rTextAttrs.transform = `rotate(180 ${rLabelX} ${labelY})`;
        }

        const splitLineY = buttonsAtTop ? (btnY + btnH + 1) : (btnY - 1);
        g.append(
            leftHalfRect,
            rightHalfRect,
            svgEl('line', {
                x1: x + 8, y1: splitLineY, x2: x + w - 8, y2: splitLineY,
                stroke: isDarkDesk ? 'rgba(255,255,255,0.16)' : 'rgba(15,23,42,0.16)',
                'stroke-width': '1.3',
            }),
            svgEl('line', {
                x1: x + w / 2, y1: btnY + 2, x2: x + w / 2, y2: btnY + btnH - 2,
                stroke: isDarkDesk ? 'rgba(255,255,255,0.16)' : 'rgba(15,23,42,0.16)',
                'stroke-width': '1.3',
            }),
            svgEl('text', lTextAttrs, 'L'),
            svgEl('text', rTextAttrs, 'R')
        );

        // Glowing touch rings (2nd ring appears during 2-finger right-click / scroll!)
        this._tpRipple1 = svgEl('circle', {
            cx: x + w / 2, cy: y + h / 2, r: 9,
            class: 'tp-ripple',
            opacity: '0',
        });
        this._tpRipple2 = svgEl('circle', {
            cx: x + w / 2 - 14, cy: y + h / 2, r: 8,
            class: 'tp-ripple tp-ripple-2',
            opacity: '0',
        });

        // Scroll indicator chevron
        this._scrollInd = svgEl('text', {
            x: x + w - 13,
            y: y + h / 2 + 5,
            'text-anchor': 'middle',
            'font-size': '14',
            'font-weight': '800',
            fill: '#38bdf8',
            opacity: '0',
        }, '▲');

        g.append(this._tpRipple1, this._tpRipple2, this._scrollInd);
        return g;
    }

    _buildMousepad(x, y, w, h, isDarkDesk) {
        const g = svgEl('g', {id: 'desk-mousepad'});
        this._ptrBounds = {x: x + 32, y: y + 36, w: w - 64, h: h - 72};
        this._ptrCenter = {x: x + w / 2, y: y + h / 2};

        g.append(
            svgEl('rect', {
                x, y, width: w, height: h, rx: 14,
                fill: isDarkDesk ? '#0f172a' : '#334155',
                stroke: '#111827',
                'stroke-width': '3.4',
            }),
            svgEl('rect', {
                x: x + 6, y: y + 6, width: w - 12, height: h - 12, rx: 10,
                fill: 'none',
                stroke: 'rgba(255,255,255,0.12)',
                'stroke-width': '1.4',
                'stroke-dasharray': '5 4',
            })
        );

        // Movable Mouse group (rotated 180° in Cat POV so buttons face away from cat!)
        this._mouseGroup = svgEl('g', {id: 'desk-mouse-device'});
        const mouseInner = svgEl('g', this._isCatPov ? {transform: 'rotate(180)'} : {});

        mouseInner.append(
            svgEl('ellipse', {cx: 2, cy: 5, rx: 22, ry: 29, fill: 'rgba(0,0,0,0.28)'}),
            svgEl('rect', {
                x: -21, y: -29, width: 42, height: 58, rx: 19,
                fill: '#f8fafc',
                stroke: '#111827',
                'stroke-width': '3.2',
            })
        );

        // Inside mouseInner (before rotation): -X is Left Click, +X is Right Click.
        // Rotating 180° in Cat POV automatically puts Left Click on screen-RIGHT
        // (cat's index finger) and Right Click on screen-LEFT (cat's middle finger)!
        this._ptrLeftBtn = svgEl('path', {
            d: 'M -19 -3 L -1 -3 L -1 -27 C -13 -27 -19 -19 -19 -8 Z',
            class: 'ptr-btn ptr-btn-left mouse-btn',
        });
        this._ptrRightBtn = svgEl('path', {
            d: 'M 1 -3 L 19 -3 L 19 -8 C 19 -19 13 -27 1 -27 Z',
            class: 'ptr-btn ptr-btn-right mouse-btn',
        });
        this._mouseSideFwd = svgEl('rect', {
            x: -23.5, y: -12, width: 3.5, height: 9, rx: 1.5,
            class: 'ptr-btn ptr-btn-side',
        });
        this._mouseSideBack = svgEl('rect', {
            x: -23.5, y: 0, width: 3.5, height: 9, rx: 1.5,
            class: 'ptr-btn ptr-btn-side',
        });

        mouseInner.append(
            this._ptrLeftBtn,
            this._ptrRightBtn,
            this._mouseSideFwd,
            this._mouseSideBack,
            svgEl('line', {x1: -20, y1: -3, x2: 20, y2: -3, stroke: '#111827', 'stroke-width': '2.3'}),
            svgEl('line', {x1: 0, y1: -28, x2: 0, y2: -3, stroke: '#111827', 'stroke-width': '2.2'})
        );

        this._mouseWheel = svgEl('rect', {
            x: -3.4, y: -20, width: 6.8, height: 12.5, rx: 3.2,
            fill: '#64748b',
            stroke: '#111827',
            'stroke-width': '1.8',
        });
        mouseInner.append(this._mouseWheel);

        this._mouseGroup.append(mouseInner);
        g.append(this._mouseGroup);
        return g;
    }

    _buildPawsLayer() {
        const g = svgEl('g', {id: 'desk-paws'});

        this._leftShadow = svgEl('ellipse', {
            cx: 0, cy: 0, rx: 15, ry: 6, fill: 'rgba(15, 23, 42, 0.24)', opacity: '0',
        });
        this._leftPawPath = svgEl('path', {
            fill: '#ffffff', stroke: '#111827', 'stroke-width': '4.4',
            'stroke-linecap': 'round', 'stroke-linejoin': 'round',
        });
        this._leftBeans = this._createPawBeans();

        this._rightShadow = svgEl('ellipse', {
            cx: 0, cy: 0, rx: 16, ry: 6, fill: 'rgba(15, 23, 42, 0.24)', opacity: '0',
        });
        this._rightPawPath = svgEl('path', {
            fill: '#ffffff', stroke: '#111827', 'stroke-width': '4.4',
            'stroke-linecap': 'round', 'stroke-linejoin': 'round',
        });
        this._rightBeans = this._createPawBeans();

        g.append(
            this._leftShadow, this._leftPawPath, this._leftBeans,
            this._rightShadow, this._rightPawPath, this._rightBeans
        );
        return g;
    }

    _createPawBeans() {
        const g = svgEl('g', {opacity: '0'});
        g.append(
            svgEl('ellipse', {cx: 0, cy: 2.5, rx: 7, ry: 5.5, fill: '#f9a8d4'}),
            svgEl('circle', {cx: -6.5, cy: -5, r: 3.0, fill: '#f9a8d4'}),
            svgEl('circle', {cx: 0, cy: -7, r: 3.2, fill: '#f9a8d4'}),
            svgEl('circle', {cx: 6.5, cy: -5, r: 3.0, fill: '#f9a8d4'})
        );
        return g;
    }

    _pointerSvgCoords(nx, ny) {
        // Map normalized hardware coordinates (0..1) into SVG coordinates.
        const naturalCatFlip = this._isCatPov && this._opts.ptrDir !== 'screen';
        const vx = naturalCatFlip ? (1 - nx) : nx;
        const vy = naturalCatFlip ? (1 - ny) : ny;
        const b = this._ptrBounds;
        return {
            x: b.x + vx * b.w,
            y: b.y + vy * b.h,
            vx,
            vy,
        };
    }

    _palmRestPosition(hand) {
        if (this._layout === 'laptop') {
            if (hand === 'R')
                return {x: this._isCatPov ? 168 : 472, y: 248, raised: false};
            return {x: this._isCatPov ? 472 : 168, y: 248, raised: false};
        }
        // Side mouse/trackpad: rest gently along front edge of keyboard
        if (hand === 'R')
            return {x: this._isCatPov ? 320 : 300, y: 244, raised: false};
        return {x: this._isCatPov ? 500 : 120, y: 244, raised: false};
    }

    _raisedPosition(hand) {
        const sh = this._shoulder[hand];
        const dx = sh.screenSide === 'L' ? -24 : 24;
        return {x: sh.x + dx, y: sh.y - 6, raised: true, onPointer: false};
    }

    _restPosition(hand) {
        if (hand === 'R') {
            const p = this._pointerSvgCoords(this._curPtrX, this._curPtrY);
            const yShift = this._layout === 'side-mouse' ? (this._isCatPov ? -8 : 8) : 0;
            return {x: p.x, y: p.y + yShift, raised: false, onPointer: true};
        }
        return this._raisedPosition('L');
    }

    _resolveVisualKeyCode(rawCode) {
        const mapped = KEY_ALIASES.get(rawCode) ?? rawCode;
        if (this._opts.keys === 'gaming' && !GAMING_KEYS.has(mapped))
            return {visual: this._zoneForCode(mapped), masked: true};
        if (this._opts.keys === 'zones')
            return {visual: mapped === 57 ? 57 : this._zoneForCode(mapped), masked: true};
        if (this._keyInfo.has(mapped))
            return {visual: mapped, masked: false};
        return {visual: this._zoneForCode(mapped), masked: true};
    }

    _zoneForCode(code) {
        const leftZones = [17, 31, 45];  // W, S, X
        const rightZones = [23, 37, 50]; // I, K, M
        const info = this._keyInfo.get(code);
        const isLeft = info ? info.hand === 'L' : (code % 2 === 0);
        const pool = isLeft ? leftZones : rightZones;
        return pool[code % pool.length];
    }

    _pickHandForKey(info) {
        // Smart Spacebar thumb selection (natural human alternation!)
        if (info.hand === 'S') {
            const rightBusy = this._opts.rightPaw === 'pointer' || this._buttonsDown.size > 0;
            if (rightBusy || this._lastKeyHand === 'R')
                return {hand: 'L', x: info.leftThumbX, y: info.cy};
            return {hand: 'R', x: info.rightThumbX, y: info.cy};
        }

        // Right-hand keys use Right hand unless user locked right hand to mouse
        // (gaming mode) or is actively dragging a click button.
        if (info.hand === 'R') {
            if (this._opts.rightPaw === 'pointer' || this._buttonsDown.size > 0)
                return {hand: 'L', x: info.cx, y: info.cy};
            return {hand: 'R', x: info.cx, y: info.cy};
        }

        return {hand: 'L', x: info.cx, y: info.cy};
    }

    handleKey(rawCode, down) {
        const now = performance.now();
        if (down) {
            const {visual, masked} = this._resolveVisualKeyCode(rawCode);
            this._pressedRawToVisual.set(rawCode, {visual, masked});
            const prevCount = this._visualKeyRefCount.get(visual) || 0;
            this._visualKeyRefCount.set(visual, prevCount + 1);

            const info = this._keyInfo.get(visual);
            if (info) {
                info.el.classList.add('pressed');

                const target = this._pickHandForKey(info);
                if (info.hand !== 'S')
                    this._lastKeyHand = target.hand;

                if (target.hand === 'R') {
                    this._lastRightKeyAt = now;
                    this._rightActive = {x: target.x, y: target.y, pressed: true};
                    clearTimeout(this._rightReleaseTimer);
                    this._rightReleaseTimer = setTimeout(() => {
                        if (!this._hasHeldHand('R')) {
                            this._rightActive = null;
                            this._requestRender();
                        }
                    }, Math.max(this._opts.duration, 100));
                } else {
                    this._lastLeftAt = now;
                    this._leftActive = {x: target.x, y: target.y, pressed: true};
                    clearTimeout(this._leftReleaseTimer);
                    this._leftReleaseTimer = setTimeout(() => {
                        if (!this._hasHeldHand('L')) {
                            this._leftActive = null;
                            this._requestRender();
                        }
                    }, Math.max(this._opts.duration, 100));
                }
                this._scheduleIdleRelax();
            }
            this._emitHud();
            this._requestRender();
        } else {
            const entry = this._pressedRawToVisual.get(rawCode);
            if (entry !== undefined) {
                this._pressedRawToVisual.delete(rawCode);
                const nextCount = Math.max(0, (this._visualKeyRefCount.get(entry.visual) || 1) - 1);
                if (nextCount === 0) {
                    this._visualKeyRefCount.delete(entry.visual);
                    const info = this._keyInfo.get(entry.visual);
                    if (info)
                        info.el.classList.remove('pressed');
                } else {
                    this._visualKeyRefCount.set(entry.visual, nextCount);
                }
            }
            if (!this._hasHeldHand('L') && this._leftActive) {
                clearTimeout(this._leftReleaseTimer);
                this._leftReleaseTimer = setTimeout(() => {
                    if (!this._hasHeldHand('L')) {
                        this._leftActive = null;
                        this._requestRender();
                    }
                }, 40);
            }
            if (!this._hasHeldHand('R') && this._rightActive) {
                clearTimeout(this._rightReleaseTimer);
                this._rightReleaseTimer = setTimeout(() => {
                    if (!this._hasHeldHand('R')) {
                        this._rightActive = null;
                        this._requestRender();
                    }
                }, 40);
            }
            this._emitHud();
            this._requestRender();
        }
    }

    _hasHeldHand(hand) {
        for (const visual of this._visualKeyRefCount.keys()) {
            const info = this._keyInfo.get(visual);
            if (!info)
                continue;
            if (info.hand === hand)
                return true;
            if (info.hand === 'S' && hand === 'L')
                return true;
        }
        return false;
    }

    _scheduleIdleRelax() {
        clearTimeout(this._idleRelaxTimer);
        this._idleRelaxTimer = setTimeout(() => {
            this._requestRender();
        }, this._idleRaiseMs + 60);
    }

    handleCursor(x, y) {
        if (!this._opts.eyes)
            return;
        const flipEyeX = this._opts.eyeDir === 'mirror';
        const sx = flipEyeX ? (1 - x) : x;
        this._eyeTargetX = (sx - 0.5) * 20;
        this._eyeTargetY = (y - 0.5) * 12;
        this._requestRender();
    }

    handleMove(x, y, touch, fingers = touch ? 1 : 0) {
        this._ptrX = Math.min(1, Math.max(0, x));
        this._ptrY = Math.min(1, Math.max(0, y));
        this._ptrFingers = fingers;
        this._lastPointerAt = performance.now();
        this._rightActive = null;
        this._scheduleIdleRelax();
        this._requestRender();
    }

    handleButton(button, down) {
        this._lastPointerAt = performance.now();
        this._rightActive = null;
        this._scheduleIdleRelax();
        if (down)
            this._buttonsDown.add(button);
        else
            this._buttonsDown.delete(button);

        const leftDown = this._buttonsDown.has(1) || this._buttonsDown.has(3);
        const rightDown = this._buttonsDown.has(2) || this._buttonsDown.has(3);
        const middleDown = this._buttonsDown.has(3);

        if (this._ptrLeftBtn)
            this._ptrLeftBtn.classList.toggle('active', leftDown);
        if (this._ptrRightBtn)
            this._ptrRightBtn.classList.toggle('active', rightDown);
        if (this._mouseWheel)
            this._mouseWheel.setAttribute('fill', middleDown ? '#a78bfa' : '#64748b');
        if (this._mouseSideBack)
            this._mouseSideBack.classList.toggle('active', this._buttonsDown.has(4));
        if (this._mouseSideFwd)
            this._mouseSideFwd.classList.toggle('active', this._buttonsDown.has(5));

        this._emitHud();
        this._requestRender();
    }

    handleWheel(dir) {
        this._lastPointerAt = performance.now();
        this._scheduleIdleRelax();
        this._wheelBadge = dir > 0 ? 'Scroll ↑' : 'Scroll ↓';

        if (this._mouseWheel) {
            this._mouseWheel.setAttribute('fill', '#38bdf8');
            this._mouseWheel.setAttribute('transform', `translate(0, ${dir > 0 ? -2.4 : 2.4})`);
        }
        if (this._scrollInd) {
            this._scrollInd.textContent = dir > 0 ? '▲' : '▼';
            this._scrollInd.setAttribute('opacity', '1');
        }
        this._emitHud();

        clearTimeout(this._wheelTimer);
        this._wheelTimer = setTimeout(() => {
            this._wheelBadge = '';
            if (this._mouseWheel) {
                this._mouseWheel.setAttribute('fill', this._buttonsDown.has(3) ? '#a78bfa' : '#64748b');
                this._mouseWheel.removeAttribute('transform');
            }
            if (this._scrollInd)
                this._scrollInd.setAttribute('opacity', '0');
            this._emitHud();
        }, 240);
    }

    _emitHud() {
        if (!this._onHudChange)
            return;

        const pills = [];
        if (this._opts.keys !== 'zones') {
            const seen = new Set();
            for (const {visual, masked} of this._pressedRawToVisual.values()) {
                if (masked)
                    continue;
                const info = this._keyInfo.get(visual);
                if (info && !seen.has(info.hud)) {
                    seen.add(info.hud);
                    pills.push({text: info.hud, kind: 'key'});
                }
            }
        }
        if (this._buttonsDown.has(1))
            pills.push({text: 'L-Click', kind: 'left'});
        if (this._buttonsDown.has(2))
            pills.push({text: 'R-Click', kind: 'right'});
        if (this._buttonsDown.has(3))
            pills.push({text: 'M-Click', kind: 'middle'});
        if (this._buttonsDown.has(4))
            pills.push({text: 'Back', kind: 'left'});
        if (this._buttonsDown.has(5))
            pills.push({text: 'Fwd', kind: 'left'});
        if (this._wheelBadge)
            pills.push({text: this._wheelBadge, kind: 'wheel'});

        this._onHudChange(pills.slice(0, 5));
    }

    setSleeping(sleeping) {
        if (this._sleeping === sleeping)
            return;
        this._sleeping = sleeping;
        this._eyesAwake.style.display = sleeping ? 'none' : '';
        this._eyesSleep.style.display = sleeping ? '' : 'none';
        if (sleeping) {
            this._mouthHappy.style.display = 'none';
            this._leftActive = null;
            this._rightActive = null;
        }
        this._requestRender();
    }

    setFastTyping(fast) {
        if (this._fastTyping === fast)
            return;
        this._fastTyping = fast;
        this._mouthHappy.style.display = (fast && !this._sleeping) ? '' : 'none';
    }

    _scheduleBlink() {
        const delay = 3200 + Math.random() * 3400;
        this._blinkTimer = setTimeout(() => {
            if (!this._sleeping && this._eyeLeftGroup && this._eyeRightGroup) {
                this._eyeLeftGroup._blinkWrap.setAttribute('transform', 'scale(1, 0.12)');
                this._eyeRightGroup._blinkWrap.setAttribute('transform', 'scale(1, 0.12)');
                setTimeout(() => {
                    this._eyeLeftGroup._blinkWrap.removeAttribute('transform');
                    this._eyeRightGroup._blinkWrap.removeAttribute('transform');
                }, 110);
            }
            this._scheduleBlink();
        }, delay);
    }

    _requestRender() {
        if (this._rafId)
            return;
        this._rafId = requestAnimationFrame(() => {
            this._rafId = 0;
            this._renderFrame();
        });
    }

    _renderFrame() {
        const now = performance.now();
        const lerp = (a, b, f) => a + (b - a) * f;

        // Smooth pointer & eye tracking
        this._curPtrX = lerp(this._curPtrX, this._ptrX, 0.45);
        this._curPtrY = lerp(this._curPtrY, this._ptrY, 0.45);
        this._curEyeX = lerp(this._curEyeX, this._eyeTargetX, 0.28);
        this._curEyeY = lerp(this._curEyeY, this._eyeTargetY, 0.28);

        if (this._opts.eyes && !this._sleeping) {
            const tx = this._curEyeX.toFixed(2);
            const ty = this._curEyeY.toFixed(2);
            this._eyeLeftGroup._pupil.setAttribute('transform', `translate(${tx}, ${ty})`);
            this._eyeRightGroup._pupil.setAttribute('transform', `translate(${tx}, ${ty})`);
        }

        const ptr = this._pointerSvgCoords(this._curPtrX, this._curPtrY);

        if (this._mouseGroup) {
            this._mouseGroup.setAttribute(
                'transform',
                `translate(${ptr.x.toFixed(1)}, ${ptr.y.toFixed(1)})`
            );
        }

        if (this._tpRipple1 && this._tpRipple2) {
            const isRight = this._buttonsDown.has(2) || this._ptrFingers >= 2;
            const isMiddle = this._buttonsDown.has(3) || this._ptrFingers >= 3;
            const active = this._ptrFingers > 0 || this._buttonsDown.size > 0;
            const clicked = this._buttonsDown.size > 0;

            this._tpRipple1.setAttribute('cx', ptr.x.toFixed(1));
            this._tpRipple1.setAttribute('cy', ptr.y.toFixed(1));
            this._tpRipple1.setAttribute('r', clicked ? '13' : '9');
            this._tpRipple1.setAttribute('opacity', active ? '1' : '0');
            this._tpRipple1.classList.toggle('right-click', isRight && !isMiddle);
            this._tpRipple1.classList.toggle('middle-click', isMiddle);

            // Second touch dot appears for 2-finger right-click / scroll!
            const offset = this._isCatPov ? -14 : 14;
            this._tpRipple2.setAttribute('cx', (ptr.x + offset).toFixed(1));
            this._tpRipple2.setAttribute('cy', ptr.y.toFixed(1));
            this._tpRipple2.setAttribute('opacity', (active && isRight) ? '0.9' : '0');
            this._tpRipple2.classList.toggle('right-click', isRight);
        }

        // 4-Stage Posture Lifecycle per hand:
        // Active -> Resting on palm-rest/pointer (< idleRaiseMs) -> Raised both with pink beans!
        const pointerActive = this._buttonsDown.size > 0 || this._ptrFingers > 0 || Boolean(this._wheelBadge);
        const leftRecent = !this._sleeping && (now - this._lastLeftAt) < this._idleRaiseMs;
        const rightKeyRecent = !this._sleeping && (now - this._lastRightKeyAt) < this._idleRaiseMs;
        const pointerRecent = !this._sleeping && (now - this._lastPointerAt) < this._idleRaiseMs;
        const typedMoreRecently = this._lastRightKeyAt > this._lastPointerAt;

        let targetL;
        if (this._leftActive) {
            targetL = {x: this._leftActive.x, y: this._leftActive.y, pressed: true, raised: false};
        } else if (leftRecent) {
            targetL = {...this._palmRestPosition('L'), pressed: false};
        } else {
            targetL = {...this._raisedPosition('L'), pressed: false};
        }

        let targetR;
        if (this._rightActive) {
            targetR = {x: this._rightActive.x, y: this._rightActive.y, pressed: true, raised: false};
        } else if (!pointerActive && !rightKeyRecent && !pointerRecent) {
            targetR = {...this._raisedPosition('R'), pressed: false};
        } else if (!pointerActive && this._layout !== 'side-mouse' && typedMoreRecently && this._opts.rightPaw !== 'pointer') {
            targetR = {...this._palmRestPosition('R'), pressed: false};
        } else {
            const pRest = this._restPosition('R');
            const clickDip = this._buttonsDown.size > 0 ? 4 : 0;
            const clickTiltX = this._buttonsDown.has(2)
                ? (this._isCatPov ? -5 : 5)
                : (this._buttonsDown.has(1) ? (this._isCatPov ? 4 : -4) : 0);
            const hoverLift = (this._layout !== 'side-mouse' && this._ptrFingers === 0 && !clickDip) ? -5 : 0;
            targetR = {
                x: pRest.x + clickTiltX,
                y: pRest.y + clickDip + hoverLift,
                pressed: clickDip > 0 || this._ptrFingers > 0 || this._layout === 'side-mouse',
                raised: false,
            };
        }

        // Smooth arm movement (snappier when pressing a key/click!)
        const speedL = targetL.pressed ? 0.56 : 0.22;
        const speedR = targetR.pressed ? 0.56 : 0.22;
        this._curLX = lerp(this._curLX, targetL.x, speedL);
        this._curLY = lerp(this._curLY, targetL.y, speedL);
        this._curRX = lerp(this._curRX, targetR.x, speedR);
        this._curRY = lerp(this._curRY, targetR.y, speedR);

        // Draw Left Paw
        const shL = this._shoulder.L;
        this._leftPawPath.setAttribute(
            'd',
            buildPawPath(shL.x, shL.y, this._curLX, this._curLY, shL.screenSide, targetL.pressed)
        );
        this._leftShadow.setAttribute('cx', this._curLX.toFixed(1));
        this._leftShadow.setAttribute('cy', (this._curLY + 8).toFixed(1));
        this._leftShadow.setAttribute('opacity', targetL.pressed ? '0.32' : '0');
        if (targetL.raised) {
            const rot = shL.screenSide === 'L' ? -18 : 18;
            this._leftBeans.setAttribute('transform', `translate(${this._curLX.toFixed(1)}, ${(this._curLY - 2).toFixed(1)}) rotate(${rot})`);
            this._leftBeans.setAttribute('opacity', '0.9');
        } else {
            this._leftBeans.setAttribute('opacity', '0');
        }

        // Draw Right Paw
        const shR = this._shoulder.R;
        this._rightPawPath.setAttribute(
            'd',
            buildPawPath(shR.x, shR.y, this._curRX, this._curRY, shR.screenSide, targetR.pressed)
        );
        this._rightShadow.setAttribute('cx', this._curRX.toFixed(1));
        this._rightShadow.setAttribute('cy', (this._curRY + 8).toFixed(1));
        this._rightShadow.setAttribute('opacity', (targetR.pressed && !targetR.raised && this._layout !== 'side-mouse') ? '0.28' : '0');
        if (targetR.raised) {
            const rot = shR.screenSide === 'L' ? -18 : 18;
            this._rightBeans.setAttribute('transform', `translate(${this._curRX.toFixed(1)}, ${(this._curRY - 2).toFixed(1)}) rotate(${rot})`);
            this._rightBeans.setAttribute('opacity', '0.9');
        } else {
            this._rightBeans.setAttribute('opacity', '0');
        }

        const settled =
            Math.abs(this._curPtrX - this._ptrX) < 0.002 &&
            Math.abs(this._curPtrY - this._ptrY) < 0.002 &&
            Math.abs(this._curEyeX - this._eyeTargetX) < 0.05 &&
            Math.abs(this._curEyeY - this._eyeTargetY) < 0.05 &&
            Math.abs(this._curLX - targetL.x) < 0.25 &&
            Math.abs(this._curLY - targetL.y) < 0.25 &&
            Math.abs(this._curRX - targetR.x) < 0.25 &&
            Math.abs(this._curRY - targetR.y) < 0.25;

        if (!settled)
            this._requestRender();
    }

    destroy() {
        clearTimeout(this._blinkTimer);
        clearTimeout(this._leftReleaseTimer);
        clearTimeout(this._rightReleaseTimer);
        clearTimeout(this._idleRelaxTimer);
        clearTimeout(this._wheelTimer);
        if (this._rafId) {
            cancelAnimationFrame(this._rafId);
            this._rafId = 0;
        }
    }
}
