<script setup lang="ts">
import type { PostStatus } from '@/types'

defineProps<{ status: PostStatus }>()

const statusConfig: Record<PostStatus, { label: string; classes: string }> = {
  open: { label: 'Open', classes: 'bg-gray-100 text-gray-700' },
  planned: { label: 'Planned', classes: 'bg-blue-100 text-blue-700' },
  in_progress: { label: 'In Progress', classes: 'bg-yellow-100 text-yellow-800' },
  done: { label: 'Done', classes: 'bg-green-100 text-green-700' },
  closed: { label: 'Closed', classes: 'bg-red-100 text-red-700' },
}
</script>

<template>
  <span
    class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium"
    :class="statusConfig[status]?.classes ?? 'bg-gray-100 text-gray-700'"
  >
    {{ statusConfig[status]?.label ?? status }}
  </span>
</template>
