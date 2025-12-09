// Three.js Background Animation
// Implements a particle system with "Human Figure" dissolution and fluid interaction

import * as THREE from 'https://unpkg.com/three@0.160.0/build/three.module.js';

const container = document.getElementById('canvas-container');

// Scene Setup
const scene = new THREE.Scene();
// Fog for that "foggy/dark" atmosphere
scene.fog = new THREE.FogExp2(0x111111, 0.002);

const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
camera.position.z = 50;
camera.position.y = 10;

const renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
renderer.setSize(window.innerWidth, window.innerHeight);
renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
container.appendChild(renderer.domElement);

// Global Uniforms
const uniforms = {
    uTime: { value: 0 },
    uMouse: { value: new THREE.Vector2(0, 0) },
    uResolution: { value: new THREE.Vector2(window.innerWidth, window.innerHeight) },
    uDissolve: { value: 0.0 } // 0 = Human, 1 = Dissolved
};

// --- Particle System Setup ---
const count = 60000; // Number of particles
const geometry = new THREE.BufferGeometry();
const positions = new Float32Array(count * 3);
const targetPositions = new Float32Array(count * 3);
const baseCurveParams = { freq: 1, amp: 1, twist: 1 }; // Store current curve params
const randomPositions = new Float32Array(count * 3);
const randoms = new Float32Array(count);

// Generate Abstract "Dance" Shape (Spiraling Curve)
// Multi-Limb: 5 clustered lines GRAVITATING towards a focal point
function generateSpiralShape(positionsArray, centerPos, offsetTime = 0) {
    const numLimbs = 5;
    const particlesPerLimb = Math.floor(count / numLimbs);

    // Reusable helpers for transform
    const tempVec = new THREE.Vector3();
    const rotationMatrix = new THREE.Matrix4();
    const rotationEuler = new THREE.Euler();

    // Base parameters
    // Increased height by ~30% (8.0 -> 10.5)
    const height = 10.5;

    // Attractor Tightness: 0.0 (tight) to 1.0 (loose)
    // Randomize how closely limbs cluster to the focal point
    const spreadFactor = Math.random();

    for (let l = 0; l < numLimbs; l++) {
        // 1. Generate Unique Transform for this Limb
        // Random Rotation (Full 3D)
        rotationEuler.set(
            Math.random() * Math.PI * 2, // X
            Math.random() * Math.PI * 2, // Y
            Math.random() * Math.PI * 2  // Z
        );
        rotationMatrix.makeRotationFromEuler(rotationEuler);

        // Random Position Offset (Clustered around centerPos)
        // Scale offset by spreadFactor (0 to 20% nearness logic)
        // Base spread range 15 units. 
        // If spreadFactor is small, offsets are small (tight).
        const spread = 5.0 + (spreadFactor * 20.0);

        const limbOffsetX = (Math.random() - 0.5) * spread;
        const limbOffsetY = (Math.random() - 0.5) * spread;
        const limbOffsetZ = (Math.random() - 0.5) * spread;

        // Curve Params
        const limbFreq = 2.0 + Math.random() * 3.0;
        const limbAmp = 2.5 + Math.random() * 4.5; // Slightly larger amplitude too
        const limbTwist = 2.0 + Math.random() * 4.0;

        for (let i = 0; i < particlesPerLimb; i++) {
            const idx = l * particlesPerLimb + i;
            if (idx >= count) break;

            const t = i / particlesPerLimb;

            // Core Curve (Vertical initially)
            const y = (t - 0.5) * height;

            // Wavy envelope
            const curveX = Math.sin(t * limbFreq * Math.PI + offsetTime) * limbAmp * Math.sin(t * Math.PI);
            const curveZ = Math.cos(t * limbFreq * Math.PI + offsetTime) * limbAmp * Math.sin(t * Math.PI);

            // Spiral Position
            const spiralRadius = 0.5 + Math.random() * 1.0;
            const angle = t * limbTwist * Math.PI * 2 + (Math.random() * Math.PI * 2);
            const r = spiralRadius * (0.5 + Math.random() * 0.5);

            const spiralX = Math.cos(angle) * r;
            const spiralZ = Math.sin(angle) * r;

            // Base Position (Local Space)
            tempVec.set(curveX + spiralX, y, curveZ + spiralZ);

            // Apply Limb Transform
            tempVec.applyMatrix4(rotationMatrix);

            // Apply Cluster Offset + Focal Point
            tempVec.x += limbOffsetX + centerPos.x;
            tempVec.y += limbOffsetY + centerPos.y;
            tempVec.z += limbOffsetZ + centerPos.z;

            positionsArray[idx * 3] = tempVec.x;
            positionsArray[idx * 3 + 1] = tempVec.y;
            positionsArray[idx * 3 + 2] = tempVec.z;
        }
    }
}

