<template>
  <div :class="rootClass">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { cardTv } from './themes';

defineOptions({ name: 'Card' });

interface Props {
  hover?: boolean;
  /** Extra class merge (legacy padding override) */
  padding?: string;
  paddingSize?: 'none' | 'sm' | 'md' | 'lg';
}

const props = withDefaults(defineProps<Props>(), {
  hover: true,
  padding: '',
  paddingSize: 'md',
});

const rootClass = computed(() =>
  cardTv({
    hover: props.hover,
    padding: props.padding ? 'none' : props.paddingSize,
    class: props.padding || undefined,
  }),
);
</script>
