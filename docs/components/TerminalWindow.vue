<template>
    <div class="terminal-content">
      <div class="terminal-output" ref="terminalOutput">
        <div v-for="(line, index) in terminalLines" :key="index" class="terminal-line">
          <span v-if="line.type === 'command'" class="command">{{ line.content }}</span>
          <span v-else-if="line.type === 'output'" class="output">{{ line.content }}</span>
          <span v-else-if="line.type === 'error'" class="error">{{ line.content }}</span>
          <span v-else-if="line.type === 'success'" class="success">{{ line.content }}</span>
          <span v-else-if="line.type === 'warning'" class="warning">{{ line.content }}</span>
          <span v-else-if="line.type === 'info'" class="info">{{ line.content }}</span>
          <span v-else-if="line.type === 'prompt'" class="prompt">{{ line.content }}</span>
          <span v-else class="output">{{ line.content }}</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, nextTick, watch } from 'vue'

const props = defineProps({
  title: { type: String, default: 'Terminal — SwitchShuttle' },
  command: { type: String, default: '' },
  output: { type: String, default: '' }
})

// Константы терминала
const defaultTerminalOutput = `Last login: Mon Jan 15 10:30:15 on ttys000`
const terminalPrompt = `user@macbook-pro ~ % `

// Функция очистки терминала
function clearTerminal() {
  terminalLines.value = []
  addTerminalLine(defaultTerminalOutput, 'prompt')
}

// Состояние терминала
const terminalOutput = ref(null)
const terminalLines = ref([])
const isExecuting = ref(false)

// Инициализация
onMounted(() => {
  // Добавляем приветственное сообщение
  addTerminalLine(defaultTerminalOutput, 'prompt')
  scrollToBottom()
  
  // Сбрасываем состояние команды и вывода при монтировании
  console.log('TerminalWindow mounted, checking for initial command')
})

// Следим за изменениями команды и вывода
watch(() => [props.command, props.output], ([command, output], oldValues) => {
  if (command && output && command !== oldValues?.[0]) {
    console.log('TerminalWindow: executing command from props:', command)
    // Добавляем небольшую задержку, чтобы компонент успел смонтироваться
    setTimeout(() => {
      executeCommandFromMenu(command, output)
    }, 100)
  }
}, { immediate: false })

// Также следим за изменениями после монтирования
watch(() => [props.command, props.output], ([command, output], oldValues) => {
  if (command && output && command !== oldValues?.[0] && terminalLines.value.length > 0) {
    console.log('TerminalWindow: executing command after mount:', command)
    executeCommandFromMenu(command, output)
  }
}, { immediate: false })

// Функции
function addTerminalLine(content, type = 'output') {
  terminalLines.value.push({ content, type })
  nextTick(() => {
    scrollToBottom()
  })
}

function scrollToBottom() {
  if (terminalOutput.value) {
    terminalOutput.value.scrollTop = terminalOutput.value.scrollHeight
  }
}

async function executeCommandFromMenu(command, output) {
  if (isExecuting.value) return
  
  isExecuting.value = true
  
  // Добавляем команду в вывод
  addTerminalLine(`${terminalPrompt}${command}`, 'prompt')
  
  // Имитируем задержку выполнения команды
  await new Promise(resolve => setTimeout(resolve, 300 + Math.random() * 800))
  
  // Разбиваем вывод на строки для постепенного вывода
  const lines = output.split('\n')
  
  // Выводим каждую строку с небольшой задержкой
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]
    if (line.trim()) {
      addTerminalLine(line, 'output')
      // Задержка между строками
      if (i < lines.length - 1) {
        await new Promise(resolve => setTimeout(resolve, 50 + Math.random() * 100))
      }
    }
  }
  
  isExecuting.value = false
}

// Экспортируем метод для внешнего вызова
defineExpose({
  executeCommandFromMenu,
  clearTerminal
})
</script>

<style scoped>
.terminal-content {
  height: 100%;
  background: #1e1e1e;
  color: #f8f8f2;
  font-family: 'JetBrains Mono', 'Fira Mono', 'Menlo', 'Consolas', monospace;
  font-size: 14px;
  line-height: 1.4;
  padding: 12px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.terminal-output {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}

.terminal-line {
  margin-bottom: 2px;
  word-wrap: break-word;
  white-space: pre-wrap;
}

.terminal-input-line {
  display: flex;
  align-items: center;
  margin-top: 4px;
}

.terminal-input {
  background: transparent;
  border: none;
  color: #f8f8f2;
  font-family: inherit;
  font-size: inherit;
  outline: none;
  flex: 1;
  margin-left: 4px;
}

.terminal-input-disabled {
  color: #666;
  font-style: italic;
  opacity: 0.7;
  margin-left: 4px;
}

.prompt {
  color: #a6e22e;
  font-weight: 500;
}

.command {
  color: #f8f8f2;
  font-weight: 500;
}

.output {
  color: #f8f8f2;
}

.error {
  color: #f92672;
}

.success {
  color: #a6e22e;
}

.warning {
  color: #f4bf75;
}

.info {
  color: #66d9ef;
}

/* Скроллбар */
.terminal-output::-webkit-scrollbar {
  width: 8px;
}

.terminal-output::-webkit-scrollbar-track {
  background: #2d2d2d;
}

.terminal-output::-webkit-scrollbar-thumb {
  background: #555;
  border-radius: 4px;
}

.terminal-output::-webkit-scrollbar-thumb:hover {
  background: #777;
}
</style> 