// Initial Generation
generateSpiralShape(targetPositions, new THREE.Vector3(0, 0, 0));

// Initialize Random (Dissolved) Positions
for (let i = 0; i < count; i++) {
    randomPositions[i * 3] = (Math.random() - 0.5) * 150;
    randomPositions[i * 3 + 1] = (Math.random() - 0.5) * 100;
    randomPositions[i * 3 + 2] = (Math.random() - 0.5) * 100;

    // Initial Position = Random (Start Dispersed)
    positions[i * 3] = randomPositions[i * 3];
    positions[i * 3 + 1] = randomPositions[i * 3 + 1];
    positions[i * 3 + 2] = randomPositions[i * 3 + 2];

    randoms[i] = Math.random();
}

geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
geometry.setAttribute('aTarget', new THREE.BufferAttribute(targetPositions, 3));
geometry.setAttribute('aRandom', new THREE.BufferAttribute(randomPositions, 3));
geometry.setAttribute('aRandSeed', new THREE.BufferAttribute(randoms, 1));

// --- Dynamic Positioning Logic ---
function updateTargetPositions() {
    // 1. Pick a Random Focal Point (Depth-based)
    // Z: -150 (far) to +30 (close)
    // Camera is at Z=50
    const centerZ = (Math.random() * 180) - 150;

    // Calculate visible bounds at this depth to ensure it's on screen
    const distance = camera.position.z - centerZ;
    const vFOV = THREE.MathUtils.degToRad(camera.fov); // 75 deg
    const visibleHeight = 2 * Math.tan(vFOV / 2) * distance;
    const visibleWidth = visibleHeight * camera.aspect;

    // Keep it somewhat central (within 80% of bounds)
    const centerX = (Math.random() - 0.5) * visibleWidth * 0.8;
    const centerY = (Math.random() - 0.5) * visibleHeight * 0.8;

    const centerPos = new THREE.Vector3(centerX, centerY, centerZ);

    // 2. Generate Shape around this point
    generateSpiralShape(geometry.attributes.aTarget.array, centerPos, Math.random() * 100);
    geometry.attributes.aTarget.needsUpdate = true;
}

// --- Color Integration ---
// Get CSS variable --clr-secondary
const cssColor = getComputedStyle(document.documentElement).getPropertyValue('--clr-secondary').trim();
const themeColor = new THREE.Color(cssColor || '#A8A8B3'); // Fallback if parsing fails

uniforms.uColor = { value: themeColor };

// --- Shader Material ---
const vertexShader = `
    uniform float uTime;
    uniform float uDissolve;
    uniform vec2 uMouse;
    uniform vec2 uResolution;
    
    attribute vec3 aTarget;
    attribute vec3 aRandom;
    attribute float aRandSeed;
    
    varying float vAlpha;
    
    void main() {
        // 1. Dissolve Interpolation
        float t = smoothstep(aRandSeed * 0.2, 1.0 - aRandSeed * 0.2, uDissolve);
        vec3 pos = mix(aTarget, aRandom, t);
        
        // 2. Fluid/Mouse Interaction
        vec4 clipPos = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
        vec2 screenPos = clipPos.xy / clipPos.w;
        
        float aspect = uResolution.x / uResolution.y;
        vec2 mouseCorrected = uMouse;
        mouseCorrected.x *= aspect;
        vec2 screenPosCorrected = screenPos;
        screenPosCorrected.x *= aspect;
        
        float dist = distance(screenPosCorrected, mouseCorrected);
        float radius = 0.3;
        float force = smoothstep(radius, 0.0, dist);
        
        // Stirring effect
        float wave = sin(uTime * 2.0 + pos.y * 0.5) * 0.5;
        pos.x += wave * (1.0 - uDissolve); 
        
        // Stronger mouse interaction as requested (5.0 -> 15.0)
        pos.x += (screenPos.x - uMouse.x) * force * 15.0;
        pos.y += (screenPos.y - uMouse.y) * force * 15.0;
        
        // Ocean drift
        pos.y += sin(uTime * 0.5 + pos.x * 0.1) * 0.2;
        
        vec4 mvPosition = modelViewMatrix * vec4(pos, 1.0);
        gl_Position = projectionMatrix * mvPosition;
        
        // Dynamic Size: Smaller when condensed (0.0), Larger when dispersed (1.0)
        float sizeState = mix(0.6, 1.5, uDissolve);
        
        // Size attenuation
        gl_PointSize = (sizeState + aRandSeed * 0.4) * (100.0 / -mvPosition.z);
        
        // Opacity Logic:
        // t=0 (Condensed): We want it dimmer (background brightness).
        // t=1 (Dispersed): We want it visible but soft.
        // Previous: (1.0 - t * 0.8) -> Condensed=1.0, Dispersed=0.2
        // New: mix(0.3, 0.8, t) -> Condensed=0.3, Dispersed=0.8
        float baseOpacity = mix(0.3, 0.8, t);
        
        vAlpha = baseOpacity * (0.5 + 0.5 * sin(uTime + aRandSeed * 10.0));
    }
`;

