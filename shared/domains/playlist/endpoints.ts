import type {
    GetPlaylistsResponse,
    GetPlaylistResponse,
    CreatePlaylistRequest,
    CreatePlaylistResponse,
    UpdatePlaylistRequest,
    UpdatePlaylistResponse
} from './api'
import { invoke } from '@tauri-apps/api/tauri'

export async function getPlaylists(): Promise<GetPlaylistsResponse> {
    try {
        return await invoke<GetPlaylistsResponse>('get_playlists')
    } catch (error) {
        return { success: false, error: 'Failed to get playlists: ' + (error as Error).message } as GetPlaylistsResponse
    }
}

export async function getPlaylist(uid: string): Promise<GetPlaylistResponse> {
    try {
        return await invoke<GetPlaylistResponse>('get_playlist', { uid })
    } catch (error) {
        return { success: false, error: 'Failed to get playlist: ' + (error as Error).message } as GetPlaylistResponse
    }
}

export async function createPlaylist(data: CreatePlaylistRequest): Promise<CreatePlaylistResponse> {
    try {
        return await invoke<CreatePlaylistResponse>('create_playlist', { ...data })
    } catch (error) {
        return { success: false, error: 'Failed to create playlist: ' + (error as Error).message } as CreatePlaylistResponse
    }
}

export async function updatePlaylist(data: UpdatePlaylistRequest): Promise<UpdatePlaylistResponse> {
    try {
        return await invoke<UpdatePlaylistResponse>('update_playlist', { ...data })
    } catch (error) {
        return { success: false, error: 'Failed to update playlist: ' + (error as Error).message } as UpdatePlaylistResponse
    }
}

export async function deletePlaylist(uid: string): Promise<{ success: boolean; error?: string }> {
    try {
        await invoke('delete_playlist', { uid })
        return { success: true }
    } catch (error) {
        return { success: false, error: 'Failed to delete playlist: ' + (error as Error).message }
    }
}