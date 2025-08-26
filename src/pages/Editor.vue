<template>
  <div class="min-h-screen bg-slate-50">
    <main class="container mx-auto px-4 py-6">
      <div class="max-w-6xl mx-auto space-y-6">
        <!-- Header -->
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold text-slate-900">Configuration Editor</h1>
            <p class="text-slate-600 mt-1">Manage and edit terminal configurations</p>
          </div>
          <div class="flex items-center space-x-2">
            <CustomButton variant="ghost" size="lg" title="Open Config Folder" @click="openConfigFolder">
              <FolderIcon class="w-5 h-5" />
            </CustomButton>

            <CustomButton size="lg" title="Create new configuration" @click="createNewConfig">
              <AddIcon class="w-5 h-5" />
            </CustomButton>
          </div>
        </div>

        <!-- Configurations List -->
        <Card>
          <div class="flex items-center justify-between mb-6">
            <h2 class="text-lg font-semibold text-slate-900">Configurations</h2>
            <div class="flex items-center space-x-2">
              <div class="relative">
                <input
                  v-model="searchQuery"
                  type="text"
                  placeholder="Search configurations..."
                  class="w-64 px-3 py-1.5 pr-8 border border-slate-300 text-sm rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                />
                <SearchIcon class="w-4 h-4 text-slate-400 absolute right-2.5 top-1/2 transform -translate-y-1/2 pointer-events-none" />
              </div>
              <CustomButton variant="ghost" size="sm" title="Refresh list" @click="loadConfigurations">
                <SpinnerIcon class="w-4 h-4" />
              </CustomButton>
            </div>
          </div>

          <div v-if="loading" class="text-center py-12">
            <div class="w-8 h-8 border-2 border-slate-200 border-t-blue-500 animate-spin mx-auto mb-4"></div>
            <p class="text-slate-500">Loading configurations...</p>
          </div>

          <div v-else-if="filteredConfigurations.length === 0" class="text-center py-12">
            <div class="w-16 h-16 bg-slate-100 flex items-center justify-center mx-auto mb-4 rounded-lg">
              <DocumentIcon class="w-8 h-8 text-slate-400" />
            </div>
            <p class="text-slate-500 mb-2">
              {{ searchQuery ? 'No configurations found' : 'No configurations found' }}
            </p>
            <CustomButton v-if="!searchQuery" @click="createNewConfig">
              Create first configuration
            </CustomButton>
          </div>

          <div v-else class="space-y-3">
            <div
              v-for="(config, index) in filteredConfigurations"
              :key="`${config.title}-${index}`"
              :class="[
                'flex items-center justify-between p-4 border transition-colors rounded-lg',
                config.enabled 
                  ? 'border-slate-200 hover:border-slate-300 bg-white' 
                  : 'border-slate-300 bg-slate-50'
              ]"
            >
              <div class="flex items-center space-x-4 min-w-0 flex-1">
                <div class="w-10 h-10 bg-blue-100 flex items-center justify-center rounded-lg flex-shrink-0">
                  <TerminalIcon class="w-5 h-5 text-blue-600" />
                </div>
                <div class="min-w-0 flex-1">
                  <h3 class="font-semibold text-slate-900 truncate">{{ config.title || `Configuration ${index + 1}` }}</h3>
                  <div class="flex items-center space-x-4 text-sm text-slate-500 mt-1 flex-wrap">
                    <span class="flex items-center space-x-1">
                      <TerminalIcon class="w-4 h-4" />
                      <span>{{ config.terminal }}</span>
                    </span>
                    <span class="flex items-center space-x-1">
                      <LightningIcon class="w-4 h-4" />
                      <span>{{ countAllCommands(config.commands) }} commands</span>
                    </span>
                  </div>
                </div>
              </div>
              
              <div class="flex items-center space-x-1 flex-shrink-0">
                <CustomButton variant="ghost" size="sm" title="Open in editor" @click="openConfig(config)">
                  <ExternalLinkIcon />
                </CustomButton>
                <CustomButton variant="ghost" size="sm" title="Edit" @click="editConfig(config)">
                  <EditIcon />
                </CustomButton>
                <CustomButton variant="ghost" size="sm" title="Duplicate" @click="duplicateConfig(config)">
                  <DuplicateIcon />
                </CustomButton>
                <CustomButton variant="danger" size="sm" title="Delete" @click="deleteConfig(config)">
                  <TrashIcon />
                </CustomButton>
              </div>
            </div>
          </div>
        </Card>
      </div>
    </main>

    <!-- Visual Editor Modal -->
    <Modal :is-open="showEditor" @close="closeEditor">
      <template #header>
        <h2 class="text-xl font-semibold text-slate-900">
          {{ editingConfig ? 'Edit Configuration' : 'Create Configuration' }}
        </h2>
      </template>
      
      <div v-if="currentConfig" class="space-y-6">
        <ConfigEditor 
          :config="currentConfig" 
          :commands="currentConfig.commands"
          :terminal-options="terminalOptions"
          :loading-terminals="loadingTerminals"
        />
      </div>
      
      <template #footer>
        <div class="flex items-center justify-end space-x-2">
          <CustomButton variant="ghost" size="sm" :disabled="saving" @click="closeEditor">
            Cancel
          </CustomButton>
          <CustomButton variant="primary" size="sm" :disabled="saving" @click="validateAndSave">
            <SpinnerIcon v-if="saving" class="w-4 h-4 animate-spin" />
            <CheckIcon v-else class="w-4 h-4" />
            {{ saving ? 'Saving...' : 'Save' }}
          </CustomButton>
        </div>
      </template>
    </Modal>



    <!-- Delete Confirmation Modal -->
    <ConfirmModal
      :is-open="showDeleteConfirm"
      @close="closeDeleteConfirm"
      @confirm="confirmDelete"
      title="Confirm Deletion"
      message="Delete configuration?"
      description="This action cannot be undone"
      :details="deleteConfigDetails"
      :loading="deleting"
      confirm-text="Delete"
      loading-text="Deleting..."
    />
  </div>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted, inject } from 'vue';
