import type {
    GetPlaylistsResponse,
    GetPlaylistResponse,
    CreatePlaylistRequest,
    CreatePlaylistResponse,
    UpdatePlaylistRequest,
    UpdatePlaylistResponse,
    DeletePlaylistResponse
} from './api'
import { invoke } from '@tauri-apps/api/tauri'

export async function getPlaylists(): Promise<GetPlaylistsResponse> {
  return await invoke<GetPlaylistsResponse>('get_playlists')
}

export async function getPlaylist(uid: string): Promise<GetPlaylistResponse> {
  return await invoke<GetPlaylistResponse>('get_playlist', { uid })
}

export async function createPlaylist(data: CreatePlaylistRequest): Promise<CreatePlaylistResponse> {
  return await invoke<CreatePlaylistResponse>('create_playlist', { ...data })
}

export async function updatePlaylist(data: UpdatePlaylistRequest): Promise<UpdatePlaylistResponse> {
  return await invoke<UpdatePlaylistResponse>('update_playlist', { ...data })
}

export async function deletePlaylist(uid: string): Promise<DeletePlaylistResponse> {
  return await invoke<DeletePlaylistResponse>('delete_playlist', { uid })
}
