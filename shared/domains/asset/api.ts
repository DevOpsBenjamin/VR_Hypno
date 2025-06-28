import type { Asset } from './types'
import type { AssetInfo } from './types'
import type { ApiResponse } from '../api'

export type GetAssetsResponse = ApiResponse<{ assets: Asset[] }>
export type GetAssetResponse = ApiResponse<{ asset: Asset }>
export type AddAssetResponse = ApiResponse<{ asset: Asset }>
export type UpdateAssetRequest = { uid: string; info: AssetInfo }
export type UpdateAssetResponse = ApiResponse<{}>
export type DeleteAssetResponse = ApiResponse<{}>
export type GetAssetUrlResponse = ApiResponse<{ url: string }>