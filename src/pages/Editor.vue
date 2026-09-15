<template>
  <div :class="shell.root()">
    <main :class="shell.main()">
      <div :class="shell.content()">
        <header :class="header.root()">
          <div :class="header.titleBlock()">
            <h1 :class="header.title()">Configuration Editor</h1>
            <p :class="header.subtitle()">
              Manage and edit terminal configurations
            </p>
          </div>
          <div :class="header.actions()">
            <CustomButton
              variant="ghost"
              size="sm"
              title="Open Config Folder"
              @click="openConfigFolder"
            >
              <FolderIcon :class="ui.iconMd()" />
            </CustomButton>

            <CustomButton
              size="sm"
              title="Create new configuration"
              @click="createNewConfig"
            >
              <AddIcon :class="ui.iconMd()" />
            </CustomButton>
          </div>
        </header>

        <div :class="ui.listCard()">
          <div :class="ui.toolbar()">
            <h2 :class="ui.listTitle()">Configurations</h2>
            <div :class="ui.toolbarActions()">
              <div :class="ui.searchWrap()">
                <input
                  v-model="searchQuery"
                  type="text"
                  placeholder="Search configurations..."
                  :class="ui.searchInput()"
                />
                <SearchIcon :class="ui.searchIcon()" />
              </div>
              <CustomButton
                variant="ghost"
                size="sm"
                title="Refresh list"
                @click="loadConfigurations"
              >
                <SpinnerIcon :class="ui.iconSm()" />
              </CustomButton>
            </div>
          </div>

          <div v-if="loading" :class="empty.root()">
            <div :class="ui.loadingSpinner()" />
            <p :class="empty.title()">Loading configurations...</p>
          </div>

          <div v-else-if="filteredConfigurations.length === 0" :class="empty.root()">
            <div :class="empty.iconWrap()">
              <DocumentIcon :class="ui.emptyIcon()" />
            </div>
            <p :class="empty.title()">
              {{
                searchQuery
                  ? 'No configurations found'
                  : 'No configurations found'
              }}
            </p>
            <div :class="empty.actions()">
              <CustomButton v-if="!searchQuery" size="sm" @click="createNewConfig">
                Create first configuration
              </CustomButton>
            </div>
          </div>

          <div v-else :class="ui.list()">
            <div
              v-for="(config, index) in filteredConfigurations"
              :key="`${config.title}-${index}`"
              :class="rowUi(config.enabled ?? true).row()"
            >
              <div :class="ui.rowLeft()">
                <div :class="ui.rowIcon()">
                  <TerminalIcon :class="ui.rowIconInner()" />
                </div>
                <div :class="ui.rowMeta()">
                  <h3 :class="ui.rowTitle()">
                    {{ config.title || `Configuration ${index + 1}` }}
                  </h3>
                  <div :class="ui.rowStats()">
                    <span :class="ui.stat()">
                      <TerminalIcon :class="ui.statIcon()" />
                      <span>{{ config.terminal }}</span>
                    </span>
                    <span :class="ui.stat()">
                      <LightningIcon :class="ui.statIcon()" />
                      <span>{{ countAllCommands(config.commands) }} commands</span>
                    </span>
                  </div>
                </div>
              </div>

              <div :class="ui.rowActions()">
                <CustomButton
                  variant="ghost"
                  size="sm"
                  title="Open in editor"
                  @click="openConfig(config)"
                >
                  <ExternalLinkIcon />
                </CustomButton>
                <CustomButton
                  variant="ghost"
                  size="sm"
                  title="Edit"
                  @click="editConfig(config)"
                >
                  <EditIcon />
                </CustomButton>
                <CustomButton
                  variant="ghost"
                  size="sm"
                  title="Duplicate"
                  @click="duplicateConfig(config)"
                >
                  <DuplicateIcon />
                </CustomButton>
                <CustomButton
                  variant="danger"
                  size="sm"
                  title="Delete"
                  @click="deleteConfig(config)"
                >
                  <TrashIcon />
                </CustomButton>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <Modal :is-open="showEditor" @close="closeEditor">
      <template #header>
        <h2 :class="ui.modalTitle()">
          {{ editingConfig ? 'Edit Configuration' : 'Create Configuration' }}
        </h2>
      </template>

      <div v-if="currentConfig" :class="ui.modalBody()">
        <ConfigEditor
          :config="currentConfig"
          :commands="currentConfig.commands"
          :terminal-options="terminalOptions"
          :loading-terminals="loadingTerminals"
        />
      </div>

      <template #footer>
        <div :class="ui.modalFooter()">
          <CustomButton
            variant="ghost"
            size="sm"
            :disabled="saving"
            @click="closeEditor"
          >
            Cancel
          </CustomButton>
          <CustomButton
            variant="primary"
            size="sm"
            :disabled="saving"
            @click="validateAndSave"
          >
            <SpinnerIcon v-if="saving" :class="ui.btnIconSpin()" />
            <CheckIcon v-else :class="ui.iconSm()" />
            {{ saving ? 'Saving...' : 'Save' }}
          </CustomButton>
        </div>
      </template>
    </Modal>

    <ConfirmModal
      :is-open="showDeleteConfirm"
      title="Confirm Deletion"
      message="Delete configuration?"
      description="This action cannot be undone"
      :details="deleteConfigDetails"
      :loading="deleting"
      confirm-text="Delete"
      loading-text="Deleting..."
      @close="closeDeleteConfirm"
      @confirm="confirmDelete"
    />
  </div>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted, inject } from 'vue';
