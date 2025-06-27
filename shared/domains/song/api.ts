import type { Song } from './types'
import type { SongInfo } from './types'
import type { ApiResponse } from '../api'

export type GetSongsResponse = ApiResponse<{ songs: Song[] }>
export type GetSongResponse = ApiResponse<{ song: Song }>
export type AddSongResponse = ApiResponse<{ song: Song }>
export type UpdateSongRequest = { uid: string; info: SongInfo }
export type UpdateSongResponse = ApiResponse<{}>
export type DeleteSongResponse = ApiResponse<{}>
export type GetSongAudioUrlResponse = ApiResponse<{ url: string }>