import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useBoardStore } from '../board'
import type { Board } from '@/types'

const mockGet = vi.fn()
const mockPost = vi.fn()
const mockDel = vi.fn()
vi.mock('@/composables/useApi', () => ({
  useApi: () => ({
    get: mockGet,
    post: mockPost,
    put: vi.fn(),
    del: mockDel,
  }),
}))

const board: Board = { id: '1', name: 'Features', slug: 'features', description: null, post_count: 5 }

describe('board store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('starts empty', () => {
    const store = useBoardStore()
    expect(store.boards).toEqual([])
    expect(store.currentBoard).toBeNull()
    expect(store.loading).toBe(false)
  })

  it('fetchBoards populates boards list', async () => {
    mockGet.mockResolvedValue([board])

    const store = useBoardStore()
    await store.fetchBoards('org-1')

    expect(mockGet).toHaveBeenCalledWith('/orgs/org-1/boards')
    expect(store.boards).toEqual([board])
  })

  it('fetchBoard sets currentBoard', async () => {
    mockGet.mockResolvedValue(board)

    const store = useBoardStore()
    await store.fetchBoard('org-1', 'features')

    expect(mockGet).toHaveBeenCalledWith('/orgs/org-1/boards/features')
    expect(store.currentBoard).toEqual(board)
  })

  it('createBoard adds to list', async () => {
    const newBoard = { id: '2', name: 'Bugs', slug: 'bugs', description: 'Bug reports', post_count: 0 }
    mockPost.mockResolvedValue(newBoard)

    const store = useBoardStore()
    const result = await store.createBoard('org-1', 'Bugs', 'Bug reports')

    expect(mockPost).toHaveBeenCalledWith('/orgs/org-1/boards', { name: 'Bugs', description: 'Bug reports' })
    expect(store.boards).toContainEqual(newBoard)
    expect(result).toEqual(newBoard)
  })

  it('deleteBoard removes from list', async () => {
    mockDel.mockResolvedValue(null)

    const store = useBoardStore()
    store.boards = [board, { id: '2', name: 'Bugs', slug: 'bugs', description: null, post_count: 0 }]

    await store.deleteBoard('org-1', 'features')

    expect(mockDel).toHaveBeenCalledWith('/orgs/org-1/boards/features')
    expect(store.boards).toHaveLength(1)
    expect(store.boards[0]!.slug).toBe('bugs')
  })

  it('fetchBoards manages loading state', async () => {
    let resolve: (v: Board[]) => void
    mockGet.mockReturnValue(new Promise((r) => { resolve = r }))

    const store = useBoardStore()
    const promise = store.fetchBoards('org-1')
    expect(store.loading).toBe(true)

    resolve!([])
    await promise
    expect(store.loading).toBe(false)
  })
})
