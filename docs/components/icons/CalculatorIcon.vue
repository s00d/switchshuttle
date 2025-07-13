<template>
  <svg 
    :width="size" 
    :height="size" 
    viewBox="0 0 24 24" 
    fill="none" 
    xmlns="http://www.w3.org/2000/svg"
    :class="className"
  >
    <rect x="4" y="2" width="16" height="20" rx="2" stroke="currentColor" stroke-width="2" fill="none"/>
    <rect x="6" y="4" width="12" height="6" rx="1" fill="currentColor" opacity="0.2"/>
    <rect x="7" y="5" width="2" height="2" fill="currentColor"/>
    <rect x="10" y="5" width="2" height="2" fill="currentColor"/>
    <rect x="13" y="5" width="2" height="2" fill="currentColor"/>
    <rect x="7" y="8" width="2" height="2" fill="currentColor"/>
    <rect x="10" y="8" width="2" height="2" fill="currentColor"/>
    <rect x="13" y="8" width="2" height="2" fill="currentColor"/>
    <rect x="7" y="11" width="2" height="2" fill="currentColor"/>
    <rect x="10" y="11" width="2" height="2" fill="currentColor"/>
    <rect x="13" y="11" width="2" height="2" fill="currentColor"/>
    <rect x="7" y="14" width="2" height="2" fill="currentColor"/>
    <rect x="10" y="14" width="2" height="2" fill="currentColor"/>
    <rect x="13" y="14" width="2" height="2" fill="currentColor"/>
    <rect x="7" y="17" width="2" height="2" fill="currentColor"/>
    <rect x="10" y="17" width="2" height="2" fill="currentColor"/>
    <rect x="13" y="17" width="2" height="2" fill="currentColor"/>
  </svg>
</template>

<script setup lang="ts">
interface Props {
  size?: number | string
  className?: string
}

withDefaults(defineProps<Props>(), {
  size: 24,
  className: ''
})
</script> 