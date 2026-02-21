<script setup lang="ts">
import { MessageSquare } from 'lucide-vue-next'
import type { PostListItem } from '@/types'
import VoteButton from './VoteButton.vue'
import StatusBadge from './StatusBadge.vue'
import TagBadge from './TagBadge.vue'

defineProps<{ post: PostListItem }>()

defineEmits<{
  vote: [postId: string]
  click: [postId: string]
}>()
</script>

<template>
  <div
    class="flex gap-4 rounded-lg border border-gray-200 bg-white p-4 transition-colors hover:border-gray-300 cursor-pointer"
    @click="$emit('click', post.id)"
  >
    <VoteButton :vote-count="post.vote_count" :has-voted="post.has_voted" @vote="$emit('vote', post.id)" />

    <div class="min-w-0 flex-1">
      <div class="flex items-center gap-2">
        <h3 class="truncate text-sm font-semibold text-gray-900">{{ post.title }}</h3>
        <StatusBadge :status="post.status" />
      </div>

      <p v-if="post.description_preview" class="mt-1 line-clamp-2 text-sm text-gray-500">
        {{ post.description_preview }}
      </p>

      <div class="mt-2 flex items-center gap-3">
        <TagBadge v-for="tag in post.tags" :key="tag.id" :tag="tag" />

        <div class="flex items-center gap-1 text-xs text-gray-400">
          <MessageSquare class="h-3.5 w-3.5" />
          {{ post.comment_count }}
        </div>

        <span v-if="post.author_name" class="text-xs text-gray-400">
          by {{ post.author_name }}
        </span>
      </div>
    </div>
  </div>
</template>
