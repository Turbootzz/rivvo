import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useOrgStore } from '../org'

const mockGet = vi.fn()
vi.mock('@/composables/useApi', () => ({
  useApi: () => ({
    get: mockGet,
    post: vi.fn(),
    put: vi.fn(),
    del: vi.fn(),
  }),
}))

describe('org store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('starts with null org', () => {
    const store = useOrgStore()
    expect(store.currentOrg).toBeNull()
    expect(store.loading).toBe(false)
  })

  it('fetchOrg sets currentOrg to first org', async () => {
    const org = { id: '1', name: 'Acme', slug: 'acme', logo_url: null, role: 'admin' }
    mockGet.mockResolvedValue([org])

    const store = useOrgStore()
    await store.fetchOrg()

    expect(mockGet).toHaveBeenCalledWith('/orgs')
    expect(store.currentOrg).toEqual(org)
    expect(store.loading).toBe(false)
  })

  it('fetchOrg sets null when no orgs', async () => {
    mockGet.mockResolvedValue([])

    const store = useOrgStore()
    await store.fetchOrg()

    expect(store.currentOrg).toBeNull()
  })

  it('fetchOrg sets loading during request', async () => {
    let resolve: (v: unknown[]) => void
    mockGet.mockReturnValue(new Promise((r) => { resolve = r }))

    const store = useOrgStore()
    const promise = store.fetchOrg()
    expect(store.loading).toBe(true)

    resolve!([])
    await promise
    expect(store.loading).toBe(false)
  })

  it('clear resets currentOrg', () => {
    const store = useOrgStore()
    store.currentOrg = { id: '1', name: 'X', slug: 'x', logo_url: null, role: 'admin' }
    store.clear()
    expect(store.currentOrg).toBeNull()
  })
})
