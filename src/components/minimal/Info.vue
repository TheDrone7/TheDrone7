<script setup lang="ts">
import { computed } from 'vue';
const startDate = new Date('1 September 2021');
const diff = computed(() => {
  const now = new Date();
  const diff = now.getTime() - startDate.getTime();
  return Math.ceil(diff / (1000 * 60 * 60 * 24 * 365));
});

const suffix = computed(() => {
  const year = diff.value;
  if (year === 1) return 'st';
  if (year === 2) return 'nd';
  if (year === 3) return 'rd';
  return 'th';
});
</script>

<template>
  <section flex="~ col gap-4" pb-4 divider>
    <h2 text-xl font-bold>Info</h2>
    <div>
      <p block py-2>
        Hi! I'm <span highlight>Harmeet Singh</span>, a software developer based
        in <span highlight>Bengaluru, India</span>.
      </p>
      <p block py-2>
        I am (mostly) self-taught in web development, cybersecurity and systems administration. I have a passion for
        learning and exploring new technologies or frameworks.
      </p>
    </div>
  </section>
</template>

<script lang="ts">
export default {
  name: 'MinimalInfo'
};
</script>
