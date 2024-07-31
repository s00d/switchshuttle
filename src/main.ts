import { createApp } from 'vue';
import App from './App.vue';
import { createRouter, createWebHistory } from 'vue-router';
// import { listen, emit } from '@tauri-apps/api/event';
import globalListenerPlugin from './plugins/globalListenerPlugin';

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
    { path: '/inputs', component: Inputs },
    { path: '/loading', component: Loading },
    { path: '/update', component: Update }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

const app = createApp(App);

app.use(router);
app.use(globalListenerPlugin);

// listen('navigate', (event: any) => {
//     router.push(event.payload).then(() => {
//         // Send a confirmation event back to the backend
//         emit('navigation_complete', { route: event.payload });
//     }).catch((error) => {
//         console.error('Navigation error:', error);
//     });
// }).catch((error) => {
//     console.error('Failed to listen for navigate event:', error);
// });

app.mount('#app');
