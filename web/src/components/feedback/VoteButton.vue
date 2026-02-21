<script setup lang="ts">
import { ChevronUp } from 'lucide-vue-next'

defineProps<{
  voteCount: number
  hasVoted: boolean
}>()

defineEmits<{
  vote: []
}>()
</script>

<template>
  <button
    @click.stop="$emit('vote')"
    class="flex flex-col items-center rounded-lg border px-3 py-2 text-sm font-medium transition-colors"
    :class="
      hasVoted
        ? 'border-primary-300 bg-primary-50 text-primary-700'
        : 'border-gray-200 bg-white text-gray-600 hover:border-gray-300 hover:bg-gray-50'
    "
  >
    <ChevronUp class="h-4 w-4" />
    <span>{{ voteCount }}</span>
  </button>
</template>
