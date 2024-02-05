<script setup lang="ts">
const works = [
  {
    title: 'Freelance Web Developer',
    date: 'April 2023 - August 2023',
    details: [
      'Developed several web applications such as portfolio websites, internal tools, and analytics dashboards.',
      'Used a variety of technologies such as Next.js, Node.js, and PostgreSQL to build these applications.',
      'Worked with clients to understand their requirements and provide them with the best possible solution.',
      'Satisfied all clients with the final product and received positive feedback.'
    ]
  },
  {
    title: 'Trust and Safety Developer & Community Manager, Replit',
    date: 'April 2020 - March 2023',
    details: [
      'Lead a team of volunteers to moderate the community and ensure a safe environment for all users.',
      'Developed security features to protect users from malicious content.',
      'Overhauled the moderation system to make it more efficient and user-friendly for the volunteers.',
      'Worked with the engineering team to develop new features and improve the platform.',
      'Organized and hosted community events to engage with the users and gather feedback.',
      'Created, maintained, and enforced the community guidelines and terms of service.'
    ]
  }
];
</script>

<template>
  <section flex="~ col gap-4" pb-4 divider>
    <h2 text-xl font-bold>Previous work</h2>
    <div flex="~ col gap-4">
      <div block py-2 v-for="work in works" :key="work.title">
        <h3 font-bold>{{ work.title }}</h3>
        <p text-white-5 light:text-black-5 pb-2>{{ work.date }}</p>
        <ul flex="~ col" pl-4>
          <li v-for="(item, i) in work.details" :key="i" v-text="item" list-disc text-white-3 light:text-black-3 />
        </ul>
      </div>
    </div>
  </section>
</template>

<script lang="ts">
export default {
  name: 'MinimalWork'
};
</script>
