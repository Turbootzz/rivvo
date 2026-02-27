<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Trash2, Pencil, Plus, X } from 'lucide-vue-next'
import { useOrgStore } from '@/stores/org'
import { useChangelogStore } from '@/stores/changelog'
import type { ChangelogEntry } from '@/types'

const orgStore = useOrgStore()
const changelogStore = useChangelogStore()

const error = ref('')
const showForm = ref(false)
const editingEntry = ref<ChangelogEntry | null>(null)
const formTitle = ref('')
const formBody = ref('')

onMounted(async () => {
  if (!orgStore.currentOrg) await orgStore.fetchOrg()
  if (orgStore.currentOrg) {
    await changelogStore.fetchAdminEntries(orgStore.currentOrg.id)
  }
})

function startCreate() {
  editingEntry.value = null
  formTitle.value = ''
  formBody.value = ''
  showForm.value = true
}

function startEdit(entry: ChangelogEntry) {
  editingEntry.value = entry
  formTitle.value = entry.title
  formBody.value = entry.body
  showForm.value = true
}

function cancelForm() {
  showForm.value = false
  editingEntry.value = null
}

async function handleSave(publish: boolean) {
  if (!orgStore.currentOrg) return
  error.value = ''
  try {
    if (editingEntry.value) {
      await changelogStore.updateEntry(
        editingEntry.value.id,
        formTitle.value,
        formBody.value,
        !publish,
      )
    } else {
      const entry = await changelogStore.createEntry(
        orgStore.currentOrg.id,
        formTitle.value,
        formBody.value,
      )
      if (publish && entry) {
        await changelogStore.updateEntry(entry.id, formTitle.value, formBody.value, false)
      }
    }
    showForm.value = false
    editingEntry.value = null
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to save entry'
  }
}

async function handleDelete(id: string) {
  error.value = ''
  try {
    await changelogStore.deleteEntry(id)
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to delete entry'
  }
}
</script>

<template>
  <div class="mx-auto max-w-3xl">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold text-gray-900">Manage Changelog</h1>
      <button
        v-if="!showForm"
        @click="startCreate"
        class="flex items-center gap-1 rounded-md bg-primary-600 px-3 py-1.5 text-xs font-semibold text-white shadow-sm hover:bg-primary-700"
      >
        <Plus class="h-3.5 w-3.5" />
        New Entry
      </button>
    </div>

    <div v-if="error" class="mt-4 rounded-md bg-red-50 p-3 text-sm text-red-700">{{ error }}</div>

    <!-- Create/Edit form -->
    <div v-if="showForm" class="mt-4 rounded-lg border border-gray-200 bg-white p-4">
      <div class="flex items-center justify-between">
        <h2 class="text-sm font-semibold text-gray-700">
          {{ editingEntry ? 'Edit Entry' : 'New Changelog Entry' }}
        </h2>
        <button @click="cancelForm" class="text-gray-400 hover:text-gray-600">
          <X class="h-4 w-4" />
        </button>
      </div>
      <input
        v-model="formTitle"
        type="text"
        placeholder="Title"
        class="mt-3 w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
      />
      <textarea
        v-model="formBody"
        rows="6"
        placeholder="What's new? Describe the changes..."
        class="mt-2 w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
      ></textarea>
      <div class="mt-3 flex gap-2">
        <button
          @click="handleSave(false)"
          :disabled="!formTitle.trim() || !formBody.trim()"
          class="rounded-md border border-gray-300 px-3 py-1.5 text-xs font-medium text-gray-700 hover:bg-gray-50 disabled:opacity-50"
        >
          Save as Draft
        </button>
        <button
          @click="handleSave(true)"
          :disabled="!formTitle.trim() || !formBody.trim()"
          class="rounded-md bg-primary-600 px-3 py-1.5 text-xs font-semibold text-white shadow-sm hover:bg-primary-700 disabled:opacity-50"
        >
          {{ editingEntry && !editingEntry.is_draft ? 'Update' : 'Publish' }}
        </button>
      </div>
    </div>

    <!-- Entries list -->
    <div v-if="changelogStore.loading" class="mt-8 text-center text-sm text-gray-500">
      Loading...
    </div>

    <div
      v-else-if="changelogStore.entries.length === 0 && !showForm"
      class="mt-8 text-center text-sm text-gray-500"
    >
      No changelog entries yet. Create one to get started.
    </div>

    <div v-else class="mt-6 space-y-4">
      <div
        v-for="entry in changelogStore.entries"
        :key="entry.id"
        class="rounded-lg border border-gray-200 bg-white p-4"
      >
        <div class="flex items-start justify-between">
          <div>
            <div class="flex items-center gap-2">
              <h3 class="font-semibold text-gray-900">{{ entry.title }}</h3>
              <span
                v-if="entry.is_draft"
                class="rounded-full bg-yellow-100 px-2 py-0.5 text-xs font-medium text-yellow-700"
              >
                Draft
              </span>
              <span
                v-else
                class="rounded-full bg-green-100 px-2 py-0.5 text-xs font-medium text-green-700"
              >
                Published
              </span>
            </div>
            <p v-if="entry.published_at" class="mt-1 text-xs text-gray-400">
              Published {{ new Date(entry.published_at).toLocaleDateString() }}
            </p>
            <p v-else class="mt-1 text-xs text-gray-400">
              Created {{ new Date(entry.created_at).toLocaleDateString() }}
            </p>
          </div>
          <div class="flex gap-1">
            <button
              @click="startEdit(entry)"
              class="rounded-md p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-600"
            >
              <Pencil class="h-4 w-4" />
            </button>
            <button
              @click="handleDelete(entry.id)"
              class="rounded-md p-1.5 text-gray-400 hover:bg-red-50 hover:text-red-600"
            >
              <Trash2 class="h-4 w-4" />
            </button>
          </div>
        </div>
        <div class="mt-2 line-clamp-3 whitespace-pre-wrap text-sm text-gray-600">
          {{ entry.body }}
        </div>
      </div>
    </div>
  </div>
</template>
