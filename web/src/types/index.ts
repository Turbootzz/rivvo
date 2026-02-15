export interface User {
  id: string
  email: string
  name: string
  avatar_url: string | null
  created_at: string
}

export interface AuthResponse {
  token: string
  user: User
}
