<template>
  <div class="validated-field">
    <slot 
      :value="modelValue" 
      :error="error" 
      :isValid="isValid"
      :updateValue="updateValue"
    />
    <div v-if="error && !hideError" class="validation-error">
      <p class="text-red-500 text-sm mt-1">{{ error }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';

export interface ValidationRule {
  test: (value: any) => boolean;
  message: string;
}

interface Props {
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
  (e: 'update:modelValue', value: any): void;
  (e: 'validation', result: { isValid: boolean; error: string }): void;
}>();

const error = ref<string>('');

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

const updateValue = (newValue: any) => {
  emit('update:modelValue', newValue);
  
  if (props.validateOnInput) {
    const result = validate(newValue);
    emit('validation', result);
  }
};

// Initial validation
const initialResult = validate(props.modelValue);
emit('validation', initialResult);

// Watch for value changes
watch(() => props.modelValue, (newValue) => {
  if (props.validateOnInput) {
    const result = validate(newValue);
    emit('validation', result);
  }
});

// Watch for rules changes
watch(() => props.rules, () => {
  const result = validate(props.modelValue);
  emit('validation', result);
}, { deep: true });
</script>

<style scoped>
.validation-error {
  animation: fadeIn 0.2s ease-in-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-2px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style> 