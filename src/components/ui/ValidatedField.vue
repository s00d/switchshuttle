<template>
  <div :class="ui.root()">
    <slot
      :value="modelValue"
      :error="error"
      :isValid="isValid"
      :updateValue="updateValue"
    />
    <div v-if="error && !hideError">
      <p :class="ui.error()">{{ error }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { validatedFieldTv } from './themes';

defineOptions({ name: 'ValidatedField' });

export interface ValidationRule {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  test: (value: any) => boolean;
  message: string;
}

interface Props {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  modelValue: any;
  rules?: ValidationRule[];
  validateOnInput?: boolean;
  hideError?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  rules: () => [],
  validateOnInput: true,
  hideError: false,
});

const emit = defineEmits<{
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  (e: 'update:modelValue', value: any): void;
  (e: 'validation', result: { isValid: boolean; error: string }): void;
}>();

const ui = validatedFieldTv();
const error = ref<string>('');

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const validate = (value: any): { isValid: boolean; error: string } => {
  for (const rule of props.rules) {
    if (!rule.test(value)) {
      error.value = rule.message;
      return { isValid: false, error: rule.message };
    }
  }

  error.value = '';
  return { isValid: true, error: '' };
};

const isValid = computed(() => !error.value);

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const updateValue = (newValue: any) => {
  emit('update:modelValue', newValue);

  if (props.validateOnInput) {
    const result = validate(newValue);
    emit('validation', result);
  }
};

const initialResult = validate(props.modelValue);
emit('validation', initialResult);

watch(
  () => props.modelValue,
  (newValue) => {
    if (props.validateOnInput) {
      const result = validate(newValue);
      emit('validation', result);
    }
  },
);

watch(
  () => props.rules,
  () => {
    const result = validate(props.modelValue);
    emit('validation', result);
  },
  { deep: true },
);
</script>