import { tv } from '@/lib/tv';
import {
  pageShellTv,
  pageHeaderTv,
  emptyStateTv,
  cardTv,
} from '@/components/ui/themes';
import CustomButton from '../components/ui/CustomButton.vue';
import Modal from '../components/ui/Modal.vue';
import ConfigEditor from '../components/config/ConfigEditor.vue';
import ExternalLinkIcon from '../components/icons/ExternalLinkIcon.vue';
import EditIcon from '../components/icons/EditIcon.vue';
import DuplicateIcon from '../components/icons/DuplicateIcon.vue';
import TrashIcon from '../components/icons/TrashIcon.vue';
import SpinnerIcon from '../components/icons/SpinnerIcon.vue';
import CheckIcon from '../components/icons/CheckIcon.vue';
import AddIcon from '../components/icons/AddIcon.vue';
import SearchIcon from '../components/icons/SearchIcon.vue';
import FolderIcon from '../components/icons/FolderIcon.vue';
import DocumentIcon from '../components/icons/DocumentIcon.vue';
import TerminalIcon from '../components/icons/TerminalIcon.vue';
import LightningIcon from '../components/icons/LightningIcon.vue';
import ConfirmModal from '../components/modals/ConfirmModal.vue';
import type { TauriInjectionKey } from '../lib/tauri-commands-plugin';
import { SwitchShuttleCommands, TerminalConfig } from '../lib/tauri-commands';

import type { Config as TauriConfig } from '../lib/tauri-commands';
import type { Config, Command } from '../types';

defineOptions({ name: 'Editor' });

const tauri = inject('tauri') as TauriInjectionKey['tauri'];

const configurations = ref<Config[]>([]);
const searchQuery = ref('');
const loading = ref(false);
const showEditor = ref(false);
const terminalOptions = ref<Record<string, TerminalConfig>>({});
const loadingTerminals = ref(true);

const currentConfig = ref<Config | null>(null);
const editingConfig = ref<Config | null>(null);
const originalFileName = ref<string>('');
const saving = ref(false);
const showDeleteConfirm = ref(false);
const configToDelete = ref<Config | null>(null);
const deleting = ref(false);

const shell = pageShellTv({ width: 'lg' });
const header = pageHeaderTv();
const empty = emptyStateTv();

