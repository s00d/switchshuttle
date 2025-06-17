<template>
  <button
    :class="[
      'inline-flex items-center justify-center gap-2 font-medium transition-all duration-200 border',
      sizeClasses[size],
      variantClasses[variant],
      disabled && 'opacity-50 cursor-not-allowed',
      !disabled && 'hover:shadow-sm active:scale-[0.98]',
      $attrs.class
    ]"
    :disabled="disabled"
    :title="title"
    @click="$emit('click', $event)"
  >
    <slot name="icon" />
    <slot />
  </button>
</template>

<script setup lang="ts">
interface Props {
  variant?: 'primary' | 'secondary' | 'danger' | 'success' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  title?: string;
}

withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  disabled: false
});

defineEmits<{
  (e: 'click', event: MouseEvent): void;
}>();

const sizeClasses = {
  sm: 'px-3 py-1.5 text-xs',
  md: 'px-4 py-2 text-sm',
  lg: 'px-6 py-2.5 text-sm'
};

const variantClasses = {
  primary: 'bg-blue-600 text-white border-blue-600 hover:bg-blue-700 hover:border-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-1',
  secondary: 'bg-white text-slate-700 border-slate-300 hover:bg-slate-50 hover:border-slate-400 focus:ring-2 focus:ring-slate-500 focus:ring-offset-1',
  danger: 'bg-red-600 text-white border-red-600 hover:bg-red-700 hover:border-red-700 focus:ring-2 focus:ring-red-500 focus:ring-offset-1',
  success: 'bg-green-600 text-white border-green-600 hover:bg-green-700 hover:border-green-700 focus:ring-2 focus:ring-green-500 focus:ring-offset-1',
  ghost: 'bg-transparent text-slate-600 border-transparent hover:bg-slate-100 hover:border-slate-300 focus:ring-2 focus:ring-slate-500 focus:ring-offset-1'
};
</script> 