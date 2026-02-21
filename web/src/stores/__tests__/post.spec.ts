import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { usePostStore } from '../post'
import type { PostListItem, Post, Comment } from '@/types'

const mockGet = vi.fn()
const mockPost = vi.fn()
const mockPut = vi.fn()
const mockDel = vi.fn()
vi.mock('@/composables/useApi', () => ({
  useApi: () => ({
    get: mockGet,
    post: mockPost,
    put: mockPut,
    del: mockDel,
  }),
}))

const makePostListItem = (overrides: Partial<PostListItem> = {}): PostListItem => ({
  id: '1',
  title: 'Test',
  description_preview: null,
  status: 'open',
  vote_count: 0,
  comment_count: 0,
  pinned: false,
  author_name: 'Alice',
  has_voted: false,
  tags: [],
  created_at: '2025-01-01',
  ...overrides,
})

const makePost = (overrides: Partial<Post> = {}): Post => ({
  id: '1',
  board_id: 'b1',
  title: 'Test',
  description: null,
  status: 'open',
  vote_count: 0,
  comment_count: 0,
  pinned: false,
  author: null,
  has_voted: false,
  tags: [],
  created_at: '2025-01-01',
  updated_at: null,
  ...overrides,
})

describe('post store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('fetchPosts sets posts list', async () => {
    const posts = [makePostListItem()]
    mockGet.mockResolvedValue(posts)

    const store = usePostStore()
    await store.fetchPosts('board-1', 'votes', 'open')

    expect(mockGet).toHaveBeenCalledWith('/boards/board-1/posts?sort=votes&status=open')
    expect(store.posts).toEqual(posts)
  })

  it('fetchPosts builds query string correctly without filters', async () => {
    mockGet.mockResolvedValue([])

    const store = usePostStore()
    await store.fetchPosts('board-1')

    expect(mockGet).toHaveBeenCalledWith('/boards/board-1/posts')
  })

  it('fetchPost sets currentPost', async () => {
    const post = makePost()
    mockGet.mockResolvedValue(post)

    const store = usePostStore()
    await store.fetchPost('board-1', 'post-1')

    expect(mockGet).toHaveBeenCalledWith('/boards/board-1/posts/post-1')
    expect(store.currentPost).toEqual(post)
  })

  it('createPost returns the created post', async () => {
    const post = makePost({ id: '99', title: 'New' })
    mockPost.mockResolvedValue(post)

    const store = usePostStore()
    const result = await store.createPost('board-1', 'New', 'Description')

    expect(mockPost).toHaveBeenCalledWith('/boards/board-1/posts', { title: 'New', description: 'Description' })
    expect(result).toEqual(post)
  })

  it('toggleVote updates both post list and currentPost', async () => {
    mockPost.mockResolvedValue({ voted: true, vote_count: 5 })

    const store = usePostStore()
    store.posts = [makePostListItem({ id: 'p1', vote_count: 4, has_voted: false })]
    store.currentPost = makePost({ id: 'p1', vote_count: 4, has_voted: false })

    await store.toggleVote('p1')

    expect(store.posts[0]!.has_voted).toBe(true)
    expect(store.posts[0]!.vote_count).toBe(5)
    expect(store.currentPost!.has_voted).toBe(true)
    expect(store.currentPost!.vote_count).toBe(5)
  })

  it('toggleVote only updates matching post in list', async () => {
    mockPost.mockResolvedValue({ voted: true, vote_count: 1 })

    const store = usePostStore()
    store.posts = [
      makePostListItem({ id: 'p1', vote_count: 0 }),
      makePostListItem({ id: 'p2', vote_count: 3 }),
    ]

    await store.toggleVote('p1')

    expect(store.posts[0]!.vote_count).toBe(1)
    expect(store.posts[1]!.vote_count).toBe(3) // unchanged
  })

  it('fetchComments sets comments', async () => {
    const comments: Comment[] = [
      { id: 'c1', body: 'Nice', is_admin_reply: false, author: null, created_at: '' },
    ]
    mockGet.mockResolvedValue(comments)

    const store = usePostStore()
    await store.fetchComments('p1')

    expect(mockGet).toHaveBeenCalledWith('/posts/p1/comments')
    expect(store.comments).toEqual(comments)
  })

  it('addComment pushes to list and increments counts', async () => {
    const comment: Comment = { id: 'c2', body: 'Reply', is_admin_reply: false, author: null, created_at: '' }
    mockPost.mockResolvedValue(comment)

    const store = usePostStore()
    store.comments = []
    store.currentPost = makePost({ id: 'p1', comment_count: 0 })
    store.posts = [makePostListItem({ id: 'p1', comment_count: 0 })]

    await store.addComment('p1', 'Reply')

    expect(store.comments).toHaveLength(1)
    expect(store.currentPost!.comment_count).toBe(1)
    expect(store.posts[0]!.comment_count).toBe(1)
  })

  it('deleteComment removes from list and decrements counts', async () => {
    mockDel.mockResolvedValue(null)

    const store = usePostStore()
    store.comments = [
      { id: 'c1', body: 'Keep', is_admin_reply: false, author: null, created_at: '' },
      { id: 'c2', body: 'Remove', is_admin_reply: false, author: null, created_at: '' },
    ]
    store.currentPost = makePost({ id: 'p1', comment_count: 2 })
    store.posts = [makePostListItem({ id: 'p1', comment_count: 2 })]

    await store.deleteComment('c2', 'p1')

    expect(store.comments).toHaveLength(1)
    expect(store.comments[0]!.id).toBe('c1')
    expect(store.currentPost!.comment_count).toBe(1)
    expect(store.posts[0]!.comment_count).toBe(1)
  })

  it('deleteComment does not go below zero', async () => {
    mockDel.mockResolvedValue(null)

    const store = usePostStore()
    store.comments = [{ id: 'c1', body: 'X', is_admin_reply: false, author: null, created_at: '' }]
    store.currentPost = makePost({ id: 'p1', comment_count: 0 })

    await store.deleteComment('c1', 'p1')

    expect(store.currentPost!.comment_count).toBe(0)
  })

  it('updateStatus updates both currentPost and list', async () => {
    mockPut.mockResolvedValue(makePost({ id: 'p1', status: 'done' }))

    const store = usePostStore()
    store.currentPost = makePost({ id: 'p1', status: 'open' })
    store.posts = [makePostListItem({ id: 'p1', status: 'open' })]

    await store.updateStatus('board-1', 'p1', 'done')

    expect(store.currentPost!.status).toBe('done')
    expect(store.posts[0]!.status).toBe('done')
  })
})
