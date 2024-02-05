<script setup lang="ts">
import { useColorMode } from '@vueuse/core';

const colorMode = useColorMode();
</script>

<template>
  <button fab @click="colorMode = colorMode === 'light' ? 'dark' : 'light'">
    {{ colorMode === 'light' ? '🌙' : '☀️' }}
  </button>
</template>
