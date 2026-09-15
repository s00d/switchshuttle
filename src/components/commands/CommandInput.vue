<template>
  <div :class="ui.root()">
    <ValidatedField
      :model-value="modelValue"
      :rules="commandRules"
      :hide-error="true"
      @update:model-value="handleModelValueUpdate"
    >
      <template #default="{ value, error, updateValue }">
        <div :class="ui.field()">
          <input
            :value="value"
            :placeholder="placeholder"
            :class="ui.input({ class: error ? ui.inputError() : undefined })"
            @input="(event) => updateValue((event.target as HTMLInputElement).value)"
            @keydown.enter="runCommand"
            @keydown.ctrl.enter="runCommand"
          />
          <div :class="ui.actions()">
            <CustomButton
              variant="ghost"
              :class="ui.actionBtn()"
              title="Run command (Ctrl+Enter)"
              @click="runCommand"
            >
              <RunIcon />
            </CustomButton>
            <CustomButton
              variant="ghost"
              :class="ui.actionBtn()"
              title="Copy command"
              @click="copyToClipboard"
            >
              <CopyIcon />
            </CustomButton>
            <CustomButton
              variant="ghost"
              :class="ui.actionBtn()"
              title="Clear command"
              @click="() => { clearInput(); updateValue(''); }"
            >
              <ClearIcon />
            </CustomButton>
            <div :class="ui.divider()"></div>
            <div :class="ui.pulse()"></div>
          </div>
        </div>
      </template>
    </ValidatedField>

    <div v-if="isRunning" :class="ui.status()">
      <div :class="ui.statusRow()">
        <SpinnerIcon />
        <span>Running command...</span>
      </div>
    </div>

    <div
      v-if="runResult"
      :class="
        ui.result({
          class: runResult.success ? ui.resultSuccess() : ui.resultError(),
        })
      "
    >
      <div
        :class="
          ui.resultHeader({
            class: runResult.success
              ? ui.resultHeaderSuccess()
              : ui.resultHeaderError(),
          })
        "
      >
        <SuccessIcon v-if="runResult.success" />
        <ErrorIcon v-else />
        <span :class="ui.resultLabel()">{{
          runResult.success ? 'Success' : 'Error'
        }}</span>
        <CustomButton :class="ui.clearResultBtn()" @click="clearRunResult">
          Clear
        </CustomButton>
      </div>
      <div
        :class="
          ui.resultOutput({
            class: runResult.success
              ? ui.resultOutputSuccess()
              : ui.resultOutputError(),
          })
        "
      >
        {{ runResult.output }}
      </div>
    </div>

    <div v-if="securityIssues.length > 0" :class="ui.securityList()">
      <div
        v-for="(issue, index) in securityIssues"
        :key="index"
        :class="
          ui.securityIssue({
            class:
              issue.type === 'error'
                ? ui.securityIssueError()
                : ui.securityIssueWarn(),
          })
        "
      >
        <div :class="ui.securityIssueHeader()">
          <span :class="ui.securityIssueTitle()">
            {{ issue.type === 'error' ? 'Security Warning' : 'Security Info' }}:
          </span>
          <span :class="ui.severityBadge()">
            {{ issue.severity }}
          </span>
        </div>
        <div :class="ui.securityIssueMsg()">
          {{ issue.message }}
        </div>
      </div>
    </div>

    <div v-if="securitySuggestions.length > 0" :class="ui.suggestions()">
      <div :class="ui.suggestionsTitle()">Security Suggestions:</div>
      <ul :class="ui.suggestionsList()">
        <li v-for="(suggestion, index) in securitySuggestions" :key="index">
          • {{ suggestion }}
        </li>
      </ul>
    </div>

    <div
      v-if="
        modelValue &&
        splitMultiCommand(modelValue).length > 1 &&
        !isRunning &&
        !runResult &&
        securityIssues.length === 0
      "
      :class="ui.multi()"
    >
      <div :class="ui.multiTitle()">Multi-command detected:</div>
      <div :class="ui.multiChips()">
        <span
          v-for="(sub, i) in splitMultiCommand(modelValue)"
          :key="i"
          :class="ui.multiChip()"
        >
          {{ i + 1 }}. {{ sub }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, inject, watch } from 'vue';
import { tv } from '@/lib/tv';
import type { TauriInjectionKey } from '../../lib/tauri-commands-plugin';
import ValidatedField from '../ui/ValidatedField.vue';
import { validationRules } from '../../lib/validation-rules';
import { securityManager } from '../../lib/security';
import type { SecurityIssue } from '../../lib/security';
import RunIcon from '../icons/RunIcon.vue';
import CopyIcon from '../icons/CopyIcon.vue';
import ClearIcon from '../icons/ClearIcon.vue';
import SpinnerIcon from '../icons/SpinnerIcon.vue';
import SuccessIcon from '../icons/SuccessIcon.vue';
import ErrorIcon from '../icons/ErrorIcon.vue';
import CustomButton from '../ui/CustomButton.vue';

defineOptions({ name: 'CommandInput' });

interface Props {
  modelValue: string;
  placeholder?: string;
  configId?: string;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: 'Enter command',
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
  enter: [];
}>();

