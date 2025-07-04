<template>
  <button
    :class="[
      'btn',
      sizeClasses[size],
      variantClasses[variant],
      disabled && 'btn-disabled',
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
  sm: 'btn-sm',
  md: 'btn-md',
  lg: 'btn-lg'
};

const variantClasses = {
  primary: 'btn-primary',
  secondary: 'btn-secondary',
  danger: 'btn-danger',
  success: 'btn-success',
  ghost: 'btn-ghost'
};
</script> 

<style scoped>
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  font-weight: 500;
  transition: all 0.2s;
  border: 1px solid;
  cursor: pointer;
}

.btn:disabled,
.btn-disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn:not(:disabled):hover {
  box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);
}

.btn:not(:disabled):active {
  transform: scale(0.98);
}

.btn-sm {
  padding: 0.375rem 0.75rem;
  font-size: 0.75rem;
}

.btn-md {
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
}

.btn-lg {
  padding: 0.625rem 1.5rem;
  font-size: 0.875rem;
}

.btn-primary {
  background-color: var(--color-blue-600);
  border-color: var(--color-blue-600);
  color: white;
}

.btn-primary:hover {
  background-color: var(--color-blue-700);
  border-color: var(--color-blue-700);
}

.btn-primary:focus {
  outline: none;
  box-shadow: 0 0 0 2px var(--color-blue-500), 0 0 0 1px white;
}

.btn-secondary {
  background-color: white;
  border-color: var(--color-slate-300);
  color: var(--color-slate-700);
}

.btn-secondary:hover {
  background-color: var(--color-slate-50);
  border-color: var(--color-slate-400);
}

.btn-secondary:focus {
  outline: none;
  box-shadow: 0 0 0 2px var(--color-slate-500), 0 0 0 1px white;
}

.btn-danger {
  background-color: var(--color-red-600);
  border-color: var(--color-red-600);
  color: white;
}

.btn-danger:hover {
  background-color: var(--color-red-700);
  border-color: var(--color-red-700);
}

.btn-danger:focus {
  outline: none;
  box-shadow: 0 0 0 2px var(--color-red-500), 0 0 0 1px white;
}

.btn-success {
  background-color: var(--color-green-600);
  border-color: var(--color-green-600);
  color: white;
}

.btn-success:hover {
  background-color: var(--color-green-700);
  border-color: var(--color-green-700);
}

.btn-success:focus {
  outline: none;
  box-shadow: 0 0 0 2px var(--color-green-500), 0 0 0 1px white;
}

.btn-ghost {
  background-color: transparent;
  border-color: transparent;
  color: var(--color-slate-600);
}

.btn-ghost:hover {
  background-color: var(--color-slate-100);
  border-color: var(--color-slate-300);
}

.btn-ghost:focus {
  outline: none;
  box-shadow: 0 0 0 2px var(--color-slate-500), 0 0 0 1px white;
}
</style> 