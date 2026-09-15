<template>
  <div :class="ui.root()">
    <main :class="ui.main()">
      <div :class="ui.card()">
        <div :class="ui.brand()">
          <img src="/logo.svg" alt="" :class="ui.logo()" />
          <div :class="ui.brandText()">
            <h1 :class="ui.title()">SwitchShuttle</h1>
            <p :class="ui.meta()">Version {{ version }} · menu bar app</p>
          </div>
        </div>

        <p :class="ui.pitch()">
          Run and organize terminal commands from the tray — with hotkeys,
          schedules, monitors, and stoppable background jobs.
        </p>

        <div :class="ui.chips()">
          <span
            v-for="chip in chips"
            :key="chip.label"
            :class="ui.chip()"
          >
            <span :class="ui.chipDot({ tone: chip.tone })" />
            {{ chip.label }}
          </span>
        </div>

        <p :class="ui.stack()">
          <span :class="ui.stackLabel()">Built with</span>
          <span>Vue</span>
          <span :class="ui.stackSep()">·</span>
          <span>Tauri</span>
          <span :class="ui.stackSep()">·</span>
          <span>Rust</span>
          <span :class="ui.stackSep()">·</span>
          <span>TypeScript</span>
        </p>

        <div :class="ui.actions()">
          <CustomButton
            variant="secondary"
            size="sm"
            @click="openLink('https://github.com/s00d/switchshuttle')"
          >
            <GitHubIcon :class="ui.actionIcon()" />
            GitHub
          </CustomButton>
          <CustomButton
            variant="secondary"
            size="sm"
            @click="openLink('https://s00d.github.io/switchshuttle/')"
          >
            <WebsiteIcon :class="ui.actionIcon()" />
            Website
          </CustomButton>
          <CustomButton
            variant="ghost"
            size="sm"
            @click="openLink('mailto:virus191288@gmail.com')"
          >
            <MailIcon :class="ui.actionIcon()" />
            Support
          </CustomButton>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { inject, onMounted, ref } from 'vue';
import { open } from '@tauri-apps/plugin-shell';
import CustomButton from '../components/ui/CustomButton.vue';
import GitHubIcon from '../components/icons/GitHubIcon.vue';
import WebsiteIcon from '../components/icons/WebsiteIcon.vue';
import MailIcon from '../components/icons/MailIcon.vue';
import type { TauriInjectionKey } from '../lib/tauri-commands-plugin';
import { aboutTv } from './aboutTheme';

defineOptions({ name: 'About' });

const tauri = inject('tauri') as TauriInjectionKey['tauri'];
const ui = aboutTv();
const version = ref('…');

const chips = [
  { label: 'Global hotkeys', tone: 'blue' as const },
  { label: 'Multi-terminal', tone: 'green' as const },
  { label: 'Background jobs', tone: 'amber' as const },
  { label: 'Visual editor', tone: 'slate' as const },
];

const openLink = (url: string) => {
  open(url);
};

onMounted(async () => {
  try {
    version.value = await tauri.get_version();
  } catch {
    version.value = 'Unknown';
  }
});
</script>
