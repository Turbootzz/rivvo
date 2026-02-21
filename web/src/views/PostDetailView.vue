<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ArrowLeft } from 'lucide-vue-next'
import { useOrgStore } from '@/stores/org'
import { usePostStore } from '@/stores/post'
import type { PostStatus } from '@/types'
import VoteButton from '@/components/feedback/VoteButton.vue'
import StatusBadge from '@/components/feedback/StatusBadge.vue'
import TagBadge from '@/components/feedback/TagBadge.vue'
import CommentItem from '@/components/feedback/CommentItem.vue'
import CommentForm from '@/components/feedback/CommentForm.vue'

const route = useRoute()
const router = useRouter()
const orgStore = useOrgStore()
const postStore = usePostStore()

const error = ref('')
const postId = computed(() => route.params.id as string)
const isAdmin = computed(() => orgStore.currentOrg?.role === 'admin')

const statusOptions: PostStatus[] = ['open', 'planned', 'in_progress', 'done', 'closed']

onMounted(async () => {
  if (!orgStore.currentOrg) await orgStore.fetchOrg()
  if (postStore.currentPost?.board_id) {
    await postStore.fetchPost(postStore.currentPost.board_id, postId.value)
  } else {
    // Try to load post with a board_id guess — we need the board_id from the post itself.
    // Fetch from all posts if we have a board context, otherwise just try fetching by iterating.
    // For simplicity, use the URL — the post detail fetches from any board_id in the path.
    // We'll use a trick: the backend accepts any board_id in the path, it just ignores it for GET.
    await postStore.fetchPost('00000000-0000-0000-0000-000000000000', postId.value)
  }
  if (postStore.currentPost) {
    await postStore.fetchComments(postStore.currentPost.id)
  }
})

async function handleVote() {
  if (!postStore.currentPost) return
  try {
    await postStore.toggleVote(postStore.currentPost.id)
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to vote'
  }
}

async function handleStatusChange(event: Event) {
  if (!postStore.currentPost) return
  const status = (event.target as HTMLSelectElement).value as PostStatus
  try {
    await postStore.updateStatus(postStore.currentPost.board_id, postStore.currentPost.id, status)
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to update status'
  }
}

async function handleAddComment(body: string) {
  if (!postStore.currentPost) return
  try {
    await postStore.addComment(postStore.currentPost.id, body)
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to add comment'
  }
}

async function handleDeleteComment(commentId: string) {
  if (!postStore.currentPost) return
  try {
    await postStore.deleteComment(commentId, postStore.currentPost.id)
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to delete comment'
  }
}
</script>

<template>
  <div class="mx-auto max-w-3xl">
    <button
      @click="router.back()"
      class="flex items-center gap-1 rounded-md p-1 text-sm text-gray-400 hover:text-gray-600"
    >
      <ArrowLeft class="h-4 w-4" />
      Back
    </button>

    <div v-if="error" class="mt-4 rounded-md bg-red-50 p-3 text-sm text-red-700">{{ error }}</div>

    <div v-if="postStore.loading" class="mt-8 text-center text-sm text-gray-500">Loading...</div>

    <div v-else-if="postStore.currentPost" class="mt-4">
      <!-- Post header -->
      <div class="flex gap-4">
        <VoteButton
          :vote-count="postStore.currentPost.vote_count"
          :has-voted="postStore.currentPost.has_voted"
          @vote="handleVote"
        />

        <div class="flex-1">
          <h1 class="text-xl font-bold text-gray-900">{{ postStore.currentPost.title }}</h1>

          <div class="mt-2 flex flex-wrap items-center gap-2">
            <StatusBadge :status="postStore.currentPost.status" />
            <TagBadge v-for="tag in postStore.currentPost.tags" :key="tag.id" :tag="tag" />
          </div>

          <!-- Admin status changer -->
          <div v-if="isAdmin" class="mt-3">
            <select
              :value="postStore.currentPost.status"
              @change="handleStatusChange"
              class="rounded-md border border-gray-300 px-2 py-1 text-xs focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
            >
              <option v-for="s in statusOptions" :key="s" :value="s">
                {{ s.replace('_', ' ') }}
              </option>
            </select>
          </div>
        </div>
      </div>

      <!-- Post body -->
      <div v-if="postStore.currentPost.description" class="mt-4 whitespace-pre-wrap text-sm text-gray-700">
        {{ postStore.currentPost.description }}
      </div>

      <div class="mt-4 text-xs text-gray-400">
        <span v-if="postStore.currentPost.author">
          Posted by {{ postStore.currentPost.author.name }}
        </span>
        <span v-if="postStore.currentPost.created_at">
          on {{ new Date(postStore.currentPost.created_at).toLocaleDateString() }}
        </span>
      </div>

      <!-- Comments section -->
      <div class="mt-8">
        <h2 class="text-lg font-semibold text-gray-900">
          Comments ({{ postStore.currentPost.comment_count }})
        </h2>

        <div class="mt-4 space-y-3">
          <CommentItem
            v-for="comment in postStore.comments"
            :key="comment.id"
            :comment="comment"
            @delete="handleDeleteComment"
          />
        </div>

        <div class="mt-4">
          <CommentForm @submit="handleAddComment" />
        </div>
      </div>
    </div>

    <div v-else class="mt-8 text-center text-sm text-gray-500">Post not found.</div>
  </div>
</template>