const ui = tv({
  slots: {
    root: 'relative group',
    field: 'relative',
    input: [
      'w-full h-8 px-2.5 bg-white border border-slate-200 rounded-md',
      'focus:ring-2 focus:ring-blue-500 focus:border-transparent',
      'transition-all duration-200 hover:border-slate-300 focus:outline-none',
      'font-mono text-sm shadow-sm pr-24',
    ],
    inputError: 'border-red-300 focus:ring-red-500',
    actions: 'absolute inset-y-0 right-0 flex items-center pr-2 gap-1',
    actionBtn: 'p-1.5 rounded-md opacity-0 group-hover:opacity-100',
    divider: 'w-px h-4 bg-slate-200',
    pulse: 'w-2 h-2 bg-blue-500 rounded-full animate-pulse',
    status: 'mt-1 p-2 bg-blue-50 rounded-md border border-blue-200',
    statusRow: 'flex items-center gap-2 text-xs text-blue-700',
    result: 'mt-1 p-2 rounded-md border',
    resultSuccess: 'bg-green-50 border-green-200',
    resultError: 'bg-red-50 border-red-200',
    resultHeader: 'flex items-center gap-2 text-xs mb-1',
    resultHeaderSuccess: 'text-green-700',
    resultHeaderError: 'text-red-700',
    resultLabel: 'font-medium',
    clearResultBtn: 'ml-auto text-xs opacity-70 hover:opacity-100',
    resultOutput: 'font-mono text-xs',
    resultOutputSuccess: 'text-green-800',
    resultOutputError: 'text-red-800',
    securityList: 'mt-1 space-y-2',
    securityIssue: 'p-2 rounded-md border text-xs',
    securityIssueError: 'bg-red-50 border-red-200 text-red-800',
    securityIssueWarn: 'bg-yellow-50 border-yellow-200 text-yellow-800',
    securityIssueHeader: 'flex items-center gap-2 mb-1',
    securityIssueTitle: 'font-medium',
    severityBadge: 'text-xs px-1 py-0.5 rounded-md bg-white/50',
    securityIssueMsg: 'font-mono text-xs opacity-90',
    suggestions: 'mt-1 p-2 bg-blue-50 rounded-md border border-blue-200',
    suggestionsTitle: 'text-xs text-blue-700 mb-1 font-medium',
    suggestionsList: 'text-xs text-blue-800 space-y-1',
    multi: 'mt-1 p-2 bg-blue-50 rounded-md border border-blue-200',
    multiTitle: 'text-xs text-blue-700 mb-1 font-medium',
    multiChips: 'flex flex-wrap gap-1',
    multiChip:
      'inline-block bg-blue-100 text-blue-800 rounded-md px-2 py-1 text-xs font-mono',
  },
})();

// Get access to Tauri commands through plugin
const tauri = inject('tauri') as TauriInjectionKey['tauri'];

const isRunning = ref(false);
const runResult = ref<{ success: boolean; output: string } | null>(null);

// Security analysis
const securityIssues = ref<SecurityIssue[]>([]);
const securitySuggestions = ref<string[]>([]);

// Validation rules for command input
const commandRules = [
  validationRules.custom(
    (value: string) => !value || value.trim().length > 0,
    'Command cannot be empty'
  ),
  validationRules.custom(
    (value: string) => !value || value.length <= 500,
    'Command is too long (maximum 500 characters)'
  ),
  validationRules.custom(
    (value: string) => !value || !value.includes('rm -rf /'),
    'Dangerous command detected'
  ),
  validationRules.custom(
    (value: string) => !value || !value.includes('sudo rm -rf'),
    'Dangerous command detected'
  )
];

function copyToClipboard() {
  navigator.clipboard.writeText(props.modelValue);
}

function clearInput() {
  // This will be handled by the parent component through v-model
  emit('update:modelValue', '');
  clearRunResult();
}

function clearRunResult() {
  runResult.value = null;
}

async function runCommand() {
  if (!props.modelValue.trim()) return;

  isRunning.value = true;
  runResult.value = null;

  try {
    // Execute command through Tauri plugin using raw command execution
    const result = await tauri.executeRawCommand(
      props.modelValue,
      props.configId
    );

    runResult.value = {
      success: true,
      output: result || 'Command executed successfully',
    };
  } catch (error) {
    runResult.value = {
      success: false,
      output: `Error executing command: ${error instanceof Error ? error.message : 'Unknown error'}`,
    };
  } finally {
    isRunning.value = false;
  }
}

function splitMultiCommand(command: string): string[] {
  return command
    .split('&&')
    .map(cmd => cmd.trim())
    .filter(cmd => cmd.length > 0);
}

function handleModelValueUpdate(value: string) {
  emit('update:modelValue', value);
}

// Watch for command changes and analyze security
watch(
  () => props.modelValue,
  async (newCommand) => {
    if (newCommand && newCommand.trim().length > 0) {
      const result = await securityManager.analyzeCommands([newCommand]);
      securityIssues.value = [...result.errors, ...result.warnings];
      securitySuggestions.value = result.suggestions;
    } else {
      securityIssues.value = [];
      securitySuggestions.value = [];
    }
  },
  { immediate: true }
);
</script>
