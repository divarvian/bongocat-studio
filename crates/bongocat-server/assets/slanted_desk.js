// Iconic Slanted (~10° diagonal) Bongo Cat SVG Renderer for OBS Studio.
// - Slanted Trackpad (default): Unified slanted laptop chassis with Trackpad
//   centered right by the Spacebar/palm-rest and Cat centered over the keyboard!
// - Slanted Mouse: Compact slanted mousepad tucked close on the cat's right side
//   with Cat centered over the keyboard so both arms stay chubby & balanced!
// - 63-key 3D perspective QWERTY keyboard with 180° POV-matched labels
// - Real screen-cursor eye tracking + Live2D-style face parallax + blinking

const SVG_NS = 'http://www.w3.org/2000/svg';

// Desk line: y(x) = DESK_Y0 + DESK_SLOPE * x
const DESK_Y0 = 136;
const DESK_SLOPE = 0.175; // ~9.93° diagonal slant

// Standard 10-finger human touch-typing split (each row sums to 15.0u).
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
        {code: 14, label: 'Back', hud: 'Bksp', w: 2, hand: 'R'},
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
        {code: 58, label: 'Cap', hud: 'Caps', w: 1.75, hand: 'L'},
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
        {code: 28, label: 'Enter', hud: 'Enter', w: 2.25, hand: 'R'},
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
    [41, 1], [111, 14], [126, 125], [96, 28], [82, 11],
    [79, 2], [80, 3], [81, 4], [75, 5], [76, 6], [77, 7], [71, 8], [72, 9], [73, 10],
]);

const GAMING_KEYS = new Set([
    1, 2, 3, 4, 5, 6, 15, 16, 17, 18, 19, 30, 31, 32, 33,
    42, 54, 29, 97, 56, 100, 57, 103, 105, 106, 108,
]);

// Layout 1: Unified Slanted Laptop (Trackpad centered right below Space / palm-rest!)
const LAPTOP_QUAD = {
    tl: [154, 166],
    tr: [518, 230],
    br: [492, 388],
    bl: [92, 318],
};

// Layout 2: Slanted Mouse (Mousepad tucked snugly on cat's right, Cat centered over Keyboard!)
const MOUSE_PAD_QUAD = {
    tl: [122, 162],
    tr: [248, 184],
    br: [188, 294],
    bl: [54, 270],
};

const MOUSE_KB_QUAD = {
    tl: [262, 188],
    tr: [586, 245],
    br: [538, 372],
    bl: [192, 311],
};

function svgEl(tag, attrs = {}, text = '') {
    const el = document.createElementNS(SVG_NS, tag);
    for (const [k, v] of Object.entries(attrs))
        el.setAttribute(k, String(v));
    if (text)
        el.textContent = text;
    return el;
}

function quadPoint(q, u, v) {
    const tx = q.tl[0] + (q.tr[0] - q.tl[0]) * u;
    const ty = q.tl[1] + (q.tr[1] - q.tl[1]) * u;
    const bx = q.bl[0] + (q.br[0] - q.bl[0]) * u;
    const by = q.bl[1] + (q.br[1] - q.bl[1]) * u;
    return [tx + (bx - tx) * v, ty + (by - ty) * v];
}

function subQuad(q, u0, v0, u1, v1) {
    return {
        tl: quadPoint(q, u0, v0),
        tr: quadPoint(q, u1, v0),
        br: quadPoint(q, u1, v1),
        bl: quadPoint(q, u0, v1),
    };
}

function quadPointsStr(q, u0 = 0, v0 = 0, u1 = 1, v1 = 1) {
    const p1 = quadPoint(q, u0, v0);
    const p2 = quadPoint(q, u1, v0);
    const p3 = quadPoint(q, u1, v1);
    const p4 = quadPoint(q, u0, v1);
    return `${p1[0].toFixed(1)},${p1[1].toFixed(1)} ` +
           `${p2[0].toFixed(1)},${p2[1].toFixed(1)} ` +
           `${p3[0].toFixed(1)},${p3[1].toFixed(1)} ` +
           `${p4[0].toFixed(1)},${p4[1].toFixed(1)}`;
}

// 2D affine matrix placing upright text rotated 180° flat onto the 3D quad
// facing the cat (positive determinant -> never mirrored!).
function perspectiveCatTextMatrix(q, u0, v0, u1, v1) {
    const uc = (u0 + u1) * 0.5;
    const vc = (v0 + v1) * 0.5;
    const [cx, cy] = quadPoint(q, uc, vc);
    const pL = quadPoint(q, u0, vc);
    const pR = quadPoint(q, u1, vc);
    const pT = quadPoint(q, uc, v0);
    const pB = quadPoint(q, uc, v1);

    const ux = pL[0] - pR[0];
    const uy = pL[1] - pR[1];
    const uLen = Math.hypot(ux, uy) || 1;
    const vx = pT[0] - pB[0];
    const vy = pT[1] - pB[1];
    const vLen = Math.hypot(vx, vy) || 1;

    const ax = ux / uLen;
    const ay = uy / uLen;
    const bx = vx / vLen;
    const by = vy / vLen;

    return `matrix(${ax.toFixed(3)} ${ay.toFixed(3)} ${bx.toFixed(3)} ${by.toFixed(3)} ${cx.toFixed(1)} ${cy.toFixed(1)})`;
}

