<template>
  <div 
    class="window" 
    :style="{ 
      left: x + 'px', 
      top: y + 'px', 
      zIndex: z.value,
      width: width + 'px',
      height: height + 'px'
    }"
    @mousedown="bringToFront"
  >
    <div class="window-titlebar" @mousedown="startDrag">
      <div class="window-buttons">
        <div class="window-button close" @click="close" v-if="closable"></div>
        <div class="window-button minimize" @click="minimize" v-if="closable"></div>
        <div class="window-button maximize" @click="maximize" v-if="closable"></div>
      </div>
      <div class="window-title">
        <slot name="titlebar">
          {{ title }}
        </slot>
      </div>
      <div class="window-titlebar-right">
        <slot name="titlebar-right"></slot>
      </div>
    </div>
    <div class="window-content">
      <slot></slot>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const props = defineProps({
  title: { type: String, default: 'Window' },
  initialX: { type: Number, default: 100 },
  initialY: { type: Number, default: 100 },
  z: { type: Number, default: 10 },
  width: { type: Number, default: 600 },
  height: { type: Number, default: 400 },
  closable: { type: Boolean, default: true }
})

const emit = defineEmits(['close', 'minimize', 'maximize'])

// Состояние окна
const x = ref(props.initialX)
const y = ref(props.initialY)
const z = ref(props.z)
const isDragging = ref(false)
const dragStartX = ref(0)
const dragStartY = ref(0)

// Функции
function startDrag(event) {
  if (event.target.closest('.window-button')) return
  
  isDragging.value = true
  dragStartX.value = event.clientX - x.value
  dragStartY.value = event.clientY - y.value
  
  document.addEventListener('mousemove', onDrag)
  document.addEventListener('mouseup', stopDrag)
}

function onDrag(event) {
  if (!isDragging.value) return
  
  x.value = event.clientX - dragStartX.value
  y.value = event.clientY - dragStartY.value
  
  // Ограничиваем окно в пределах экрана
  const maxX = window.innerWidth - props.width
  const maxY = window.innerHeight - props.height
  
  x.value = Math.max(0, Math.min(x.value, maxX))
  y.value = Math.max(0, Math.min(y.value, maxY))
}

function stopDrag() {
  isDragging.value = false
  document.removeEventListener('mousemove', onDrag)
  document.removeEventListener('mouseup', stopDrag)
}

function bringToFront() {
  // Находим максимальный z-index среди всех окон
  const windows = document.querySelectorAll('.window')
  let maxZ = 10
  
  windows.forEach(window => {
    const currentZ = parseInt(window.style.zIndex) || 10
    if (currentZ > maxZ) {
      maxZ = currentZ
    }
  })
  
  // Устанавливаем максимальный z-index + 1 для текущего окна
  z.value = maxZ + 1
}

function close() {
  emit('close')
}

function minimize() {
  emit('minimize')
}

function maximize() {
  emit('maximize')
}

// Lifecycle
onUnmounted(() => {
  document.removeEventListener('mousemove', onDrag)
  document.removeEventListener('mouseup', stopDrag)
})
</script>

<style scoped>
.window {
  position: absolute;
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
  border: 1px solid rgba(0, 0, 0, 0.1);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  min-width: 300px;
  min-height: 200px;
}

.window-titlebar {
  height: 32px;
  background: #f8f8f8;
  border-bottom: 1px solid #e0e0e0;
  display: flex;
  align-items: center;
  padding: 0 12px;
  cursor: move;
  user-select: none;
  position: relative;
}

.window-buttons {
  display: flex;
  gap: 8px;
  margin-right: 12px;
}

.window-button {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  cursor: pointer;
  transition: opacity 0.2s;
}

.window-button:hover {
  opacity: 0.8;
}

.window-button.close {
  background: #ff5f56;
}

.window-button.minimize {
  background: #ffbd2e;
}

.window-button.maximize {
  background: #27c93f;
}

.window-title {
  flex: 1;
  font-size: 13px;
  font-weight: 500;
  color: #333;
  text-align: center;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.window-titlebar-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.window-content {
  flex: 1;
  overflow: hidden;
  position: relative;
}
</style> 