<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
import { useRouter } from 'vue-router'
import { LogIn, LogOut, MessageSquarePlus } from 'lucide-vue-next'

const authStore = useAuthStore()
const router = useRouter()

function handleLogout() {
  authStore.logout()
  router.push('/')
}
</script>

<template>
  <nav class="border-b border-gray-200 bg-white">
    <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
      <div class="flex h-16 items-center justify-between">
        <div class="flex items-center gap-2">
          <MessageSquarePlus class="h-6 w-6 text-primary-600" />
          <RouterLink to="/" class="text-xl font-bold text-gray-900">Rivvo</RouterLink>
        </div>
        <div class="flex items-center gap-4">
          <template v-if="authStore.isAuthenticated">
            <span class="text-sm text-gray-600">{{ authStore.user?.name }}</span>
            <button
              @click="handleLogout()"
              class="inline-flex items-center gap-1.5 rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100"
            >
              <LogOut class="h-4 w-4" />
              Logout
            </button>
          </template>
          <template v-else>
            <RouterLink
              to="/login"
              class="inline-flex items-center gap-1.5 rounded-md bg-primary-600 px-3 py-2 text-sm font-medium text-white hover:bg-primary-700"
            >
              <LogIn class="h-4 w-4" />
              Login
            </RouterLink>
          </template>
        </div>
      </div>
    </div>
  </nav>
</template>
