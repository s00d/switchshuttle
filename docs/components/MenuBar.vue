<template>
  <div class="macos-menu-bar">
    <div class="menu-bar-left">
      <div class="menu-bar-item">
        <img :src="getIconPath('icon.png')" alt="SwitchShuttle" class="menu-bar-icon">
        <span class="menu-bar-text">SwitchShuttle Demo</span>
      </div>
    </div>
    
    <div class="menu-bar-right">
      <div class="menu-bar-item locale-switcher" @click="toggleLocaleMenu">
        <span class="menu-bar-text">{{ currentLocale }}</span>
        <div class="locale-arrow">▼</div>
      </div>
      <div class="menu-bar-item links-menu" @click="toggleLinksMenu">
        <span class="menu-bar-text">Links</span>
        <div class="locale-arrow">▼</div>
      </div>
      <div class="menu-bar-item" @click="toggleMenu" v-if="props.showSwitchShuttleIcon" ref="switchShuttleIconRef">
        <img :src="getIconPath('icon.png')" alt="SwitchShuttle" class="menu-bar-icon">
      </div>
      <div class="menu-bar-item">
        <span class="menu-bar-text">{{ currentTime }}</span>
      </div>
      <!-- Слот для дополнительных элементов справа -->
      <slot name="right"></slot>
    </div>
  </div>

  <!-- Dropdown Menu - телепортирован в body -->
  <Teleport to="body">
    <div class="dropdown-menu" v-show="isMenuOpen" @click.stop>
      <div v-for="(item, index) in menuItems" :key="'menu-' + index" class="menu-item">
        <div v-if="item.action === 'separator'" class="menu-separator"></div>
        <div v-else class="menu-item-content" @click="handleMenuItem(item)">
          <span v-if="getItemCheckmark(item)" class="menu-checkmark">✓</span>
          <img v-if="item.icon" :src="getIconPath(item.icon)" :alt="item.name" class="menu-item-icon">
          <span class="menu-item-text">{{ getItemName(item) }}</span>
          <span v-if="item.hotkey" class="menu-item-hotkey">{{ item.hotkey }}</span>
          <div v-if="item.submenu" class="submenu-arrow">▶</div>
        </div>
        
        <!-- Submenu -->
        <div v-if="item.submenu" class="submenu submenu-left" v-show="openSubmenu === index">
          <div v-for="(subItem, subIndex) in item.submenu" :key="'submenu-' + subIndex" class="submenu-item">
            <div class="menu-item-content" @click="handleMenuItem(subItem)">
              <img v-if="subItem.icon" :src="getIconPath(subItem.icon)" :alt="subItem.name" class="menu-item-icon">
              <span class="menu-item-text">{{ subItem.name }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Teleport>

  <!-- Locale Menu - телепортирован в body -->
  <Teleport to="body">
    <div class="dropdown-menu locale-menu" v-show="isLocaleMenuOpen" @click.stop>
      <div v-for="loc in availableLocales" :key="loc.code" class="menu-item" @click="switchLocale(loc.code)">
        <div class="menu-item-content">
          <span class="menu-item-text">{{ getLocaleDisplayName(loc.code) }}</span>
        </div>
      </div>
    </div>
  </Teleport>

  <!-- Links Menu - телепортирован в body -->
  <Teleport to="body">
    <div class="dropdown-menu links-menu" v-show="isLinksMenuOpen" @click.stop>
      <div class="menu-item" @click="openLink('https://github.com/s00d/switchshuttle')">
        <div class="menu-item-content">
          <span class="menu-item-text">📥 Download</span>
        </div>
      </div>
      <div class="menu-item" @click="openLink('https://s00d.github.io/switchshuttle/')">
        <div class="menu-item-content">
          <span class="menu-item-text">📚 Documentation</span>
        </div>
      </div>
      <div class="menu-item" @click="openLink('https://github.com/s00d')">
        <div class="menu-item-content">
          <span class="menu-item-text">👨‍💻 Author</span>
        </div>
      </div>
      <div class="menu-item" @click="openLink('https://github.com/s00d/switchshuttle/issues')">
        <div class="menu-item-content">
          <span class="menu-item-text">🐛 Issues</span>
        </div>
      </div>
      <div class="menu-item" @click="openLink('https://github.com/s00d/switchshuttle/blob/main/LICENSE')">
        <div class="menu-item-content">
          <span class="menu-item-text">📄 License</span>
        </div>
      </div>
    </div>
  </Teleport>

  <!-- Click Hint - телепортирован в body -->
  <Teleport to="body">
    <div 
      class="click-hint-overlay" 
      v-show="props.showSwitchShuttleIcon && isClickHintVisible"
      :style="{
        top: clickHintPosition.top + 'px',
        left: clickHintPosition.left + 'px'
      }"
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
import { useI18n } from 'vue-i18n'
import { menuStructure } from '~/config/menu'