const editorTv = tv({
  slots: {
    listCard: cardTv({ hover: false, padding: 'md' }),
    toolbar:
      'flex items-center justify-between gap-2 sticky top-[3.25rem] z-[9] -mx-3 -mt-3 mb-3 px-3 py-2 bg-white/95 backdrop-blur border-b border-slate-200',
    listTitle: 'text-sm font-semibold text-slate-900',
    toolbarActions: 'flex items-center gap-1.5',
    searchWrap: 'relative',
    searchInput: [
      'w-56 h-8 px-2.5 pr-8 border border-slate-300 text-sm rounded-md bg-white',
      'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
    ],
    searchIcon:
      'w-3.5 h-3.5 text-slate-400 absolute right-2.5 top-1/2 -translate-y-1/2 pointer-events-none',
    iconSm: 'w-4 h-4',
    iconMd: 'w-4 h-4',
    loadingSpinner:
      'w-7 h-7 border-2 border-slate-200 border-t-blue-500 animate-spin mx-auto mb-3 rounded-full',
    emptyIcon: 'w-6 h-6 text-slate-400',
    list: 'space-y-1.5',
    row: [
      'flex items-center justify-between py-2.5 px-2.5 border transition-colors rounded-md',
    ],
    rowLeft: 'flex items-center gap-2.5 min-w-0 flex-1',
    rowIcon:
      'w-8 h-8 bg-blue-100 flex items-center justify-center rounded-md flex-shrink-0',
    rowIconInner: 'w-4 h-4 text-blue-600',
    rowMeta: 'min-w-0 flex-1',
    rowTitle: 'font-medium text-sm text-slate-900 truncate',
    rowStats: 'flex items-center gap-3 text-xs text-slate-500 mt-0.5 flex-wrap',
    stat: 'flex items-center gap-1',
    statIcon: 'w-3.5 h-3.5',
    rowActions: 'flex items-center gap-0.5 flex-shrink-0',
    modalTitle: 'text-base font-semibold text-slate-900',
    modalBody: 'space-y-4',
    modalFooter: 'flex items-center justify-end gap-1.5',
    btnIconSpin: 'w-4 h-4 animate-spin',
  },
  variants: {
    enabled: {
      true: {
        row: 'border-slate-200 hover:border-slate-300 bg-white',
      },
      false: {
        row: 'border-slate-300 bg-slate-50',
      },
    },
  },
  defaultVariants: {
    enabled: true,
  },
});

const ui = editorTv();
const rowUi = (enabled: boolean) => editorTv({ enabled });

const countAllCommands = (commands: Command[]): number => {
  let count = 0;
  for (const command of commands) {
    count++;
    if (command.submenu && command.submenu.length > 0) {
      count += countAllCommands(command.submenu);
    }
  }
  return count;
};

const filteredConfigurations = computed(() => {
  if (!searchQuery.value) return configurations.value;

  const query = searchQuery.value.toLowerCase();
  return configurations.value.filter(
    (config) =>
      config.title.toLowerCase().includes(query) ||
      config.terminal.toLowerCase().includes(query) ||
      config.theme.toLowerCase().includes(query),
  );
});

const deleteConfigDetails = computed(() => {
  if (!configToDelete.value) return undefined;

  return {
    title: configToDelete.value.title,
    terminal: configToDelete.value.terminal,
    theme: configToDelete.value.theme,
  };
});

const loadTerminals = async () => {
  try {
    terminalOptions.value = await SwitchShuttleCommands.get_terminals_list();
  } catch (error) {
    console.error('[Editor] Failed to load terminals:', error);
    terminalOptions.value = {};
  } finally {
    loadingTerminals.value = false;
  }
};

const loadConfigurations = async () => {
  loading.value = true;
  try {
    configurations.value = await tauri.get_configurations();
  } catch (error) {
    console.error('Failed to load configurations:', error);
    await tauri.show_error_notification(
      'Error Loading Configurations',
      'Failed to load configurations list',
    );
    configurations.value = [];
  } finally {
    loading.value = false;
  }
};

