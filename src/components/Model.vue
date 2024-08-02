<template>
  <transition
    name="modal"
    @before-enter="beforeEnter"
    @enter="enter"
    @leave="leave"
  >
    <div v-if="show" class="modal">
      <div class="modal-dialog">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">{{ title }}</h5>
            <button type="button" class="close" @click="$emit('close')">
              <span aria-hidden="true">&times;</span>
            </button>
          </div>
          <div class="modal-body">
            <slot></slot>
          </div>
          <div class="modal-footer">
            <slot name="footer"></slot>
          </div>
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

const emit = defineEmits<{
  (e: 'close'): void;
}>();

function beforeEnter(el: HTMLElement) {
  el.style.opacity = '0';
  el.style.transform = 'scale(0.9)';
}

function enter(el: HTMLElement, done: () => void) {
  el.offsetHeight; // trigger reflow
  el.style.transition = 'opacity 0.3s ease, transform 0.3s ease';
  el.style.opacity = '1';
  el.style.transform = 'scale(1)';
  done();
}

function leave(el: HTMLElement, done: () => void) {
  el.style.transition = 'opacity 0.3s ease, transform 0.3s ease';
  el.style.opacity = '0';
  el.style.transform = 'scale(0.9)';
  setTimeout(done, 300);
}
</script>

<style scoped>
.modal {
  display: block;
  background: rgba(0, 0, 0, 0.5);
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  justify-content: center;
  align-items: center;
  z-index: 3;
}

.modal-dialog {
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  padding: 20px;
  width: 80%;
  max-width: 400px;
  margin: auto;
  margin-top: 50px;
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.modal-header, .modal-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header {
  border-bottom: 1px solid #e0e0e0;
  padding-bottom: 10px;
  margin-bottom: 20px;
}

.modal-footer {
  border-top: 1px solid #e0e0e0;
  padding-top: 10px;
  margin-top: 20px;
}

.modal-title {
  font-size: 18px;
  color: #333;
}

.close {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
  transform: scale(0.9);
}
</style>
