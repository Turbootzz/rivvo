import { useAuthStore } from '@/stores/auth'
import { useOrgStore } from '@/stores/org'
import { useBoardStore } from '@/stores/board'
import { usePostStore } from '@/stores/post'
import router from '@/router'

const BASE_URL = '/api'

interface ApiOptions {
  method?: string
  body?: unknown
  headers?: Record<string, string>
  skipAuthRedirect?: boolean
}

export function useApi() {
  const authStore = useAuthStore()

  async function request<T>(endpoint: string, options: ApiOptions = {}): Promise<T> {
    const { method = 'GET', body, headers = {}, skipAuthRedirect = false } = options

    if (authStore.token) {
      headers['Authorization'] = `Bearer ${authStore.token}`
    }

    const config: RequestInit = {
      method,
      headers: {
        'Content-Type': 'application/json',
        ...headers,
      },
    }

    if (body) {
      config.body = JSON.stringify(body)
    }

    const response = await fetch(`${BASE_URL}${endpoint}`, config)

    if (response.status === 401 && !skipAuthRedirect) {
      authStore.logout()
      useOrgStore().clear()
      useBoardStore().clear()
      usePostStore().clear()
      router.push({ name: 'login' })
      throw new Error('Unauthorized')
    }

    if (!response.ok) {
      const error = await response.json().catch(() => ({ error: 'Request failed' }))
      throw new Error(error.error || 'Request failed')
    }

    if (response.status === 204 || response.headers.get('content-length') === '0') {
      return null as T
    }

    return response.json()
  }

  return {
    get: <T>(endpoint: string) => request<T>(endpoint),
    post: <T>(endpoint: string, body: unknown, options?: Omit<ApiOptions, 'method' | 'body'>) =>
      request<T>(endpoint, { method: 'POST', body, ...options }),
    put: <T>(endpoint: string, body: unknown) =>
      request<T>(endpoint, { method: 'PUT', body }),
    del: <T>(endpoint: string) => request<T>(endpoint, { method: 'DELETE' }),
  }
}
