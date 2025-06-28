import { PlayerManager } from '@/core/PlayerManager';

let instance: PlayerManager | null = null;
export function usePlayerManager() {
  if (!instance) instance = new PlayerManager();
  return instance;
}