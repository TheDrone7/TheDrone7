<script setup lang="ts">
const projects = [
  {
    title: 'FHEDB',
    desc: 'A No-SQL database engine and query language written from scratch in rust',
    links: [
      {
        link: 'https://github.com/TheDrone7/fhedb',
        text: 'Source'
      }
    ]
  },
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
        link: 'https://github.com/goodanimee/website',
        text: 'Source'
      },
      {
        link: 'https://goodanime.thedrone7.dev',
        text: 'Website'
      }
    ]
  },
  {
    title: 'GoodAnime mobile',
    desc: 'A mobile app for managing AniList anime lists.',
    links: [
      {
        link: 'https://github.com/goodanimee/mobile',
        text: 'Source'
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
  }
];
</script>

<template>
  <section flex="~ col gap-4" pb-4 divider>
    <h2 text-xl font-bold>My projects</h2>
    <table>
      <tr v-for="project in projects" :key="project.title">
        <th v-text="project.title" text="right" font-bold pr-4 border="r white-5/20 solid" />
        <td pl-4 py-2>
          <p v-text="project.desc" />
          <div flex="~ row gap-2">
            <a
              v-for="(link, i) in project.links"
              :key="link.text"
              :href="link.link"
              target="_blank"
              rel="noopener noreferrer"
              pr-2
              :class="{ 'border-r border-solid border-white-5/10': i !== project.links.length - 1 }"
            >
              {{ link.text }}
            </a>
          </div>
        </td>
      </tr>
    </table>
    <div flex="~ col gap-3">
      <div v-for="project in projects" :key="project.title" flex="~ row gap-1"></div>
      <p font-italic text-white-5 light:text-black-5>There are probably more.</p>
    </div>
  </section>
</template>

<script lang="ts">
export default {
  name: 'MinimalProjects'
};
</script>
