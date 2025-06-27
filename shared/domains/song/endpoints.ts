import type {
  GetSongsResponse,
  GetSongResponse,
  AddSongRequest,
  AddSongResponse,
  UpdateSongRequest,
  UpdateSongResponse,
  DeleteSongResponse
} from './api'

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

export async function uploadSongFile(uid: string, file: File): Promise<{ success: boolean; error?: string }> {
  return { success: false, error: 'Not implemented' }
}

export async function updateSong(data: UpdateSongRequest): Promise<UpdateSongResponse> {
  return {} as UpdateSongResponse
}

export async function deleteSong(uid: string): Promise<DeleteSongResponse> {
  return {} as DeleteSongResponse
}