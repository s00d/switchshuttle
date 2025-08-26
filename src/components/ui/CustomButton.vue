<template>
  <button
    :class="[
      'btn',
      sizeClasses[size],
      variantClasses[variant],
      disabled && 'btn-disabled',
      $attrs.class,
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
  disabled: false,
});

defineEmits<{
  (e: 'click', event: MouseEvent): void;
}>();

const sizeClasses = {
  sm: 'btn-sm',
  md: 'btn-md',
  lg: 'btn-lg',
};

const variantClasses = {
  primary: 'btn-primary',
  secondary: 'btn-secondary',
  danger: 'btn-danger',
  success: 'btn-success',
  ghost: 'btn-ghost',
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
  border-radius: 0.375rem;
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
  background-color: rgb(37 99 235);
  border-color: rgb(37 99 235);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: rgb(29 78 216);
  border-color: rgb(29 78 216);
}

.btn-primary:focus {
  outline: none;
  box-shadow: 0 0 0 2px rgb(59 130 246), 0 0 0 1px white;
}

.btn-secondary {
  background-color: white;
  border-color: rgb(203 213 225);
  color: rgb(51 65 85);
}

.btn-secondary:hover:not(:disabled) {
  background-color: rgb(248 250 252);
  border-color: rgb(148 163 184);
}

.btn-secondary:focus {
  outline: none;
  box-shadow: 0 0 0 2px rgb(100 116 139), 0 0 0 1px white;
}

.btn-danger {
  background-color: rgb(220 38 38);
  border-color: rgb(220 38 38);
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background-color: rgb(185 28 28);
  border-color: rgb(185 28 28);
}

.btn-danger:focus {
  outline: none;
  box-shadow: 0 0 0 2px rgb(239 68 68), 0 0 0 1px white;
}

.btn-success {
  background-color: rgb(22 163 74);
  border-color: rgb(22 163 74);
  color: white;
}

.btn-success:hover:not(:disabled) {
  background-color: rgb(21 128 61);
  border-color: rgb(21 128 61);
}

.btn-success:focus {
  outline: none;
  box-shadow: 0 0 0 2px rgb(34 197 94), 0 0 0 1px white;
}

.btn-ghost {
  background-color: transparent;
  border-color: transparent;
  color: rgb(71 85 105);
}

.btn-ghost:hover:not(:disabled) {
  background-color: rgb(241 245 249);
  border-color: rgb(203 213 225);
}

.btn-ghost:focus {
  outline: none;
  box-shadow: 0 0 0 2px rgb(100 116 139), 0 0 0 1px white;
}
</style>
