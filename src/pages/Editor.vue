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
            <Button @click="openConfigFolder" variant="ghost" size="lg" title="Open Config Folder">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-5l-2-2H5a2 2 0 00-2 2z" />
              </svg>
            </Button>

            <Button @click="createNewConfig" size="lg" title="Create new configuration">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
              </svg>
            </Button>
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
                <svg class="w-4 h-4 text-slate-400 absolute right-2.5 top-1/2 transform -translate-y-1/2 pointer-events-none" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
              </div>
              <Button @click="loadConfigurations" variant="ghost" size="sm" title="Refresh list">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
              </Button>
            </div>
          </div>

          <div v-if="loading" class="text-center py-12">
            <div class="w-8 h-8 border-2 border-slate-200 border-t-blue-500 animate-spin mx-auto mb-4"></div>
            <p class="text-slate-500">Loading configurations...</p>
          </div>

          <div v-else-if="filteredConfigurations.length === 0" class="text-center py-12">
            <div class="w-16 h-16 bg-slate-100 flex items-center justify-center mx-auto mb-4 rounded-lg">
              <svg class="w-8 h-8 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
            </div>
            <p class="text-slate-500 mb-2">
              {{ searchQuery ? 'No configurations found' : 'No configurations found' }}
            </p>
            <Button @click="createNewConfig" v-if="!searchQuery">
              Create first configuration
            </Button>
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
                  <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                  </svg>
                </div>
                <div class="min-w-0 flex-1">
                  <h3 class="font-semibold text-slate-900 truncate">{{ config.title || `Configuration ${index + 1}` }}</h3>
                  <div class="flex items-center space-x-4 text-sm text-slate-500 mt-1 flex-wrap">
                    <span class="flex items-center space-x-1">
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                      </svg>
                      <span>{{ config.terminal }}</span>
                    </span>
                    <span class="flex items-center space-x-1">
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                      </svg>
                      <span>{{ countAllCommands(config.commands) }} commands</span>
                    </span>
                    <span v-if="config.menu_hotkey" class="flex items-center space-x-1">
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                      </svg>
                      <span>{{ config.menu_hotkey }}</span>
                    </span>

                  </div>
                </div>
              </div>
              
              <div class="flex items-center space-x-1 flex-shrink-0">
                <Button @click="openConfig(config)" variant="ghost" size="sm" title="Open in editor">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                  </svg>
                </Button>
                <Button @click="editConfig(config)" variant="ghost" size="sm" title="Edit">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                  </svg>
                </Button>
                <Button @click="duplicateConfig(config)" variant="ghost" size="sm" title="Duplicate">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                  </svg>
                </Button>
                <Button @click="deleteConfig(config)" variant="danger" size="sm" title="Delete">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </Button>
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
        />
      </div>
      
      <template #footer>
        <div class="flex items-center justify-end space-x-2">
          <Button @click="closeEditor" variant="ghost" size="sm" :disabled="saving">
            Cancel
          </Button>
          <Button @click="validateAndSave" variant="primary" size="sm" :disabled="saving">
            <svg v-if="saving" class="w-4 h-4 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
            </svg>
            {{ saving ? 'Saving...' : 'Save' }}
          </Button>
        </div>
      </template>
    </Modal>



    <!-- Delete Confirmation Modal -->
    <Modal :is-open="showDeleteConfirm" @close="closeDeleteConfirm">
      <template #header>
        <h2 class="text-xl font-semibold text-slate-900">Confirm Deletion</h2>
      </template>
      
      <div class="space-y-4">
        <div class="flex items-center space-x-3">
          <div class="w-10 h-10 bg-red-100 flex items-center justify-center rounded-full">
            <svg class="w-5 h-5 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
            </svg>
          </div>
          <div>
            <p class="text-slate-900 font-medium">Delete configuration?</p>
            <p class="text-slate-600 text-sm">This action cannot be undone</p>
          </div>
        </div>
        
        <div class="bg-slate-50 p-3 rounded-lg">
          <p class="text-slate-700">
            <span class="font-medium">Title:</span> {{ configToDelete?.title }}
          </p>
          <p class="text-slate-700">
            <span class="font-medium">Terminal:</span> {{ configToDelete?.terminal }}
          </p>
          <p class="text-slate-700">
            <span class="font-medium">Theme:</span> {{ configToDelete?.theme }}
          </p>
        </div>
      </div>
      
      <template #footer>
        <div class="flex items-center justify-end space-x-2">
          <Button @click="closeDeleteConfirm" variant="ghost" size="sm" :disabled="deleting">
            Cancel
          </Button>
          <Button @click="confirmDelete" variant="danger" size="sm" :disabled="deleting">
            <svg v-if="deleting" class="w-4 h-4 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
            {{ deleting ? 'Deleting...' : 'Delete' }}
          </Button>
        </div>
      </template>
    </Modal>
  </div>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import Card from '../components/Card.vue';
import Button from '../components/Button.vue';
import Modal from '../components/Modal.vue';
import ConfigEditor from '../components/ConfigEditor.vue';

import type { Config, Command } from '../types';

const configurations = ref<Config[]>([]);
const searchQuery = ref('');
const loading = ref(false);
const showEditor = ref(false);

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

const loadConfigurations = async () => {
  loading.value = true;
  try {
    configurations.value = await invoke('get_configurations');
  } catch (error) {
    console.error('Failed to load configurations:', error);
    configurations.value = [];
  } finally {
    loading.value = false;
  }
};

const createNewConfig = async () => {
  try {
    currentConfig.value = await invoke('create_new_configuration');
    editingConfig.value = null;
    showEditor.value = true;
  } catch (error) {
    console.error('Failed to create new configuration:', error);
    alert(`Error creating: ${error}`);
  }
};

const editConfig = (config: Config) => {
  currentConfig.value = { ...config };
  editingConfig.value = config;
  originalFileName.value = config.title;
  showEditor.value = true;
};

const duplicateConfig = async (config: Config) => {
  try {
    currentConfig.value = await invoke('duplicate_configuration', { config });
    editingConfig.value = null;
    showEditor.value = true;
  } catch (error) {
    console.error('Failed to duplicate configuration:', error);
    alert(`Error duplicating: ${error}`);
  }
};

const validateAndSave = async () => {
  if (!currentConfig.value) return;
  
  saving.value = true;
  
  // Ждем обновления DOM чтобы показался лоадер
  setTimeout(async() => {
    try {
      // Используем универсальную команду для сохранения/обновления
      const originalTitle = editingConfig.value ? originalFileName.value : null;
      
      await invoke('save_or_update_configuration', { 
        config: currentConfig.value,
        originalTitle: originalTitle
      });
      
      closeEditor();
      // Refresh configurations list after saving
      await loadConfigurations();
    } catch (error) {
      console.error('Error saving configuration:', error);
      alert('Failed to save configuration');
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
    await invoke('open_configuration', { id: config.title });
  } catch (error) {
    console.error('Failed to open configuration:', error);
    alert(`Error opening configuration: ${error}`);
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
    await invoke('delete_configuration', { id: configToDelete.value.title });
    // Refresh configurations list after deletion
    await loadConfigurations();
    closeDeleteConfirm();
  } catch (error) {
    console.error('Failed to delete configuration:', error);
    alert(`Error deleting: ${error}`);
  } finally {
    deleting.value = false;
  }
};


const openConfigFolder = async () => {
  try {
    await invoke('open_config_folder');
  } catch (error) {
    console.error('Failed to open config folder:', error);
  }
};



onMounted(() => {
  loadConfigurations();
});
</script>

