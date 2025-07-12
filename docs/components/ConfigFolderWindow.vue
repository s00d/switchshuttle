<template>
  <Window
    title="Config Folder — SwitchShuttle"
    :width="600"
    :height="400"
    :initial-x="200"
    :initial-y="100"
    :z="1500"
    @close="$emit('close')"
  >
    <template #titlebar>
      <div class="window-title">Config Folder — SwitchShuttle</div>
    </template>
    <template #titlebar-right>
      <div class="window-menu-icon" title="View Options">
        <span class="menu-icon">👁️</span>
      </div>
    </template>
    
    <div class="finder-container">
      <!-- Toolbar -->
      <div class="finder-toolbar">
        <div class="toolbar-buttons">
          <button class="toolbar-btn" title="Back">
            <span>◀</span>
          </button>
          <button class="toolbar-btn" title="Forward">
            <span>▶</span>
          </button>
          <button class="toolbar-btn" title="Up">
            <span>▲</span>
          </button>
        </div>
        <div class="path-bar">
          <span class="path-text">~/Library/Application Support/SwitchShuttle/</span>
        </div>
        <div class="view-buttons">
          <button class="view-btn active" title="Icon View">
            <span>📁</span>
          </button>
          <button class="view-btn" title="List View">
            <span>📋</span>
          </button>
        </div>
      </div>

      <!-- Content Area -->
      <div class="finder-content">
        <div class="files-grid">
          <div 
            v-for="file in configFiles" 
            :key="file"
            class="file-item"
            @click="openFile(file)"
            @dblclick="openFile(file)"
          >
            <div class="file-icon">
              <span class="icon">📄</span>
            </div>
            <div class="file-info">
              <div class="file-name">{{ file }}</div>
              <div class="file-meta">JSON File</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Status Bar -->
      <div class="finder-status">
        <span class="status-text">{{ configFiles.length }} items</span>
        <span class="status-size">Available</span>
      </div>
    </div>
  </Window>
</template>

<script setup lang="ts">
import { configFiles } from '~/config/menu'

const emit = defineEmits<{
  close: []
  openFile: [fileName: string]
}>()

function openFile(fileName: string) {
  emit('openFile', fileName)
}
</script>

<style scoped>
.finder-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f5f5f7;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.finder-toolbar {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  background: #f5f5f7;
  border-bottom: 1px solid #d1d1d6;
  gap: 12px;
}

.toolbar-buttons {
  display: flex;
  gap: 4px;
}

.toolbar-btn {
  width: 28px;
  height: 28px;
  border: 1px solid #d1d1d6;
  background: #fff;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  font-size: 12px;
  color: #1d1d1f;
  transition: all 0.2s ease;
}

.toolbar-btn:hover {
  background: #f0f0f0;
  border-color: #c7c7cc;
}

.toolbar-btn:active {
  background: #e5e5e7;
}

.path-bar {
  flex: 1;
  background: #fff;
  border: 1px solid #d1d1d6;
  border-radius: 6px;
  padding: 6px 12px;
  margin: 0 8px;
}

.path-text {
  font-size: 13px;
  color: #1d1d1f;
  font-family: 'SF Mono', Monaco, 'Cascadia Code', monospace;
}

.view-buttons {
  display: flex;
  gap: 4px;
}

.view-btn {
  width: 28px;
  height: 28px;
  border: 1px solid #d1d1d6;
  background: #fff;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  font-size: 12px;
  color: #1d1d1f;
  transition: all 0.2s ease;
}

.view-btn:hover {
  background: #f0f0f0;
  border-color: #c7c7cc;
}

.view-btn.active {
  background: #007aff;
  border-color: #007aff;
  color: #fff;
}

.finder-content {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
}

.files-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 16px;
  padding: 8px;
}

.file-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 8px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  user-select: none;
  text-align: center;
  min-height: 80px;
}

.file-item:hover {
  background: rgba(0, 122, 255, 0.1);
}

.file-item:active {
  background: rgba(0, 122, 255, 0.2);
}

.file-icon {
  margin-bottom: 8px;
}

.file-icon .icon {
  font-size: 32px;
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1));
}

.file-info {
  width: 100%;
}

.file-name {
  font-size: 12px;
  font-weight: 500;
  color: #1d1d1f;
  margin-bottom: 2px;
  word-break: break-word;
  line-height: 1.2;
}

.file-meta {
  font-size: 10px;
  color: #86868b;
  line-height: 1.2;
}

.finder-status {
  display: flex;
  justify-content: space-between;
  padding: 8px 12px;
  background: #f5f5f7;
  border-top: 1px solid #d1d1d6;
  font-size: 12px;
  color: #86868b;
}

.status-text {
  font-weight: 500;
}

.status-size {
  opacity: 0.8;
}

.window-menu-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s ease;
  
  &:hover {
    background: rgba(0, 0, 0, 0.1);
  }
  
  .menu-icon {
    font-size: 14px;
    opacity: 0.7;
  }
}
</style> 