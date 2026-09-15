<template>
  <div :class="shell.root()">
    <main :class="shell.main()">
      <div :class="shell.content()">
        <div :class="ui.hero()">
          <div :class="ui.heroInner()">
            <div :class="ui.logoWrap()">
              <img src="/logo.svg" alt="SwitchShuttle" :class="ui.logo()" />
            </div>
            <div>
              <h2 :class="ui.heroTitle()">SwitchShuttle</h2>
              <p :class="ui.heroVersion()">Version {{ version }}</p>
            </div>
            <p :class="ui.heroDesc()">
              SwitchShuttle is a powerful tool for managing terminal
              configurations. Create, edit, and switch between different
              terminal settings with ease.
            </p>
          </div>
        </div>

        <div :class="ui.section()">
          <h2 :class="ui.sectionTitle()">Features</h2>
          <div :class="ui.featureGrid()">
            <div :class="ui.feature()">
              <div :class="ui.featureHead()">
                <div :class="ui.featureIcon({ tone: 'blue' })">
                  <AddIcon :class="ui.featureIconInner({ tone: 'blue' })" />
                </div>
                <h3 :class="ui.featureName()">Configuration Creation</h3>
              </div>
              <p :class="ui.featureDesc()">
                Create customizable configurations for various terminals with
                unique commands and hotkeys.
              </p>
            </div>

            <div :class="ui.feature()">
              <div :class="ui.featureHead()">
                <div :class="ui.featureIcon({ tone: 'green' })">
                  <EditIcon :class="ui.featureIconInner({ tone: 'green' })" />
                </div>
                <h3 :class="ui.featureName()">Editing</h3>
              </div>
              <p :class="ui.featureDesc()">
                Easily edit existing configurations, add new commands, and
                modify settings.
              </p>
            </div>

            <div :class="ui.feature()">
              <div :class="ui.featureHead()">
                <div :class="ui.featureIcon({ tone: 'purple' })">
                  <LightningIcon
                    :class="ui.featureIconInner({ tone: 'purple' })"
                  />
                </div>
                <h3 :class="ui.featureName()">Quick Access</h3>
              </div>
              <p :class="ui.featureDesc()">
                Configure hotkeys for quick access to commands and
                configurations.
              </p>
            </div>

            <div :class="ui.feature()">
              <div :class="ui.featureHead()">
                <div :class="ui.featureIcon({ tone: 'orange' })">
                  <TerminalIcon
                    :class="ui.featureIconInner({ tone: 'orange' })"
                  />
                </div>
                <h3 :class="ui.featureName()">Terminal Support</h3>
              </div>
              <p :class="ui.featureDesc()">
                Support for popular terminals: iTerm2, Terminal.app, Alacritty,
                Hyper, Warp.
              </p>
            </div>
          </div>
        </div>

        <div :class="ui.section()">
          <h2 :class="ui.sectionTitle()">Technical Information</h2>
          <div :class="ui.techGrid()">
            <div>
              <h3 :class="ui.techTitle()">Technologies</h3>
              <ul :class="ui.techList()">
                <li :class="ui.techItem()">
                  <span :class="ui.dot({ tone: 'blue' })" />
                  <span>Vue.js 3 - Frontend Framework</span>
                </li>
                <li :class="ui.techItem()">
                  <span :class="ui.dot({ tone: 'green' })" />
                  <span>Tauri - Desktop Framework</span>
                </li>
                <li :class="ui.techItem()">
                  <span :class="ui.dot({ tone: 'purple' })" />
                  <span>TypeScript - Type Safety</span>
                </li>
                <li :class="ui.techItem()">
                  <span :class="ui.dot({ tone: 'orange' })" />
                  <span>Tailwind CSS - Styling</span>
                </li>
              </ul>
            </div>

            <div>
              <h3 :class="ui.techTitle()">System Requirements</h3>
              <ul :class="ui.techList()">
                <li :class="ui.techItem()">
                  <span :class="ui.dot({ tone: 'blue' })" />
                  <span>macOS 10.15 or newer</span>
                </li>
                <li :class="ui.techItem()">
                  <span :class="ui.dot({ tone: 'green' })" />
                  <span>4 GB RAM</span>
                </li>
                <li :class="ui.techItem()">
                  <span :class="ui.dot({ tone: 'purple' })" />
                  <span>100 MB free space</span>
                </li>
              </ul>
            </div>
          </div>
        </div>

        <div :class="ui.section()">
          <h2 :class="ui.sectionTitle()">Links</h2>
          <div :class="ui.links()">
            <CustomButton
              variant="secondary"
              @click="openLink('https://github.com/s00d/switchshuttle')"
            >
              <GitHubIcon :class="ui.linkIcon()" />
              GitHub
            </CustomButton>
            <CustomButton
              variant="secondary"
              @click="openLink('https://s00d.github.io/switchshuttle/')"
            >
              <WebsiteIcon :class="ui.linkIcon()" />
              Website
            </CustomButton>
            <CustomButton
              variant="secondary"
              @click="openLink('mailto:virus191288@gmail.com')"
            >
              <MailIcon :class="ui.linkIcon()" />
              Support
            </CustomButton>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, inject } from 'vue';
import { tv } from '@/lib/tv';
import { pageShellTv, cardTv } from '@/components/ui/themes';
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

defineOptions({ name: 'About' });

const tauri = inject('tauri') as TauriInjectionKey['tauri'];

const version = ref('Loading...');

const shell = pageShellTv({ width: 'md' });

const aboutTv = tv({
  slots: {
    hero: cardTv({ hover: false, padding: 'md' }),
    heroInner: 'text-center space-y-3',
    logoWrap: 'flex justify-center',
    logo: 'w-16 h-16',
    heroTitle: 'text-lg font-semibold text-slate-900',
    heroVersion: 'text-sm text-slate-500 mt-0.5',
    heroDesc: 'text-xs text-slate-600 max-w-xl mx-auto',
    section: [cardTv({ hover: false, padding: 'md' }), 'space-y-3'].join(' '),
    sectionTitle: 'text-sm font-semibold text-slate-900',
    featureGrid: 'grid grid-cols-1 md:grid-cols-2 gap-3',
    feature: 'space-y-1.5',
    featureHead: 'flex items-center gap-2',
    featureIcon: 'w-7 h-7 flex items-center justify-center rounded-md',
    featureIconInner: 'w-3.5 h-3.5',
    featureName: 'font-medium text-sm text-slate-900',
    featureDesc: 'text-xs text-slate-600',
    techGrid: 'grid grid-cols-1 md:grid-cols-2 gap-4',
    techTitle: 'font-medium text-sm text-slate-900 mb-2',
    techList: 'space-y-1.5 text-xs text-slate-600',
    techItem: 'flex items-center gap-2',
    dot: 'w-1.5 h-1.5 rounded-full',
    links: 'flex flex-wrap gap-2',
    linkIcon: 'w-4 h-4',
  },
  variants: {
    tone: {
      blue: {
        featureIcon: 'bg-blue-100',
        featureIconInner: 'text-blue-600',
        dot: 'bg-blue-500',
      },
      green: {
        featureIcon: 'bg-green-100',
        featureIconInner: 'text-green-600',
        dot: 'bg-green-500',
      },
      purple: {
        featureIcon: 'bg-purple-100',
        featureIconInner: 'text-purple-600',
        dot: 'bg-purple-500',
      },
      orange: {
        featureIcon: 'bg-orange-100',
        featureIconInner: 'text-orange-600',
        dot: 'bg-orange-500',
      },
    },
  },
});

const ui = aboutTv();

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
