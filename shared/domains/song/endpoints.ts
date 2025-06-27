import type {
  GetSongsResponse,
  GetSongResponse,
  AddSongRequest,
  AddSongResponse,
  UpdateSongRequest,
  UpdateSongResponse,
  DeleteSongResponse,
  GetSongAudioUrlResponse
} from './api'
import { open } from '@tauri-apps/api/dialog'
import { invoke } from '@tauri-apps/api/tauri'
import { convertFileSrc } from '@tauri-apps/api/tauri'

export async function getSongs(): Promise<GetSongsResponse> {
  try {
    const data = await invoke<GetSongsResponse>('get_songs')
    return data
  } catch (error) {
    return { success: false, error: 'Failed to get songs: ' + (error as Error).message } as GetSongsResponse
  }
}

export async function getSong(uid: string): Promise<GetSongResponse> {
  try {
    const data = await invoke<GetSongResponse>('get_song', { uid })
    return data
  } catch (error) {
    return { success: false, error: 'Failed to get song: ' + (error as Error).message } as GetSongResponse
  }
}

export async function addSong(data: AddSongRequest): Promise<AddSongResponse> {
  try {
    const result = await invoke<AddSongResponse>('add_song', { info: data.info })
    return result
  } catch (error) {
    return { success: false, error: 'Failed to add song: ' + (error as Error).message } as AddSongResponse
  }
}

export async function uploadSongFile(): Promise<{ success: boolean; error?: string }> {
  const file = await open({ filters: [{ name: 'MP3 Audio', extensions: ['mp3'] }] })
  if (!file) return { success: false, error: 'No file selected' }
  try {
    await invoke('import_song_audio', { sourcePath: file })
    return { success: true }
  } catch (error) {
    return { success: false, error: 'Failed to upload song file: ' + (error as Error).message }
  }
}

export async function updateSong(data: UpdateSongRequest): Promise<UpdateSongResponse> {
  try {
    await invoke('update_song', { uid: data.uid, info: data.info })
    return { success: true } as UpdateSongResponse
  } catch (error) {
    return { success: false, error: 'Failed to update song: ' + (error as Error).message } as UpdateSongResponse
  }
}

export async function deleteSong(uid: string): Promise<DeleteSongResponse> {
  try {
    await invoke('delete_song', { uid })
    return { success: true } as DeleteSongResponse
  } catch (error) {
    return { success: false, error: 'Failed to delete song: ' + (error as Error).message } as DeleteSongResponse
  }
}

export async function getSongAudioUrl(uid: string): Promise<GetSongAudioUrlResponse> {
  try {
    const path = await invoke<string>('get_song_audio_url', { uid })
    if (!path) {
      return { success: false, error: 'No path returned' }
    }
    const url = convertFileSrc(path)
    return { success: true, data: { url } }
  } catch (error) {
    return { success: false, error: 'Failed to get song audio URL: ' + (error as Error).message }
  }
}