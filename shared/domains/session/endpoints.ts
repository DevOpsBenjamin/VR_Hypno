import type {
  GetSessionsResponse,
  GetSessionResponse,
  CreateSessionRequest,
  CreateSessionResponse,
  UpdateSessionRequest,
  UpdateSessionResponse,
  DeleteSessionResponse
} from './api'

// All endpoints return empty for debugging purposes

export async function getSessions(): Promise<GetSessionsResponse> {
  return {} as GetSessionsResponse
}

export async function getSession(uid: string): Promise<GetSessionResponse> {
  return {} as GetSessionResponse
}

export async function createSession(data: CreateSessionRequest): Promise<CreateSessionResponse> {
  return {} as CreateSessionResponse
}

export async function updateSession(data: UpdateSessionRequest): Promise<UpdateSessionResponse> {
  return {} as UpdateSessionResponse
}

export async function deleteSession(uid: string): Promise<DeleteSessionResponse> {
  return {} as DeleteSessionResponse
}