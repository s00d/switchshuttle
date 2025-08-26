<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-50 flex items-center justify-center p-4"
      >
        <!-- Backdrop -->
        <div
          class="absolute inset-0 bg-black/50 backdrop-blur-sm"
          @click="closeOnBackdrop && $emit('close')"
        />

        <!-- Modal -->
        <div
          class="relative bg-white border border-slate-300 shadow-xl w-full max-w-6xl max-h-[calc(95vh-1rem)] overflow-hidden flex flex-col"
          @click.stop
        >
          <!-- Header -->
          <div
            v-if="title || $slots.header"
            class="flex items-center justify-between p-4 border-b border-slate-200 flex-shrink-0"
          >
            <slot name="header">
              <h2 class="text-lg font-semibold text-slate-900">{{ title }}</h2>
            </slot>
            <CustomButton
              v-if="showCloseButton"
              variant="ghost"
              class="p-1"
              @click="$emit('close')"
            >
              <XIcon class="w-5 h-5" />
            </CustomButton>
          </div>

          <!-- Content -->
          <div class="flex-1 overflow-y-auto p-4">
            <slot />
          </div>

          <!-- Footer -->
          <div
            v-if="$slots.footer"
            class="flex items-center justify-end gap-2 p-4 border-t border-slate-200 flex-shrink-0"
          >
            <slot name="footer" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import XIcon from '../icons/XIcon.vue';
import CustomButton from './CustomButton.vue';

interface Props {
  isOpen: boolean;
  title?: string;
  showCloseButton?: boolean;
  closeOnBackdrop?: boolean;
}

withDefaults(defineProps<Props>(), {
  showCloseButton: true,
  closeOnBackdrop: true,
});

defineEmits<{
  (e: 'close'): void;
}>();
</script>

<style scoped>
.modal-enter-active,
.modal-leave-active {
  transition: all 0.2s ease;
}

.modal-enter-from {
  opacity: 0;
  transform: scale(0.95);
}

.modal-leave-to {
  opacity: 0;
  transform: scale(0.95);
}
</style>
