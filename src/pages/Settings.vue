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
              <p class="text-slate-600">
                Configure SwitchShuttle application settings
              </p>
            </div>
          </div>
        </Card>

        <!-- Loading State -->
        <div v-if="loading" class="flex justify-center py-8">
          <div
            class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"
          ></div>
        </div>

        <!-- Settings Form -->
        <div v-else-if="settingsSchema && settings" class="space-y-6">
          <form @submit.prevent>
            <div class="space-y-6">
              <!-- Sections -->
              <div
                v-for="section in settingsSchema.sections"
                :key="section.id"
                class="space-y-4"
              >
                <Card>
                  <div class="space-y-4">
                    <div>
                      <h2 class="text-lg font-semibold text-slate-900">
                        {{ section.title }}
                      </h2>
                      <p class="text-sm text-slate-600">
                        {{ section.description }}
                      </p>
                    </div>

                    <div class="space-y-4">
                      <div
                        v-for="field in section.fields"
                        :key="field.id"
                        class="space-y-2"
                      >
                        <!-- Boolean Field -->
                        <Toggle
                          v-if="field.type === 'boolean'"
                          :id="field.id"
                          :model-value="getFieldValue(field.id) as boolean"
                          :label="field.label"
                          :description="field.description"
                          @update:model-value="
                            updateFieldValue(field.id, $event)
                          "
                        />

                        <!-- Custom Select Field (for Security Level) -->
                        <div
                          v-else-if="field.type === 'custom-select'"
                          class="space-y-2"
                        >
                          <label
                            :for="field.id"
                            class="block text-sm font-medium text-slate-700"
                          >
                            {{ field.label }}
                          </label>
                          <CustomSelect
                            :id="field.id"
                            :model-value="getFieldValue(field.id) as string"
                            :options="field.options || []"
                            :placeholder="field.description"
                            @update:model-value="
                              updateFieldValue(field.id, $event)
                            "
                          />
                          <p
                            v-if="field.description"
                            class="text-xs text-slate-500"
                          >
                            {{ field.description }}
                          </p>
                        </div>

                        <!-- Tag Editor Field -->
                        <div
                          v-else-if="field.type === 'tag-editor'"
                          class="space-y-2"
                        >
                          <TagEditor
                            :id="field.id"
                            :model-value="(getFieldValue(field.id) as unknown as string[]) || []"
                            :label="field.label"
                            :description="field.description"
                            :placeholder="field.placeholder"
                            @update:model-value="
                              updateFieldValue(field.id, $event)
                            "
                          />
                        </div>

                        <!-- Regular Select Field -->
                        <Input
                          v-else-if="field.type === 'select'"
                          :id="field.id"
                          :model-value="getFieldValue(field.id) as string"
                          :label="field.label"
                          :hint="field.description"
                          type="select"
                          :options="field.options"
                          @update:model-value="
                            updateFieldValue(field.id, $event)
                          "
                        />

                        <!-- Number Field -->
                        <Input
                          v-else-if="field.type === 'number'"
                          :id="field.id"
                          :model-value="
                            (
                              (getFieldValue(field.id) as number) || 0
                            ).toString()
                          "
                          :label="field.label"
                          :hint="field.description"
                          type="number"
                          :min="field.min"
                          :max="field.max"
                          @update:model-value="
                            updateFieldValue(field.id, parseInt($event) || 0)
                          "
                        />
                      </div>
                    </div>
                  </div>
                </Card>
              </div>
            </div>

            <!-- Status -->
            <div class="flex justify-end space-x-3 pt-6">
              <div
                v-if="saving"
                class="flex items-center space-x-2 text-sm text-blue-600"
              >
                <span
                  class="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-600"
                ></span>
                <span>Saving...</span>
              </div>
              <div
                v-else-if="lastSaved"
                class="flex items-center space-x-2 text-sm text-green-600"
              >
                <CheckIcon class="w-4 h-4" />
                <span>Settings saved</span>
              </div>
            </div>
          </form>

          <!-- Updates Section -->
          <Card>
            <div class="space-y-4">
              <div>
                <h2 class="text-lg font-semibold text-slate-900">Updates</h2>
                <p class="text-sm text-slate-600">
                  Check for application updates
                </p>
              </div>

              <div class="space-y-4">
                <!-- Update Status -->
                <div
                  v-if="updateLoading"
                  class="flex items-center space-x-3 text-sm text-blue-600"
                >
                  <span
                    class="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-600"
                  ></span>
                  <span>Checking for updates...</span>
                </div>

                <div v-else-if="updateMessage" class="text-sm text-slate-700">
                  {{ updateMessage }}
                </div>

                <!-- Update Button -->
                <div class="flex justify-start">
                  <CustomButton
                    variant="primary"
                    :disabled="updateLoading"
                    @click="checkForUpdates"
                  >
                    {{ updateLoading ? 'Checking...' : 'Check for Updates' }}
                  </CustomButton>

                  <CustomButton
                    v-if="updateUrl"
                    variant="secondary"
                    class="ml-3"
                    @click="downloadUpdate"
                  >
                    Download Update
                  </CustomButton>
                </div>
              </div>
            </div>
          </Card>
        </div>

        <!-- Error State -->
        <Card v-else-if="error" class="border-red-200 bg-red-50">
          <div class="text-center">
            <h3 class="text-lg font-semibold text-red-800">
              Error Loading Settings
            </h3>
            <p class="text-red-600 mt-2">{{ error }}</p>
            <CustomButton variant="secondary" class="mt-4" @click="loadSettings">
              Try Again
            </CustomButton>
          </div>
        </Card>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, inject, computed } from 'vue';
