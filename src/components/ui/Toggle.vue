<template>
  <div :class="ui.root()">
    <div :class="ui.labelCol()">
      <label :for="id" :class="ui.label()">{{ label }}</label>
      <p v-if="description" :class="ui.description()">
        {{ description }}
      </p>
    </div>
    <button
      :id="id"
      type="button"
      :class="ui.track()"
      :disabled="disabled"
      @click="$emit('update:modelValue', !modelValue)"
    >
      <span :class="ui.thumb()" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { toggleTv } from './themes';

defineOptions({ name: 'Toggle' });

interface Props {
  modelValue: boolean;
  label: string;
  description?: string;
  id?: string;
  disabled?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
});

defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
}>();

const ui = computed(() =>
  toggleTv({
    isOn: props.modelValue,
  }),
);
</script>
