import type {
  GetSongsResponse,
  GetSongResponse,
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
  return await invoke<GetSongsResponse>('get_songs')
}

export async function getSong(uid: string): Promise<GetSongResponse> {
  return await invoke<GetSongResponse>('get_song', { uid })
}

export async function uploadSongFile(): Promise<AddSongResponse> {
  const file = await open({ filters: [{ name: 'MP3 Audio', extensions: ['mp3'] }] })
  if (!file) return { success: false, error: 'No file selected' }
  return await invoke<AddSongResponse>('import_song_audio', { sourcePath: file })
}

export async function updateSong(data: UpdateSongRequest): Promise<UpdateSongResponse> {
  return await invoke<UpdateSongResponse>('update_song', { uid: data.uid, info: data.info })
}

export async function deleteSong(uid: string): Promise<DeleteSongResponse> {
  return await invoke<DeleteSongResponse>('delete_song', { uid })
}

export async function getSongAudioUrl(uid: string): Promise<GetSongAudioUrlResponse> {
  const ret = await invoke<GetSongAudioUrlResponse>('get_song_audio_url', { uid })
  if (!ret.success || !ret.data || !ret.data.url) {
    return { success: false, error: ret.error || 'Failed to get song audio URL' }
  }
  ret.data.url = convertFileSrc(ret.data.url)
  return ret;
}