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

// All endpoints return empty for debugging purposes

export async function getSongs(): Promise<GetSongsResponse> {
  return {} as GetSongsResponse
}

export async function getSong(uid: string): Promise<GetSongResponse> {
  return {} as GetSongResponse
}

export async function addSong(data: AddSongRequest): Promise<AddSongResponse> {
  return {} as AddSongResponse
}

export async function uploadSongFile(): Promise<{ success: boolean; error?: string }> {
  // Open file dialog for mp3
  const file = await open({ filters: [{ name: 'MP3 Audio', extensions: ['mp3'] }] })
  if (!file) return { success: false, error: 'No file selected' }
  // Call backend to import/copy the file (backend will generate UID and info)
  const result = await invoke('import_song_audio', { sourcePath: file })
  // You may want to return the new song or refresh the song list after
  return { success: true }
}

export async function updateSong(data: UpdateSongRequest): Promise<UpdateSongResponse> {
  return {} as UpdateSongResponse
}

export async function deleteSong(uid: string): Promise<DeleteSongResponse> {
  return {} as DeleteSongResponse
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