<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="isOpen" :class="ui.overlay()">
        <div
          :class="ui.backdrop()"
          @click="closeOnBackdrop && $emit('close')"
        />

        <div :class="ui.panel()" @click.stop>
          <div v-if="title || $slots.header" :class="ui.header()">
            <slot name="header">
              <h2 :class="ui.title()">{{ title }}</h2>
            </slot>
            <CustomButton
              v-if="showCloseButton"
              variant="ghost"
              size="sm"
              @click="$emit('close')"
            >
              <XIcon class="w-4 h-4" />
            </CustomButton>
          </div>

          <div :class="ui.body()">
            <slot />
          </div>

          <div v-if="$slots.footer" :class="ui.footer()">
            <slot name="footer" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import XIcon from '../icons/XIcon.vue';
import CustomButton from './CustomButton.vue';
import { modalTv } from './themes';

defineOptions({ name: 'Modal' });

interface Props {
  isOpen: boolean;
  title?: string;
  showCloseButton?: boolean;
  closeOnBackdrop?: boolean;
  size?: 'sm' | 'md' | 'lg' | 'xl';
}

const props = withDefaults(defineProps<Props>(), {
  showCloseButton: true,
  closeOnBackdrop: true,
  size: 'xl',
});

defineEmits<{
  (e: 'close'): void;
}>();

const ui = computed(() =>
  modalTv({
    size: props.size,
  }),
);
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
