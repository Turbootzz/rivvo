<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ArrowLeft } from 'lucide-vue-next'
import { useApi } from '@/composables/useApi'
import { useAuthStore } from '@/stores/auth'
import type { Post, Comment, VoteResult } from '@/types'
import VoteButton from '@/components/feedback/VoteButton.vue'
import StatusBadge from '@/components/feedback/StatusBadge.vue'
import TagBadge from '@/components/feedback/TagBadge.vue'
import CommentItem from '@/components/feedback/CommentItem.vue'
import CommentForm from '@/components/feedback/CommentForm.vue'

const route = useRoute()
const router = useRouter()
const api = useApi()
const authStore = useAuthStore()

const post = ref<Post | null>(null)
const comments = ref<Comment[]>([])
const loading = ref(false)
const error = ref('')
const commentFormRef = ref<InstanceType<typeof CommentForm> | null>(null)

const orgSlug = computed(() => route.params.orgSlug as string)
const boardSlug = computed(() => route.params.boardSlug as string)
const postId = computed(() => route.params.postId as string)

async function loadPost(id: string) {
  loading.value = true
  error.value = ''
  try {
    const base = `/public/${orgSlug.value}/boards/${boardSlug.value}/posts/${id}`
    post.value = await api.get<Post>(base)
    comments.value = (await api.get<Comment[]>(`${base}/comments`)) ?? []
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to load post'
  } finally {
    loading.value = false
  }
}

onMounted(() => loadPost(postId.value))
watch(postId, (newId) => loadPost(newId))

async function handleVote() {
  if (!post.value) return
  try {
    const result = await api.post<VoteResult>(`/posts/${post.value.id}/vote`, {})
    if (result && post.value) {
      post.value.has_voted = result.voted
      post.value.vote_count = result.vote_count
    }
  } catch {
    // useApi handles 401 -> redirect to login
  }
}

async function handleAddComment(body: string) {
  if (!post.value) return
  try {
    const comment = await api.post<Comment>(`/posts/${post.value.id}/comments`, { body })
    if (comment) {
      comments.value.push(comment)
      post.value.comment_count++
      commentFormRef.value?.reset()
    }
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to add comment'
  }
}

async function handleDeleteComment(commentId: string) {
  try {
    await api.del(`/comments/${commentId}`)
    comments.value = comments.value.filter((c) => c.id !== commentId)
    if (post.value) post.value.comment_count--
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

    <div v-if="loading" class="mt-8 text-center text-sm text-gray-500">Loading...</div>

    <div v-else-if="post" class="mt-4">
      <!-- Post header -->
      <div class="flex gap-4">
        <VoteButton :vote-count="post.vote_count" :has-voted="post.has_voted" @vote="handleVote" />

        <div class="flex-1">
          <h1 class="text-xl font-bold text-gray-900">{{ post.title }}</h1>

          <div class="mt-2 flex flex-wrap items-center gap-2">
            <StatusBadge :status="post.status" />
            <TagBadge v-for="tag in post.tags" :key="tag.id" :tag="tag" />
          </div>
        </div>
      </div>

      <!-- Post body -->
      <div v-if="post.description" class="mt-4 whitespace-pre-wrap text-sm text-gray-700">
        {{ post.description }}
      </div>

      <div class="mt-4 text-xs text-gray-400">
        <span v-if="post.author">Posted by {{ post.author.name }}</span>
        <span v-if="post.created_at">
          on {{ new Date(post.created_at).toLocaleDateString() }}
        </span>
      </div>

      <!-- Comments section -->
      <div class="mt-8">
        <h2 class="text-lg font-semibold text-gray-900">Comments ({{ post.comment_count }})</h2>

        <div class="mt-4 space-y-3">
          <CommentItem
            v-for="comment in comments"
            :key="comment.id"
            :comment="comment"
            @delete="handleDeleteComment"
          />
        </div>

        <div v-if="authStore.isAuthenticated" class="mt-4">
          <CommentForm ref="commentFormRef" @submit="handleAddComment" />
        </div>
        <p v-else class="mt-4 text-sm text-gray-500">
          <RouterLink to="/login" class="text-primary-600 hover:underline">Log in</RouterLink>
          to leave a comment.
        </p>
      </div>
    </div>

    <div v-else class="mt-8 text-center text-sm text-gray-500">Post not found.</div>
  </div>
</template>
