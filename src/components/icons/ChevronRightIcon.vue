<template>
  <svg :class="['w-5 h-5 transition-transform', collapsed ? 'rotate-[-90deg]' : 'rotate-0']" fill="none" stroke="currentColor" viewBox="0 0 24 24">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
  </svg>
</template>

<script lang="ts" setup>
defineProps<{
  collapsed?: boolean;
}>();
</script> 