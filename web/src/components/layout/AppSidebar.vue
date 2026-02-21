<script setup lang="ts">
import { onMounted } from 'vue'
import { LayoutDashboard, Map as MapIcon, Newspaper } from 'lucide-vue-next'
import { useOrgStore } from '@/stores/org'
import { useBoardStore } from '@/stores/board'

const orgStore = useOrgStore()
const boardStore = useBoardStore()

onMounted(async () => {
  if (!orgStore.currentOrg) await orgStore.fetchOrg()
  if (orgStore.currentOrg && boardStore.boards.length === 0) {
    await boardStore.fetchBoards(orgStore.currentOrg.id)
  }
})
</script>

<template>
  <aside class="hidden w-64 border-r border-gray-200 bg-gray-50 p-4 lg:block">
    <nav class="space-y-1">
      <RouterLink
        to="/boards"
        class="flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-200"
      >
        <LayoutDashboard class="h-4 w-4" />
        All Boards
      </RouterLink>

      <RouterLink
        v-for="board in boardStore.boards"
        :key="board.id"
        :to="{ name: 'board-detail', params: { slug: board.slug } }"
        class="flex items-center gap-3 rounded-md px-3 py-2 pl-8 text-sm text-gray-600 hover:bg-gray-200"
      >
        {{ board.name }}
      </RouterLink>

      <div class="my-2 border-t border-gray-200" />

      <RouterLink
        to="/roadmap"
        class="flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-200"
      >
        <MapIcon class="h-4 w-4" />
        Roadmap
      </RouterLink>
      <RouterLink
        to="/changelog"
        class="flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-200"
      >
        <Newspaper class="h-4 w-4" />
        Changelog
      </RouterLink>
    </nav>
  </aside>
</template>
