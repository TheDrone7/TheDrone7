<script setup lang="ts">
import BrandIcon from '@/components/icons/Brand.vue';
</script>

<template>
  <div flex items-center gap-2>
    <BrandIcon h-8 w-8 />
    <span text-lg font-bold>Harmeet Singh Bhatia</span>
  </div>
</template>

<script lang="ts">
export default {
  name: 'NavbarBrand'
};
</script>
