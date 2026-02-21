import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useApi } from '@/composables/useApi'
import type { Organization } from '@/types'

export const useOrgStore = defineStore('org', () => {
  const currentOrg = ref<Organization | null>(null)
  const loading = ref(false)

  async function fetchOrg() {
    const api = useApi()
    loading.value = true
    try {
      const orgs = await api.get<Organization[]>('/orgs')
      currentOrg.value = orgs[0] ?? null
    } finally {
      loading.value = false
    }
  }

  function clear() {
    currentOrg.value = null
  }

  return { currentOrg, loading, fetchOrg, clear }
})