// Seamless curved cartoon Bongo Cat paw with anatomically fixed shoulders.
// Because shoulder anchors stay fixed inside the white torso just above the desk
// line (rather than rotating with the arm vector), the arm NEVER pokes into the
// eyes/blush and NEVER notches the desk line regardless of reach angle!
function buildSeamlessArm(sOuter, sInner, tx, ty, side, tipHalf) {
    const sx = (sOuter.x + sInner.x) * 0.5;
    const sy = (sOuter.y + sInner.y) * 0.5;
    const dx = tx - sx;
    const dy = ty - sy;
    const dist = Math.hypot(dx, dy) || 1;
    const ux = dx / dist;
    const uy = dy / dist;
    // Perpendicular pointing screen-LEFT when arm reaches downward (dy > 0)
    const nx = -uy;
    const ny = ux;

    // For screen-LEFT arm ('L'): outer = +normal (screen-left), inner = -normal (screen-right)
    // For screen-RIGHT arm ('R'): outer = -normal (screen-right), inner = +normal (screen-left)
    const outN = side === 'L' ? 1 : -1;
    const tOutX = tx + nx * outN * tipHalf;
    const tOutY = ty + ny * outN * tipHalf;
    const tInX = tx - nx * outN * tipHalf;
    const tInY = ty - ny * outN * tipHalf;

    // Soft chubby outward arch along the arm body
    const bowOut = Math.min(18, dist * 0.13);
    const bowIn = Math.min(8, dist * 0.06);
    const cOutX = (sOuter.x + tOutX) * 0.5 + nx * outN * bowOut;
    const cOutY = (sOuter.y + tOutY) * 0.5 + ny * outN * bowOut;
    const cInX = (sInner.x + tInX) * 0.5 - nx * outN * bowIn;
    const cInY = (sInner.y + tInY) * 0.5 - ny * outN * bowIn;

    // Plump rounded paw tip
    const tipX = tx + ux * (tipHalf * 1.18);
    const tipY = ty + uy * (tipHalf * 1.18);

    const open =
        `M ${sOuter.x.toFixed(1)} ${sOuter.y.toFixed(1)} ` +
        `Q ${cOutX.toFixed(1)} ${cOutY.toFixed(1)} ${tOutX.toFixed(1)} ${tOutY.toFixed(1)} ` +
        `Q ${tipX.toFixed(1)} ${tipY.toFixed(1)} ${tInX.toFixed(1)} ${tInY.toFixed(1)} ` +
        `Q ${cInX.toFixed(1)} ${cInY.toFixed(1)} ${sInner.x.toFixed(1)} ${sInner.y.toFixed(1)}`;

    return {open, closed: `${open} Z`};
}

export class SlantedDeskRenderer {
    constructor(container, opts, callbacks = {}) {
        this._container = container;
        this._opts = opts;
        this._onHudChange = callbacks.onHudChange || null;
        this._isMouse = opts.layout === 'side-mouse' || opts.pointer === 'mouse';

        // Slide cat cleanly along the diagonal desk line so the cat is ALWAYS
        // centered directly over the keyboard (short, balanced, chubby arms!).
        this._catShiftX = this._isMouse ? 42 : 0;
        this._catShiftY = this._catShiftX * DESK_SLOPE;

        // Active geometry quads
        if (this._isMouse) {
            this._padQuad = MOUSE_PAD_QUAD;
            this._kbQuad = MOUSE_KB_QUAD;
        } else {
            // Trackpad centered at the laptop palm-rest right beside Spacebar,
            // full 63-key keyboard right below it!
            this._padQuad = subQuad(LAPTOP_QUAD, 0.28, 0.04, 0.72, 0.31);
            this._kbQuad = subQuad(LAPTOP_QUAD, 0.04, 0.35, 0.96, 0.95);
        }

        // Anatomically fixed shoulder anchors tucked cleanly above the desk line
        // and below the cheeks!
        // Cat's RIGHT shoulder (screen-LEFT) and Cat's LEFT shoulder (screen-RIGHT)
        this._shoulderR = {
            x: 204 + this._catShiftX,
            y: 160 + this._catShiftY,
            outer: {x: 170 + this._catShiftX, y: 156 + this._catShiftY},
            inner: {x: 230 + this._catShiftX, y: 168 + this._catShiftY},
        };
        this._shoulderL = {
            x: 438 + this._catShiftX,
            y: 196 + this._catShiftY,
            inner: {x: 412 + this._catShiftX, y: 186 + this._catShiftY},
            outer: {x: 468 + this._catShiftX, y: 198 + this._catShiftY},
        };

        // Natural palm-rest standby spots (lower keyboard / laptop palm-rest)
        const [prRx, prRy] = this._isMouse
            ? quadPoint(MOUSE_KB_QUAD, 0.24, 0.07)
            : quadPoint(LAPTOP_QUAD, 0.15, 0.22);
        const [prLx, prLy] = this._isMouse
            ? quadPoint(MOUSE_KB_QUAD, 0.78, 0.07)
            : quadPoint(LAPTOP_QUAD, 0.85, 0.22);
        this._palmRestR = {x: prRx, y: prRy};
        this._palmRestL = {x: prLx, y: prLy};

        // Keyboard state
        this._pressedRawToVisual = new Map();
        this._visualKeyRefCount = new Map();
        this._keyInfo = new Map();
        this._lastKeyHand = 'L';
        this._idleRaiseMs = Math.max(1000, (opts.idleRaiseSec ?? 4) * 1000);
        this._lastLeftAt = -Infinity;
        this._lastRightKeyAt = -Infinity;

        // Pointer pad state (0..1)
        this._ptrX = 0.5;
        this._ptrY = 0.5;
        this._ptrFingers = 0;
        this._lastPointerAt = -Infinity;
        this._buttonsDown = new Set();
        this._wheelBadge = '';
        this._wheelTimer = 0;

        // Real screen cursor state for eyes + face parallax (0..1)
        this._cursorX = 0.5;
        this._cursorY = 0.5;

        // Animation states (starts calmly with both paws raised!)
        this._sleeping = false;
        this._fastTyping = false;
        this._leftActive = null;
        this._rightActive = null;
        this._leftReleaseTimer = 0;
        this._rightReleaseTimer = 0;
        this._idleRelaxTimer = 0;
        this._blinkTimer = 0;
        this._leftRaise = 1;
        this._rightRaise = 1;

        // Smooth interpolation coordinates
        this._curPtrX = 0.5;
        this._curPtrY = 0.5;
        this._curEyeX = 0;
        this._curEyeY = 0;
        this._eyeTargetX = 0;
        this._eyeTargetY = 0;
        this._rafId = 0;

        this._buildSvg();

        this._curRX = this._palmRestR.x;
        this._curRY = this._palmRestR.y;
        this._curLX = this._palmRestL.x;
        this._curLY = this._palmRestL.y;

        this._scheduleBlink();
        this._renderFrame();
    }

