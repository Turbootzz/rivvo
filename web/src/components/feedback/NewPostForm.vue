<script setup lang="ts">
import { ref } from 'vue'

const emit = defineEmits<{
  submit: [title: string, description: string]
  cancel: []
}>()

const title = ref('')
const description = ref('')
const loading = ref(false)

async function handleSubmit() {
  if (!title.value.trim()) return
  loading.value = true
  try {
    emit('submit', title.value, description.value)
    title.value = ''
    description.value = ''
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <form @submit.prevent="handleSubmit" class="rounded-lg border border-gray-200 bg-white p-4">
    <h3 class="text-sm font-semibold text-gray-900">New Feature Request</h3>

    <div class="mt-3 space-y-3">
      <input
        v-model="title"
        type="text"
        required
        placeholder="Title"
        class="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
      />

      <textarea
        v-model="description"
        rows="3"
        placeholder="Describe your idea... (optional)"
        class="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
      />

      <div class="flex gap-2">
        <button
          type="submit"
          :disabled="loading || !title.trim()"
          class="rounded-md bg-primary-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-primary-700 disabled:opacity-50"
        >
          Submit
        </button>
        <button
          type="button"
          @click="$emit('cancel')"
          class="rounded-md border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"
        >
          Cancel
        </button>
      </div>
    </div>
  </form>
</template>
