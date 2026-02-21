import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

// Provide a mock localStorage before importing the store
const storage: Record<string, string> = {}
const localStorageMock = {
  getItem: vi.fn((key: string) => storage[key] ?? null),
  setItem: vi.fn((key: string, value: string) => { storage[key] = value }),
  removeItem: vi.fn((key: string) => { delete storage[key] }),
}
Object.defineProperty(globalThis, 'localStorage', { value: localStorageMock, writable: true })

import { useAuthStore } from '../auth'

function clearStorage() {
  for (const key of Object.keys(storage)) delete storage[key]
  vi.clearAllMocks()
}

describe('auth store', () => {
  beforeEach(() => {
    clearStorage()
    setActivePinia(createPinia())
  })

  it('starts unauthenticated when localStorage is empty', () => {
    const store = useAuthStore()
    expect(store.token).toBeNull()
    expect(store.user).toBeNull()
    expect(store.isAuthenticated).toBe(false)
  })

  it('restores token and user from localStorage', () => {
    const user = { id: '1', email: 'a@b.com', name: 'Alice', avatar_url: null, created_at: '' }
    storage['token'] = 'jwt-token'
    storage['user'] = JSON.stringify(user)

    setActivePinia(createPinia())
    const store = useAuthStore()
    expect(store.token).toBe('jwt-token')
    expect(store.user?.name).toBe('Alice')
    expect(store.isAuthenticated).toBe(true)
  })

  it('setAuth persists token and user', () => {
    const store = useAuthStore()
    const user = { id: '2', email: 'b@b.com', name: 'Bob', avatar_url: null, created_at: '' }

    store.setAuth('new-token', user)

    expect(store.token).toBe('new-token')
    expect(store.user?.name).toBe('Bob')
    expect(store.isAuthenticated).toBe(true)
    expect(localStorageMock.setItem).toHaveBeenCalledWith('token', 'new-token')
  })

  it('logout clears everything', () => {
    const store = useAuthStore()
    store.setAuth('tok', { id: '1', email: 'a@b.com', name: 'A', avatar_url: null, created_at: '' })

    store.logout()

    expect(store.token).toBeNull()
    expect(store.user).toBeNull()
    expect(store.isAuthenticated).toBe(false)
    expect(localStorageMock.removeItem).toHaveBeenCalledWith('token')
    expect(localStorageMock.removeItem).toHaveBeenCalledWith('user')
  })

  it('handles corrupted user JSON in localStorage', () => {
    storage['user'] = 'not-json'
    setActivePinia(createPinia())
    const store = useAuthStore()
    expect(store.user).toBeNull()
    expect(localStorageMock.removeItem).toHaveBeenCalledWith('user')
  })
})