    _buildSvg() {
        const svg = svgEl('svg', {
            id: 'slanted-svg',
            viewBox: '0 0 640 400',
            width: '100%',
            height: '100%',
            'aria-label': 'Bongo Cat Slanted Desk Overlay',
        });

        const theme = this._opts.deskTheme || 'white';
        const isDark = theme === 'dark';
        const isPastel = theme === 'pastel';
        const isTransparentDesk = theme === 'transparent';

        const yLeft = DESK_Y0 - 10 * DESK_SLOPE;
        const yRight = DESK_Y0 + 650 * DESK_SLOPE;

        const defs = svgEl('defs');
        // Clip cat torso cleanly right along the diagonal desk line!
        const clip = svgEl('clipPath', {id: 'above-desk-clip'});
        clip.append(svgEl('polygon', {
            points: `-20,-20 660,-20 660,${yRight.toFixed(1)} -20,${yLeft.toFixed(1)}`,
        }));

        const chassisGrad = svgEl('linearGradient', {id: 'slanted-chassis-grad', x1: '0', y1: '0', x2: '0', y2: '1'});
        chassisGrad.append(
            svgEl('stop', {offset: '0%', 'stop-color': isDark ? '#334155' : (isPastel ? '#fdf4ff' : '#dde3ec')}),
            svgEl('stop', {offset: '100%', 'stop-color': isDark ? '#1e293b' : (isPastel ? '#f3e8ff' : '#c4ccd8')})
        );

        const padGrad = svgEl('linearGradient', {id: 'slanted-pad-grad', x1: '0', y1: '0', x2: '1', y2: '1'});
        padGrad.append(
            svgEl('stop', {offset: '0%', 'stop-color': isDark ? '#475569' : '#c8cfd8'}),
            svgEl('stop', {offset: '100%', 'stop-color': isDark ? '#273244' : '#adb5c0'})
        );
        defs.append(clip, chassisGrad, padGrad);
        svg.append(defs);

        // 1. Cat Body & Head (centered over keyboard, clipped cleanly along desk line!)
        const catLayer = svgEl('g', {'clip-path': 'url(#above-desk-clip)'});
        catLayer.append(this._buildCatBody());
        svg.append(catLayer);

        // 2. Desk Tabletop surface below the diagonal line
        const deskFill = isTransparentDesk
            ? 'none'
            : (isDark ? '#181e2b' : (isPastel ? '#fdf2f8' : '#ffffff'));
        if (!isTransparentDesk) {
            svg.append(svgEl('polygon', {
                points: `-10,${yLeft.toFixed(1)} 650,${yRight.toFixed(1)} 650,410 -10,410`,
                fill: deskFill,
            }));
        }
        // Bold iconic diagonal desk line
        svg.append(svgEl('line', {
            x1: -10, y1: yLeft.toFixed(1), x2: 650, y2: yRight.toFixed(1),
            stroke: '#111827',
            'stroke-width': '6.0',
            'stroke-linecap': 'round',
        }));

        // 3. Unified Slanted Laptop base (when in trackpad mode!)
        if (!this._isMouse)
            svg.append(this._buildSlantedLaptopBase(isDark));

        // 4. Slanted Trackpad (centered in laptop) OR Slanted Mouse + Pad (beside keyboard)
        svg.append(this._buildSlantedPointerArea(isDark));

        // 5. 3D Perspective Slanted QWERTY Keyboard (63 keys!)
        svg.append(this._buildSlantedKeyboard(isDark));

        // 6. Seamless Animated Paws (top layer!)
        svg.append(this._buildPawsLayer());

        this._svg = svg;
        this._container.replaceChildren(svg);
    }

    _buildCatBody() {
        const g = svgEl('g', {
            id: 'slanted-cat-body',
            transform: `translate(${this._catShiftX.toFixed(1)}, ${this._catShiftY.toFixed(1)})`,
        });

        // Iconic chubby slanted Bongo Cat silhouette centered around x=334!
        const silhouette = svgEl('path', {
            d: 'M 156 178 ' +
               'C 162 106, 218 54, 276 40 ' +
               'C 286 22, 298 10, 308 9 ' +
               'C 318 11, 325 31, 329 44 ' +
               'C 374 51, 416 68, 452 91 ' +
               'C 471 80, 490 73, 499 76 ' +
               'C 504 86, 497 105, 488 119 ' +
               'C 506 152, 518 196, 524 244 ' +
               'L 150 182 Z',
            fill: '#ffffff',
            stroke: '#111827',
            'stroke-width': '5.8',
            'stroke-linecap': 'round',
            'stroke-linejoin': 'round',
        });
        g.append(silhouette);

        // Face group centered at (326, 138), tilted 10° along the head!
        this._faceWrap = svgEl('g', {transform: 'translate(326, 138) rotate(10)'});
        this._faceParallax = svgEl('g', {id: 'slanted-face-parallax'});

        this._faceParallax.append(
            svgEl('ellipse', {cx: -78, cy: 14, rx: 13.5, ry: 7.2, fill: '#f9a8d4', opacity: '0.42'}),
            svgEl('ellipse', {cx: 70, cy: 14, rx: 13.5, ry: 7.2, fill: '#f9a8d4', opacity: '0.42'})
        );

        this._eyesAwake = svgEl('g', {id: 'slanted-eyes-awake'});
        this._eyeLeft = this._createSlantedEye(-60, -2);
        this._eyeRight = this._createSlantedEye(50, 0);
        this._eyesAwake.append(this._eyeLeft, this._eyeRight);

        this._eyesSleep = svgEl('g', {id: 'slanted-eyes-sleep', style: 'display:none'});
        this._eyesSleep.append(
            svgEl('path', {
                d: 'M -72 -2 Q -60 9 -48 -2',
                fill: 'none', stroke: '#111827', 'stroke-width': '4.5', 'stroke-linecap': 'round',
            }),
            svgEl('path', {
                d: 'M 38 0 Q 50 11 62 0',
                fill: 'none', stroke: '#111827', 'stroke-width': '4.5', 'stroke-linecap': 'round',
            }),
            svgEl('text', {
                x: 134, y: -70, fill: '#38bdf8', 'font-size': '25', 'font-weight': '800',
                class: 'zzz-float',
            }, 'Z'),
            svgEl('text', {
                x: 158, y: -90, fill: '#a78bfa', 'font-size': '18', 'font-weight': '800',
                class: 'zzz-float zzz-2',
            }, 'z')
        );

        this._mouthCalm = svgEl('path', {
            d: 'M -31 10 C -28 23, -11 21, -6 10 C -1 21, 16 23, 19 10',
            fill: 'none',
            stroke: '#111827',
            'stroke-width': '5.0',
            'stroke-linecap': 'round',
            'stroke-linejoin': 'round',
        });
        this._mouthHappy = svgEl('path', {
            d: 'M -15 17 Q -6 30 3 17 Z',
            fill: '#fb7185',
            stroke: '#111827',
            'stroke-width': '2.6',
            'stroke-linejoin': 'round',
            style: 'display:none',
        });

        this._faceParallax.append(
            this._eyesAwake, this._eyesSleep, this._mouthCalm, this._mouthHappy
        );
        this._faceWrap.append(this._faceParallax);
        g.append(this._faceWrap);
        return g;
    }

