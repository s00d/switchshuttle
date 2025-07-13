<template>
  <div 
    class="window" 
    :class="{ 
      'window-dragging': isDragging, 
      'window-resizing': isResizing,
      'window-minimized': win?.minimized
    }"
    :style="windowStyles"
    @click="handleWindowClick"
  >
    <div class="window-titlebar" @mousedown="startDrag">
      <div class="window-buttons">
        <div class="window-button close" @click="handleButtonClick(() => win?.id && closeWindow(win.id))"></div>
        <div class="window-button minimize" @click="handleButtonClick(() => win?.id && minimizeWindow(win.id))"></div>
        <div class="window-button maximize" @click="handleButtonClick(toggleMaximize)"></div>
      </div>
      <div class="window-title">
        <slot name="titlebar">{{ win?.props?.title || 'Window' }}</slot>
      </div>
      <div class="window-titlebar-right">
        <slot name="titlebar-right"></slot>
      </div>
    </div>
    <div class="window-content">
      <slot></slot>
    </div>
    
    <!-- Resize handles -->
    <div class="resize-handle resize-handle-n" @mousedown.stop="(e) => startResize('n', e)"></div>
    <div class="resize-handle resize-handle-s" @mousedown.stop="(e) => startResize('s', e)"></div>
    <div class="resize-handle resize-handle-e" @mousedown.stop="(e) => startResize('e', e)"></div>
    <div class="resize-handle resize-handle-w" @mousedown.stop="(e) => startResize('w', e)"></div>
    <div class="resize-handle resize-handle-ne" @mousedown.stop="(e) => startResize('ne', e)"></div>
    <div class="resize-handle resize-handle-nw" @mousedown.stop="(e) => startResize('nw', e)"></div>
    <div class="resize-handle resize-handle-se" @mousedown.stop="(e) => startResize('se', e)"></div>
    <div class="resize-handle resize-handle-sw" @mousedown.stop="(e) => startResize('sw', e)"></div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useWindowManager } from '~/composables/useWindowManager'

const props = defineProps<{ windowId: string }>()
const { windows, bringToFront, closeWindow, minimizeWindow, maximizeWindow, restoreWindow, moveWindow, resizeWindow } = useWindowManager()

const win = computed(() => windows.value.find(w => w.id === props.windowId))

function handleMouseDown() {
  if (win.value) {
    bringToFront(win.value.id)
  }
}

function handleWindowClick() {
  if (win.value && !isDragging.value && !isResizing.value) {
    bringToFront(win.value.id)
  }
}

function handleButtonClick(action: () => void) {
  if (win.value) {
    console.log('Button click for window:', win.value.id)
    bringToFront(win.value.id)
    action()
  }
}

// Drag
const isDragging = ref(false)
const dragOffset = ref({ x: 0, y: 0 })
function startDrag(e: MouseEvent) {
  if (!win.value || win.value.maximized || !win.value.position) return
  
  // Поднимаем окно наверх при начале перетаскивания
  bringToFront(win.value.id)
  
  isDragging.value = true
  dragOffset.value = {
    x: e.clientX - win.value.position.x,
    y: e.clientY - win.value.position.y
  }
  document.addEventListener('mousemove', onDrag)
  document.addEventListener('mouseup', stopDrag)
  e.preventDefault()
  e.stopPropagation()
}
function onDrag(e: MouseEvent) {
  if (!isDragging.value || !win.value || !win.value.position || !win.value.size) return
  e.preventDefault()
  const x = e.clientX - dragOffset.value.x
  const y = e.clientY - dragOffset.value.y
  moveWindow(win.value.id, {
    x: Math.max(0, Math.min(x, window.innerWidth - win.value.size.width)),
    y: Math.max(0, Math.min(y, window.innerHeight - win.value.size.height))
  })
}
function stopDrag() {
  isDragging.value = false
  document.removeEventListener('mousemove', onDrag)
  document.removeEventListener('mouseup', stopDrag)
}

// Resize
const isResizing = ref(false)
const resizeDirection = ref('')
const resizeStart = ref({ x: 0, y: 0, width: 0, height: 0, left: 0, top: 0 })

function startResize(direction: string, e?: MouseEvent) {
  if (!win.value || !win.value.size || !win.value.position) return
  
  // Поднимаем окно наверх при начале изменения размера
  bringToFront(win.value.id)
  
  isResizing.value = true
  resizeDirection.value = direction
  resizeStart.value = {
    x: e?.clientX || 0,
    y: e?.clientY || 0,
    width: win.value.size.width,
    height: win.value.size.height,
    left: win.value.position.x,
    top: win.value.position.y
  }
  document.addEventListener('mousemove', onResize)
  document.addEventListener('mouseup', stopResize)
  if (e) {
    e.preventDefault()
  }
}

