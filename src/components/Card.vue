<template>
  <div 
    :class="[
      'bg-white border border-slate-300 transition-all duration-200',
      hover && 'hover:border-slate-400 hover:shadow-md',
      padding && padding,
      !padding && 'p-4'
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
  padding: ''
});
</script> 