    _createSlantedEye(cx, cy) {
        const outer = svgEl('g', {transform: `translate(${cx}, ${cy})`});
        const blinkWrap = svgEl('g', {class: 'eye-blink'});
        const isExpressive = this._opts.eyeStyle === 'expressive';

        if (isExpressive) {
            blinkWrap.append(svgEl('ellipse', {
                cx: 0, cy: 0, rx: 13, ry: 14,
                fill: '#ffffff', stroke: '#111827', 'stroke-width': '2.4',
            }));
        }

        const pupil = svgEl('g', {class: 'eye-pupil'});
        if (isExpressive) {
            pupil.append(
                svgEl('ellipse', {cx: 0, cy: 0, rx: 8.2, ry: 9.5, fill: '#111827'}),
                svgEl('circle', {cx: -2.5, cy: -3.0, r: 2.7, fill: '#ffffff'}),
                svgEl('circle', {cx: 2.5, cy: 2.6, r: 1.3, fill: '#ffffff', opacity: '0.85'})
            );
        } else {
            pupil.append(
                svgEl('ellipse', {cx: 0, cy: 0, rx: 8.8, ry: 9.8, fill: '#111827'}),
                svgEl('circle', {cx: -2.6, cy: -3.0, r: 2.1, fill: '#ffffff', opacity: '0.45'})
            );
        }

        blinkWrap.append(pupil);
        outer.append(blinkWrap);
        outer._pupil = pupil;
        outer._blinkWrap = blinkWrap;
        return outer;
    }

    _buildSlantedLaptopBase(isDark) {
        const g = svgEl('g', {id: 'slanted-laptop-base'});
        const q = LAPTOP_QUAD;
        const depth = 7;

        // 3D front bevel edge of laptop chassis
        g.append(
            svgEl('polygon', {
                points:
                    `${q.bl[0]},${q.bl[1]} ${q.br[0]},${q.br[1]} ${q.tr[0]},${q.tr[1]} ` +
                    `${q.tr[0]},${q.tr[1] + depth} ${q.br[0]},${q.br[1] + depth} ${q.bl[0]},${q.bl[1] + depth}`,
                fill: isDark ? '#0f172a' : '#8d96a3',
                stroke: '#111827',
                'stroke-width': '2.2',
                'stroke-linejoin': 'round',
            }),
            // Main laptop deck surface
            svgEl('polygon', {
                points: quadPointsStr(q),
                fill: 'url(#slanted-chassis-grad)',
                stroke: '#111827',
                'stroke-width': '3.4',
                'stroke-linejoin': 'round',
            }),
            // Subtle hinge bar at the monitor-facing edge (bottom of laptop)
            svgEl('polygon', {
                points: quadPointsStr(q, 0.16, 0.965, 0.84, 1.0),
                fill: '#1e293b',
            })
        );
        return g;
    }

