<template>
  <transition
      name="modal"
      @before-enter="beforeEnter"
      @enter="enter"
      @leave="leave"
  >
    <div
        v-if="show"
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
    >
      <div
          class="bg-white w-full max-w-md rounded-lg shadow-xl p-6 transition-all"
      >
        <div class="flex items-center justify-between border-b pb-3 mb-4">
          <h5 class="text-lg font-semibold text-gray-800">{{ title }}</h5>
          <button
              type="button"
              class="text-gray-400 hover:text-gray-600 text-2xl leading-none"
              @click="$emit('close')"
          >
            &times;
          </button>
        </div>
        <div class="modal-body">
          <slot></slot>
        </div>
        <div class="flex items-center justify-end gap-2 border-t pt-4 mt-6">
          <slot name="footer" />
        </div>
      </div>
    </div>
  </transition>
</template>

<script lang="ts" setup>
defineProps({
  show: {
    type: Boolean,
    required: true
  },
  title: {
    type: String,
    required: true
  }
});

defineEmits<{
  (e: 'close'): void;
}>();

function beforeEnter(el: Element) {
  (el as HTMLElement).style.opacity = '0';
  (el as HTMLElement).style.transform = 'scale(0.9)';
}

function enter(el: Element, done: () => void) {
  (el as HTMLElement).offsetHeight; // trigger reflow
  (el as HTMLElement).style.transition = 'opacity 0.3s ease, transform 0.3s ease';
  (el as HTMLElement).style.opacity = '1';
  (el as HTMLElement).style.transform = 'scale(1)';
  done();
}

function leave(el: Element, done: () => void) {
  (el as HTMLElement).style.transition = 'opacity 0.3s ease, transform 0.3s ease';
  (el as HTMLElement).style.opacity = '0';
  (el as HTMLElement).style.transform = 'scale(0.9)';
  setTimeout(done, 300);
}
</script>