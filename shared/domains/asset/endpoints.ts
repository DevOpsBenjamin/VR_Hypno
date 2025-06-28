import type {
  GetAssetsResponse,
  GetAssetResponse,
  AddAssetResponse,
  UpdateAssetRequest,
  UpdateAssetResponse,
  DeleteAssetResponse,
  GetAssetUrlResponse
} from './api'
import { open } from '@tauri-apps/api/dialog'
import { invoke } from '@tauri-apps/api/tauri'
import { convertFileSrc } from '@tauri-apps/api/tauri'

export async function getAssets(): Promise<GetAssetsResponse> {
  return await invoke<GetAssetsResponse>('get_assets')
}

export async function getAsset(uid: string): Promise<GetAssetResponse> {
  return await invoke<GetAssetResponse>('get_asset', { uid })
}

export async function uploadAssetFile(): Promise<AddAssetResponse> {
  const file = await open({ filters: [{ name: 'GLB Model', extensions: ['glb'] }] })
  if (!file) return { success: false, error: 'No file selected' }
  return await invoke<AddAssetResponse>('import_asset_obj', { sourcePath: file })
}

export async function updateAsset(data: UpdateAssetRequest): Promise<UpdateAssetResponse> {
  return await invoke<UpdateAssetResponse>('update_asset', { uid: data.uid, info: data.info })
}

export async function deleteAsset(uid: string): Promise<DeleteAssetResponse> {
  return await invoke<DeleteAssetResponse>('delete_asset', { uid })
}

export async function getAssetUrl(uid: string): Promise<GetAssetUrlResponse> {
  const ret = await invoke<GetAssetUrlResponse>('get_asset_obj', { uid })
  if (!ret.success || !ret.data || !ret.data.url) {
    return { success: false, error: ret.error || 'Failed to get asset URL' }
  }
  ret.data.url = convertFileSrc(ret.data.url)
  return ret;
}