function onResize(e: MouseEvent) {
  if (!isResizing.value || !win.value || !win.value.size || !win.value.position) return
  e.preventDefault()
  
  const dx = e.clientX - resizeStart.value.x
  const dy = e.clientY - resizeStart.value.y
  
  let newWidth = resizeStart.value.width
  let newHeight = resizeStart.value.height
  let newX = resizeStart.value.left
  let newY = resizeStart.value.top
  
  const direction = resizeDirection.value
  
  // Обработка изменения размера в зависимости от направления
  if (direction.includes('e')) {
    newWidth = Math.max(300, resizeStart.value.width + dx)
  }
  if (direction.includes('w')) {
    const maxDx = resizeStart.value.width - 300
    const actualDx = Math.min(dx, maxDx)
    newWidth = Math.max(300, resizeStart.value.width - actualDx)
    newX = resizeStart.value.left + actualDx
  }
  if (direction.includes('s')) {
    newHeight = Math.max(200, resizeStart.value.height + dy)
  }
  if (direction.includes('n')) {
    const maxDy = resizeStart.value.height - 200
    const actualDy = Math.min(dy, maxDy)
    newHeight = Math.max(200, resizeStart.value.height - actualDy)
    newY = resizeStart.value.top + actualDy
  }
  
  // Ограничиваем размеры окна размерами экрана
  const maxWidth = window.innerWidth - newX
  const maxHeight = window.innerHeight - newY
  
  newWidth = Math.min(newWidth, maxWidth)
  newHeight = Math.min(newHeight, maxHeight)
  
  // Применяем изменения
  resizeWindow(win.value.id, {
    width: newWidth,
    height: newHeight
  })
  
  if (newX !== resizeStart.value.left || newY !== resizeStart.value.top) {
    moveWindow(win.value.id, {
      x: newX,
      y: newY
    })
  }
}

function stopResize() {
  isResizing.value = false
  resizeDirection.value = ''
  document.removeEventListener('mousemove', onResize)
  document.removeEventListener('mouseup', stopResize)
}

function toggleMaximize() {
  if (!win.value) return
  if (win.value.maximized) {
    restoreWindow(win.value.id)
  } else {
    maximizeWindow(win.value.id)
  }
}

const windowStyles = computed(() => {
  if (!win.value || !win.value.position || !win.value.size) return {}
  
  if (win.value.maximized) {
    return {
      left: '0px',
      top: '34px',
      width: '100vw',
      height: 'calc(100vh - 74px)',
      zIndex: win.value.zIndex
    }
  }
  return {
    left: win.value.position.x + 'px',
    top: win.value.position.y + 'px',
    width: win.value.size.width + 'px',
    height: win.value.size.height + 'px',
    zIndex: win.value.zIndex
  }
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
  will-change: transform;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.window-dragging,
.window-resizing {
  transition: none !important;
  user-select: none;
}

.window-minimized {
  display: none !important;
}
.window-maximized {
  border-radius: 0;
  box-shadow: none;
  border: none;
}
.window-dragging {
  transition: none !important;
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
.window-button.close { background: #ff5f56; }
.window-button.minimize { background: #ffbd2e; }
.window-button.maximize { background: #27c93f; }
.window-title { flex: 1; font-size: 13px; font-weight: 500; color: #333; text-align: center; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.window-titlebar-right { display: flex; align-items: center; gap: 8px; }
.window-content { flex: 1; overflow: hidden; position: relative; }

/* Resize handles */
.resize-handle {
  position: absolute;
  background: transparent;
  z-index: 10;
}

.resize-handle-n {
  top: 0;
  left: 8px;
  right: 8px;
  height: 4px;
  cursor: n-resize;
}

.resize-handle-s {
  bottom: 0;
  left: 8px;
  right: 8px;
  height: 4px;
  cursor: s-resize;
}

.resize-handle-e {
  top: 8px;
  right: 0;
  bottom: 8px;
  width: 4px;
  cursor: e-resize;
}

.resize-handle-w {
  top: 8px;
  left: 0;
  bottom: 8px;
  width: 4px;
  cursor: w-resize;
}

.resize-handle-ne {
  top: 0;
  right: 0;
  width: 8px;
  height: 8px;
  cursor: ne-resize;
}

.resize-handle-nw {
  top: 0;
  left: 0;
  width: 8px;
  height: 8px;
  cursor: nw-resize;
}

.resize-handle-se {
  bottom: 0;
  right: 0;
  width: 8px;
  height: 8px;
  cursor: se-resize;
}

.resize-handle-sw {
  bottom: 0;
  left: 0;
  width: 8px;
  height: 8px;
  cursor: sw-resize;
}

/* Hover effects for resize handles */
.resize-handle:hover {
  background: rgba(0, 0, 0, 0.1);
}

.resize-handle-n:hover,
.resize-handle-s:hover {
  background: rgba(0, 0, 0, 0.1);
}

.resize-handle-e:hover,
.resize-handle-w:hover {
  background: rgba(0, 0, 0, 0.1);
}

.resize-handle-ne:hover,
.resize-handle-nw:hover,
.resize-handle-se:hover,
.resize-handle-sw:hover {
  background: rgba(0, 0, 0, 0.15);
}
</style> 