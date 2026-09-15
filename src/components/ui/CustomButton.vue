<template>
  <button
    v-bind="attrsWithoutClass"
    :class="rootClass"
    :disabled="disabled"
    :title="title"
    @click="$emit('click', $event)"
  >
    <slot name="icon" />
    <slot />
  </button>
</template>

<script setup lang="ts">
import { computed, useAttrs } from 'vue';
import { buttonTv } from './themes';

defineOptions({ name: 'CustomButton', inheritAttrs: false });

interface Props {
  variant?: 'primary' | 'secondary' | 'danger' | 'success' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  title?: string;
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  disabled: false,
});

defineEmits<{
  (e: 'click', event: MouseEvent): void;
}>();

const attrs = useAttrs();

const attrsWithoutClass = computed(() => {
  const { class: _class, ...rest } = attrs as Record<string, unknown>;
  return rest;
});

const rootClass = computed(() =>
  buttonTv({
    variant: props.variant,
    size: props.size,
    class: attrs.class as string | undefined,
  }),
);
</script>
