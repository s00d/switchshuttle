<template>
  <div class="min-h-screen bg-slate-50">
    <main class="container mx-auto px-4 py-6">
      <div class="max-w-4xl mx-auto space-y-6">
        <!-- Header -->
        <Card>
          <div class="flex items-center space-x-3">
            <SettingsIcon size="lg" class="text-blue-600" />
            <div>
              <h1 class="text-2xl font-bold text-slate-900">Settings</h1>
              <p class="text-slate-600">Configure SwitchShuttle application settings</p>
            </div>
          </div>
        </Card>

        <!-- Loading State -->
        <div v-if="loading" class="flex justify-center py-8">
          <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        </div>

        <!-- Settings Form -->
        <div v-else-if="settingsSchema && settings" class="space-y-6">
          <form @submit.prevent>
            <div class="space-y-6">
              <!-- Sections -->
              <div v-for="section in settingsSchema.sections" :key="section.id" class="space-y-4">
                <Card>
                  <div class="space-y-4">
                    <div>
                      <h2 class="text-lg font-semibold text-slate-900">{{ section.title }}</h2>
                      <p class="text-sm text-slate-600">{{ section.description }}</p>
                    </div>
                    
                    <div class="space-y-4">
                      <div v-for="field in section.fields" :key="field.id" class="space-y-2">
                        <!-- Boolean Field -->
                        <Toggle
                          v-if="field.type === 'boolean'"
                          :id="field.id"
                          :model-value="getFieldValue(field.id) as boolean"
                          :label="field.label"
                          :description="field.description"
                          @update:model-value="updateFieldValue(field.id, $event)"
                        />
                        
                        <!-- Select Field -->
                        <Input
                          v-else-if="field.type === 'select'"
                          :id="field.id"
                          :model-value="getFieldValue(field.id) as string"
                          :label="field.label"
                          :hint="field.description"
                          type="select"
                          :options="field.options"
                          @update:model-value="updateFieldValue(field.id, $event)"
                        />
                        
                        <!-- Number Field -->
                        <Input
                          v-else-if="field.type === 'number'"
                          :id="field.id"
                          :model-value="(getFieldValue(field.id) as number || 0).toString()"
                          :label="field.label"
                          :hint="field.description"
                          type="number"
                          :min="field.min"
                          :max="field.max"
                          @update:model-value="updateFieldValue(field.id, parseInt($event) || 0)"
                        />
                      </div>
                    </div>
                  </div>
                </Card>
              </div>
            </div>

            <!-- Status -->
            <div class="flex justify-end space-x-3 pt-6">
              <div v-if="saving" class="flex items-center space-x-2 text-sm text-blue-600">
                <span class="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-600"></span>
                <span>Saving...</span>
              </div>
              <div v-else-if="lastSaved" class="flex items-center space-x-2 text-sm text-green-600">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
                <span>Settings saved</span>
              </div>
            </div>
          </form>

          <!-- Updates Section -->
          <Card>
            <div class="space-y-4">
              <div>
                <h2 class="text-lg font-semibold text-slate-900">Updates</h2>
                <p class="text-sm text-slate-600">Check for application updates</p>
              </div>
              
              <div class="space-y-4">
                <!-- Update Status -->
                <div v-if="updateLoading" class="flex items-center space-x-3 text-sm text-blue-600">
                  <span class="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-600"></span>
                  <span>Checking for updates...</span>
                </div>
                
                <div v-else-if="updateMessage" class="text-sm text-slate-700">
                  {{ updateMessage }}
                </div>
                
                <!-- Update Button -->
                <div class="flex justify-start">
                  <Button
                    variant="primary"
                    :disabled="updateLoading"
                    @click="checkForUpdates"
                  >
                    {{ updateLoading ? 'Checking...' : 'Check for Updates' }}
                  </Button>
                  
                  <Button
                    v-if="updateUrl"
                    variant="secondary"
                    class="ml-3"
                    @click="downloadUpdate"
                  >
                    Download Update
                  </Button>
                </div>
              </div>
            </div>
          </Card>
        </div>

        <!-- Error State -->
        <Card v-else-if="error" class="border-red-200 bg-red-50">
          <div class="text-center">
            <h3 class="text-lg font-semibold text-red-800">Error Loading Settings</h3>
            <p class="text-red-600 mt-2">{{ error }}</p>
            <Button variant="secondary" @click="loadSettings" class="mt-4">
              Try Again
            </Button>
          </div>
        </Card>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, inject } from 'vue';
