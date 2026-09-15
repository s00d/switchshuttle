<template>
  <div :class="shell.root()">
    <main :class="shell.main()">
      <div :class="shell.content()">
        <header :class="header.root()">
          <div :class="header.titleBlock()">
            <h1 :class="header.title()">Help</h1>
            <p :class="header.subtitle()">
              Short reference for tray, configs, and common fixes
            </p>
          </div>
        </header>

        <div :class="ui.layout()">
          <div :class="ui.searchWrap()">
            <input
              v-model="searchQuery"
              type="search"
              placeholder="Filter sections…"
              :class="ui.searchInput()"
            />
            <SearchIcon :class="ui.searchIcon()" />
          </div>

          <nav :class="ui.nav()" aria-label="Help sections">
            <button
              v-for="section in visibleSections"
              :key="section.id"
              type="button"
              :class="[
                ui.navBtn(),
                { [ui.navBtnActive()]: activeSection === section.id },
              ]"
              @click="scrollTo(section.id)"
            >
              {{ section.title }}
            </button>
          </nav>

          <p v-if="visibleSections.length === 0" :class="ui.empty()">
            Nothing matched “{{ searchQuery }}”.
          </p>

          <section
            v-if="isVisible('start')"
            id="help-start"
            :class="ui.section()"
          >
            <h2 :class="ui.sectionTitle()">Quick start</h2>
            <ol :class="ui.steps()">
              <li>Open the tray icon → <strong>Edit Config</strong> → Visual Editor.</li>
              <li>Create or enable a config, add a command, save.</li>
              <li>Tray → <strong>Refresh Configurations</strong> (or restart the app).</li>
            </ol>
            <div :class="ui.grid()">
              <div :class="ui.item()">
                <p :class="ui.itemTitle()">Config folder</p>
                <p :class="ui.itemBody()">
                  macOS / Linux:
                  <code :class="ui.inlineCode()">~/.config/switch-shuttle/</code>
                </p>
                <p :class="ui.itemBody()">
                  Windows:
                  <code :class="ui.inlineCode()">%APPDATA%\switch-shuttle\</code>
                </p>
              </div>
              <div :class="ui.item()">
                <p :class="ui.itemTitle()">Tray essentials</p>
                <p :class="ui.itemBody()">
                  Commands live in the tray menu. Background jobs show under
                  <strong>Running</strong> with Stop / Stop All.
                </p>
              </div>
            </div>
          </section>

          <section
            v-if="isVisible('config')"
            id="help-config"
            :class="ui.section()"
          >
            <h2 :class="ui.sectionTitle()">Config fields</h2>
            <p :class="ui.sectionLead()">
              Top-level settings for each JSON file in the config folder.
            </p>
            <div :class="ui.list()">
              <div
                v-for="row in configFields"
                :key="row.key"
                :class="ui.listRow()"
              >
                <code :class="ui.listKey()">{{ row.key }}</code>
                <p :class="ui.listVal()">{{ row.desc }}</p>
              </div>
            </div>
            <pre :class="ui.code()"><code>{{ configExample }}</code></pre>
          </section>

          <section
            v-if="isVisible('commands')"
            id="help-commands"
            :class="ui.section()"
          >
            <h2 :class="ui.sectionTitle()">Command recipes</h2>
            <p :class="ui.sectionLead()">
              Put these objects inside <code :class="ui.inlineCode()">commands</code>.
            </p>
            <div :class="ui.grid()">
              <div
                v-for="recipe in commandRecipes"
                :key="recipe.title"
                :class="ui.item()"
              >
                <p :class="ui.itemTitle()">{{ recipe.title }}</p>
                <p :class="ui.itemBody()">{{ recipe.desc }}</p>
                <pre :class="[ui.code(), 'mt-2']"><code>{{ recipe.sample }}</code></pre>
              </div>
            </div>
          </section>

          <section
            v-if="isVisible('hotkeys')"
            id="help-hotkeys"
            :class="ui.section()"
          >
            <h2 :class="ui.sectionTitle()">Hotkeys</h2>
            <div :class="ui.list()">
              <div :class="ui.listRow()">
                <span :class="ui.listKey()">Format</span>
                <p :class="ui.listVal()">
                  <code :class="ui.inlineCode()">Ctrl+Shift+D</code>,
                  <code :class="ui.inlineCode()">Cmd+Option+S</code>,
                  <code :class="ui.inlineCode()">Alt+F1</code>
                </p>
              </div>
              <div :class="ui.listRow()">
                <span :class="ui.listKey()">macOS</span>
                <p :class="ui.listVal()">
                  Grant Accessibility to SwitchShuttle if global hotkeys do not
                  fire (System Settings → Privacy &amp; Security → Accessibility).
                </p>
              </div>
              <div :class="ui.listRow()">
                <span :class="ui.listKey()">Conflicts</span>
                <p :class="ui.listVal()">
                  Pick an unused combo, save the config, then Refresh
                  Configurations.
                </p>
              </div>
            </div>
          </section>

          <section
            v-if="isVisible('cli')"
            id="help-cli"
            :class="ui.section()"
          >
            <h2 :class="ui.sectionTitle()">CLI</h2>
            <p :class="ui.sectionLead()">
              Run the app binary with these flags (exact path depends on install).
            </p>
            <div :class="ui.list()">
              <div
                v-for="row in cliRows"
                :key="row.key"
                :class="ui.listRow()"
              >
                <code :class="ui.listKey()">{{ row.key }}</code>
                <p :class="ui.listVal()">{{ row.desc }}</p>
              </div>
            </div>
          </section>

          <section
            v-if="isVisible('fix')"
            id="help-fix"
            :class="ui.section()"
          >
            <h2 :class="ui.sectionTitle()">Fix common issues</h2>
            <div :class="ui.grid()">
              <div
                v-for="issue in issues"
                :key="issue.title"
                :class="ui.item()"
              >
                <p :class="ui.itemTitle()">{{ issue.title }}</p>
                <p :class="ui.itemBody()">{{ issue.fix }}</p>
              </div>
            </div>
          </section>

          <p :class="ui.footer()">
            More docs and issues:
            <a
              href="https://github.com/s00d/switchshuttle"
              :class="ui.footerLink()"
              target="_blank"
              rel="noopener noreferrer"
              >github.com/s00d/switchshuttle</a
            >
          </p>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { pageShellTv, pageHeaderTv } from '@/components/ui/themes';
