<template>
  <div class="browser-content">
    <div class="browser-header">
      <div class="browser-controls">
        <button class="browser-btn back" @click="goBack" :disabled="!canGoBack">←</button>
        <button class="browser-btn forward" @click="goForward" :disabled="!canGoForward">→</button>
        <button class="browser-btn refresh" @click="refresh">🔄</button>
      </div>
      <div class="browser-address-bar">
        <input 
          v-model="currentUrl" 
          @keyup.enter="navigate"
          class="browser-url-input"
          placeholder="Enter URL..."
        />
        <button class="browser-btn go" @click="navigate">Go</button>
      </div>
    </div>
    <div class="browser-frame-container">
      <iframe 
        v-if="currentUrl"
        :src="currentUrl" 
        class="browser-frame"
        @load="onFrameLoad"
        @error="onFrameError"
      ></iframe>
      <div v-else class="browser-placeholder">
        <div class="placeholder-content">
          <div class="placeholder-icon">🌐</div>
          <h3>Browser</h3>
          <p>Enter a URL above to start browsing</p>
          <button class="placeholder-btn" @click="loadDefaultPage">Load GitHub</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const currentUrl = ref('')
const canGoBack = ref(false)
const canGoForward = ref(false)
const isLoading = ref(false)

const history: string[] = []
let currentIndex = -1

onMounted(() => {
  // Загружаем GitHub по умолчанию
  loadDefaultPage()
})

function loadDefaultPage() {
  currentUrl.value = 'https://github.com/s00d/SwitchShuttle'
  navigate()
}

function navigate() {
  if (!currentUrl.value) return
  
  // Добавляем протокол если его нет
  if (!currentUrl.value.startsWith('http://') && !currentUrl.value.startsWith('https://')) {
    currentUrl.value = 'https://' + currentUrl.value
  }
  
  // Добавляем в историю
  if (currentIndex < history.length - 1) {
    history.splice(currentIndex + 1)
  }
  history.push(currentUrl.value)
  currentIndex = history.length - 1
  
  updateNavigationState()
}

function goBack() {
  if (currentIndex > 0) {
    currentIndex--
    currentUrl.value = history[currentIndex]
    updateNavigationState()
  }
}

function goForward() {
  if (currentIndex < history.length - 1) {
    currentIndex++
    currentUrl.value = history[currentIndex]
    updateNavigationState()
  }
}

function refresh() {
  // Простое обновление - перезагружаем текущий URL
  const url = currentUrl.value
  currentUrl.value = ''
  setTimeout(() => {
    currentUrl.value = url
  }, 100)
}

function updateNavigationState() {
  canGoBack.value = currentIndex > 0
  canGoForward.value = currentIndex < history.length - 1
}

function onFrameLoad() {
  isLoading.value = false
}

function onFrameError() {
  isLoading.value = false
  console.error('Failed to load iframe content')
}
</script>

<style lang="scss" scoped>
.browser-content {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #f5f5f5;
}

.browser-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: #fff;
  border-bottom: 1px solid #e0e0e0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.browser-controls {
  display: flex;
  gap: 8px;
}

.browser-btn {
  padding: 8px 12px;
  border: 1px solid #d0d0d0;
  border-radius: 6px;
  background: #fff;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
  
  &:hover:not(:disabled) {
    background: #f0f0f0;
    border-color: #b0b0b0;
  }
  
  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  &.go {
    background: #007AFF;
    color: white;
    border-color: #007AFF;
    
    &:hover {
      background: #0056CC;
      border-color: #0056CC;
    }
  }
}

.browser-address-bar {
  flex: 1;
  display: flex;
  gap: 8px;
}

.browser-url-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #d0d0d0;
  border-radius: 6px;
  font-size: 14px;
  outline: none;
  
  &:focus {
    border-color: #007AFF;
    box-shadow: 0 0 0 2px rgba(0, 122, 255, 0.2);
  }
}

.browser-frame-container {
  flex: 1;
  position: relative;
  overflow: hidden;
}

.browser-frame {
  width: 100%;
  height: 100%;
  border: none;
  background: #fff;
}

.browser-placeholder {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f8f9fa;
}

.placeholder-content {
  text-align: center;
  color: #6c757d;
}

.placeholder-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.placeholder-content h3 {
  margin: 0 0 8px 0;
  color: #495057;
  font-size: 20px;
}

.placeholder-content p {
  margin: 0 0 20px 0;
  color: #6c757d;
}

.placeholder-btn {
  padding: 12px 24px;
  background: #007AFF;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: background 0.2s ease;
  
  &:hover {
    background: #0056CC;
  }
}

// Responsive design
@media (max-width: 768px) {
  .browser-header {
    flex-direction: column;
    gap: 8px;
  }
  
  .browser-controls {
    order: 2;
  }
  
  .browser-address-bar {
    order: 1;
    width: 100%;
  }
}
</style> 