import Card from '../components/ui/Card.vue';
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

// Получаем доступ к командам через плагин
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

// Функция для подсчета всех команд, включая подкоманды
const countAllCommands = (commands: Command[]): number => {
  let count = 0;
  for (const command of commands) {
    count++; // Считаем текущую команду
    if (command.submenu && command.submenu.length > 0) {
      count += countAllCommands(command.submenu); // Рекурсивно считаем подкоманды
    }
  }
  return count;
};

const filteredConfigurations = computed(() => {
  if (!searchQuery.value) return configurations.value;
  
  const query = searchQuery.value.toLowerCase();
  return configurations.value.filter(config => 
    config.title.toLowerCase().includes(query) ||
    config.terminal.toLowerCase().includes(query) ||
    config.theme.toLowerCase().includes(query)
  );
});

const deleteConfigDetails = computed(() => {
  if (!configToDelete.value) return undefined;
  
  return {
    title: configToDelete.value.title,
    terminal: configToDelete.value.terminal,
    theme: configToDelete.value.theme
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
    await tauri.show_error_notification('Error Loading Configurations', 'Failed to load configurations list');
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
    await tauri.show_success_notification('Configuration Created', 'New configuration created successfully');
  } catch (error) {
    console.error('Failed to create new configuration:', error);
    await tauri.show_error_notification('Error Creating Configuration', `Failed to create new configuration: ${error}`);
  }
};

const editConfig = (config: Config) => {
  currentConfig.value = { ...config };
  editingConfig.value = config;
  // Извлекаем оригинальное название, убрав все (2), (3) и т.д.
  const originalTitle = config.title;
  originalFileName.value = config.title;
  console.log('originalTitle', originalTitle, config);
  showEditor.value = true;
};

const duplicateConfig = async (config: Config) => {
  try {
    currentConfig.value = await tauri.duplicate_configuration(config as TauriConfig);
    editingConfig.value = null;
    showEditor.value = true;
    await tauri.show_success_notification('Configuration Duplicated', 'Configuration duplicated successfully');
  } catch (error) {
    console.error('Failed to duplicate configuration:', error);
    await tauri.show_error_notification('Error Duplicating Configuration', `Failed to duplicate configuration: ${error}`);
  }
};

const validateAndSave = async () => {
  if (!currentConfig.value) return;
  
  saving.value = true;
  
  // Ждем обновления DOM чтобы показался лоадер
  setTimeout(async() => {
    try {
      // Определяем оригинальное название файла
      // Если originalFileName не пустой - это редактирование существующей конфигурации
      // Если пустой - это новая или дублированная конфигурация
      const originalTitle = originalFileName.value || undefined;
      
      await tauri.save_or_update_configuration(currentConfig.value as TauriConfig, originalTitle);
      
      closeEditor();
      // Refresh configurations list after saving
      await loadConfigurations();
      
      const message = editingConfig.value ? 'Configuration updated successfully' : 'Configuration saved successfully';
      await tauri.show_success_notification('Configuration Saved', message);
    } catch (error) {
      console.error('Error saving configuration:', error);
      await tauri.show_error_notification('Error Saving Configuration', 'Failed to save configuration');
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
    await tauri.show_success_notification('Configuration Opened', 'Configuration opened in default editor');
  } catch (error) {
    console.error('Failed to open configuration:', error);
    await tauri.show_error_notification('Error Opening Configuration', `Failed to open configuration: ${error}`);
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
    // Refresh configurations list after deletion
    await loadConfigurations();
    closeDeleteConfirm();
    await tauri.show_success_notification('Configuration Deleted', 'Configuration deleted successfully');
  } catch (error) {
    console.error('Failed to delete configuration:', error);
    await tauri.show_error_notification('Error Deleting Configuration', `Failed to delete configuration: ${error}`);
  } finally {
    deleting.value = false;
  }
};

const openConfigFolder = async () => {
  try {
    await tauri.open_config_folder();
    await tauri.show_success_notification('Config Folder Opened', 'Configuration folder opened in file explorer');
  } catch (error) {
    console.error('Failed to open config folder:', error);
    await tauri.show_error_notification('Error Opening Config Folder', `Failed to open configuration folder: ${error}`);
  }
};

onMounted(() => {
  loadConfigurations();
  loadTerminals();
});
</script>