const emit = defineEmits(['showNotification', 'showHelp', 'showTerminal', 'editConfig', 'toggleItem', 'showHelpWindow', 'showAboutWindow', 'showHomepageWindow', 'showJsonEditorWindow', 'showConfigFolderWindow', 'hideMenuBar'])

// Props для управления видимостью иконки SwitchShuttle
const props = defineProps({
  showSwitchShuttleIcon: {
    type: Boolean,
    default: true
  }
})

// i18n
const { locale, locales, setLocale } = useI18n()

// Доступные локали (исключаем текущую)
const availableLocales = computed(() => {
  return locales.value.filter(loc => loc.code !== locale.value)
})

// Состояние
const isMenuOpen = ref(false)
const isLocaleMenuOpen = ref(false)
const isLinksMenuOpen = ref(false)
const openSubmenu = ref(null)
const currentTime = ref('')
const currentLocale = ref('🇺🇸 EN')

// Позиция подсказки клика
const clickHintPosition = ref({ top: 0, left: 0 })
const switchShuttleIconRef = ref(null)
const isClickHintVisible = ref(false)

// Динамические значения для мониторинга
const cpuUsage = ref('25')
const ramUsage = ref('1500 MB')

// Меню
const menuItems = ref(menuStructure)

// Состояние toggle элементов
const toggleStates = ref({
  '🔧 System Monitoring': true,
  'Launch at Login': false
})

// Функции
function toggleMenu() {
  isMenuOpen.value = !isMenuOpen.value
  openSubmenu.value = null
  isLocaleMenuOpen.value = false
}

function toggleLocaleMenu() {
  isLocaleMenuOpen.value = !isLocaleMenuOpen.value
  isMenuOpen.value = false
  isLinksMenuOpen.value = false
  openSubmenu.value = null
}

function toggleLinksMenu() {
  isLinksMenuOpen.value = !isLinksMenuOpen.value
  isMenuOpen.value = false
  isLocaleMenuOpen.value = false
  openSubmenu.value = null
}

function getLocaleDisplayName(localeCode) {
  const localeMap = {
    'en': { flag: '🇺🇸', name: 'English' },
    'ru': { flag: '🇷🇺', name: 'Русский' },
    'de': { flag: '🇩🇪', name: 'Deutsch' },
    'ja': { flag: '🇯🇵', name: '日本語' },
    'zh': { flag: '🇨🇳', name: '中文' }
  }
  
  const localeInfo = localeMap[localeCode]
  return localeInfo ? `${localeInfo.flag} ${localeInfo.name}` : localeCode
}

function setCurrentLocale(localeCode) {
  const localeMap = {
    'en': { flag: '🇺🇸', code: 'EN' },
    'ru': { flag: '🇷🇺', code: 'RU' },
    'de': { flag: '🇩🇪', code: 'DE' },
    'ja': { flag: '🇯🇵', code: 'JA' },
    'zh': { flag: '🇨🇳', code: 'ZH' }
  }
  
  const localeInfo = localeMap[localeCode]
  if (localeInfo) {
    currentLocale.value = `${localeInfo.flag} ${localeInfo.code}`
  }
}

function switchLocale(localeCode) {
  // Переключаем локаль через setLocale
  setLocale(localeCode)
  setCurrentLocale(localeCode)
  isLocaleMenuOpen.value = false
}

function openLink(url) {
  window.open(url, '_blank')
  isLinksMenuOpen.value = false
}

function getIconPath(icon) {
  // Получаем baseURL из конфигурации Nuxt
  const config = useRuntimeConfig()
  const baseURL = config.app.baseURL || ''
  
  // Если baseURL пустой или равен '/', используем относительный путь
  if (!baseURL || baseURL === '/') {
    return `/${icon}`
  }
  
  // Иначе добавляем baseURL
  return `${baseURL}/${icon}`
}

function getItemName(item) {
  if (item.name.includes('{usage}')) {
    let value = item.usage || '0'
    
    // Используем реактивные значения на клиенте
    if (process.client) {
      if (item.name.includes('CPU')) {
        value = cpuUsage.value
      } else if (item.name.includes('RAM')) {
        value = ramUsage.value
      }
    }
    
    return item.name.replace('{usage}', value)
  }
  return item.name
}

function getItemCheckmark(item) {
  if (item.action === 'toggle') {
    return toggleStates.value[item.name]
  }
  return item.checkmark
}

