<template>
  <div :class="shell.root()">
    <main :class="shell.main()">
      <div :class="shell.content()">
        <header :class="header.root()">
          <div :class="header.titleBlock()">
            <div :class="ui.headerRow()">
              <SettingsIcon size="md" :class="ui.headerIcon()" />
              <div>
                <h1 :class="header.title()">Settings</h1>
                <p :class="header.subtitle()">
                  Configure SwitchShuttle application settings
                </p>
              </div>
            </div>
          </div>
        </header>

        <div v-if="loading" :class="ui.loading()">
          <div :class="ui.spinner()" />
        </div>

        <div v-else-if="settingsSchema && settings" :class="ui.formWrap()">
          <form @submit.prevent>
            <div :class="ui.sections()">
              <div
                v-for="section in settingsSchema.sections"
                :key="section.id"
                :class="ui.sectionCard()"
              >
                <div :class="ui.sectionInner()">
                  <div>
                    <h2 :class="ui.sectionTitle()">{{ section.title }}</h2>
                    <p :class="ui.sectionDesc()">{{ section.description }}</p>
                  </div>

                  <div :class="ui.fields()">
                    <div
                      v-for="field in section.fields"
                      :key="field.id"
                      :class="ui.field()"
                    >
                      <Toggle
                        v-if="field.type === 'boolean'"
                        :id="field.id"
                        :model-value="getFieldValue(field.id) as boolean"
                        :label="field.label"
                        :description="field.description"
                        @update:model-value="updateFieldValue(field.id, $event)"
                      />

                      <div
                        v-else-if="field.type === 'custom-select'"
                        :class="ui.field()"
                      >
                        <label :for="field.id" :class="ui.fieldLabel()">
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
                        <p v-if="field.description" :class="ui.fieldHint()">
                          {{ field.description }}
                        </p>
                      </div>

                      <div v-else-if="field.type === 'tag-editor'" :class="ui.field()">
                        <TagEditor
                          :id="field.id"
                          :model-value="
                            (getFieldValue(field.id) as unknown as string[]) ||
                            []
                          "
                          :label="field.label"
                          :description="field.description"
                          :placeholder="field.placeholder"
                          @update:model-value="
                            updateFieldValue(field.id, $event)
                          "
                        />
                      </div>

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

                      <Input
                        v-else-if="field.type === 'number'"
                        :id="field.id"
                        :model-value="
                          ((getFieldValue(field.id) as number) || 0).toString()
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
              </div>
            </div>

            <div :class="ui.statusBar()">
              <div v-if="saving" :class="ui.statusSaving()">
                <span :class="ui.statusSpinner()" />
                <span>Saving...</span>
              </div>
              <div v-else-if="lastSaved" :class="ui.statusSaved()">
                <CheckIcon :class="ui.checkIcon()" />
                <span>Settings saved</span>
              </div>
            </div>
          </form>

          <div :class="ui.sectionCard()">
            <div :class="ui.sectionInner()">
              <div>
                <h2 :class="ui.sectionTitle()">Updates</h2>
                <p :class="ui.sectionDesc()">Check for application updates</p>
              </div>

              <div :class="ui.fields()">
                <div v-if="updateLoading" :class="ui.statusSaving()">
                  <span :class="ui.statusSpinner()" />
                  <span>Checking for updates...</span>
                </div>

                <div v-else-if="updateMessage" :class="ui.updateMsg()">
                  {{ updateMessage }}
                </div>

                <div :class="ui.updateActions()">
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
                    @click="downloadUpdate"
                  >
                    Download Update
                  </CustomButton>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div v-else-if="error" :class="ui.errorCard()">
          <div :class="ui.errorInner()">
            <h3 :class="ui.errorTitle()">Error Loading Settings</h3>
            <p :class="ui.errorText()">{{ error }}</p>
            <CustomButton variant="secondary" @click="loadSettings">
              Try Again
            </CustomButton>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, inject } from 'vue';
