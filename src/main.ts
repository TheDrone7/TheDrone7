import '@unocss/reset/eric-meyer.css';
import 'virtual:uno.css';
import './assets/base.css';
import './assets/dm.css';

import { createApp } from 'vue';

import App from './App.vue';
import router from './router';

const app = createApp(App);

app.use(router);

app.mount('#app');
