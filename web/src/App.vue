<script setup lang="ts">
import { useRoute } from 'vue-router'
import { computed } from 'vue'
import AppNavbar from '@/components/layout/AppNavbar.vue'
import AppSidebar from '@/components/layout/AppSidebar.vue'
import AppFooter from '@/components/layout/AppFooter.vue'

const route = useRoute()
const showChrome = computed(() => !!route.meta.requiresAuth)
</script>

<template>
  <div class="flex min-h-screen flex-col bg-gray-50">
    <AppNavbar v-if="showChrome" />
    <div class="flex flex-1">
      <AppSidebar v-if="showChrome" />
      <main class="flex-1 p-6">
        <RouterView />
      </main>
    </div>
    <AppFooter />
  </div>
</template>
