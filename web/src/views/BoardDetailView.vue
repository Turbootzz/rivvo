<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ArrowLeft, MessageSquarePlus } from 'lucide-vue-next'
import { useOrgStore } from '@/stores/org'
import { useBoardStore } from '@/stores/board'
import { usePostStore } from '@/stores/post'
import PostCard from '@/components/feedback/PostCard.vue'
import NewPostForm from '@/components/feedback/NewPostForm.vue'

const route = useRoute()
const router = useRouter()
const orgStore = useOrgStore()
const boardStore = useBoardStore()
const postStore = usePostStore()

const showNewPost = ref(false)
const sort = ref('votes')
const statusFilter = ref<string | undefined>(undefined)
const error = ref('')

const slug = computed(() => route.params.slug as string)

const statuses: { value: string | undefined; label: string }[] = [
  { value: undefined, label: 'All' },
  { value: 'open', label: 'Open' },
  { value: 'planned', label: 'Planned' },
  { value: 'in_progress', label: 'In Progress' },
  { value: 'done', label: 'Done' },
]

async function loadBoard() {
  if (!orgStore.currentOrg) await orgStore.fetchOrg()
  if (orgStore.currentOrg) {
    await boardStore.fetchBoard(orgStore.currentOrg.id, slug.value)
    if (boardStore.currentBoard) {
      await postStore.fetchPosts(boardStore.currentBoard.id, sort.value, statusFilter.value)
    }
  }
}

onMounted(loadBoard)

watch(slug, loadBoard)

watch([sort, statusFilter], async () => {
  if (boardStore.currentBoard) {
    await postStore.fetchPosts(boardStore.currentBoard.id, sort.value, statusFilter.value)
  }
})

async function handleCreatePost(title: string, description: string) {
  if (!boardStore.currentBoard) return
  error.value = ''
  try {
    await postStore.createPost(boardStore.currentBoard.id, title, description || undefined)
    showNewPost.value = false
    await postStore.fetchPosts(boardStore.currentBoard.id, sort.value, statusFilter.value)
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to create post'
  }
}

async function handleVote(postId: string) {
  try {
    await postStore.toggleVote(postId)
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to vote'
  }
}

function openPost(postId: string) {
  router.push({ name: 'post-detail', params: { id: postId } })
}
</script>

<template>
  <div class="mx-auto max-w-3xl">
    <div class="flex items-center gap-3">
      <button @click="router.push({ name: 'boards' })" class="rounded-md p-1 text-gray-400 hover:bg-gray-200 hover:text-gray-600">
        <ArrowLeft class="h-5 w-5" />
      </button>
      <h1 class="text-2xl font-bold text-gray-900">{{ boardStore.currentBoard?.name ?? 'Board' }}</h1>
    </div>

    <p v-if="boardStore.currentBoard?.description" class="mt-1 text-sm text-gray-500">
      {{ boardStore.currentBoard.description }}
    </p>

    <div v-if="error" class="mt-4 rounded-md bg-red-50 p-3 text-sm text-red-700">{{ error }}</div>

    <!-- Controls -->
    <div class="mt-4 flex flex-wrap items-center justify-between gap-3">
      <div class="flex gap-1">
        <button
          v-for="s in statuses"
          :key="s.label"
          @click="statusFilter = s.value"
          class="rounded-md px-3 py-1.5 text-xs font-medium transition-colors"
          :class="statusFilter === s.value ? 'bg-primary-100 text-primary-700' : 'text-gray-500 hover:bg-gray-100'"
        >
          {{ s.label }}
        </button>
      </div>

      <div class="flex items-center gap-2">
        <select
          v-model="sort"
          class="rounded-md border border-gray-300 px-2 py-1.5 text-xs focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
        >
          <option value="votes">Most Voted</option>
          <option value="recent">Recent</option>
          <option value="oldest">Oldest</option>
        </select>

        <button
          @click="showNewPost = !showNewPost"
          class="flex items-center gap-1 rounded-md bg-primary-600 px-3 py-1.5 text-xs font-semibold text-white shadow-sm hover:bg-primary-700"
        >
          <MessageSquarePlus class="h-3.5 w-3.5" />
          New Post
        </button>
      </div>
    </div>

    <!-- New post form -->
    <div v-if="showNewPost" class="mt-4">
      <NewPostForm @submit="handleCreatePost" @cancel="showNewPost = false" />
    </div>

    <!-- Posts -->
    <div v-if="postStore.loading" class="mt-8 text-center text-sm text-gray-500">Loading...</div>

    <div v-else-if="postStore.posts.length === 0" class="mt-12 text-center">
      <p class="text-sm text-gray-500">No posts yet. Be the first to submit a feature request!</p>
    </div>

    <div v-else class="mt-4 space-y-3">
      <PostCard
        v-for="post in postStore.posts"
        :key="post.id"
        :post="post"
        @vote="handleVote"
        @click="openPost"
      />
    </div>
  </div>
</template>
