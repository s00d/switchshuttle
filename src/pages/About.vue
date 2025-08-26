<template>
  <div class="min-h-screen bg-slate-50">
    <main class="container mx-auto px-4 py-6">
      <div class="max-w-4xl mx-auto space-y-6">
        <!-- App Info -->
        <Card>
          <div class="text-center space-y-6">
            <div class="flex justify-center">
              <img src="/logo.svg" alt="SwitchShuttle" class="w-24 h-24" />
            </div>
            <div>
              <h2 class="text-3xl font-bold text-slate-900 mb-2">
                SwitchShuttle
              </h2>
              <p class="text-lg text-slate-600">Version {{ version }}</p>
            </div>
            <p class="text-slate-600 max-w-2xl mx-auto">
              SwitchShuttle is a powerful tool for managing terminal
              configurations. Create, edit, and switch between different
              terminal settings with ease.
            </p>
          </div>
        </Card>

        <!-- Features -->
        <Card>
          <h2 class="text-xl font-semibold text-slate-900 mb-6">Features</h2>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-3">
              <div class="flex items-center space-x-3">
                <div
                  class="w-8 h-8 bg-blue-100 flex items-center justify-center"
                >
                  <AddIcon class="w-4 h-4 text-blue-600" />
                </div>
                <h3 class="font-semibold text-slate-900">
                  Configuration Creation
                </h3>
              </div>
              <p class="text-slate-600 text-sm">
                Create customizable configurations for various terminals with
                unique commands and hotkeys.
              </p>
            </div>

            <div class="space-y-3">
              <div class="flex items-center space-x-3">
                <div
                  class="w-8 h-8 bg-green-100 flex items-center justify-center"
                >
                  <EditIcon class="w-4 h-4 text-green-600" />
                </div>
                <h3 class="font-semibold text-slate-900">Editing</h3>
              </div>
              <p class="text-slate-600 text-sm">
                Easily edit existing configurations, add new commands, and
                modify settings.
              </p>
            </div>

            <div class="space-y-3">
              <div class="flex items-center space-x-3">
                <div
                  class="w-8 h-8 bg-purple-100 flex items-center justify-center"
                >
                  <LightningIcon class="w-4 h-4 text-purple-600" />
                </div>
                <h3 class="font-semibold text-slate-900">Quick Access</h3>
              </div>
              <p class="text-slate-600 text-sm">
                Configure hotkeys for quick access to commands and
                configurations.
              </p>
            </div>

            <div class="space-y-3">
              <div class="flex items-center space-x-3">
                <div
                  class="w-8 h-8 bg-orange-100 flex items-center justify-center"
                >
                  <TerminalIcon class="w-4 h-4 text-orange-600" />
                </div>
                <h3 class="font-semibold text-slate-900">Terminal Support</h3>
              </div>
              <p class="text-slate-600 text-sm">
                Support for popular terminals: iTerm2, Terminal.app, Alacritty,
                Hyper, Warp.
              </p>
            </div>
          </div>
        </Card>

        <!-- Technical Info -->
        <Card>
          <h2 class="text-xl font-semibold text-slate-900 mb-6">
            Technical Information
          </h2>
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
              <h3 class="font-semibold text-slate-900 mb-3">
                System Requirements
              </h3>
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
            <CustomButton
              variant="secondary"
              @click="openLink('https://github.com/s00d/switchshuttle')"
            >
              <GitHubIcon class="w-4 h-4" />
              GitHub
            </CustomButton>
            <CustomButton
              variant="secondary"
              @click="openLink('https://s00d.github.io/switchshuttle/')"
            >
              <WebsiteIcon class="w-4 h-4" />
              Website
            </CustomButton>
            <CustomButton
              variant="secondary"
              @click="openLink('mailto:virus191288@gmail.com')"
            >
              <MailIcon class="w-4 h-4" />
              Support
            </CustomButton>
          </div>
        </Card>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, inject } from 'vue';
import Card from '../components/ui/Card.vue';
import CustomButton from '../components/ui/CustomButton.vue';
import AddIcon from '../components/icons/AddIcon.vue';
import EditIcon from '../components/icons/EditIcon.vue';
import LightningIcon from '../components/icons/LightningIcon.vue';
import TerminalIcon from '../components/icons/TerminalIcon.vue';
import GitHubIcon from '../components/icons/GitHubIcon.vue';
import WebsiteIcon from '../components/icons/WebsiteIcon.vue';
import MailIcon from '../components/icons/MailIcon.vue';
import { open } from '@tauri-apps/plugin-shell';
import type { TauriInjectionKey } from '../lib/tauri-commands-plugin';

// Get access to commands through plugin
const tauri = inject('tauri') as TauriInjectionKey['tauri'];

const version = ref('Loading...');

const openLink = (url: string) => {
  open(url);
};

const loadVersion = async () => {
  try {
    version.value = await tauri.get_version();
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
  border: 1px solid #e2e8f0; /* border-slate-200 */
  background-color: #f8fafc; /* bg-slate-50 */
  color: #1e293b; /* text-slate-800 */
  font-family: system-ui, sans-serif;
  font-size: 0.95rem;
  line-height: 1.7;
  text-align: center;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  border-radius: 0.75rem;
}

/* Header */
#message h2 {
  font-size: 1.5rem; /* text-2xl */
  font-weight: 700;
  margin-bottom: 1rem;
  color: #0f172a; /* text-slate-900 */
  border-bottom: 1px solid #e2e8f0; /* border-slate-200 */
  padding-bottom: 0.75rem;
}

/* Paragraphs */
#message p {
  margin-bottom: 1rem;
  font-size: 0.95rem;
  color: #334155; /* text-slate-700 */
}

/* Last paragraph without bottom margin */
#message p:last-child {
  margin-bottom: 0;
}

/* Links */
#message a {
  color: #3b82f6; /* text-blue-600 */
  text-decoration: underline;
  font-weight: 500;
  transition: color 0.2s ease;
}

#message a:hover {
  color: #2563eb; /* text-blue-700 */
  text-decoration: underline;
}

/* Fine elements (©, version, etc.) */
#message p small {
  font-size: 0.8rem;
  color: #64748b; /* text-slate-500 */
}
</style>
