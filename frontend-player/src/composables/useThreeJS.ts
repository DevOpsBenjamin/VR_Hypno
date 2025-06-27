import { ref, onMounted, onUnmounted, watch, isRef } from 'vue';
import * as THREE from 'three';
import type { ThreeJSConfig } from '@shared/types';

export function useThreeJS(canvasId: string, configSource?: ThreeJSConfig | null | import('vue').Ref<ThreeJSConfig | null>) {
  const renderer = ref<THREE.WebGLRenderer | null>(null);
  const scene = ref<THREE.Scene | null>(null);
  const camera = ref<THREE.PerspectiveCamera | null>(null);
  let animationId: number | null = null;

  function initThree() {
    const canvas = document.getElementById(canvasId) as HTMLCanvasElement | null;
    if (!canvas) return;
    renderer.value = new THREE.WebGLRenderer({ canvas });
    renderer.value.setSize(window.innerWidth, window.innerHeight);
    scene.value = new THREE.Scene();
    camera.value = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    camera.value.position.z = 5;
    animate();
  }

  function animate() {
    if (!renderer.value || !scene.value || !camera.value) return;
    animationId = requestAnimationFrame(animate);
    renderer.value.render(scene.value, camera.value);
  }

  function cleanup() {
    if (animationId) cancelAnimationFrame(animationId);
    renderer.value = null;
    scene.value = null;
    camera.value = null;
  }

  // Watch for config changes and apply to scene
  if (configSource) {
    if (isRef(configSource)) {
      watch(
        configSource,
        (config) => {
          if (config && scene.value && camera.value) {
            while (scene.value.children.length > 0) {
              scene.value.remove(scene.value.children[0]);
            }
            applyThreeJSConfig(config, scene.value, camera.value);
          }
        },
        { immediate: true }
      );
    } else {
      // configSource is a direct object
      if (configSource && scene.value && camera.value) {
        applyThreeJSConfig(configSource, scene.value, camera.value);
      }
    }
  }

  onMounted(initThree);
  onUnmounted(cleanup);

  return { renderer, scene, camera };
}

export function applyThreeJSConfig(config: ThreeJSConfig, scene: THREE.Scene, camera: THREE.PerspectiveCamera) {
  if (config.backgroundColor) {
    scene.background = new THREE.Color(config.backgroundColor);
  }
  if (config.camera) {
    camera.position.set(...config.camera.position);
    if (config.camera.fov) camera.fov = config.camera.fov;
    camera.updateProjectionMatrix();
  }
  if (config.objects) {
    for (const obj of config.objects) {
      let mesh: THREE.Mesh | null = null;
      if (obj.type === 'box') {
        const geometry = new THREE.BoxGeometry(...(obj.size || [1, 1, 1]));
        const material = new THREE.MeshStandardMaterial({ color: obj.color || '#fff' });
        mesh = new THREE.Mesh(geometry, material);
      } else if (obj.type === 'sphere') {
        const geometry = new THREE.SphereGeometry((obj.size?.[0] || 1) / 2);
        const material = new THREE.MeshStandardMaterial({ color: obj.color || '#fff' });
        mesh = new THREE.Mesh(geometry, material);
      }
      // TODO: Add support for 'custom' and more types
      if (mesh) {
        mesh.position.set(...obj.position);
        scene.add(mesh);
      }
    }
  }
}
