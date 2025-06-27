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
  try {
    return await invoke<GetSessionsResponse>('get_sessions')
  } catch (error) {
    return { success: false, error: 'Failed to get sessions: ' + (error as Error).message } as GetSessionsResponse
  }
}

export async function getSession(uid: string): Promise<GetSessionResponse> {
  try {
    return await invoke<GetSessionResponse>('get_session', { uid })
  } catch (error) {
    return { success: false, error: 'Failed to get session: ' + (error as Error).message } as GetSessionResponse
  }
}

export async function createSession(data: CreateSessionRequest): Promise<CreateSessionResponse> {
  try {
    return await invoke<CreateSessionResponse>('create_session', { ...data })
  } catch (error) {
    return { success: false, error: 'Failed to create session: ' + (error as Error).message } as CreateSessionResponse
  }
}

export async function updateSession(data: UpdateSessionRequest): Promise<UpdateSessionResponse> {
  try {
    return await invoke<UpdateSessionResponse>('update_session', { ...data })
  } catch (error) {
    return { success: false, error: 'Failed to update session: ' + (error as Error).message } as UpdateSessionResponse
  }
}

export async function deleteSession(uid: string): Promise<DeleteSessionResponse> {
  try {
    await invoke('delete_session', { uid })
    return { success: true } as DeleteSessionResponse
  } catch (error) {
    return { success: false, error: 'Failed to delete session: ' + (error as Error).message } as DeleteSessionResponse
  }
}