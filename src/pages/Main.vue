<template>
  <div :class="shell.root()">
    <main :class="shell.main()">
      <div :class="shell.content()">
        <div :class="hero.card()">
          <div :class="hero.inner()">
            <div :class="hero.logoWrap()">
              <img src="/logo.svg" alt="SwitchShuttle" :class="hero.logo()" />
            </div>
            <div>
              <h1 :class="hero.title()">Welcome to SwitchShuttle</h1>
              <p :class="hero.subtitle()">
                Manage terminal configurations and switch between them with ease
              </p>
            </div>
            <nav :class="hero.nav()">
              <router-link
                v-for="link in links"
                :key="link.to"
                :to="link.to"
                :class="hero.link()"
              >
                <component :is="link.icon" :class="hero.linkIcon()" />
                {{ link.label }}
              </router-link>
            </nav>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { tv } from '@/lib/tv';
import { pageShellTv, cardTv } from '@/components/ui/themes';
import EditIcon from '../components/icons/EditIcon.vue';
import SettingsIcon from '../components/icons/SettingsIcon.vue';
import DocumentIcon from '../components/icons/DocumentIcon.vue';
import TerminalIcon from '../components/icons/TerminalIcon.vue';

defineOptions({ name: 'Main' });

const links = [
  { to: '/editor', label: 'Editor', icon: EditIcon },
  { to: '/settings', label: 'Settings', icon: SettingsIcon },
  { to: '/help', label: 'Help', icon: DocumentIcon },
  { to: '/about', label: 'About', icon: TerminalIcon },
] as const;

const shell = pageShellTv({ width: 'md' });

const mainHeroTv = tv({
  slots: {
    card: cardTv({ hover: false, padding: 'md' }),
    inner: 'text-center space-y-3',
    logoWrap: 'flex justify-center',
    logo: 'w-12 h-12',
    title: 'text-lg font-semibold text-slate-900',
    subtitle: 'text-xs text-slate-500 max-w-md mx-auto mt-1',
    nav: 'flex flex-wrap items-center justify-center gap-1.5 pt-1',
    link: [
      'inline-flex items-center gap-1.5 px-2.5 py-1.5 text-xs font-medium rounded-md',
      'border border-slate-200 bg-white text-slate-700',
      'hover:border-blue-300 hover:bg-blue-50 hover:text-blue-700 transition-colors',
    ],
    linkIcon: 'w-3.5 h-3.5',
  },
});

const hero = mainHeroTv();
</script>
