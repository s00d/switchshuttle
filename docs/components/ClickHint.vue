<template>
  <Teleport to="body">
    <div 
      class="click-hint-overlay" 
      v-show="isVisible"
      :style="positionStyle"
    >
      <div class="click-hint">
        <div class="click-icon">👆</div>
        <div class="click-text">Click to open</div>
      </div>
    </div>
  </Teleport>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch, computed } from 'vue'

const props = defineProps({
  targetClass: {
    type: String,
    required: true
  },
  show: {
    type: Boolean,
    default: false
  }
})

// Состояние
const isVisible = ref(false)
const position = ref({ top: 0, left: 0 })

// Вычисляемое свойство для стилей позиции
const positionStyle = computed(() => ({
  top: position.value.top + 'px',
  left: position.value.left + 'px'
}))

// Функция обновления позиции
function updatePosition() {
  if (props.show) {
    const targetElement = document.querySelector(props.targetClass)
    
    if (targetElement) {
      const rect = targetElement.getBoundingClientRect()
      
      if (rect.width > 0 && rect.height > 0) {
        // Сначала устанавливаем позицию без учета размера подсказки
        position.value = {
          top: rect.bottom + 8,
          left: rect.left + (rect.width / 2)
        }
        
        // Показываем подсказку
        isVisible.value = true
        
        // После показа получаем реальные размеры и корректируем позицию
        setTimeout(() => {
          const hintElement = document.querySelector('.click-hint')
          if (hintElement) {
            const hintWidth = hintElement.offsetWidth
            position.value = {
              top: rect.bottom + 8,
              left: rect.left + (rect.width / 2) - (hintWidth / 2)
            }
          }
        }, 50)
      }
    }
  }
}

// Следим за изменением видимости
watch(() => props.show, (newValue) => {
  if (newValue) {
    // Скрываем подсказку до правильного позиционирования
    isVisible.value = false
    // Обновляем позицию с задержкой
    setTimeout(updatePosition, 100)
  } else {
    // Скрываем подсказку при скрытии
    isVisible.value = false
  }
})

// Обработчик изменения размера окна
function handleResize() {
  if (props.show && isVisible.value) {
    updatePosition()
  }
}

// Lifecycle
onMounted(() => {
  // Скрываем подсказку до правильного позиционирования
  isVisible.value = false
  // Обновляем позицию с задержкой
  setTimeout(updatePosition, 100)
  
  // Добавляем обработчик изменения размера окна
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
})
</script>

<style scoped>
/* Click Hint Overlay */
.click-hint-overlay {
  position: fixed;
  z-index: 10;
  pointer-events: none;
  animation: fadeInUp 0.6s cubic-bezier(0.16, 1, 0.3, 1);
}

/* Click Hint Container */
.click-hint {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.85) 0%, 
    rgba(240, 245, 255, 0.9) 50%, 
    rgba(235, 240, 255, 0.85) 100%);
  backdrop-filter: blur(25px) saturate(1.3) brightness(1.1);
  border-radius: 6px;
  padding: 12px 16px;
  box-shadow: 
    0 12px 40px rgba(0, 0, 0, 0.08),
    0 4px 16px rgba(0, 122, 255, 0.12),
    0 0 0 1px rgba(255, 255, 255, 0.9),
    0 0 0 1px rgba(0, 122, 255, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.8),
    inset 0 -1px 0 rgba(0, 0, 0, 0.05);
  border: 1px solid rgba(0, 122, 255, 0.2);
  min-width: 80px;
  transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  animation: float 3s ease-in-out infinite;
  position: relative;
  overflow: hidden;
}

.click-hint::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.4) 0%, 
    rgba(240, 245, 255, 0.3) 50%, 
    rgba(235, 240, 255, 0.4) 100%);
  pointer-events: none;
  z-index: -1;
}

.click-hint:hover {
  transform: translateY(-5px) scale(1.05);
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.95) 0%, 
    rgba(245, 250, 255, 0.95) 50%, 
    rgba(240, 245, 255, 0.95) 100%);
  box-shadow: 
    0 16px 48px rgba(0, 0, 0, 0.12),
    0 6px 20px rgba(0, 122, 255, 0.15),
    0 0 0 1px rgba(255, 255, 255, 0.95),
    0 0 0 1px rgba(0, 122, 255, 0.15),
    inset 0 1px 0 rgba(255, 255, 255, 0.9),
    inset 0 -1px 0 rgba(0, 0, 0, 0.03);
  border-color: rgba(0, 122, 255, 0.3);
  animation-play-state: paused;
}

.click-hint:hover::before {
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.5) 0%, 
    rgba(245, 250, 255, 0.4) 50%, 
    rgba(240, 245, 255, 0.5) 100%);
}

/* Click Icon */
.click-icon {
  font-size: 20px;
  animation: pulse 2s ease-in-out infinite;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.15));
  position: relative;
  z-index: 1;
}

/* Click Text */
.click-text {
  font-size: 12px;
  font-weight: 600;
  color: #2d3748;
  text-align: center;
  letter-spacing: 0.4px;
  line-height: 1.2;
  position: relative;
  z-index: 1;
  text-shadow: 0 1px 2px rgba(255, 255, 255, 0.8);
}

/* Animations */
@keyframes pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.1);
    opacity: 0.8;
  }
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(15px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-8px);
  }
}
</style> 