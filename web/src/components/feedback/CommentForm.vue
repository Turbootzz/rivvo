<script setup lang="ts">
import { ref } from 'vue'

const emit = defineEmits<{
  submit: [body: string]
}>()

const body = ref('')
const loading = ref(false)

async function handleSubmit() {
  if (!body.value.trim()) return
  loading.value = true
  try {
    emit('submit', body.value)
    body.value = ''
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <form @submit.prevent="handleSubmit" class="flex gap-2">
    <textarea
      v-model="body"
      rows="2"
      required
      placeholder="Write a comment..."
      class="flex-1 rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
    />
    <button
      type="submit"
      :disabled="loading || !body.trim()"
      class="self-end rounded-md bg-primary-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-primary-700 disabled:opacity-50"
    >
      Reply
    </button>
  </form>
</template>
