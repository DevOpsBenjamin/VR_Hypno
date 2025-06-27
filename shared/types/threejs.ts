export interface ThreeJSConfig {
  backgroundColor?: string;
  objects?: Array<{
    type: 'box' | 'sphere' | 'custom';
    position: [number, number, number];
    color?: string;
    size?: [number, number, number];
    // ...other ThreeJS object params
  }>;
  camera?: {
    position: [number, number, number];
    fov?: number;
  };
  // ...other scene-level config
}
