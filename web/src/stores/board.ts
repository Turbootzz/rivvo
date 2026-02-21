import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useApi } from '@/composables/useApi'
import type { Board } from '@/types'

export const useBoardStore = defineStore('board', () => {
  const boards = ref<Board[]>([])
  const currentBoard = ref<Board | null>(null)
  const loading = ref(false)

  async function fetchBoards(orgId: string) {
    const api = useApi()
    loading.value = true
    try {
      boards.value = await api.get<Board[]>(`/orgs/${orgId}/boards`)
    } finally {
      loading.value = false
    }
  }

  async function fetchBoard(orgId: string, slug: string) {
    const api = useApi()
    loading.value = true
    try {
      currentBoard.value = await api.get<Board>(`/orgs/${orgId}/boards/${slug}`)
    } finally {
      loading.value = false
    }
  }

  async function createBoard(orgId: string, name: string, description?: string) {
    const api = useApi()
    const board = await api.post<Board>(`/orgs/${orgId}/boards`, { name, description })
    if (!board) throw new Error('Failed to create board')
    boards.value.push(board)
    return board
  }

  async function deleteBoard(orgId: string, slug: string) {
    const api = useApi()
    await api.del(`/orgs/${orgId}/boards/${slug}`)
    boards.value = boards.value.filter((b) => b.slug !== slug)
  }

  function clear() {
    boards.value = []
    currentBoard.value = null
  }

  return { boards, currentBoard, loading, fetchBoards, fetchBoard, createBoard, deleteBoard, clear }
})