    _buildSlantedPointerArea(isDark) {
        const g = svgEl('g', {id: 'slanted-pointer-area'});
        const q = this._padQuad;

        if (!this._isMouse) {
            // SLANTED LAPTOP TRACKPAD (centered right beside Spacebar!)
            g.append(
                svgEl('polygon', {
                    points: quadPointsStr(q),
                    fill: 'url(#slanted-pad-grad)',
                    stroke: isDark ? '#020617' : '#64748b',
                    'stroke-width': '2.2',
                    'stroke-linejoin': 'round',
                })
            );

            // Click strips closest to cat's wrists (top edge of trackpad, v: 0.04..0.32).
            // Cat POV: screen-RIGHT half = Left Click, screen-LEFT half = Right Click!
            this._ptrRightBtn = svgEl('polygon', {
                points: quadPointsStr(q, 0.02, 0.04, 0.495, 0.32),
                class: 'ptr-btn ptr-btn-right',
            });
            this._ptrLeftBtn = svgEl('polygon', {
                points: quadPointsStr(q, 0.505, 0.04, 0.98, 0.32),
                class: 'ptr-btn ptr-btn-left',
            });

            const pDivL = quadPoint(q, 0.03, 0.33);
            const pDivR = quadPoint(q, 0.97, 0.33);
            const pMidT = quadPoint(q, 0.5, 0.04);
            const pMidB = quadPoint(q, 0.5, 0.33);

            const lMat = perspectiveCatTextMatrix(q, 0.505, 0.04, 0.98, 0.32);
            const rMat = perspectiveCatTextMatrix(q, 0.02, 0.04, 0.495, 0.32);
            const labelFill = isDark ? 'rgba(226,232,240,0.48)' : 'rgba(30,41,59,0.48)';

            const [cx, cy] = quadPoint(q, 0.5, 0.6);
            this._tpRipple1 = svgEl('circle', {cx, cy, r: 9, class: 'tp-ripple', opacity: '0'});
            this._tpRipple2 = svgEl('circle', {cx: cx - 15, cy, r: 8, class: 'tp-ripple tp-ripple-2', opacity: '0'});

            const [scX, scY] = quadPoint(q, 0.12, 0.66);
            this._scrollInd = svgEl('text', {
                x: scX.toFixed(1), y: scY.toFixed(1),
                'text-anchor': 'middle', 'font-size': '14', 'font-weight': '800',
                fill: '#38bdf8', opacity: '0',
            }, '▲');

            g.append(
                this._ptrRightBtn,
                this._ptrLeftBtn,
                svgEl('line', {
                    x1: pDivL[0], y1: pDivL[1], x2: pDivR[0], y2: pDivR[1],
                    stroke: isDark ? 'rgba(255,255,255,0.16)' : 'rgba(15,23,42,0.18)',
                    'stroke-width': '1.3',
                }),
                svgEl('line', {
                    x1: pMidT[0], y1: pMidT[1], x2: pMidB[0], y2: pMidB[1],
                    stroke: isDark ? 'rgba(255,255,255,0.16)' : 'rgba(15,23,42,0.18)',
                    'stroke-width': '1.3',
                }),
                svgEl('text', {
                    x: 0, y: 0, transform: lMat,
                    'text-anchor': 'middle', 'dominant-baseline': 'central',
                    'font-size': '7.5', 'font-weight': '800', fill: labelFill,
                }, 'L'),
                svgEl('text', {
                    x: 0, y: 0, transform: rMat,
                    'text-anchor': 'middle', 'dominant-baseline': 'central',
                    'font-size': '7.5', 'font-weight': '800', fill: labelFill,
                }, 'R'),
                this._tpRipple1,
                this._tpRipple2,
                this._scrollInd
            );
        } else {
            // SLANTED MOUSE + COMPACT PAD (tucked close on cat's right side!)
            const depth = 6;
            g.append(
                svgEl('polygon', {
                    points:
                        `${q.bl[0]},${q.bl[1]} ${q.br[0]},${q.br[1]} ${q.tr[0]},${q.tr[1]} ` +
                        `${q.tr[0]},${q.tr[1] + depth} ${q.br[0]},${q.br[1] + depth} ${q.bl[0]},${q.bl[1] + depth}`,
                    fill: isDark ? '#0f172a' : '#8d95a0',
                    stroke: isDark ? '#020617' : '#64748b',
                    'stroke-width': '1.6',
                    'stroke-linejoin': 'round',
                }),
                svgEl('polygon', {
                    points: quadPointsStr(q),
                    fill: 'url(#slanted-pad-grad)',
                    stroke: isDark ? '#020617' : '#7b8491',
                    'stroke-width': '2.4',
                    'stroke-linejoin': 'round',
                })
            );

            this._mouseGroup = svgEl('g', {id: 'slanted-mouse-device'});
            const inner = svgEl('g', {transform: 'rotate(202)'});

            inner.append(
                svgEl('ellipse', {cx: 0, cy: 4, rx: 30, ry: 23, fill: 'rgba(15,23,42,0.18)'}),
                svgEl('ellipse', {
                    cx: 0, cy: 0, rx: 28, ry: 22,
                    fill: '#ffffff', stroke: '#111827', 'stroke-width': '4.6',
                })
            );
            this._ptrLeftBtn = svgEl('path', {
                d: 'M -25 -2 C -24 -15, -13 -20, -1 -20 L -1 -2 Z',
                class: 'ptr-btn ptr-btn-left mouse-btn',
            });
            this._ptrRightBtn = svgEl('path', {
                d: 'M 1 -20 C 13 -20, 24 -15, 25 -2 L 1 -2 Z',
                class: 'ptr-btn ptr-btn-right mouse-btn',
            });
            this._mouseWheel = svgEl('rect', {
                x: -3.0, y: -16, width: 6.0, height: 10.5, rx: 3,
                fill: '#64748b', stroke: '#111827', 'stroke-width': '1.7',
            });
            inner.append(
                this._ptrLeftBtn,
                this._ptrRightBtn,
                svgEl('line', {x1: -26, y1: -2, x2: 26, y2: -2, stroke: '#111827', 'stroke-width': '2.8'}),
                svgEl('line', {x1: 0, y1: -21, x2: 0, y2: -2, stroke: '#111827', 'stroke-width': '2.8'}),
                this._mouseWheel
            );
            this._mouseGroup.append(inner);
            g.append(this._mouseGroup);
        }

        return g;
    }

    _buildSlantedKeyboard(isDark) {
        const g = svgEl('g', {id: 'slanted-keyboard'});
        const q = this._kbQuad;

        // Outer keyboard bezel (with 3D bevel in Mouse standalone mode, recessed tray in Laptop mode!)
        if (this._isMouse) {
            const depth = 6;
            g.append(svgEl('polygon', {
                points:
                    `${q.bl[0]},${q.bl[1]} ${q.br[0]},${q.br[1]} ${q.tr[0]},${q.tr[1]} ` +
                    `${q.tr[0]},${q.tr[1] + depth} ${q.br[0]},${q.br[1] + depth} ${q.bl[0]},${q.bl[1] + depth}`,
                fill: isDark ? '#020617' : '#334155',
                stroke: '#111827',
                'stroke-width': '2',
                'stroke-linejoin': 'round',
            }));
        }

        g.append(svgEl('polygon', {
            points: quadPointsStr(q),
            fill: isDark ? '#0f172a' : (this._isMouse ? '#94a3b8' : '#1e293b'),
            stroke: '#111827',
            'stroke-width': this._isMouse ? '3.0' : '2.4',
            'stroke-linejoin': 'round',
        }));

        const rows = [...KEY_ROWS].reverse();
        const padU = 0.022;
        const padV = 0.045;
        const gapU = 0.0055;
        const gapV = 0.022;
        const usableU = 1 - padU * 2;
        const usableV = 1 - padV * 2;
        const rowV = (usableV - gapV * 4) / 5;
        const unitU = (usableU - gapU * 14) / 15;
        const hideLabels = this._opts.keys === 'zones';

        for (let r = 0; r < rows.length; r++) {
            const row = [...rows[r]].reverse();
            const v0 = padV + r * (rowV + gapV);
            const v1 = v0 + rowV;
            let uCur = padU;

            for (const key of row) {
                const u0 = uCur;
                const u1 = u0 + key.w * unitU + (key.w - 1) * gapU;
                const [cx, cy] = quadPoint(q, (u0 + u1) * 0.5, (v0 + v1) * 0.5);

                const keyGroup = svgEl('g', {
                    class: 'kb-key',
                    'data-code': key.code,
                });

                keyGroup.append(svgEl('polygon', {
                    points: quadPointsStr(q, u0, v0, u1, v1),
                    class: 'kb-cap',
                }));

                if (!hideLabels) {
                    const mat = perspectiveCatTextMatrix(q, u0, v0, u1, v1);
                    const fontSize = key.label.length > 3 ? 6.4 : (key.label.length > 1 ? 7.4 : 8.8);
                    keyGroup.append(svgEl('text', {
                        x: 0, y: 0,
                        transform: mat,
                        'text-anchor': 'middle',
                        'dominant-baseline': 'central',
                        'font-size': fontSize,
                        class: 'kb-label',
                    }, key.label));
                }

                g.append(keyGroup);

                const [ltX, ltY] = quadPoint(q, u0 + (u1 - u0) * 0.68, (v0 + v1) * 0.5);
                const [rtX, rtY] = quadPoint(q, u0 + (u1 - u0) * 0.32, (v0 + v1) * 0.5);

                this._keyInfo.set(key.code, {
                    el: keyGroup,
                    cx,
                    cy,
                    leftThumbX: ltX,
                    leftThumbY: ltY,
                    rightThumbX: rtX,
                    rightThumbY: rtY,
                    hand: key.hand,
                    label: key.label,
                    hud: key.hud,
                });

                uCur = u1 + gapU;
            }
        }

        return g;
    }

