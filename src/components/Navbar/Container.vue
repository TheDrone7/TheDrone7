<template>
  <nav flex items-center justify-between px-4 py-2 bg-white dark:bg-gray-8>
    <slot name="left"></slot>
    <slot name="center"></slot>
    <slot name="right"></slot>
  </nav>
</template>

<script lang="ts">
export default {
  name: 'NavbarContainer'
};
</script>