import { open } from '@tauri-apps/plugin-shell';
import Card from '../components/ui/Card.vue';
import CustomButton from '../components/ui/CustomButton.vue';
import Input from '../components/ui/Input.vue';
import Toggle from '../components/ui/Toggle.vue';
import CustomSelect from '../components/forms/CustomSelect.vue';
import SettingsIcon from '../components/icons/SettingsIcon.vue';
import CheckIcon from '../components/icons/CheckIcon.vue';
import ShieldIcon from '../components/icons/ShieldIcon.vue';
import type { TauriInjectionKey } from '../lib/tauri-commands-plugin';
import type { AppSettings, SettingsSchema } from '../lib/tauri-commands';
import TagEditor from '../components/forms/TagEditor.vue';

// Get access to commands through plugin
const tauri = inject('tauri') as TauriInjectionKey['tauri'];

const loading = ref(true);
const saving = ref(false);
const error = ref('');
const settingsSchema = ref<SettingsSchema | null>(null);
const settings = ref<AppSettings | null>(null);
const originalSettings = ref<AppSettings | null>(null);
const saveTimeout = ref<any | null>(null);
const lastSaved = ref(false);

// Update variables
const updateLoading = ref(false);
const updateMessage = ref('');
const updateUrl = ref('');

const getFieldValue = (fieldId: string): string | number | boolean | string[] | null => {
  if (!settings.value) return null;

  const [section, field] = fieldId.split('.');
  if (section && field && settings.value[section]) {
    const sectionData = settings.value[section];
    return sectionData[field];
  }
  return null;
};

const updateFieldValue = (
  fieldId: string,
  value: string | number | boolean | string[]
) => {
  if (!settings.value) return;

  const [section, field] = fieldId.split('.');
  if (section && field && settings.value[section]) {
    const sectionData = settings.value[section];
    sectionData[field] = value;

    // Auto-save with delay
    autoSaveSettings();
  }
};

const autoSaveSettings = () => {
  // Clear previous timeout
  if (saveTimeout.value) {
    clearTimeout(saveTimeout.value);
  }

  // Set new timeout for saving after 1 second
  saveTimeout.value = setTimeout(async () => {
    if (settings.value) {
      try {
        saving.value = true;
        lastSaved.value = false;
        await tauri.save_settings(settings.value);
        originalSettings.value = JSON.parse(JSON.stringify(settings.value));
        lastSaved.value = true;

        // Hide save message after 3 seconds
        setTimeout(() => {
          lastSaved.value = false;
        }, 3000);
      } catch (err) {
        error.value =
          err instanceof Error ? err.message : 'Failed to save settings';
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
      tauri.get_settings(),
    ]);

    settingsSchema.value = schemaResult as SettingsSchema;
    settings.value = settingsResult as AppSettings;
    originalSettings.value = JSON.parse(JSON.stringify(settingsResult));
  } catch (err) {
    error.value =
      err instanceof Error ? err.message : 'Failed to load settings';
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
