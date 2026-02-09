let scene, camera, renderer, papers = [], decoys = [];
let state = 'TITLE'; 
const SEARCH_INPUT = document.getElementById('searchInput');
const chars = ["D", "O", "C", "-", "V", "A", "U", "L", "T"];

function createTextTexture(char) {
    const canvas = document.createElement('canvas');
    canvas.width = 256; canvas.height = 350;
    const ctx = canvas.getContext('2d');
    ctx.fillStyle = "white"; ctx.fillRect(0, 0, 256, 350);
    ctx.fillStyle = "black"; ctx.font = "800 160px Syne"; ctx.textAlign = "center";
    ctx.fillText(char, 128, 210);
    return new THREE.CanvasTexture(canvas);
}

function init() {
    scene = new THREE.Scene();
    camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(window.innerWidth, window.innerHeight);
    document.getElementById('canvas-container').appendChild(renderer.domElement);

    const verticalCenter = 3.0; // Point 1: Centers 3D objects relative to high search bar

    chars.forEach((char, i) => {
        const geometry = new THREE.BoxGeometry(0.8, 1.1, 0.15); 
        const materials = Array(6).fill(new THREE.MeshPhongMaterial({color: 0xeeeeee}));
        materials[4] = new THREE.MeshPhongMaterial({map: createTextTexture(char)});
        const paper = new THREE.Mesh(geometry, materials);
        
        // Initial Spell-out Position (Higher Up)
        paper.position.set((i - 4) * 1.1, verticalCenter + 2, 0); 
        paper.userData = { angle: (i / chars.length) * Math.PI * 2, center: verticalCenter };
        papers.push(paper);
        scene.add(paper);
    });

    for(let j=0; j<45; j++) {
        const dGeo = new THREE.PlaneGeometry(0.5, 0.7);
        const dMat = new THREE.MeshPhongMaterial({color: 0x444444, transparent: true, opacity: 0.1});
        const decoy = new THREE.Mesh(dGeo, dMat);
        decoy.position.set(Math.random()*40-20, Math.random()*40-20, -15);
        decoys.push(decoy);
        scene.add(decoy);
    }

    const light = new THREE.PointLight(0xffffff, 1.2, 100);
    light.position.set(5, 5, 15);
    scene.add(light);
    scene.add(new THREE.AmbientLight(0x404040));
    camera.position.z = 12;
}

SEARCH_INPUT.addEventListener('focus', () => {
    state = 'SEARCH';
    document.body.classList.add('light-mode');
});

SEARCH_INPUT.addEventListener('blur', () => {
    if (SEARCH_INPUT.value === "") {
        state = 'TITLE';
        document.body.classList.remove('light-mode');
        papers.forEach((p, i) => {
            gsap.to(p.position, { 
                x: (i - 4) * 1.1, 
                y: p.userData.center + 2, 
                z: 0, 
                duration: 3.5, 
                ease: "power2.inOut" 
            });
            gsap.to(p.rotation, { x: 0, y: 0, z: 0, duration: 3.5 });
        });
    }
});

function animate() {
    requestAnimationFrame(animate);
    const time = Date.now() * 0.0003;

    if (state === 'SEARCH') {
        papers.forEach((p, i) => {
            const radiusX = 10; // Point 1: Wide orbit to stay away from search bar
            const radiusY = 4.5; // Tall orbit to stay away from text blocks
            const orbitAngle = time + p.userData.angle;
            
            p.position.x = Math.cos(orbitAngle) * radiusX;
            // Center the Y-rotation on the search bar's vertical coordinate
            p.position.y = Math.sin(orbitAngle) * radiusY + p.userData.center; 
            p.position.z = 4;
            
            p.rotation.y += 0.01;
            p.rotation.x += 0.005;
        });
    }

    decoys.forEach((d) => { d.rotation.z += 0.001; });
    renderer.render(scene, camera);
}

function setTheme(theme) { document.body.setAttribute('data-theme', theme); }
init();
animate();