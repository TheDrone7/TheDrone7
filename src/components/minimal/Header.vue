<script setup lang="ts"></script>

<template>
  <header flex="~ row items-center justify-start gap-4" w="full" p="b-4" divider>
    <img src="@/assets/pfp.png" alt="Custom profile picture" h-12 w-12 />
    <div>
      <h1 text-3xl font="bold italic mono">
         Harry Baines (<span
          highlight
          title="This is just my online handle. It turned out to be fairly unique so I use it everywhere I can."
          >@TheDrone7</span
        >)
      </h1>
      <a href="mailto:h@thedrone7.dev" target="_blank" rel="noopener noreferrer" flex="~ row items-center gap-2"
        ><i class="i-material-symbols:mail" text-lg /> h@thedrone7.dev</a
      >
    </div>
  </header>
</template>

<script lang="ts">
export default {
  name: 'MinimalHeader'
};
</script>
