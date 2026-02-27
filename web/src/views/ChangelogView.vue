<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import { useChangelogStore } from '@/stores/changelog'

const route = useRoute()
const changelogStore = useChangelogStore()

const orgSlug = computed(() => route.params.orgSlug as string)

onMounted(() => changelogStore.fetchPublicEntries(orgSlug.value))
</script>

<template>
  <div class="mx-auto max-w-3xl">
    <h1 class="text-2xl font-bold text-gray-900">Changelog</h1>

    <div v-if="changelogStore.loading" class="mt-8 text-center text-sm text-gray-500">
      Loading...
    </div>

    <div
      v-else-if="changelogStore.entries.length === 0"
      class="mt-8 text-center text-sm text-gray-500"
    >
      No changelog entries yet.
    </div>

    <div v-else class="mt-6 space-y-8">
      <article
        v-for="entry in changelogStore.entries"
        :key="entry.id"
        class="border-l-2 border-gray-200 pl-4"
      >
        <time v-if="entry.published_at" class="text-xs text-gray-400">
          {{ new Date(entry.published_at).toLocaleDateString() }}
        </time>
        <h2 class="mt-1 text-lg font-semibold text-gray-900">{{ entry.title }}</h2>
        <div class="mt-2 whitespace-pre-wrap text-sm text-gray-700">{{ entry.body }}</div>
      </article>
    </div>
  </div>
</template>
