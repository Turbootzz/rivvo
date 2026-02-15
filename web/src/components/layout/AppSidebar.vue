<script setup lang="ts">
import { LayoutDashboard, Map, Newspaper } from 'lucide-vue-next'
</script>

<template>
  <aside class="hidden w-64 border-r border-gray-200 bg-gray-50 p-4 lg:block">
    <nav class="space-y-1">
      <RouterLink
        to="/board"
        class="flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-200"
      >
        <LayoutDashboard class="h-4 w-4" />
        Boards
      </RouterLink>
      <RouterLink
        to="/roadmap"
        class="flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-200"
      >
        <Map class="h-4 w-4" />
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
