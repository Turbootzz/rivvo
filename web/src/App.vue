<script setup lang="ts">
import Navbar from '@/components/layout/Navbar.vue'
import Sidebar from '@/components/layout/Sidebar.vue'
import Footer from '@/components/layout/Footer.vue'
</script>

<template>
  <div class="flex min-h-screen flex-col bg-gray-50">
    <Navbar />
    <div class="flex flex-1">
      <Sidebar />
      <main class="flex-1 p-6">
        <RouterView />
      </main>
    </div>
    <Footer />
  </div>
</template>
