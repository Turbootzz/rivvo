<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useApi } from '@/composables/useApi'
import { useAuthStore } from '@/stores/auth'
import type { AuthResponse } from '@/types'

const router = useRouter()
const api = useApi()
const authStore = useAuthStore()

const isRegister = ref(false)
const email = ref('')
const password = ref('')
const name = ref('')
const error = ref('')
const loading = ref(false)

async function handleSubmit() {
  error.value = ''
  loading.value = true

  try {
    let response: AuthResponse

    if (isRegister.value) {
      response = await api.post<AuthResponse>('/auth/register', {
        email: email.value,
        name: name.value,
        password: password.value,
      })
    } else {
      response = await api.post<AuthResponse>('/auth/login', {
        email: email.value,
        password: password.value,
      })
    }

    authStore.setAuth(response.token, response.user)
    router.push({ name: 'board' })
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Something went wrong'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="flex min-h-[60vh] items-center justify-center">
    <div class="w-full max-w-md rounded-lg border border-gray-200 bg-white p-8 shadow-sm">
      <h2 class="text-center text-2xl font-bold text-gray-900">
        {{ isRegister ? 'Create an account' : 'Sign in to Rivvo' }}
      </h2>

      <form @submit.prevent="handleSubmit" class="mt-6 space-y-4">
        <div v-if="error" class="rounded-md bg-red-50 p-3 text-sm text-red-700">
          {{ error }}
        </div>

        <div v-if="isRegister">
          <label for="name" class="block text-sm font-medium text-gray-700">Name</label>
          <input
            id="name"
            v-model="name"
            type="text"
            required
            class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
            placeholder="Your name"
          />
        </div>

        <div>
          <label for="email" class="block text-sm font-medium text-gray-700">Email</label>
          <input
            id="email"
            v-model="email"
            type="email"
            required
            class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
            placeholder="you@example.com"
          />
        </div>

        <div>
          <label for="password" class="block text-sm font-medium text-gray-700">Password</label>
          <input
            id="password"
            v-model="password"
            type="password"
            required
            minlength="8"
            class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
            placeholder="Min. 8 characters"
          />
        </div>

        <button
          type="submit"
          :disabled="loading"
          class="w-full rounded-md bg-primary-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-primary-700 disabled:opacity-50"
        >
          {{ loading ? 'Please wait...' : isRegister ? 'Create Account' : 'Sign In' }}
        </button>
      </form>

      <p class="mt-4 text-center text-sm text-gray-600">
        {{ isRegister ? 'Already have an account?' : "Don't have an account?" }}
        <button
          @click="isRegister = !isRegister; error = ''"
          class="font-medium text-primary-600 hover:text-primary-700"
        >
          {{ isRegister ? 'Sign in' : 'Create one' }}
        </button>
      </p>
    </div>
  </div>
</template>