    _buildPawsLayer() {
        const g = svgEl('g', {id: 'slanted-paws'});

        // 1. RIGHT PAW OF CAT (Screen-LEFT):
        //    (a) Raised idle pose mirroring left paw with 4 adorable pink paw beans!
        this._rightIdlePaw = svgEl('g', {
            id: 'slanted-right-idle-paw',
            transform: `translate(${this._catShiftX.toFixed(1)}, ${this._catShiftY.toFixed(1)})`,
        });
        this._rightIdlePaw.append(
            svgEl('path', {
                d: 'M 158 164 C 154 114, 176 76, 204 82 C 232 88, 236 130, 224 176 Z',
                fill: '#ffffff',
                stroke: 'none',
            }),
            svgEl('path', {
                d: 'M 158 164 C 154 114, 176 76, 204 82 C 232 88, 236 130, 224 176',
                fill: 'none',
                stroke: '#111827',
                'stroke-width': '5.4',
                'stroke-linecap': 'round',
                'stroke-linejoin': 'round',
            }),
            svgEl('ellipse', {
                cx: 194, cy: 138, rx: 12, ry: 15.0,
                transform: 'rotate(10 194 138)',
                fill: '#fb7185', opacity: '0.78',
            }),
            svgEl('ellipse', {cx: 177, cy: 114, rx: 5.5, ry: 6.6, fill: '#fb7185', opacity: '0.78'}),
            svgEl('ellipse', {cx: 195, cy: 104, rx: 5.8, ry: 7.0, fill: '#fb7185', opacity: '0.78'}),
            svgEl('ellipse', {cx: 212, cy: 116, rx: 5.8, ry: 7.0, fill: '#fb7185', opacity: '0.78'})
        );

        //    (b) Active / Resting Right Arm (onto Trackpad, Mouse, or right keys!)
        this._rightShadow = svgEl('ellipse', {
            cx: 280, cy: 230, rx: 19, ry: 7, fill: 'rgba(15,23,42,0.22)', opacity: '0',
        });
        this._rightArmFill = svgEl('path', {fill: '#ffffff', stroke: 'none', opacity: '0'});
        this._rightArmStroke = svgEl('path', {
            fill: 'none', stroke: '#111827', 'stroke-width': '5.4',
            'stroke-linecap': 'round', 'stroke-linejoin': 'round', opacity: '0',
        });

        // 2. LEFT PAW OF CAT (Screen-RIGHT):
        //    (a) Raised idle pose with 4 adorable pink paw beans (follows cat shift!)
        this._leftIdlePaw = svgEl('g', {
            id: 'slanted-left-idle-paw',
            transform: `translate(${this._catShiftX.toFixed(1)}, ${this._catShiftY.toFixed(1)})`,
        });
        this._leftIdlePaw.append(
            svgEl('path', {
                d: 'M 404 205 C 392 158, 405 116, 434 120 C 462 124, 478 166, 472 218 Z',
                fill: '#ffffff',
                stroke: 'none',
            }),
            svgEl('path', {
                d: 'M 404 205 C 392 158, 405 116, 434 120 C 462 124, 478 166, 472 218',
                fill: 'none',
                stroke: '#111827',
                'stroke-width': '5.4',
                'stroke-linecap': 'round',
                'stroke-linejoin': 'round',
            }),
            svgEl('ellipse', {
                cx: 437, cy: 180, rx: 12, ry: 15.5,
                transform: 'rotate(-8 437 180)',
                fill: '#fb7185', opacity: '0.78',
            }),
            svgEl('ellipse', {cx: 416, cy: 158, rx: 5.8, ry: 7.0, fill: '#fb7185', opacity: '0.78'}),
            svgEl('ellipse', {cx: 430, cy: 144, rx: 5.8, ry: 7.0, fill: '#fb7185', opacity: '0.78'}),
            svgEl('ellipse', {cx: 446, cy: 152, rx: 5.5, ry: 6.6, fill: '#fb7185', opacity: '0.78'})
        );

        //    (b) Active / Resting Left Arm (reaches exact keycap or rests on palm-rest!)
        this._leftShadow = svgEl('ellipse', {
            cx: 420, cy: 280, rx: 17, ry: 6.5, fill: 'rgba(15,23,42,0.25)', opacity: '0',
        });
        this._leftArmFill = svgEl('path', {fill: '#ffffff', stroke: 'none', opacity: '0'});
        this._leftArmStroke = svgEl('path', {
            fill: 'none', stroke: '#111827', 'stroke-width': '5.4',
            'stroke-linecap': 'round', 'stroke-linejoin': 'round', opacity: '0',
        });

        g.append(
            this._rightIdlePaw, this._rightShadow, this._rightArmFill, this._rightArmStroke,
            this._leftIdlePaw, this._leftShadow, this._leftArmFill, this._leftArmStroke
        );
        return g;
    }

    _padCoords(nx, ny) {
        const naturalFlip = this._opts.ptrDir !== 'screen';
        const fx = naturalFlip ? (1 - nx) : nx;
        const fy = naturalFlip ? (1 - ny) : ny;
        const u = this._isMouse ? (0.24 + fx * 0.56) : (0.16 + fx * 0.68);
        const v = this._isMouse ? (0.22 + fy * 0.50) : (0.34 + fy * 0.52);
        const [x, y] = quadPoint(this._padQuad, u, v);
        return {x, y};
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
        const leftZones = [17, 31, 45];
        const rightZones = [23, 37, 50];
        const info = this._keyInfo.get(code);
        const isLeft = info ? info.hand === 'L' : (code % 2 === 0);
        const pool = isLeft ? leftZones : rightZones;
        return pool[code % pool.length];
    }