import { open } from '@tauri-apps/plugin-shell';
import Card from '../components/Card.vue';
import Button from '../components/Button.vue';
import Input from '../components/Input.vue';
import Toggle from '../components/Toggle.vue';
import SettingsIcon from '../components/icons/SettingsIcon.vue';
import type { TauriInjectionKey } from '../lib/tauri-commands-plugin';
import type { AppSettings, SettingsSchema } from '../lib/tauri-commands';

// Получаем доступ к командам через плагин
const tauri = inject('tauri') as TauriInjectionKey['tauri'];

const loading = ref(true);
const saving = ref(false);
const error = ref('');
const settingsSchema = ref<SettingsSchema | null>(null);
const settings = ref<AppSettings | null>(null);
const originalSettings = ref<AppSettings | null>(null);
const saveTimeout = ref<NodeJS.Timeout | null>(null);
const lastSaved = ref(false);

// Update variables
const updateLoading = ref(false);
const updateMessage = ref('');
const updateUrl = ref('');

const getFieldValue = (fieldId: string): string | number | boolean | null => {
  if (!settings.value) return null;
  
  const [section, field] = fieldId.split('.');
  if (section && field && settings.value[section]) {
    const sectionData = settings.value[section];
    return sectionData[field];
  }
  return null;
};

const updateFieldValue = (fieldId: string, value: string | number | boolean) => {
  if (!settings.value) return;
  
  const [section, field] = fieldId.split('.');
  if (section && field && settings.value[section]) {
    const sectionData = settings.value[section];
    sectionData[field] = value;
    
    // Автоматическое сохранение с задержкой
    autoSaveSettings();
  }
};

const autoSaveSettings = () => {
  // Очищаем предыдущий таймаут
  if (saveTimeout.value) {
    clearTimeout(saveTimeout.value);
  }
  
  // Устанавливаем новый таймаут для сохранения через 1 секунду
  saveTimeout.value = setTimeout(async () => {
    if (settings.value) {
      try {
        saving.value = true;
        lastSaved.value = false;
        await tauri.save_settings(settings.value);
        originalSettings.value = JSON.parse(JSON.stringify(settings.value));
        lastSaved.value = true;
        
        // Скрываем сообщение о сохранении через 3 секунды
        setTimeout(() => {
          lastSaved.value = false;
        }, 3000);
      } catch (err) {
        error.value = err instanceof Error ? err.message : 'Failed to save settings';
      } finally {
        saving.value = false;
      }
    }
  }, 1000);
};

const loadSettings = async () => {
  try {
    loading.value = true;
    error.value = '';
    
    const [schemaResult, settingsResult] = await Promise.all([
      tauri.get_settings_schema(),
      tauri.get_settings()
    ]);
    
    settingsSchema.value = schemaResult as SettingsSchema;
    settings.value = settingsResult as AppSettings;
    originalSettings.value = JSON.parse(JSON.stringify(settingsResult));
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to load settings';
  } finally {
    loading.value = false;
  }
};

// Update functions
const checkForUpdates = async () => {
  try {
    updateLoading.value = true;
    updateMessage.value = '';
    updateUrl.value = '';
    
    const result = await tauri.check_for_updates();
    updateMessage.value = result.message;
    updateUrl.value = result.url;
  } catch (error) {
    updateMessage.value = (error as Error).message;
  } finally {
    updateLoading.value = false;
  }
};

const downloadUpdate = () => {
  if (updateUrl.value) {
    open(updateUrl.value);
  }
};

onMounted(() => {
  loadSettings();
});
</script> 