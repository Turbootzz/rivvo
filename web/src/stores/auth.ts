import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { User } from '@/types'

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(localStorage.getItem('token'))
  let parsedUser: User | null = null
  try {
    const storedUser = localStorage.getItem('user')
    if (storedUser) parsedUser = JSON.parse(storedUser)
  } catch {
    localStorage.removeItem('user')
  }
  const user = ref<User | null>(parsedUser)

  const isAuthenticated = computed(() => !!token.value && !!user.value)

  function setAuth(newToken: string, newUser: User) {
    token.value = newToken
    user.value = newUser
    localStorage.setItem('token', newToken)
    localStorage.setItem('user', JSON.stringify(newUser))
  }

  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('token')
    localStorage.removeItem('user')
  }

  return { token, user, isAuthenticated, setAuth, logout }
})