    _pickHandForKey(info) {
        if (info.hand === 'S') {
            const rightBusy = this._opts.rightPaw === 'pointer' || this._buttonsDown.size > 0;
            if (rightBusy || this._lastKeyHand === 'R')
                return {hand: 'L', x: info.leftThumbX, y: info.leftThumbY};
            return {hand: 'R', x: info.rightThumbX, y: info.rightThumbY};
        }
        if (info.hand === 'R') {
            if (this._opts.rightPaw === 'pointer' || this._buttonsDown.size > 0)
                return {hand: 'L', x: info.cx, y: info.cy};
            return {hand: 'R', x: info.cx, y: info.cy};
        }
        return {hand: 'L', x: info.cx, y: info.cy};
    }

    handleCursor(x, y) {
        this._cursorX = Math.min(1, Math.max(0, x));
        this._cursorY = Math.min(1, Math.max(0, y));
        if (this._opts.eyes) {
            const flipEyeX = this._opts.eyeDir === 'mirror';
            const sx = flipEyeX ? (1 - this._cursorX) : this._cursorX;
            this._eyeTargetX = (sx - 0.5) * 24;
            this._eyeTargetY = (this._cursorY - 0.5) * 14;
        }
        this._requestRender();
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
                    this._rightActive = {x: target.x, y: target.y};
                    clearTimeout(this._rightReleaseTimer);
                    this._rightReleaseTimer = setTimeout(() => {
                        if (!this._hasHeldHand('R')) {
                            this._rightActive = null;
                            this._requestRender();
                        }
                    }, Math.max(this._opts.duration, 100));
                } else {
                    this._lastLeftAt = now;
                    this._leftActive = {x: target.x, y: target.y};
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
        this._idleRelaxTimer = setTimeout(() => this._requestRender(), this._idleRaiseMs + 60);
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

        this._emitHud();
        this._requestRender();
    }