const fragmentShader = `
    varying float vAlpha;
    uniform vec3 uColor;
    
    void main() {
        vec2 coord = gl_PointCoord - vec2(0.5);
        float dist = length(coord);
        if (dist > 0.5) discard;
        float alpha = 1.0 - smoothstep(0.4, 0.5, dist);
        
        // Use theme color
        gl_FragColor = vec4(uColor, alpha * vAlpha);
    }
`;

const material = new THREE.ShaderMaterial({
    vertexShader,
    fragmentShader,
    uniforms,
    transparent: true,
    depthWrite: false,
    blending: THREE.AdditiveBlending
});

const points = new THREE.Points(geometry, material);
scene.add(points);

// --- Animation Loop ---
const clock = new THREE.Clock();

// Animation State Machine
const STATE_DISPERSED = 0;
const STATE_CONDENSING = 1;
const STATE_DISPERSING = 3;

let currentState = STATE_DISPERSED;
let stateTimer = 0;

// Continuous random cycle duration (7s to 20s)
let currentCycleDuration = 10.0;

// Initial State: Dispersed
uniforms.uDissolve.value = 1.0;

let lastFrameTime = 0;

function animate() {
    requestAnimationFrame(animate);

    const elapsedTime = clock.getElapsedTime();
    // Robust delta calculation
    let deltaTime = elapsedTime - lastFrameTime;
    lastFrameTime = elapsedTime;

    // Cap delta to prevent huge jumps (e.g. tab switch)
    if (deltaTime > 0.1) deltaTime = 0.1;

    uniforms.uTime.value = elapsedTime;

    // State Machine Logic
    stateTimer += deltaTime;

    switch (currentState) {
        case STATE_DISPERSED:
            // Start condensing immediately (continuous)
            // Pick a random duration for this cycle
            currentCycleDuration = 7.0 + Math.random() * 13.0; // 7 to 20 seconds

            console.log("State: CONDENSING (Duration: " + currentCycleDuration.toFixed(1) + "s)");
            currentState = STATE_CONDENSING;
            stateTimer = 0;
            updateTargetPositions(); // New position/rotation/cluster
            break;

        case STATE_CONDENSING:
            // Tween 1.0 -> 0.0
            if (stateTimer >= currentCycleDuration) {
                console.log("State: DISPERSING");
                currentState = STATE_DISPERSING; // Skip HOLD, go straight to dispersing
                stateTimer = 0;
                uniforms.uDissolve.value = 0.0;
            } else {
                const progress = stateTimer / currentCycleDuration;
                // Smooth ease-in-out
                const t = progress < 0.5 ? 2 * progress * progress : -1 + (4 - 2 * progress) * progress;
                uniforms.uDissolve.value = 1.0 - t;
            }
            break;

        case STATE_DISPERSING:
            // Tween 0.0 -> 1.0
            if (stateTimer >= currentCycleDuration) {
                console.log("State: DISPERSED");
                currentState = STATE_DISPERSED;
                stateTimer = 0;
                uniforms.uDissolve.value = 1.0;
            } else {
                const progress = stateTimer / currentCycleDuration;
                const t = progress < 0.5 ? 2 * progress * progress : -1 + (4 - 2 * progress) * progress;
                uniforms.uDissolve.value = t;
            }
            break;
    }

    // Gentle camera movement
    camera.position.x += (Math.sin(elapsedTime * 0.1) * 0.5 - camera.position.x) * 0.05;
    camera.position.y += (Math.cos(elapsedTime * 0.1) * 0.5 + 10 - camera.position.y) * 0.05;
    camera.lookAt(0, 0, 0);

    renderer.render(scene, camera);
}

animate();

// --- Resize Handler ---
window.addEventListener('resize', () => {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(window.innerWidth, window.innerHeight);
    uniforms.uResolution.value.set(window.innerWidth, window.innerHeight);
});

// --- Mouse Handler ---
document.addEventListener('mousemove', (e) => {
    // Normalize mouse to -1 to 1
    uniforms.uMouse.value.x = (e.clientX / window.innerWidth) * 2 - 1;
    uniforms.uMouse.value.y = -(e.clientY / window.innerHeight) * 2 + 1;
});
