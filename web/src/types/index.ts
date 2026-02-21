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

export interface Organization {
  id: string
  name: string
  slug: string
  logo_url: string | null
  role: string
}

export interface Board {
  id: string
  name: string
  slug: string
  description: string | null
  post_count: number
}

export type PostStatus = 'open' | 'planned' | 'in_progress' | 'done' | 'closed'

export interface PostListItem {
  id: string
  title: string
  description_preview: string | null
  status: PostStatus
  vote_count: number
  comment_count: number
  pinned: boolean
  author_name: string | null
  has_voted: boolean
  tags: Tag[]
  created_at: string
}

export interface PostAuthor {
  id: string
  name: string
  avatar_url: string | null
}

export interface Post {
  id: string
  board_id: string
  title: string
  description: string | null
  status: PostStatus
  vote_count: number
  comment_count: number
  pinned: boolean
  author: PostAuthor | null
  has_voted: boolean
  tags: Tag[]
  created_at: string
  updated_at: string | null
}

export interface Comment {
  id: string
  body: string
  is_admin_reply: boolean
  author: CommentAuthor | null
  created_at: string
}

export interface CommentAuthor {
  id: string
  name: string
  avatar_url: string | null
}

export interface Tag {
  id: string
  name: string
  color: string
}

export interface VoteResult {
  voted: boolean
  vote_count: number
}
