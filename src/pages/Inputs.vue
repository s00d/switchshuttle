<template>
  <div :class="ui.root()">
    <div :class="ui.wrap()">
      <div v-if="previewCommand" :class="ui.preview()">
        <code :class="ui.previewCode()">{{ previewCommand }}</code>
      </div>

      <div v-if="errorMessage" :class="ui.error()">
        <span :class="ui.errorText()">{{ errorMessage }}</span>
      </div>

      <div :class="ui.card()">
        <form id="inputForm" :class="ui.form()" @submit.prevent="submitForm">
          <div v-for="(_, key) in inputs" :key="key" :class="ui.field()">
            <label :for="key" :class="ui.label()">
              {{ key }}
            </label>
            <div :class="ui.inputWrap()">
              <input
                :id="key"
                v-model="inputs[key]"
                :name="key"
                :placeholder="`Enter ${String(key).toLowerCase()}`"
                :class="ui.input()"
                @keydown.enter="submitForm"
              />
              <div :class="ui.dotWrap()">
                <div :class="ui.dot()" />
              </div>
            </div>
          </div>
        </form>

        <div :class="ui.actions()">
          <CustomButton variant="secondary" :class="ui.actionBtn()" @click="onClose">
            <CancelIcon :class="ui.btnIcon()" />
            Cancel
          </CustomButton>
          <CustomButton variant="primary" :class="ui.actionBtn()" @click="submitForm">
            <RunIcon :class="ui.btnIcon()" />
            Execute
          </CustomButton>
        </div>
      </div>

      <div :class="ui.footer()">
        <p :class="ui.footerText()">
          <kbd :class="ui.kbd()">Enter</kbd> to submit •
          <kbd :class="ui.kbd()">Esc</kbd> to cancel
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick, inject, computed } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRouter, useRoute } from 'vue-router';
import { tv } from '@/lib/tv';
import { cardTv } from '@/components/ui/themes';
import CustomButton from '../components/ui/CustomButton.vue';
import CancelIcon from '../components/icons/CancelIcon.vue';
import RunIcon from '../components/icons/RunIcon.vue';
import type { TauriInjectionKey } from '../lib/tauri-commands-plugin';

defineOptions({ name: 'Inputs' });

const router = useRouter();
const route = useRoute();

const tauri = inject('tauri') as TauriInjectionKey['tauri'];

const command = ref('');
const inputs = ref({});
const errorMessage = ref('');
const commandTemplate = ref('');

const inputsTv = tv({
  slots: {
    root: 'min-h-full bg-slate-50 flex items-start justify-center pt-3 p-3',
    wrap: 'w-full max-w-sm',
    preview: 'mb-2.5 px-3 py-2 bg-blue-50 border border-blue-200 rounded-md',
    previewCode:
      'text-xs text-blue-800 bg-blue-100 px-1.5 py-0.5 rounded-md font-mono break-all',
    error: 'mb-2.5 px-3 py-2 bg-red-50 border border-red-200 rounded-md',
    errorText: 'text-xs text-red-700',
    card: cardTv({ hover: false, padding: 'none' }),
    form: 'p-3 space-y-2.5',
    field: 'flex items-center',
    label:
      'text-sm font-medium text-slate-700 bg-slate-50 border border-r-0 border-slate-200 px-2.5 py-1.5 rounded-l-md',
    inputWrap: 'relative flex-1',
    input: [
      'w-full px-2.5 py-1.5 bg-white border-l-0 border border-slate-200 rounded-r-md text-sm',
      'focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200',
      'hover:border-slate-300 focus:outline-none',
    ],
    dotWrap: 'absolute inset-y-0 right-0 flex items-center pr-2',
    dot: 'w-1.5 h-1.5 bg-blue-500 rounded-full',
    actions: 'p-3 pt-0 flex gap-2',
    actionBtn: 'flex-1',
    btnIcon: 'w-3.5 h-3.5 mr-1.5',
    footer: 'text-center mt-2.5',
    footerText: 'text-xs text-slate-400',
    kbd: 'px-1.5 py-0.5 text-xs font-semibold text-slate-500 bg-slate-100 border border-slate-200 rounded-md',
  },
});

const ui = inputsTv();

const previewCommand = computed(() => {
  if (!commandTemplate.value) return '';

  let preview = commandTemplate.value;

  Object.entries(inputs.value).forEach(([key, value]) => {
    const placeholder = `{${key}}`;
    if (preview.includes(placeholder)) {
      preview = preview.replace(
        new RegExp(placeholder.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'g'),
        String(value || ''),
      );
    }
  });

  return preview;
});

async function submitForm() {
  try {
    await tauri.execute_command_with_inputs(inputs.value, command.value);
    onClose();
  } catch (error) {
    errorMessage.value = error as string;
  }
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Enter' && (event.target as HTMLElement).tagName !== 'INPUT') {
    event.preventDefault();
    submitForm();
  }
  if (event.key === 'Escape') {
    event.preventDefault();
    onClose();
  }
}

async function fetchInputData() {
  try {
    const data = await tauri.fetch_input_data(command.value);
    inputs.value = data;

    await fetchCommandTemplate();
  } catch (error) {
    errorMessage.value = error as string;
  }
}

async function fetchCommandTemplate() {
  try {
    const commandInfo = await tauri.get_command_info(command.value);
    if (commandInfo && commandInfo.commands && commandInfo.commands.length > 0) {
      commandTemplate.value = commandInfo.commands[0];
    } else if (commandInfo && commandInfo.command) {
      commandTemplate.value = commandInfo.command;
    }
  } catch (error) {
    console.warn('Failed to fetch command template:', error);
  }
}

function onClose() {
  inputs.value = {};
  errorMessage.value = '';
  command.value = '';
  commandTemplate.value = '';

  router
    .push('/')
    .then(() => {
      getCurrentWindow().hide();
    })
    .catch(() => {
      getCurrentWindow().hide();
    });
}

onMounted(() => {
  if (!route.params.id) {
    console.warn('No command ID provided, redirecting to home');
    router.push('/').catch(() => {});
    return;
  }

  command.value = route.params.id as string;
  fetchInputData();

  document.addEventListener('keydown', handleKeydown);
});

watch(
  inputs,
  (newInputs) => {
    if (newInputs && Object.keys(newInputs).length > 0) {
      nextTick(() => {
        const firstInput = document.querySelector('input');
        if (firstInput) {
          firstInput.focus();
        }
      });
    }
  },
  { immediate: true },
);

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown);
});
</script>
