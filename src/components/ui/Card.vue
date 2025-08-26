<template>
  <div
    :class="[
      'card',
      hover && 'card-hover',
      padding && padding,
      !padding && 'card-padding',
    ]"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
interface Props {
  hover?: boolean;
  padding?: string;
}

withDefaults(defineProps<Props>(), {
  hover: true,
  padding: '',
});
</script>

<style scoped>
.card {
  background-color: white;
  border: 1px solid var(--color-slate-300);
  transition: all 0.2s;
}

.card-hover:hover {
  border-color: var(--color-slate-400);
  box-shadow:
    0 4px 6px -1px rgb(0 0 0 / 0.1),
    0 2px 4px -2px rgb(0 0 0 / 0.1);
}

.card-padding {
  padding: 1rem;
}
</style>