function handleMenuItem(item) {
  if (item.submenu) {
    const index = menuItems.value.indexOf(item)
    openSubmenu.value = openSubmenu.value === index ? null : index
    return
  }
  
  closeMenu()
  
  switch (item.action) {
    case 'separator':
      // Разделители не обрабатываются
      break
    case 'terminal':
      let output = item.terminalOutput || ''
      if (item.dynamicValue) {
        const dynamicValue = item.dynamicValue()
        output = output.replace(/{usage}/g, dynamicValue)
      }
      emit('showTerminal', item.command, output)
      break
    case 'notification':
      const title = item.title || item.name
      const message = item.message || ''
      emit('showNotification', title, message)
      break
    case 'help':
      emit('showHelp')
      break
    case 'showHelp':
      emit('showHelpWindow')
      break
    case 'showAbout':
      emit('showAboutWindow')
      break
    case 'showHomepage':
      emit('showHomepageWindow')
      break
    case 'showJsonEditor':
      emit('showJsonEditorWindow', item.config)
      break
    case 'showConfigFolder':
      emit('showConfigFolderWindow')
      break
    case 'refresh':
      // Просто закрываем меню без дополнительных действий
      break
    case 'config':
      emit('editConfig', item.config)
      break
    case 'toggle':
      toggleStates.value[item.name] = !toggleStates.value[item.name]
      emit('toggleItem', item.name)
      break
    case 'hideMenuBar':
      emit('hideMenuBar')
      break
  }
}

function closeMenu() {
  isMenuOpen.value = false
  isLocaleMenuOpen.value = false
  isLinksMenuOpen.value = false
  openSubmenu.value = null
}

function updateTime() {
  const now = new Date()
  currentTime.value = now.toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit'
  })
}

function updateClickHintPosition() {
  if (switchShuttleIconRef.value && props.showSwitchShuttleIcon) {
    const rect = switchShuttleIconRef.value.getBoundingClientRect()
    
    if (rect.width > 0 && rect.height > 0) {
      // Сначала устанавливаем позицию без учета размера подсказки
      clickHintPosition.value = {
        top: rect.bottom + 8,
        left: rect.left + (rect.width / 2)
      }
      
      // Показываем подсказку
      isClickHintVisible.value = true
      
      // После показа получаем реальные размеры и корректируем позицию
      setTimeout(() => {
        const hintElement = document.querySelector('.click-hint')
        if (hintElement) {
          const hintWidth = hintElement.offsetWidth
          clickHintPosition.value = {
            top: rect.bottom + 8,
            left: rect.left + (rect.width / 2) - (hintWidth / 2)
          }
        }
      }, 50)
    }
  }
}

// Следим за изменениями локали
watch(locale, (newLocale) => {
  setCurrentLocale(newLocale)
})

// Следим за изменением видимости иконки
watch(() => props.showSwitchShuttleIcon, (newValue) => {
  if (newValue) {
    // Скрываем подсказку до правильного позиционирования
    isClickHintVisible.value = false
    // Обновляем позицию с задержкой
    setTimeout(updateClickHintPosition, 100)
  } else {
    // Скрываем подсказку при скрытии иконки
    isClickHintVisible.value = false
  }
})

// Lifecycle
onMounted(() => {
  updateTime()
  
  // Устанавливаем текущую локаль на основе i18n
  setCurrentLocale(locale.value)
  
  // Скрываем подсказку до правильного позиционирования
  isClickHintVisible.value = false
  // Обновляем позицию подсказки с задержкой
  setTimeout(updateClickHintPosition, 100)
  
  const timeInterval = setInterval(updateTime, 1000)
  
  // Обновляем динамические значения каждую секунду
  const dynamicInterval = setInterval(() => {
    // Обновляем значения мониторинга
    cpuUsage.value = `${Math.floor(Math.random() * 30 + 10)}`
    ramUsage.value = `${Math.floor(Math.random() * 2000 + 500)} MB`
  }, 1000)
  
  // Обновляем позицию подсказки при изменении размера окна
  window.addEventListener('resize', () => {
    if (props.showSwitchShuttleIcon && isClickHintVisible.value) {
      updateClickHintPosition()
    }
  })
  
  document.addEventListener('click', (e) => {
    if (!e.target.closest('.dropdown-menu') && !e.target.closest('.locale-menu') && !e.target.closest('.links-menu') && !e.target.closest('.menu-bar-right') && !e.target.closest('.menu-bar-left')) {
      closeMenu()
    }
  })
  
  onUnmounted(() => {
    clearInterval(timeInterval)
    clearInterval(dynamicInterval)
    window.removeEventListener('resize', updateClickHintPosition)
  })
})
</script>

<style scoped>
/* Стили уже определены в main.css */
</style> 