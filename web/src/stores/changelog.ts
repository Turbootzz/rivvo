import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useApi } from '@/composables/useApi'
import type { ChangelogEntry } from '@/types'

export const useChangelogStore = defineStore('changelog', () => {
  const entries = ref<ChangelogEntry[]>([])
  const loading = ref(false)

  async function fetchPublicEntries(orgSlug: string) {
    const api = useApi()
    loading.value = true
    try {
      entries.value = (await api.get<ChangelogEntry[]>(`/public/${orgSlug}/changelog`)) ?? []
    } finally {
      loading.value = false
    }
  }

  async function fetchAdminEntries(orgId: string) {
    const api = useApi()
    loading.value = true
    try {
      entries.value = (await api.get<ChangelogEntry[]>(`/orgs/${orgId}/changelog`)) ?? []
    } finally {
      loading.value = false
    }
  }

  async function createEntry(orgId: string, title: string, body: string) {
    const api = useApi()
    const entry = await api.post<ChangelogEntry>(`/orgs/${orgId}/changelog`, { title, body })
    if (entry) entries.value.unshift(entry)
    return entry
  }

  async function updateEntry(id: string, title: string, body: string, is_draft: boolean) {
    const api = useApi()
    const updated = await api.put<ChangelogEntry>(`/changelog/${id}`, { title, body, is_draft })
    if (updated) {
      const idx = entries.value.findIndex((e) => e.id === id)
      if (idx !== -1) entries.value[idx] = updated
    }
    return updated
  }

  async function deleteEntry(id: string) {
    const api = useApi()
    await api.del(`/changelog/${id}`)
    entries.value = entries.value.filter((e) => e.id !== id)
  }

  function clear() {
    entries.value = []
  }

  return {
    entries,
    loading,
    fetchPublicEntries,
    fetchAdminEntries,
    createEntry,
    updateEntry,
    deleteEntry,
    clear,
  }
})
