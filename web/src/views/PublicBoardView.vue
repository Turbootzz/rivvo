<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Search } from 'lucide-vue-next'
import { useApi } from '@/composables/useApi'
import type { PostListItem, VoteResult } from '@/types'
import PostCard from '@/components/feedback/PostCard.vue'

const route = useRoute()
const router = useRouter()
const api = useApi()

const posts = ref<PostListItem[]>([])
const loading = ref(false)
const sort = ref('votes')
const statusFilter = ref<string | undefined>(undefined)
const searchQuery = ref('')

const orgSlug = computed(() => route.params.orgSlug as string)
const boardSlug = computed(() => route.params.boardSlug as string)

const statuses: { value: string | undefined; label: string }[] = [
  { value: undefined, label: 'All' },
  { value: 'open', label: 'Open' },
  { value: 'planned', label: 'Planned' },
  { value: 'in_progress', label: 'In Progress' },
  { value: 'done', label: 'Done' },
]

async function loadPosts() {
  loading.value = true
  try {
    const params = new URLSearchParams()
    if (sort.value) params.set('sort', sort.value)
    if (statusFilter.value) params.set('status', statusFilter.value)
    if (searchQuery.value.trim()) params.set('search', searchQuery.value.trim())
    const qs = params.toString()
    posts.value =
      (await api.get<PostListItem[]>(
        `/public/${orgSlug.value}/boards/${boardSlug.value}/posts${qs ? `?${qs}` : ''}`,
      )) ?? []
  } finally {
    loading.value = false
  }
}

onMounted(loadPosts)
watch([sort, statusFilter], loadPosts)

let searchTimeout: ReturnType<typeof setTimeout>
watch(searchQuery, () => {
  clearTimeout(searchTimeout)
  searchTimeout = setTimeout(loadPosts, 300)
})

function openPost(postId: string) {
  router.push({
    name: 'public-post',
    params: { orgSlug: orgSlug.value, boardSlug: boardSlug.value, postId },
  })
}

async function handleVote(postId: string) {
  try {
    const result = await api.post<VoteResult>(`/posts/${postId}/vote`, {})
    if (result) {
      const item = posts.value.find((p) => p.id === postId)
      if (item) {
        item.has_voted = result.voted
        item.vote_count = result.vote_count
      }
    }
  } catch {
    // useApi handles 401 -> redirect to login
  }
}
</script>

<template>
  <div class="mx-auto max-w-3xl">
    <h1 class="text-2xl font-bold text-gray-900">Feedback Board</h1>

    <!-- Search -->
    <div class="relative mt-4">
      <Search class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-gray-400" />
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search posts..."
        class="w-full rounded-md border border-gray-300 py-2 pr-3 pl-9 text-sm focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
      />
    </div>

    <!-- Controls -->
    <div class="mt-4 flex flex-wrap items-center justify-between gap-3">
      <div class="flex gap-1">
        <button
          v-for="s in statuses"
          :key="s.label"
          @click="statusFilter = s.value"
          class="rounded-md px-3 py-1.5 text-xs font-medium transition-colors"
          :class="
            statusFilter === s.value
              ? 'bg-primary-100 text-primary-700'
              : 'text-gray-500 hover:bg-gray-100'
          "
        >
          {{ s.label }}
        </button>
      </div>

      <select
        v-model="sort"
        class="rounded-md border border-gray-300 px-2 py-1.5 text-xs focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
      >
        <option value="votes">Most Voted</option>
        <option value="recent">Recent</option>
        <option value="oldest">Oldest</option>
      </select>
    </div>

    <!-- Posts -->
    <div v-if="loading" class="mt-8 text-center text-sm text-gray-500">Loading...</div>

    <div v-else-if="posts.length === 0" class="mt-12 text-center">
      <p class="text-sm text-gray-500">No posts found.</p>
    </div>

    <div v-else class="mt-4 space-y-3">
      <PostCard
        v-for="post in posts"
        :key="post.id"
        :post="post"
        @vote="handleVote"
        @click="openPost"
      />
    </div>
  </div>
</template>
