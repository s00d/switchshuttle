<template>
  <div :class="ui.root()">
    <div :class="ui.header()">
      <h3 :class="ui.title()">Configuration Selection</h3>
      <div :class="ui.actions()">
        <CustomButton variant="ghost" size="sm" @click="refreshConfigs">
          <SpinnerIcon :class="ui.iconSm()" />
        </CustomButton>
        <CustomButton
          variant="secondary"
          size="sm"
          @click="$emit('showAddConfigModal')"
        >
          <AddIcon :class="ui.iconSm()" />
          Add
        </CustomButton>
        <CustomButton
          variant="danger"
          size="sm"
          @click="$emit('showDeleteConfigModal')"
        >
          <TrashIcon :class="ui.iconSm()" />
          Delete
        </CustomButton>
      </div>
    </div>

    <div v-if="configFiles.length === 0" :class="ui.empty()">
      <div :class="ui.emptyIcon()">
        <DocumentIcon :class="ui.emptyIconInner()" />
      </div>
      <p :class="ui.emptyTitle()">No configurations found</p>
      <p :class="ui.emptyHint()">
        Create your first configuration to get started
      </p>
    </div>

    <div v-else :class="ui.list()">
      <div
        v-for="file in configFiles"
        :key="file.path"
        :class="rowUi(currentConfig === file.path).row()"
      >
        <div :class="ui.rowLeft()">
          <div :class="ui.rowIcon()">
            <DocumentIcon :class="ui.rowIconInner()" />
          </div>
          <div>
            <h4 :class="ui.rowName()">{{ file.name }}</h4>
            <p :class="ui.rowPath()">{{ file.path }}</p>
          </div>
        </div>

        <div :class="ui.rowActions()">
          <CustomButton
            variant="ghost"
            size="sm"
            @click="selectConfig(file.path)"
          >
            {{ currentConfig === file.path ? 'Selected' : 'Select' }}
          </CustomButton>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { tv } from '@/lib/tv';
import { ConfigFile, Config } from '../../types';
import CustomButton from '../ui/CustomButton.vue';
import SpinnerIcon from '../icons/SpinnerIcon.vue';
import AddIcon from '../icons/AddIcon.vue';
import TrashIcon from '../icons/TrashIcon.vue';
import DocumentIcon from '../icons/DocumentIcon.vue';

defineOptions({ name: 'ConfigSelector' });

const configFiles = ref<ConfigFile[]>([]);
const currentConfig = ref<string>('');

defineProps<{
  modelValue: Config;
}>();

const emit = defineEmits<{
  (e: 'showAddConfigModal'): void;
  (e: 'showDeleteConfigModal'): void;
  (e: 'update:modelValue', value: Config): void;
}>();

const configSelectorTv = tv({
  slots: {
    root: 'space-y-3',
    header: 'flex items-center justify-between gap-2',
    title: 'text-base font-semibold text-slate-900',
    actions: 'flex items-center gap-1.5',
    iconSm: 'w-4 h-4',
    empty: 'text-center py-6',
    emptyIcon:
      'w-12 h-12 bg-slate-100 flex items-center justify-center mx-auto mb-3 rounded-md',
    emptyIconInner: 'w-6 h-6 text-slate-400',
    emptyTitle: 'text-sm text-slate-500 mb-1',
    emptyHint: 'text-xs text-slate-400',
    list: 'space-y-1.5',
    row: [
      'flex items-center justify-between p-2.5 border border-slate-200 rounded-md',
      'hover:border-slate-300 transition-colors',
    ],
    rowLeft: 'flex items-center gap-2.5 min-w-0',
    rowIcon: 'w-7 h-7 bg-blue-100 flex items-center justify-center rounded-md',
    rowIconInner: 'w-4 h-4 text-blue-600',
    rowName: 'font-medium text-sm text-slate-900 truncate',
    rowPath: 'text-xs text-slate-500 truncate',
    rowActions: 'flex items-center gap-1.5 flex-shrink-0',
  },
  variants: {
    selected: {
      true: { row: 'bg-blue-50 border-blue-300' },
      false: { row: 'bg-white' },
    },
  },
  defaultVariants: {
    selected: false,
  },
});

const ui = configSelectorTv();

const rowUi = (selected: boolean) => configSelectorTv({ selected });

async function loadConfigs() {
  try {
    const configs = (await invoke('get_config_files')) as ConfigFile[];
    configFiles.value = configs;

    if (configFiles.value.length > 0) {
      currentConfig.value = configFiles.value[0].path;
      loadConfig();
    }
  } catch (error) {
    console.error('Failed to load configs:', error);
    configFiles.value = [
      {
        path: '/test/config1.json',
        name: 'Test Configuration',
      },
      {
        path: '/test/config2.json',
        name: 'Second Configuration',
      },
    ];

    if (configFiles.value.length > 0) {
      currentConfig.value = configFiles.value[0].path;
      loadConfig();
    }
  }
}

async function loadConfig() {
  try {
    const config = (await invoke('load_config', {
      path: currentConfig.value,
    })) as Config;
    emit('update:modelValue', config);
  } catch (error) {
    console.error('Failed to load config:', error);
    const testConfig: Config = {
      terminal: 'iterm',
      launch_in: 'current',
      theme: 'Homebrew',
      title: 'Test Configuration',
      commands: [
        {
          name: 'Open Terminal',
          command: 'open -a iTerm',
          hotkey: undefined,
          submenu: null,
          commands: [],
        },
      ],
      menu_hotkey: 'Cmd+Shift+S',
    };
    emit('update:modelValue', testConfig);
  }
}

const selectConfig = (path: string) => {
  currentConfig.value = path;
  loadConfig();
};

const refreshConfigs = () => {
  loadConfigs();
};

watch(currentConfig, () => {
  if (currentConfig.value) {
    loadConfig();
  }
});

onMounted(loadConfigs);
</script>
