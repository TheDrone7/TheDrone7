<script setup lang="ts">
import MinimalHeader from '@/components/minimal/Header.vue';
import MinimalInfo from '@/components/minimal/Info.vue';
import MinimalProjects from '@/components/minimal/Projects.vue';
import MinimalFooter from '@/components/minimal/Footer.vue';
</script>

<template>
  <main w="full" max-w-720px flex="~ col gap-4">
    <MinimalHeader />
    <MinimalInfo />
    <MinimalProjects />
    <MinimalFooter />
  </main>
</template>