    handleWheel(dir) {
        this._lastPointerAt = performance.now();
        this._scheduleIdleRelax();
        this._wheelBadge = dir > 0 ? 'Scroll ↑' : 'Scroll ↓';
        if (this._mouseWheel) {
            this._mouseWheel.setAttribute('fill', '#38bdf8');
            this._mouseWheel.setAttribute('transform', `translate(0, ${dir > 0 ? -2.2 : 2.2})`);
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
            if (!this._sleeping && this._eyeLeft && this._eyeRight) {
                this._eyeLeft._blinkWrap.setAttribute('transform', 'scale(1, 0.12)');
                this._eyeRight._blinkWrap.setAttribute('transform', 'scale(1, 0.12)');
                setTimeout(() => {
                    this._eyeLeft._blinkWrap.removeAttribute('transform');
                    this._eyeRight._blinkWrap.removeAttribute('transform');
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
        const lerp = (a, b, f) => a + (b - a) * f;

        this._curPtrX = lerp(this._curPtrX, this._ptrX, 0.45);
        this._curPtrY = lerp(this._curPtrY, this._ptrY, 0.45);
        this._curEyeX = lerp(this._curEyeX, this._eyeTargetX, 0.28);
        this._curEyeY = lerp(this._curEyeY, this._eyeTargetY, 0.28);

        // Live2D-style face parallax + eye tracking toward REAL screen cursor!
        if (this._opts.eyes && !this._sleeping) {
            const fx = (this._curEyeX * 0.42).toFixed(2);
            const fy = (this._curEyeY * 0.36).toFixed(2);
            this._faceParallax.setAttribute('transform', `translate(${fx}, ${fy})`);

            const px = (this._curEyeX * 0.58).toFixed(2);
            const py = (this._curEyeY * 0.58).toFixed(2);
            this._eyeLeft._pupil.setAttribute('transform', `translate(${px}, ${py})`);
            this._eyeRight._pupil.setAttribute('transform', `translate(${px}, ${py})`);
        }

        const pad = this._padCoords(this._curPtrX, this._curPtrY);

        if (this._mouseGroup) {
            this._mouseGroup.setAttribute('transform', `translate(${pad.x.toFixed(1)}, ${pad.y.toFixed(1)})`);
        }

        if (this._tpRipple1 && this._tpRipple2) {
            const isRight = this._buttonsDown.has(2) || this._ptrFingers >= 2;
            const isMiddle = this._buttonsDown.has(3) || this._ptrFingers >= 3;
            const active = this._ptrFingers > 0 || this._buttonsDown.size > 0;
            const clicked = this._buttonsDown.size > 0;

            this._tpRipple1.setAttribute('cx', pad.x.toFixed(1));
            this._tpRipple1.setAttribute('cy', pad.y.toFixed(1));
            this._tpRipple1.setAttribute('r', clicked ? '12' : '8.5');
            this._tpRipple1.setAttribute('opacity', active ? '1' : '0');
            this._tpRipple1.classList.toggle('right-click', isRight && !isMiddle);
            this._tpRipple1.classList.toggle('middle-click', isMiddle);

            this._tpRipple2.setAttribute('cx', (pad.x - 14).toFixed(1));
            this._tpRipple2.setAttribute('cy', (pad.y - 2.5).toFixed(1));
            this._tpRipple2.setAttribute('opacity', (active && isRight) ? '0.9' : '0');
            this._tpRipple2.classList.toggle('right-click', isRight);
        }

        // 4-Stage Posture Lifecycle per hand:
        // Active -> Resting on palm-rest/pointer (< idleRaiseMs) -> Raised pink beans (>= idleRaiseMs or sleeping)
        const now = performance.now();
        const pointerActive = this._buttonsDown.size > 0 || this._ptrFingers > 0 || Boolean(this._wheelBadge);
        const rightKeyRecent = (now - this._lastRightKeyAt) < this._idleRaiseMs;
        const pointerRecent = (now - this._lastPointerAt) < this._idleRaiseMs;
        const leftRecent = (now - this._lastLeftAt) < this._idleRaiseMs;

        const rightShouldRaise = this._sleeping || (
            !this._rightActive && !this._hasHeldHand('R') && !pointerActive && !rightKeyRecent && !pointerRecent
        );
        const leftShouldRaise = this._sleeping || (
            !this._leftActive && !this._hasHeldHand('L') && !leftRecent
        );

        const targetRightRaise = rightShouldRaise ? 1 : 0;
        const targetLeftRaise = leftShouldRaise ? 1 : 0;
        this._rightRaise = lerp(this._rightRaise, targetRightRaise, targetRightRaise ? 0.16 : 0.42);
        this._leftRaise = lerp(this._leftRaise, targetLeftRaise, targetLeftRaise ? 0.16 : 0.42);
        if (Math.abs(this._rightRaise - targetRightRaise) < 0.01) this._rightRaise = targetRightRaise;
        if (Math.abs(this._leftRaise - targetLeftRaise) < 0.01) this._leftRaise = targetLeftRaise;

        // Right arm (screen-LEFT): Active key/pointer -> Resting on mouse or laptop palm-rest -> Raised!
        const typedMoreRecently = this._lastRightKeyAt > this._lastPointerAt;
        const restRightOnKeyboard = !this._isMouse && !pointerActive && this._opts.rightPaw !== 'pointer' && typedMoreRecently;

        let targetRX, targetRY, rightPressed;
        if (this._rightActive) {
            targetRX = this._rightActive.x;
            targetRY = this._rightActive.y;
            rightPressed = true;
        } else if (restRightOnKeyboard) {
            targetRX = this._palmRestR.x;
            targetRY = this._palmRestR.y;
            rightPressed = false;
        } else {
            const dip = this._buttonsDown.size > 0 ? 4.5 : 0;
            const hoverLift = (!this._isMouse && this._ptrFingers === 0 && !dip) ? -4.5 : 0;
            const mouseY = this._isMouse ? -10 : 0;
            targetRX = pad.x + (this._buttonsDown.has(2) ? -4.5 : (this._buttonsDown.has(1) ? 3.5 : 0));
            targetRY = pad.y + mouseY + dip + hoverLift;
            rightPressed = dip > 0 || this._ptrFingers > 0 || this._isMouse;
        }

        this._curRX = lerp(this._curRX, targetRX, rightPressed ? 0.56 : 0.24);
        this._curRY = lerp(this._curRY, targetRY, rightPressed ? 0.56 : 0.24);

        const armR = buildSeamlessArm(
            this._shoulderR.outer, this._shoulderR.inner,
            this._curRX, this._curRY, 'L', 20
        );
        const rArmAlpha = Math.max(0, Math.min(1, 1 - this._rightRaise));
        this._rightArmFill.setAttribute('d', armR.closed);
        this._rightArmStroke.setAttribute('d', armR.open);
        this._rightArmFill.setAttribute('opacity', rArmAlpha.toFixed(3));
        this._rightArmStroke.setAttribute('opacity', rArmAlpha.toFixed(3));
        this._rightShadow.setAttribute('cx', this._curRX.toFixed(1));
        this._rightShadow.setAttribute('cy', (this._curRY + 10).toFixed(1));
        this._rightShadow.setAttribute(
            'opacity',
            (!this._isMouse && rightPressed && rArmAlpha > 0.2) ? (0.24 * rArmAlpha).toFixed(3) : '0'
        );

        const rLift = (1 - this._rightRaise) * 14;
        this._rightIdlePaw.setAttribute(
            'transform',
            `translate(${this._catShiftX.toFixed(1)}, ${(this._catShiftY + rLift).toFixed(1)})`
        );
        this._rightIdlePaw.setAttribute('opacity', this._rightRaise.toFixed(3));

        // Left arm (screen-RIGHT): Active key -> Resting on palm-rest -> Raised pink beans!
        const targetLX = this._leftActive ? this._leftActive.x : this._palmRestL.x;
        const targetLY = this._leftActive ? this._leftActive.y : this._palmRestL.y;
        const leftPressed = Boolean(this._leftActive);
        this._curLX = lerp(this._curLX, targetLX, leftPressed ? 0.62 : 0.24);
        this._curLY = lerp(this._curLY, targetLY, leftPressed ? 0.62 : 0.24);

        const armL = buildSeamlessArm(
            this._shoulderL.outer, this._shoulderL.inner,
            this._curLX, this._curLY, 'R', 19.5
        );
        const lArmAlpha = Math.max(0, Math.min(1, 1 - this._leftRaise));
        this._leftArmFill.setAttribute('d', armL.closed);
        this._leftArmStroke.setAttribute('d', armL.open);
        this._leftArmFill.setAttribute('opacity', lArmAlpha.toFixed(3));
        this._leftArmStroke.setAttribute('opacity', lArmAlpha.toFixed(3));
        this._leftShadow.setAttribute('cx', this._curLX.toFixed(1));
        this._leftShadow.setAttribute('cy', (this._curLY + 8).toFixed(1));
        this._leftShadow.setAttribute(
            'opacity',
            (leftPressed && lArmAlpha > 0.2) ? (0.26 * lArmAlpha).toFixed(3) : '0'
        );

        const lLift = (1 - this._leftRaise) * 14;
        this._leftIdlePaw.setAttribute(
            'transform',
            `translate(${this._catShiftX.toFixed(1)}, ${(this._catShiftY + lLift).toFixed(1)})`
        );
        this._leftIdlePaw.setAttribute('opacity', this._leftRaise.toFixed(3));

        const settled =
            Math.abs(this._curPtrX - this._ptrX) < 0.002 &&
            Math.abs(this._curPtrY - this._ptrY) < 0.002 &&
            Math.abs(this._curEyeX - this._eyeTargetX) < 0.05 &&
            Math.abs(this._curEyeY - this._eyeTargetY) < 0.05 &&
            Math.abs(this._rightRaise - targetRightRaise) < 0.01 &&
            Math.abs(this._leftRaise - targetLeftRaise) < 0.01 &&
            Math.abs(this._curRX - targetRX) < 0.25 &&
            Math.abs(this._curRY - targetRY) < 0.25 &&
            Math.abs(this._curLX - targetLX) < 0.25 &&
            Math.abs(this._curLY - targetLY) < 0.25;

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
