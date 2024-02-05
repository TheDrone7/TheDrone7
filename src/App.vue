<script setup lang="ts">
import { ref } from 'vue';
import { RouterView } from 'vue-router';
import NavigationBar from '@/components/Navbar/Index.vue';
import { useLocalStorage, useColorMode } from '@vueuse/core';

const colorMode = useColorMode();
const preferredMode = useLocalStorage<'full' | 'minimal'>('preferredMode', 'minimal');

const switchStatus = ref<'off' | 'transitioning' | 'transitioned'>('off');
const toggleTheme = () => {
  switchStatus.value = 'transitioning';
  setTimeout(() => {
    switchStatus.value = 'transitioned';
    colorMode.value = colorMode.value === 'light' ? 'dark' : 'light';
  }, 300);
  setTimeout(() => {
    switchStatus.value = 'off';
  }, 600);
};
</script>

<template>
  <button fab @click="toggleTheme" :class="{ 'animate-spin': switchStatus !== 'off' }">
    {{ colorMode === 'light' ? '☀️' : '🌙' }}
  </button>
  <div class="main" :class="switchStatus === 'transitioning' ? 'animate-fade-out' : 'animate-fade-in'">
    <NavigationBar v-if="preferredMode === 'full'" />
    <RouterView />
  </div>
</template>
