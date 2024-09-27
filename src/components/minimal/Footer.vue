<script setup lang="ts">
import { computed } from 'vue';
import DiscordIcon from '@/components/icons/Discord.vue';
import GithubIcon from '@/components/icons/Github.vue';
import ReplitIcon from '@/components/icons/Replit.vue';
import XIcon from '@/components/icons/X.vue';

const currentYear = computed(() => new Date().getFullYear());
</script>

<template>
  <footer flex="~ row justify-between" text-white-5:70 light:text-black-5:70>
    <div id="copyright">&copy; 2021-{{ currentYear }} &middot; Harry Baines</div>
    <div flex="~ row gap-3 items-center" id="socials">
      <a
        href="https://discord.com/users/374886124126208000"
        target="_blank"
        rel="noopener noreferrer"
        branding
        dark:hover:text-branding-discord:70
        light:hover:text-branding-discord:70
      >
        <DiscordIcon w-6 h-6 />
      </a>
      <a
        href="https://github.com/TheDrone7"
        target="_blank"
        rel="noopener noreferrer"
        branding
        dark:hover:text-branding-github-light:70
        light:hover:text-branding-github:70
      >
        <GithubIcon w-6 h-6 />
      </a>
      <a
        href="https://replit.com/@TheDrone7"
        target="_blank"
        rel="noopener noreferrer"
        branding
        dark:hover:text-branding-replit:70
        light:hover:text-branding-replit:70
      >
        <ReplitIcon w-6 h-6 />
      </a>
      <a
        href="https://x.com/TheDrone_7"
        target="_blank"
        rel="noopener noreferrer"
        branding
        dark:hover:text-branding-x-light:70
        light:hover:text-branding-x-dark:70
      >
        <XIcon w-6 h-6 />
      </a>
    </div>
  </footer>
</template>

<script lang="ts">
export default {
  name: 'MinimalFooter'
};
</script>
