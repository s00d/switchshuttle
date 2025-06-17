<template>
  <div class="min-h-screen bg-slate-50">
    
    <main class="container mx-auto px-4 py-6">
      <div class="max-w-4xl mx-auto space-y-6">
        <!-- Header -->
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold text-slate-900">About</h1>
            <p class="text-slate-600 mt-1">Learn more about SwitchShuttle</p>
          </div>
        </div>

        <!-- App Info -->
        <Card>
          <div class="text-center space-y-6">
            <div class="flex justify-center">
              <img src="/logo.svg" alt="SwitchShuttle" class="w-24 h-24" />
            </div>
            <div>
              <h2 class="text-3xl font-bold text-slate-900 mb-2">SwitchShuttle</h2>
              <p class="text-lg text-slate-600">Version {{ version }}</p>
            </div>
            <p class="text-slate-600 max-w-2xl mx-auto">
              SwitchShuttle is a powerful tool for managing terminal configurations. 
              Create, edit, and switch between different terminal settings with ease.
            </p>
          </div>
        </Card>

        <!-- Features -->
        <Card>
          <h2 class="text-xl font-semibold text-slate-900 mb-6">Features</h2>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-3">
              <div class="flex items-center space-x-3">
                <div class="w-8 h-8 bg-blue-100 flex items-center justify-center">
                  <svg class="w-4 h-4 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                  </svg>
                </div>
                <h3 class="font-semibold text-slate-900">Configuration Creation</h3>
              </div>
              <p class="text-slate-600 text-sm">
                Create customizable configurations for various terminals with unique commands and hotkeys.
              </p>
            </div>
            
            <div class="space-y-3">
              <div class="flex items-center space-x-3">
                <div class="w-8 h-8 bg-green-100 flex items-center justify-center">
                  <svg class="w-4 h-4 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                  </svg>
                </div>
                <h3 class="font-semibold text-slate-900">Editing</h3>
              </div>
              <p class="text-slate-600 text-sm">
                Easily edit existing configurations, add new commands, and modify settings.
              </p>
            </div>
            
            <div class="space-y-3">
              <div class="flex items-center space-x-3">
                <div class="w-8 h-8 bg-purple-100 flex items-center justify-center">
                  <svg class="w-4 h-4 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                  </svg>
                </div>
                <h3 class="font-semibold text-slate-900">Quick Access</h3>
              </div>
              <p class="text-slate-600 text-sm">
                Configure hotkeys for quick access to commands and configurations.
              </p>
            </div>
            
            <div class="space-y-3">
              <div class="flex items-center space-x-3">
                <div class="w-8 h-8 bg-orange-100 flex items-center justify-center">
                  <svg class="w-4 h-4 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                  </svg>
                </div>
                <h3 class="font-semibold text-slate-900">Terminal Support</h3>
              </div>
              <p class="text-slate-600 text-sm">
                Support for popular terminals: iTerm2, Terminal.app, Alacritty, Hyper, Warp.
              </p>
            </div>
          </div>
        </Card>

        <!-- Technical Info -->
        <Card>
          <h2 class="text-xl font-semibold text-slate-900 mb-6">Technical Information</h2>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h3 class="font-semibold text-slate-900 mb-3">Technologies</h3>
              <ul class="space-y-2 text-sm text-slate-600">
                <li class="flex items-center space-x-2">
                  <span class="w-2 h-2 bg-blue-500"></span>
                  <span>Vue.js 3 - Frontend Framework</span>
                </li>
                <li class="flex items-center space-x-2">
                  <span class="w-2 h-2 bg-green-500"></span>
                  <span>Tauri - Desktop Framework</span>
                </li>
                <li class="flex items-center space-x-2">
                  <span class="w-2 h-2 bg-purple-500"></span>
                  <span>TypeScript - Type Safety</span>
                </li>
                <li class="flex items-center space-x-2">
                  <span class="w-2 h-2 bg-orange-500"></span>
                  <span>Tailwind CSS - Styling</span>
                </li>
              </ul>
            </div>
            
            <div>
              <h3 class="font-semibold text-slate-900 mb-3">System Requirements</h3>
              <ul class="space-y-2 text-sm text-slate-600">
                <li class="flex items-center space-x-2">
                  <span class="w-2 h-2 bg-blue-500"></span>
                  <span>macOS 10.15 or newer</span>
                </li>
                <li class="flex items-center space-x-2">
                  <span class="w-2 h-2 bg-green-500"></span>
                  <span>4 GB RAM</span>
                </li>
                <li class="flex items-center space-x-2">
                  <span class="w-2 h-2 bg-purple-500"></span>
                  <span>100 MB free space</span>
                </li>
              </ul>
            </div>
          </div>
        </Card>

        <!-- Links -->
        <Card>
          <h2 class="text-xl font-semibold text-slate-900 mb-6">Links</h2>
          <div class="flex flex-wrap gap-4">
            <Button variant="secondary" @click="openLink('https://github.com/s00d/switchshuttle')">
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
              GitHub
            </Button>
            <Button variant="secondary" @click="openLink('https://s00d.github.io/switchshuttle/')">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9v-9m0-9v9" />
              </svg>
              Website
            </Button>
            <Button variant="secondary" @click="openLink('mailto:virus191288@gmail.com')">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 4.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
              </svg>
              Support
            </Button>
          </div>
        </Card>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import Card from '../components/Card.vue';
import Button from '../components/Button.vue';
import { open } from '@tauri-apps/plugin-shell';

const version = ref('Loading...');

const openLink = (url: string) => {
  open(url);
};

const loadVersion = async () => {
  try {
    version.value = await invoke('get_version');
  } catch (error) {
    console.error('Failed to load version:', error);
    version.value = 'Unknown';
  }
};

onMounted(() => {
  loadVersion();
});
</script>

<style>
#message {
  padding: 1rem;
  border: 1px solid #e2e8f0;       /* border-slate-200 */
  background-color: #f8fafc;       /* bg-slate-50 */
  color: #1e293b;                  /* text-slate-800 */
  font-family: system-ui, sans-serif;
  font-size: 0.95rem;
  line-height: 1.7;
  text-align: center;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  border-radius: 0.75rem;
}

/* Header */
#message h2 {
  font-size: 1.5rem;               /* text-2xl */
  font-weight: 700;
  margin-bottom: 1rem;
  color: #0f172a;                 /* text-slate-900 */
  border-bottom: 1px solid #e2e8f0; /* border-slate-200 */
  padding-bottom: 0.75rem;
}

/* Paragraphs */
#message p {
  margin-bottom: 1rem;
  font-size: 0.95rem;
  color: #334155;                 /* text-slate-700 */
}

/* Last paragraph without bottom margin */
#message p:last-child {
  margin-bottom: 0;
}

/* Links */
#message a {
  color: #3b82f6;                 /* text-blue-600 */
  text-decoration: underline;
  font-weight: 500;
  transition: color 0.2s ease;
}

#message a:hover {
  color: #2563eb;                 /* text-blue-700 */
  text-decoration: underline;
}

/* Fine elements (©, version, etc.) */
#message p small {
  font-size: 0.8rem;
  color: #64748b;                 /* text-slate-500 */
}
</style>
