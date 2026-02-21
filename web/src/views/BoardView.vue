<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Plus, LayoutDashboard } from 'lucide-vue-next'
import { useOrgStore } from '@/stores/org'
import { useBoardStore } from '@/stores/board'

const router = useRouter()
const orgStore = useOrgStore()
const boardStore = useBoardStore()

const showForm = ref(false)
const newName = ref('')
const newDescription = ref('')
const error = ref('')

onMounted(async () => {
  if (!orgStore.currentOrg) await orgStore.fetchOrg()
  if (orgStore.currentOrg) {
    await boardStore.fetchBoards(orgStore.currentOrg.id)
  }
})

async function createBoard() {
  if (!newName.value.trim() || !orgStore.currentOrg) return
  error.value = ''
  try {
    await boardStore.createBoard(orgStore.currentOrg.id, newName.value, newDescription.value || undefined)
    newName.value = ''
    newDescription.value = ''
    showForm.value = false
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to create board'
  }
}

function openBoard(slug: string) {
  router.push({ name: 'board-detail', params: { slug } })
}
</script>

<template>
  <div class="mx-auto max-w-3xl">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold text-gray-900">Boards</h1>
      <button
        @click="showForm = !showForm"
        class="flex items-center gap-1 rounded-md bg-primary-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-primary-700"
      >
        <Plus class="h-4 w-4" />
        New Board
      </button>
    </div>

    <form
      v-if="showForm"
      @submit.prevent="createBoard"
      class="mt-4 rounded-lg border border-gray-200 bg-white p-4"
    >
      <div v-if="error" class="mb-3 rounded-md bg-red-50 p-3 text-sm text-red-700">{{ error }}</div>
      <input
        v-model="newName"
        type="text"
        required
        placeholder="Board name"
        class="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
      />
      <textarea
        v-model="newDescription"
        rows="2"
        placeholder="Description (optional)"
        class="mt-2 block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
      />
      <div class="mt-3 flex gap-2">
        <button
          type="submit"
          class="rounded-md bg-primary-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-primary-700"
        >
          Create
        </button>
        <button
          type="button"
          @click="showForm = false"
          class="rounded-md border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"
        >
          Cancel
        </button>
      </div>
    </form>

    <div v-if="boardStore.loading" class="mt-8 text-center text-sm text-gray-500">Loading...</div>

    <div v-else-if="boardStore.boards.length === 0" class="mt-16 flex flex-col items-center text-center">
      <div class="flex h-16 w-16 items-center justify-center rounded-full bg-primary-100">
        <LayoutDashboard class="h-8 w-8 text-primary-600" />
      </div>
      <h2 class="mt-4 text-lg font-semibold text-gray-900">No boards yet</h2>
      <p class="mt-1 text-sm text-gray-500">Create your first board to start collecting feedback.</p>
    </div>

    <div v-else class="mt-4 space-y-3">
      <div
        v-for="board in boardStore.boards"
        :key="board.id"
        @click="openBoard(board.slug)"
        class="flex items-center justify-between rounded-lg border border-gray-200 bg-white p-4 cursor-pointer transition-colors hover:border-gray-300"
      >
        <div>
          <h3 class="font-semibold text-gray-900">{{ board.name }}</h3>
          <p v-if="board.description" class="mt-0.5 text-sm text-gray-500">{{ board.description }}</p>
        </div>
        <span class="text-sm text-gray-400">{{ board.post_count }} posts</span>
      </div>
    </div>
  </div>
</template>
