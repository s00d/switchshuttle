import { createApp } from 'vue';
import App from './App.vue';
import { createRouter, createWebHistory } from 'vue-router';

import About from './pages/About.vue';
import Create from './pages/Create.vue';
import Editor from './pages/Editor.vue';
import Inputs from './pages/Inputs.vue';
import Loading from './pages/Loading.vue';
import Main from './pages/Main.vue';
import Update from './pages/Update.vue';

const routes = [
    { path: '/', component: Main },
    { path: '/about', component: About },
    { path: '/create', component: Create },
    { path: '/editor', component: Editor },
    { path: '/inputs/:id', component: Inputs },
    { path: '/loading', component: Loading },
    { path: '/update', component: Update }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

const app = createApp(App);

app.use(router);

app.mount('#app');
