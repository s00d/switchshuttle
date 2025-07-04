import "./main.css"
import { createApp } from 'vue';
import App from './App.vue';
import { createRouter, createWebHistory } from 'vue-router';

import About from './pages/About.vue';
import Editor from './pages/Editor.vue';
import Help from './pages/Help.vue';
import Inputs from './pages/Inputs.vue';
import Loading from './pages/Loading.vue';
import Main from './pages/Main.vue';
import Update from './pages/Update.vue';

const routes = [
    { path: '/', component: Main },
    { path: '/about', component: About },
    { path: '/editor', component: Editor },
    { path: '/help', component: Help },
    { path: '/inputs/:id', component: Inputs },
    { path: '/loading', component: Loading },
    { path: '/update', component: Update }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

// Handle router errors
router.onError((error) => {
    console.warn('Router error:', error);
    // Redirect to home page if route not found
    if (error.message.includes('No match found')) {
        router.push('/').catch(() => {});
    }
});

const app = createApp(App);

app.use(router);

app.mount('#app');

