import type {
  GetSessionsResponse,
  GetSessionResponse,
  CreateSessionRequest,
  CreateSessionResponse,
  UpdateSessionRequest,
  UpdateSessionResponse,
  DeleteSessionResponse
} from './api'
import { invoke } from '@tauri-apps/api/tauri'

export async function getSessions(): Promise<GetSessionsResponse> {
  return await invoke<GetSessionsResponse>('get_sessions')
}

export async function getSession(uid: string): Promise<GetSessionResponse> {
  return await invoke<GetSessionResponse>('get_session', { uid })
}

export async function createSession(data: CreateSessionRequest): Promise<CreateSessionResponse> {
  return await invoke<CreateSessionResponse>('create_session', { ...data })
}

export async function updateSession(data: UpdateSessionRequest): Promise<UpdateSessionResponse> {
  return await invoke<UpdateSessionResponse>('update_session', { ...data })
}

export async function deleteSession(uid: string): Promise<DeleteSessionResponse> {
  return await invoke<DeleteSessionResponse>('delete_session', { uid })
}