import { open } from '@tauri-apps/plugin-shell';
import { tv } from '@/lib/tv';
import { pageShellTv, pageHeaderTv, cardTv } from '@/components/ui/themes';
import CustomButton from '../components/ui/CustomButton.vue';
import Input from '../components/ui/Input.vue';
import Toggle from '../components/ui/Toggle.vue';
import CustomSelect from '../components/forms/CustomSelect.vue';
import SettingsIcon from '../components/icons/SettingsIcon.vue';
import CheckIcon from '../components/icons/CheckIcon.vue';
import type { TauriInjectionKey } from '../lib/tauri-commands-plugin';
import type { AppSettings, SettingsSchema } from '../lib/tauri-commands';
import TagEditor from '../components/forms/TagEditor.vue';

defineOptions({ name: 'Settings' });

const tauri = inject('tauri') as TauriInjectionKey['tauri'];

const loading = ref(true);
const saving = ref(false);
const error = ref('');
const settingsSchema = ref<SettingsSchema | null>(null);
const settings = ref<AppSettings | null>(null);
const originalSettings = ref<AppSettings | null>(null);
const saveTimeout = ref<any | null>(null);
const lastSaved = ref(false);

const updateLoading = ref(false);
const updateMessage = ref('');
const updateUrl = ref('');

const shell = pageShellTv({ width: 'md' });
const header = pageHeaderTv();

const settingsTv = tv({
  slots: {
    headerRow: 'flex items-center gap-2',
    headerIcon: 'text-blue-600 flex-shrink-0',
    loading: 'flex justify-center py-8',
    spinner:
      'animate-spin rounded-full h-7 w-7 border-b-2 border-blue-600',
    formWrap: 'space-y-3',
    sections: 'space-y-3',
    sectionCard: cardTv({ hover: false, padding: 'none' }),
    sectionInner: 'space-y-3 p-3',
    sectionTitle: 'text-sm font-semibold text-slate-900',
    sectionDesc: 'text-xs text-slate-500 mt-0.5',
    fields: 'space-y-3',
    field: 'space-y-1.5',
    fieldLabel: 'block text-sm font-medium text-slate-700',
    fieldHint: 'text-xs text-slate-500',
    statusBar: 'flex justify-end gap-2 pt-2',
    statusSaving: 'flex items-center gap-1.5 text-xs text-blue-600',
    statusSpinner:
      'animate-spin rounded-full h-3.5 w-3.5 border-b-2 border-blue-600',
    statusSaved: 'flex items-center gap-1.5 text-xs text-green-600',
    checkIcon: 'w-3.5 h-3.5',
    updateMsg: 'text-sm text-slate-700',
    updateActions: 'flex justify-start gap-2',
    errorCard: [
      cardTv({ hover: false, padding: 'md' }),
      'border-red-200 bg-red-50',
    ].join(' '),
    errorInner: 'text-center space-y-2',
    errorTitle: 'text-sm font-semibold text-red-800',
    errorText: 'text-xs text-red-600',
  },
});

const ui = settingsTv();

const getFieldValue = (
  fieldId: string,
): string | number | boolean | string[] | null => {
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
  value: string | number | boolean | string[],
) => {
  if (!settings.value) return;

  const [section, field] = fieldId.split('.');
  if (section && field && settings.value[section]) {
    const sectionData = settings.value[section];
    sectionData[field] = value;

    autoSaveSettings();
  }
};

const autoSaveSettings = () => {
  if (saveTimeout.value) {
    clearTimeout(saveTimeout.value);
  }

  saveTimeout.value = setTimeout(async () => {
    if (settings.value) {
      try {
        saving.value = true;
        lastSaved.value = false;
        await tauri.save_settings(settings.value);
        originalSettings.value = JSON.parse(JSON.stringify(settings.value));
        lastSaved.value = true;

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
