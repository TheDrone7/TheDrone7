<script setup lang="ts">
const projects = [
  {
    title: 'Shieldbow',
    desc: "Node.js library for interfacing with RIOT Games's API.",
    links: [
      {
        link: 'https://github.com/TheDrone7/shieldbow',
        text: 'Source'
      },
      {
        link: 'https://shieldbow.thedrone7.dev',
        text: 'Docs'
      }
    ]
  },
  {
    title: 'GoodAnime',
    desc: 'A simple random anime suggestion generator.',
    links: [
      {
        link: 'https://github.com/TheDrone7/goodanime',
        text: 'Source'
      },
      {
        link: 'https://goodanime.thedrone7.dev',
        text: 'Website'
      }
    ]
  },
  {
    title: 'CoronaJam',
    desc: 'A game made in 48 hours for a COVID-based hackathon.',
    links: [
      {
        link: 'https://replit.com/@TheDrone7/cjnine',
        text: 'Source'
      },
      {
        link: 'coronajam.thedrone7.dev',
        text: 'Play'
      }
    ]
  },
  {
    title: 'Portfolio',
    desc: 'The current website you are viewing.',
    links: [
      {
        link: 'https://github.com/TheDrone7/TheDrone7',
        text: 'Source'
      },
      {
        link: '#',
        text: 'Website'
      }
    ]
  },
  {
    title: 'CPPest',
    desc: 'A simple C++ testing framework inspired by Jest (WIP).',
    links: [
      {
        link: 'https://gitlab.com/TheDrone7/cppest',
        text: 'Source'
      }
    ]
  }
];
</script>

<template>
  <section flex="~ col gap-4" pb-4 divider>
    <h2 text-xl font-bold>Other projects</h2>
    <div flex="~ col gap-3">
      <div v-for="project in projects" :key="project.title">
        <div flex="~ row gap-1">
          <h3 font-bold>{{ project.title }}</h3>
          :
          <p text-white-3 light:text-black-3>{{ project.desc }}</p>
        </div>
        <div flex="~ row gap-2">
          <div flex="~ row gap-2 justify-center items-center" v-for="(link, i) in project.links" :key="i">
            <a :href="link.link" target="_blank" rel="noopener noreferrer">
              {{ link.text }}
            </a>
            <span v-if="i < project.links.length - 1" text-white-5:30 light:text-black-5:30>|</span>
          </div>
        </div>
      </div>
      <p font-italic text-white-5 light:text-black-5>There are probably more.</p>
    </div>
  </section>
</template>

<script lang="ts">
export default {
  name: 'MinimalProjects'
};
</script>
