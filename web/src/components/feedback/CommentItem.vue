<script setup lang="ts">
import { computed } from 'vue'
import { Trash2 } from 'lucide-vue-next'
import type { Comment } from '@/types'
import { useAuthStore } from '@/stores/auth'

const props = defineProps<{ comment: Comment }>()

defineEmits<{
  delete: [commentId: string]
}>()

const authStore = useAuthStore()
const isOwner = computed(
  () => !!authStore.user && !!props.comment.author && authStore.user.id === props.comment.author.id,
)
</script>

<template>
  <div class="rounded-lg border border-gray-100 bg-gray-50 p-3">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <span class="text-sm font-medium text-gray-900">
          {{ comment.author?.name ?? 'Deleted user' }}
        </span>
        <span
          v-if="comment.is_admin_reply"
          class="rounded-full bg-primary-100 px-2 py-0.5 text-xs font-medium text-primary-700"
        >
          Admin
        </span>
        <span class="text-xs text-gray-400">
          {{ new Date(comment.created_at).toLocaleDateString() }}
        </span>
      </div>

      <button
        v-if="isOwner"
        @click="$emit('delete', comment.id)"
        class="rounded p-1 text-gray-400 hover:bg-gray-200 hover:text-red-500"
      >
        <Trash2 class="h-3.5 w-3.5" />
      </button>
    </div>

    <p class="mt-1 whitespace-pre-wrap text-sm text-gray-700">{{ comment.body }}</p>
  </div>
</template>
