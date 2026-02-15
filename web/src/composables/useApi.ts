import { useAuthStore } from '@/stores/auth'
import { useRouter } from 'vue-router'

const BASE_URL = '/api'

interface ApiOptions {
  method?: string
  body?: unknown
  headers?: Record<string, string>
}

export function useApi() {
  const authStore = useAuthStore()
  const router = useRouter()

  async function request<T>(endpoint: string, options: ApiOptions = {}): Promise<T> {
    const { method = 'GET', body, headers = {} } = options

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

    if (response.status === 401) {
      authStore.logout()
      router.push({ name: 'login' })
      throw new Error('Unauthorized')
    }

    if (!response.ok) {
      const error = await response.json().catch(() => ({ error: 'Request failed' }))
      throw new Error(error.error || 'Request failed')
    }

    return response.json()
  }

  return {
    get: <T>(endpoint: string) => request<T>(endpoint),
    post: <T>(endpoint: string, body: unknown) =>
      request<T>(endpoint, { method: 'POST', body }),
    put: <T>(endpoint: string, body: unknown) =>
      request<T>(endpoint, { method: 'PUT', body }),
    del: <T>(endpoint: string) => request<T>(endpoint, { method: 'DELETE' }),
  }
}