const createNewConfig = async () => {
  try {
    currentConfig.value = await tauri.create_new_configuration();
    editingConfig.value = null;
    showEditor.value = true;
    await tauri.show_success_notification(
      'Configuration Created',
      'New configuration created successfully',
    );
  } catch (error) {
    console.error('Failed to create new configuration:', error);
    await tauri.show_error_notification(
      'Error Creating Configuration',
      `Failed to create new configuration: ${error}`,
    );
  }
};

const editConfig = (config: Config) => {
  currentConfig.value = { ...config };
  editingConfig.value = config;
  const originalTitle = config.title;
  originalFileName.value = config.title;
  console.log('originalTitle', originalTitle, config);
  showEditor.value = true;
};

const duplicateConfig = async (config: Config) => {
  try {
    currentConfig.value = await tauri.duplicate_configuration(
      config as TauriConfig,
    );
    editingConfig.value = null;
    showEditor.value = true;
    await tauri.show_success_notification(
      'Configuration Duplicated',
      'Configuration duplicated successfully',
    );
  } catch (error) {
    console.error('Failed to duplicate configuration:', error);
    await tauri.show_error_notification(
      'Error Duplicating Configuration',
      `Failed to duplicate configuration: ${error}`,
    );
  }
};

const validateAndSave = async () => {
  if (!currentConfig.value) return;

  saving.value = true;

  setTimeout(async () => {
    try {
      const originalTitle = originalFileName.value || undefined;

      await tauri.save_or_update_configuration(
        currentConfig.value as TauriConfig,
        originalTitle,
      );

      closeEditor();
      await loadConfigurations();

      const message = editingConfig.value
        ? 'Configuration updated successfully'
        : 'Configuration saved successfully';
      await tauri.show_success_notification('Configuration Saved', message);
    } catch (error) {
      console.error('Error saving configuration:', error);
      await tauri.show_error_notification(
        'Error Saving Configuration',
        'Failed to save configuration',
      );
    } finally {
      saving.value = false;
    }
  }, 500);
};

const closeEditor = () => {
  showEditor.value = false;
  currentConfig.value = null;
  editingConfig.value = null;
  originalFileName.value = '';
};

const openConfig = async (config: Config) => {
  try {
    await tauri.open_configuration(config.title);
    await tauri.show_success_notification(
      'Configuration Opened',
      'Configuration opened in default editor',
    );
  } catch (error) {
    console.error('Failed to open configuration:', error);
    await tauri.show_error_notification(
      'Error Opening Configuration',
      `Failed to open configuration: ${error}`,
    );
  }
};

const deleteConfig = async (config: Config) => {
  configToDelete.value = config;
  showDeleteConfirm.value = true;
};

const closeDeleteConfirm = () => {
  showDeleteConfirm.value = false;
  configToDelete.value = null;
};

const confirmDelete = async () => {
  if (!configToDelete.value) return;

  deleting.value = true;
  try {
    await tauri.delete_configuration(configToDelete.value.title);
    await loadConfigurations();
    closeDeleteConfirm();
    await tauri.show_success_notification(
      'Configuration Deleted',
      'Configuration deleted successfully',
    );
  } catch (error) {
    console.error('Failed to delete configuration:', error);
    await tauri.show_error_notification(
      'Error Deleting Configuration',
      `Failed to delete configuration: ${error}`,
    );
  } finally {
    deleting.value = false;
  }
};

const openConfigFolder = async () => {
  try {
    await tauri.open_config_folder();
    await tauri.show_success_notification(
      'Config Folder Opened',
      'Configuration folder opened in file explorer',
    );
  } catch (error) {
    console.error('Failed to open config folder:', error);
    await tauri.show_error_notification(
      'Error Opening Config Folder',
      `Failed to open configuration folder: ${error}`,
    );
  }
};

onMounted(() => {
  loadConfigurations();
  loadTerminals();
});
</script>
