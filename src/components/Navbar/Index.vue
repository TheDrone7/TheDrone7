<script setup lang="ts">
import NavbarContainer from './Container.vue';
import NavbarBrand from './Brand.vue';
</script>

<template>
  <NavbarContainer>
    <template #left>
      <NavbarBrand />
    </template>
  </NavbarContainer>
</template>

<script lang="ts">
export default {
  name: 'NavigationBar'
};
</script>