import { helpTv } from './helpTheme';
import SearchIcon from '../components/icons/SearchIcon.vue';

defineOptions({ name: 'Help' });

const shell = pageShellTv({ width: 'md' });
const header = pageHeaderTv();
const ui = helpTv();

const searchQuery = ref('');
const activeSection = ref('start');

type SectionId = 'start' | 'config' | 'commands' | 'hotkeys' | 'cli' | 'fix';

const sections: { id: SectionId; title: string; keywords: string }[] = [
  {
    id: 'start',
    title: 'Quick start',
    keywords: 'install tray editor refresh folder config path',
  },
  {
    id: 'config',
    title: 'Config fields',
    keywords: 'terminal launch_in title enabled theme json',
  },
  {
    id: 'commands',
    title: 'Commands',
    keywords:
      'submenu inputs switch monitor scheduler background hotkey commands',
  },
  {
    id: 'hotkeys',
    title: 'Hotkeys',
    keywords: 'shortcut accessibility conflict cmd ctrl',
  },
  {
    id: 'cli',
    title: 'CLI',
    keywords: 'command list search cli terminal',
  },
  {
    id: 'fix',
    title: 'Fixes',
    keywords: 'troubleshoot hotkey terminal json not loading running stop',
  },
];

const visibleSections = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return sections;
  return sections.filter(
    (s) =>
      s.title.toLowerCase().includes(q) ||
      s.keywords.includes(q) ||
      s.keywords.split(' ').some((w) => w.includes(q) || q.includes(w)),
  );
});

const isVisible = (id: SectionId) =>
  visibleSections.value.some((s) => s.id === id);

const scrollTo = (id: SectionId) => {
  activeSection.value = id;
  document.getElementById(`help-${id}`)?.scrollIntoView({
    behavior: 'smooth',
    block: 'start',
  });
};

const configFields = [
  {
    key: 'terminal',
    desc: 'App to open: iterm, terminal, alacritty, warp, hyper, …',
  },
  {
    key: 'launch_in',
    desc: 'current | new_tab | new_window',
  },
  { key: 'title', desc: 'Optional window/tab title.' },
  {
    key: 'enabled',
    desc: 'false hides the whole file from the tray without deleting it.',
  },
  { key: 'theme', desc: 'Optional terminal theme name (where supported).' },
];

const configExample = `{
  "terminal": "iterm",
  "launch_in": "new_tab",
  "title": "Dev",
  "enabled": true,
  "commands": [
    { "name": "Start", "command": "npm run dev", "hotkey": "Ctrl+Shift+D" }
  ]
}`;

const commandRecipes = [
  {
    title: 'Single / multi command',
    desc: 'Use command or commands[].',
    sample: `{
  "name": "Bootstrap",
  "commands": ["git pull", "npm i", "npm run build"]
}`,
  },
  {
    title: 'Submenu',
    desc: 'Nest related actions.',
    sample: `{
  "name": "Docker",
  "submenu": [
    { "name": "Up", "command": "docker compose up -d" }
  ]
}`,
  },
  {
    title: 'Inputs',
    desc: 'Prompts before run; use [name] in the command.',
    sample: `{
  "name": "New component",
  "inputs": { "name": "Button" },
  "command": "mkdir -p src/[name]"
}`,
  },
  {
    title: 'Switch',
    desc: 'Toggle with a status probe (checkbox in tray).',
    sample: `{
  "name": "Wi‑Fi",
  "command": "networksetup -setairportpower en0 on",
  "switch": "networksetup -getairportpower en0 | grep -q On"
}`,
  },
  {
    title: 'Monitor',
    desc: 'Live label from monitor; click runs command.',
    sample: `{
  "name": "Load",
  "monitor": "uptime",
  "command": "top"
}`,
  },
  {
    title: 'Background + schedule',
    desc: 'Tracked under Running; cron keeps firing.',
    sample: `{
  "name": "Backup",
  "commands": ["rsync -a ~/work/ ~/Backup/"],
  "background": true,
  "scheduler": "0 2 * * *"
}`,
  },
];

const cliRows = [
  { key: '--command', desc: 'Run by command id or name.' },
  { key: '--list', desc: 'Print enabled commands.' },
  { key: '--search', desc: 'Filter commands by name substring.' },
];

const issues = [
  {
    title: 'Hotkeys silent',
    fix: 'Check Accessibility (macOS), pick a free combo, Refresh Configurations.',
  },
  {
    title: 'Config missing in tray',
    fix: 'Validate JSON, set "enabled": true, confirm the file is in the config folder, then refresh.',
  },
  {
    title: 'Terminal does not open',
    fix: 'Install the configured terminal, or change "terminal" to one you have.',
  },
  {
    title: 'Cannot Stop a job',
    fix: 'Only background: true jobs appear under Running. Terminal.app/iTerm sessions are not stoppable from the tray.',
  },
];
</script>
