<script setup lang="ts">
import { MessageSquarePlus, Vote, Map } from 'lucide-vue-next'
</script>

<template>
  <div class="flex flex-col items-center justify-center py-16">
    <h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl">
      Collect feedback, build what matters
    </h1>
    <p class="mt-4 max-w-2xl text-center text-lg text-gray-600">
      Let your users submit feature requests, vote on ideas, and track progress.
      Open-source, self-hostable, and blazing fast.
    </p>
    <div class="mt-8 flex gap-4">
      <RouterLink
        to="/login"
        class="rounded-md bg-primary-600 px-6 py-3 text-sm font-semibold text-white shadow-sm hover:bg-primary-700"
      >
        Get Started
      </RouterLink>
      <a
        href="https://github.com/turbootzz/rivvo"
        target="_blank"
        class="rounded-md border border-gray-300 bg-white px-6 py-3 text-sm font-semibold text-gray-700 shadow-sm hover:bg-gray-50"
      >
        View on GitHub
      </a>
    </div>

    <div class="mt-16 grid max-w-4xl grid-cols-1 gap-8 sm:grid-cols-3">
      <div class="flex flex-col items-center text-center">
        <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-primary-100">
          <MessageSquarePlus class="h-6 w-6 text-primary-600" />
        </div>
        <h3 class="mt-4 text-lg font-semibold text-gray-900">Feedback Boards</h3>
        <p class="mt-2 text-sm text-gray-600">
          Organize feature requests into boards. Users submit ideas, you stay organized.
        </p>
      </div>
      <div class="flex flex-col items-center text-center">
        <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-primary-100">
          <Vote class="h-6 w-6 text-primary-600" />
        </div>
        <h3 class="mt-4 text-lg font-semibold text-gray-900">Voting</h3>
        <p class="mt-2 text-sm text-gray-600">
          Let users upvote the features they want most. Prioritize by demand.
        </p>
      </div>
      <div class="flex flex-col items-center text-center">
        <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-primary-100">
          <Map class="h-6 w-6 text-primary-600" />
        </div>
        <h3 class="mt-4 text-lg font-semibold text-gray-900">Public Roadmap</h3>
        <p class="mt-2 text-sm text-gray-600">
          Share your roadmap with users. Keep them informed on what's coming next.
        </p>
      </div>
    </div>
  </div>
</template>
