<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useApi } from '@/composables/useApi'
import { useOrgStore } from '@/stores/org'
import { useBoardStore } from '@/stores/board'
import type { RoadmapData, VoteResult } from '@/types'
import PostCard from '@/components/feedback/PostCard.vue'

const route = useRoute()
const router = useRouter()
const api = useApi()
const orgStore = useOrgStore()
const boardStore = useBoardStore()

const roadmap = ref<RoadmapData>({ planned: [], in_progress: [], done: [] })
const loading = ref(false)

const orgSlug = computed(() => route.params.orgSlug as string | undefined)
const boardSlug = computed(() => route.params.boardSlug as string | undefined)

const isPublicRoute = computed(() => !!orgSlug.value && !!boardSlug.value)

async function loadRoadmap() {
  // If no org/board slug in route, redirect from admin sidebar to public roadmap
  if (!isPublicRoute.value) {
    if (!orgStore.currentOrg) await orgStore.fetchOrg()
    if (orgStore.currentOrg && boardStore.boards.length === 0) {
      await boardStore.fetchBoards(orgStore.currentOrg.id)
    }
    const org = orgStore.currentOrg
    const firstBoard = boardStore.boards[0]
    if (org && firstBoard) {
      router.replace({
        name: 'public-roadmap',
        params: {
          orgSlug: org.slug,
          boardSlug: firstBoard.slug,
        },
      })
    }
    return
  }

  loading.value = true
  try {
    const data = await api.get<RoadmapData>(
      `/public/${orgSlug.value}/boards/${boardSlug.value}/roadmap`,
    )
    if (data) roadmap.value = data
  } finally {
    loading.value = false
  }
}

onMounted(loadRoadmap)

function openPost(postId: string) {
  if (orgSlug.value && boardSlug.value) {
    router.push({
      name: 'public-post',
      params: { orgSlug: orgSlug.value, boardSlug: boardSlug.value, postId },
    })
  }
}

async function handleVote(postId: string) {
  try {
    const result = await api.post<VoteResult>(`/posts/${postId}/vote`, {})
    if (result) {
      for (const col of [roadmap.value.planned, roadmap.value.in_progress, roadmap.value.done]) {
        const item = col.find((p) => p.id === postId)
        if (item) {
          item.has_voted = result.voted
          item.vote_count = result.vote_count
          break
        }
      }
    }
  } catch {
    // useApi handles 401 -> redirect to login
  }
}

const columns = computed(() => [
  {
    key: 'planned',
    label: 'Planned',
    colorDot: 'bg-blue-500',
    colorText: 'text-blue-700',
    items: roadmap.value.planned,
  },
  {
    key: 'in_progress',
    label: 'In Progress',
    colorDot: 'bg-yellow-500',
    colorText: 'text-yellow-700',
    items: roadmap.value.in_progress,
  },
  {
    key: 'done',
    label: 'Done',
    colorDot: 'bg-green-500',
    colorText: 'text-green-700',
    items: roadmap.value.done,
  },
])
</script>

<template>
  <div v-if="!isPublicRoute" class="text-center text-sm text-gray-500">
    Redirecting to roadmap...
  </div>

  <div v-else class="mx-auto max-w-6xl">
    <h1 class="text-2xl font-bold text-gray-900">Roadmap</h1>

    <div v-if="loading" class="mt-8 text-center text-sm text-gray-500">Loading...</div>

    <div v-else class="mt-6 grid grid-cols-1 gap-6 md:grid-cols-3">
      <div v-for="col in columns" :key="col.key">
        <h2 class="mb-3 flex items-center gap-2 text-sm font-semibold" :class="col.colorText">
          <span class="h-2 w-2 rounded-full" :class="col.colorDot"></span>
          {{ col.label }} ({{ col.items.length }})
        </h2>
        <div class="space-y-3">
          <PostCard
            v-for="post in col.items"
            :key="post.id"
            :post="post"
            @click="openPost"
            @vote="handleVote"
          />
        </div>
        <p v-if="col.items.length === 0" class="text-sm text-gray-400">No items</p>
      </div>
    </div>
  </div>
</template>
