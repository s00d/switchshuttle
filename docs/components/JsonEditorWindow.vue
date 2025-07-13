<template>
  <div class="json-editor">
    <div class="editor-header">
      <div class="file-tabs">
        <div class="tab active">
          <span class="tab-icon">📄</span>
          <span class="tab-title">{{ props.configFile || 'config.json' }}</span>
          <span class="tab-close">×</span>
        </div>
      </div>
      <div class="editor-toolbar">
        <button class="toolbar-btn" title="Save">
          <span>💾</span>
        </button>
        <button class="toolbar-btn" title="Format">
          <span>🔧</span>
        </button>
        <div class="separator"></div>
        <span class="status">Line 1, Column 1</span>
      </div>
    </div>
    <div class="editor-content">
      <div class="line-numbers">
        <div v-for="line in totalLines" :key="line" class="line-number">
          {{ line }}
        </div>
      </div>
      <div class="code-area" ref="codeArea">
        <pre class="code-content" ref="codeContent">{{ jsonContent }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

const props = defineProps<{
  configFile?: string
}>()

const codeArea = ref<HTMLElement>()
const codeContent = ref<HTMLElement>()

import { configFileContents } from '~/config/menu'

const getConfigContent = (fileName: string) => {
  return configFileContents[fileName] || configFileContents['switch-shuttle.json']
}

const jsonContent = ref(getConfigContent(props.configFile || 'switch-shuttle.json'))

const totalLines = computed(() => {
  return jsonContent.value.split('\n').length
})

onMounted(() => {
  if (codeArea.value && codeContent.value) {
    // Синхронизация скролла между номерами строк и кодом
    codeArea.value.addEventListener('scroll', () => {
      if (codeArea.value) {
        const scrollTop = codeArea.value.scrollTop
        const lineNumbers = codeArea.value.parentElement?.querySelector('.line-numbers')
        if (lineNumbers) {
          lineNumbers.scrollTop = scrollTop
        }
      }
    })
  }
})
</script>

<style scoped>
.json-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #1e1e1e;
  color: #d4d4d4;
  font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.5;
}

.editor-header {
  background: #2d2d30;
  border-bottom: 1px solid #3e3e42;
  display: flex;
  flex-direction: column;
}

.file-tabs {
  display: flex;
  background: #2d2d30;
  border-bottom: 1px solid #3e3e42;
}

.tab {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  background: #1e1e1e;
  border-right: 1px solid #3e3e42;
  cursor: pointer;
  user-select: none;
  min-width: 120px;
}

.tab.active {
  background: #007acc;
  color: white;
}

.tab-icon {
  margin-right: 6px;
  font-size: 12px;
}

.tab-title {
  flex: 1;
  font-size: 12px;
}

.tab-close {
  margin-left: 8px;
  font-size: 14px;
  opacity: 0.7;
  cursor: pointer;
}

.tab-close:hover {
  opacity: 1;
}

.editor-toolbar {
  display: flex;
  align-items: center;
  padding: 4px 12px;
  background: #2d2d30;
  border-bottom: 1px solid #3e3e42;
}

.toolbar-btn {
  background: none;
  border: none;
  color: #cccccc;
  padding: 4px 8px;
  margin-right: 4px;
  border-radius: 3px;
  cursor: pointer;
  font-size: 12px;
}

.toolbar-btn:hover {
  background: #3e3e42;
}

.separator {
  width: 1px;
  height: 16px;
  background: #3e3e42;
  margin: 0 8px;
}

.status {
  font-size: 11px;
  color: #999999;
  margin-left: auto;
}

.editor-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.line-numbers {
  background: #252526;
  color: #858585;
  padding: 8px 0;
  text-align: right;
  min-width: 50px;
  border-right: 1px solid #3e3e42;
  font-size: 12px;
  line-height: 1.5;
  user-select: none;
}

.line-number {
  padding: 0 8px;
  height: 19.5px;
  display: flex;
  align-items: center;
  justify-content: flex-end;
}

.code-area {
  flex: 1;
  overflow: auto;
  position: relative;
}

.code-content {
  margin: 0;
  padding: 8px 12px;
  white-space: pre;
  font-family: inherit;
  font-size: inherit;
  line-height: inherit;
  color: inherit;
  background: transparent;
  border: none;
  outline: none;
  resize: none;
  width: 100%;
  height: 100%;
  box-sizing: border-box;
}

/* Стили для подсветки синтаксиса JSON */
.code-content {
  color: #d4d4d4;
}

/* Ключи */
.code-content {
  color: #9cdcfe;
}

/* Строки */
.code-content {
  color: #ce9178;
}

/* Числа */
.code-content {
  color: #b5cea8;
}

/* Булевы значения */
.code-content {
  color: #569cd6;
}

/* Null */
.code-content {
  color: #569cd6;
}

/* Скроллбар */
.code-area::-webkit-scrollbar {
  width: 14px;
}

.code-area::-webkit-scrollbar-track {
  background: #1e1e1e;
}

.code-area::-webkit-scrollbar-thumb {
  background: #424242;
  border-radius: 7px;
  border: 2px solid #1e1e1e;
}

.code-area::-webkit-scrollbar-thumb:hover {
  background: #4f4f4f;
}

.line-numbers::-webkit-scrollbar {
  width: 0;
}
</style> 