<template>
  <div class="download-window">
    <!-- Header Section -->
    <div class="download-header">
      <div class="header-content">
        <div class="logo-section">
          <div class="logo-icon">
            <img src="/switchshuttle.svg" alt="SwitchShuttle" />
          </div>
          <div class="logo-text">
            <h1>{{ $t('download.title') }}</h1>
            <div class="version-badge">
              <span class="version-label">{{ $t('download.latestVersion') }}</span>
              <span class="version-number">{{ latestVersion }}</span>
            </div>
          </div>
        </div>
        <div class="release-info">
          <span class="release-date">{{ $t('download.releaseDate') }}: {{ releaseDate }}</span>
        </div>
      </div>
    </div>

    <div class="download-content">
      <!-- Loading State -->
      <div v-if="isLoading" class="loading-state">
        <div class="loading-animation">
          <div class="loading-spinner"></div>
          <div class="loading-dots">
            <span></span>
            <span></span>
            <span></span>
          </div>
        </div>
        <p class="loading-text">{{ $t('download.loading') }}</p>
      </div>
      
      <!-- Main Download Section -->
      <div v-if="!isLoading" class="main-download-section">
        <div class="section-header">
          <h2>{{ $t('download.downloadForCurrentOS') }}</h2>
          <p class="section-description">{{ $t('download.recommendedForYourSystem') }}</p>
        </div>
        
        <div class="primary-download-card">
          <div class="card-content">
            <div class="platform-badge">
              <span class="platform-icon">{{ currentOSIcon }}</span>
              <span class="platform-name">{{ currentOSName }}</span>
            </div>
            <div class="file-details">
              <div class="file-info">
                <span class="file-size">{{ getCurrentOSFile()?.size }}</span>
                <span class="file-type">{{ $t('download.universalVersion') }}</span>
              </div>
            </div>
            <button class="download-button primary" @click="showInstructionsForOS(currentOS)">
              <span class="button-icon">⬇️</span>
              <span class="button-text">{{ $t('download.download') }}</span>
            </button>
          </div>
        </div>
      </div>

      <!-- Other Platforms Section -->
      <div v-if="!isLoading" class="other-platforms-section">
        <div class="section-header">
          <h2>{{ $t('download.downloadForOtherOS') }}</h2>
          <p class="section-description">{{ $t('download.availableForOtherPlatforms') }}</p>
        </div>
        
        <div class="platforms-grid">
          <!-- Windows -->
          <div class="platform-card" v-if="currentOS !== 'windows'">
            <div class="card-header">
              <div class="platform-badge">
                <span class="platform-icon">🪟</span>
                <span class="platform-name">{{ $t('download.platforms.windows') }}</span>
              </div>
            </div>
            <div class="card-content">
              <div class="download-options">
                <div class="download-option" v-if="getFileInfo('switch-shuttle_' + latestVersion + '_x64-setup.exe')">
                  <div class="option-info">
                    <span class="option-name">{{ $t('download.fileTypes.x64Setup') }}</span>
                    <span class="option-size">{{ getFileInfo('switch-shuttle_' + latestVersion + '_x64-setup.exe')?.size }}</span>
                  </div>
                  <button class="download-button secondary" @click="showInstructionsForOS('windows')">
                    {{ $t('download.download') }}
                  </button>
                </div>
                <div class="download-option" v-if="getFileInfo('switch-shuttle_' + latestVersion + '_x86-setup.exe')">
                  <div class="option-info">
                    <span class="option-name">{{ $t('download.fileTypes.x86Setup') }}</span>
                    <span class="option-size">{{ getFileInfo('switch-shuttle_' + latestVersion + '_x86-setup.exe')?.size }}</span>
                  </div>
                  <button class="download-button secondary" @click="showInstructionsForOS('windows')">
                    {{ $t('download.download') }}
                  </button>
                </div>
                <div class="download-option" v-if="getFileInfo('switch-shuttle_' + latestVersion + '_x64_en-US.msi')">
                  <div class="option-info">
                    <span class="option-name">{{ $t('download.fileTypes.x64Msi') }}</span>
                    <span class="option-size">{{ getFileInfo('switch-shuttle_' + latestVersion + '_x64_en-US.msi')?.size }}</span>
                  </div>
                  <button class="download-button secondary" @click="showInstructionsForOS('windows')">
                    {{ $t('download.download') }}
                  </button>
                </div>
                <div class="download-option" v-if="getFileInfo('switch-shuttle_' + latestVersion + '_x86_en-US.msi')">
                  <div class="option-info">
                    <span class="option-name">{{ $t('download.fileTypes.x86Msi') }}</span>
                    <span class="option-size">{{ getFileInfo('switch-shuttle_' + latestVersion + '_x86_en-US.msi')?.size }}</span>
                  </div>
                  <button class="download-button secondary" @click="showInstructionsForOS('windows')">
                    {{ $t('download.download') }}
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- Linux -->
          <div class="platform-card" v-if="currentOS !== 'linux'">
            <div class="card-header">
              <div class="platform-badge">
                <span class="platform-icon">🐧</span>
                <span class="platform-name">{{ $t('download.platforms.linux') }}</span>
              </div>
            </div>
            <div class="card-content">
              <div class="download-options">
                <div class="download-option" v-if="getFileInfo('switch-shuttle_' + latestVersion + '_amd64.AppImage')">
                  <div class="option-info">
                    <span class="option-name">{{ $t('download.fileTypes.appImage') }}</span>
                    <span class="option-size">{{ getFileInfo('switch-shuttle_' + latestVersion + '_amd64.AppImage')?.size }}</span>
                  </div>
                  <button class="download-button secondary" @click="showInstructionsForOS('linux')">
                    {{ $t('download.download') }}
                  </button>
                </div>
                <div class="download-option" v-if="getFileInfo('switch-shuttle_' + latestVersion + '_amd64.deb')">
                  <div class="option-info">
                    <span class="option-name">{{ $t('download.fileTypes.deb') }}</span>
                    <span class="option-size">{{ getFileInfo('switch-shuttle_' + latestVersion + '_amd64.deb')?.size }}</span>
                  </div>
                  <button class="download-button secondary" @click="showInstructionsForOS('linux')">
                    {{ $t('download.download') }}
                  </button>
                </div>
                <div class="download-option" v-if="getFileInfo('switch-shuttle-' + latestVersion + '-1.x86_64.rpm')">
                  <div class="option-info">
                    <span class="option-name">{{ $t('download.fileTypes.rpm') }}</span>
                    <span class="option-size">{{ getFileInfo('switch-shuttle-' + latestVersion + '-1.x86_64.rpm')?.size }}</span>
                  </div>
                  <button class="download-button secondary" @click="showInstructionsForOS('linux')">
                    {{ $t('download.download') }}
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- macOS -->
          <div class="platform-card" v-if="currentOS !== 'macos'">
            <div class="card-header">
              <div class="platform-badge">
                <span class="platform-icon">🍎</span>
                <span class="platform-name">{{ $t('download.platforms.macos') }}</span>
              </div>
            </div>
            <div class="card-content">
              <div class="download-options">
                <div class="download-option" v-if="getFileInfo('switch-shuttle_' + latestVersion + '_universal.dmg')">
                  <div class="option-info">
                    <span class="option-name">{{ $t('download.fileTypes.dmg') }}</span>
                    <span class="option-size">{{ getFileInfo('switch-shuttle_' + latestVersion + '_universal.dmg')?.size }}</span>
                  </div>
                  <button class="download-button secondary" @click="showInstructionsForOS('macos')">
                    {{ $t('download.download') }}
                  </button>
                </div>
                <div class="download-option" v-if="getFileInfo('switch-shuttle_universal.app.tar.gz')">
                  <div class="option-info">
                    <span class="option-name">{{ $t('download.fileTypes.tarGz') }}</span>
                    <span class="option-size">{{ getFileInfo('switch-shuttle_universal.app.tar.gz')?.size }}</span>
                  </div>
                  <button class="download-button secondary" @click="showInstructionsForOS('macos')">
                    {{ $t('download.download') }}
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Instructions Modal -->
      <div v-if="showInstructions" class="instructions-modal">
        <div class="modal-overlay" @click="closeInstructions"></div>
        <div class="modal-content">
          <div class="modal-header">
            <h2>{{ $t('download.installationInstructions') }}</h2>
            <button class="close-button" @click="closeInstructions">×</button>
          </div>
          
          <div class="modal-body">
            <!-- macOS Instructions -->
            <div v-if="selectedOS === 'macos'" class="instruction-panel">
              <div class="instruction-header">
                <h3>{{ $t('download.macosInstructions.title') }}</h3>
                <div class="difficulty-badge easy">{{ $t('download.difficulty.easy') }}</div>
              </div>
              <div class="instruction-steps">
                <div class="step">
                  <div class="step-number">1</div>
                  <div class="step-content">
                    <h4>{{ $t('download.macosInstructions.step1') }}</h4>
                    <p>{{ $t('download.macosInstructions.step1Description') }}</p>
                  </div>
                </div>
                <div class="step">
                  <div class="step-number">2</div>
                  <div class="step-content">
                    <h4>{{ $t('download.macosInstructions.step2') }}</h4>
                    <p>{{ $t('download.macosInstructions.step2Description') }}</p>
                  </div>
                </div>
                <div class="step">
                  <div class="step-number">3</div>
                  <div class="step-content">
                    <h4>{{ $t('download.macosInstructions.step3') }}</h4>
                    <p>{{ $t('download.macosInstructions.step3Description') }}</p>
                    <div class="code-block">
                      <div class="code-header">
                        <span class="code-label">{{ $t('download.terminalCommands') }}</span>
                        <button class="copy-button" @click="copyToClipboard($t('download.macosInstructions.terminalCommands'))">
                          <span class="copy-icon">📋</span>
                          <span class="copy-text">{{ $t('download.copy') }}</span>
                        </button>
                      </div>
                      <pre class="code-content"><code>{{ $t('download.macosInstructions.terminalCommands') }}</code></pre>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Windows Instructions -->
            <div v-if="selectedOS === 'windows'" class="instruction-panel">
              <div class="instruction-header">
                <h3>{{ $t('download.windowsInstructions.title') }}</h3>
                <div class="difficulty-badge easy">{{ $t('download.difficulty.easy') }}</div>
              </div>
              <div class="instruction-steps">
                <div class="step">
                  <div class="step-number">1</div>
                  <div class="step-content">
                    <h4>{{ $t('download.windowsInstructions.step1') }}</h4>
                    <p>{{ $t('download.windowsInstructions.step1Description') }}</p>
                  </div>
                </div>
                <div class="step">
                  <div class="step-number">2</div>
                  <div class="step-content">
                    <h4>{{ $t('download.windowsInstructions.step2') }}</h4>
                    <p>{{ $t('download.windowsInstructions.step2Description') }}</p>
                  </div>
                </div>
                <div class="step">
                  <div class="step-number">3</div>
                  <div class="step-content">
                    <h4>{{ $t('download.windowsInstructions.step3') }}</h4>
                    <p>{{ $t('download.windowsInstructions.step3Description') }}</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- Linux Instructions -->
            <div v-if="selectedOS === 'linux'" class="instruction-panel">
              <div class="instruction-header">
                <h3>{{ $t('download.linuxInstructions.title') }}</h3>
                <div class="difficulty-badge medium">{{ $t('download.difficulty.medium') }}</div>
              </div>
              <div class="instruction-steps">
                <div class="step">
                  <div class="step-number">1</div>
                  <div class="step-content">
                    <h4>{{ $t('download.linuxInstructions.step1') }}</h4>
                    <p>{{ $t('download.linuxInstructions.step1Description') }}</p>
                  </div>
                </div>
                <div class="step">
                  <div class="step-number">2</div>
                  <div class="step-content">
                    <h4>{{ $t('download.linuxInstructions.step2') }}</h4>
                    <p>{{ $t('download.linuxInstructions.step2Description') }}</p>
                  </div>
                </div>
                <div class="step">
                  <div class="step-number">3</div>
                  <div class="step-content">
                    <h4>{{ $t('download.linuxInstructions.step3') }}</h4>
                    <p>{{ $t('download.linuxInstructions.step3Description') }}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <div class="modal-footer">
            <button class="download-button primary" @click="downloadForSelectedOS">
              <span class="button-icon">⬇️</span>
              <span class="button-text">{{ $t('download.downloadNow') }}</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useNotifications } from '~/composables/useNotifications'

const { t } = useI18n()

// Состояние
const latestVersion = ref('1.12.0')
const releaseDate = ref('')
const showInstructions = ref(false)
const selectedOS = ref('')

// Определение текущей ОС
const currentOS = ref('macos') // По умолчанию macOS, можно определить динамически
const currentOSIcon = ref('🍎')
const currentOSName = ref('macOS')

interface FileInfo {
  size: string
  downloadUrl: string
}

interface GitHubAsset {
  name: string
  size: number
  browser_download_url: string
}

interface GitHubRelease {
  tag_name: string
  published_at: string
  assets: GitHubAsset[]
}

// Состояние для файловой информации
const fileInfo = ref<Record<string, FileInfo>>({})
const isLoading = ref(true)

// Функции
function getFileInfo(filename: string): FileInfo | null {
  return fileInfo.value[filename] || null
}

// Получение файла для текущей ОС
function getCurrentOSFile(): FileInfo | null {
  switch (currentOS.value) {
    case 'macos':
      return getFileInfo(`switch-shuttle_${latestVersion.value}_universal.dmg`)
    case 'windows':
      return getFileInfo(`switch-shuttle_${latestVersion.value}_x64-setup.exe`)
    case 'linux':
      return getFileInfo(`switch-shuttle_${latestVersion.value}_amd64.AppImage`)
    default:
      return getFileInfo(`switch-shuttle_${latestVersion.value}_universal.dmg`)
  }
}

// Показать инструкции для выбранной ОС
function showInstructionsForOS(os: string) {
  selectedOS.value = os
  showInstructions.value = true
}

// Закрыть модальное окно с инструкциями
function closeInstructions() {
  showInstructions.value = false
  selectedOS.value = ''
}

// Скачивание для выбранной ОС
function downloadForSelectedOS() {
  if (selectedOS.value === 'windows') {
    downloadFile(`switch-shuttle_${latestVersion.value}_x64-setup.exe`)
  } else if (selectedOS.value === 'macos') {
    downloadFile(`switch-shuttle_${latestVersion.value}_universal.dmg`)
  } else if (selectedOS.value === 'linux') {
    downloadFile(`switch-shuttle_${latestVersion.value}_amd64.AppImage`)
  }
  closeInstructions()
}

// Копирование в буфер обмена
async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text)
    
    // Показываем уведомление об успешном копировании
    const { addNotification } = useNotifications()
    addNotification({
      title: t('download.copied'),
      message: t('download.copiedMessage'),
      type: 'success'
    })
    
    console.log('Text copied to clipboard:', text)
  } catch (error) {
    console.error('Failed to copy to clipboard:', error)
    
    // Fallback для старых браузеров
    try {
      const textArea = document.createElement('textarea')
      textArea.value = text
      document.body.appendChild(textArea)
      textArea.select()
      document.execCommand('copy')
      document.body.removeChild(textArea)
      
      const { addNotification } = useNotifications()
      addNotification({
        title: t('download.copied'),
        message: t('download.copiedMessage'),
        type: 'success'
      })
    } catch (fallbackError) {
      console.error('Fallback copy failed:', fallbackError)
      
      const { addNotification } = useNotifications()
      addNotification({
        title: 'Copy Error',
        message: 'Failed to copy to clipboard. Please copy manually.',
        type: 'error'
      })
    }
  }
}

function downloadFile(filename: string) {
  const fileData = fileInfo.value[filename]
  if (!fileData) {
    console.warn(`File info not found for ${filename}`)
    return
  }
  
  try {
    // Создаем временную ссылку для скачивания
    const link = document.createElement('a')
    link.href = fileData.downloadUrl
    link.download = filename
    link.target = '_blank'
    link.rel = 'noopener noreferrer'
    
    // Добавляем ссылку в DOM
    document.body.appendChild(link)
    
    // Симулируем клик
    link.click()
    
    // Удаляем ссылку из DOM
    document.body.removeChild(link)
    
    // Показываем уведомление
    const { addNotification } = useNotifications()
    addNotification({
      title: t('download.downloadStarted'),
      message: t('download.downloadStartedMessage', { filename }),
      type: 'success'
    })
    
    console.log(`Download started for ${filename}: ${fileData.downloadUrl}`)
  } catch (error) {
    console.error('Download failed:', error)
    
    // Показываем уведомление об ошибке
    const { addNotification } = useNotifications()
    addNotification({
      title: 'Download Error',
      message: `Failed to download ${filename}. Please try again.`,
      type: 'error'
    })
  }
}

// Получение данных о релизе с GitHub
async function fetchReleaseData() {
  try {
    isLoading.value = true
    
    console.log('Fetching release data from GitHub...')
    
    // Получаем последний релиз
    const response = await fetch('https://api.github.com/repos/s00d/switchshuttle/releases/latest')
    
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }
    
    const release: GitHubRelease = await response.json()
    
    console.log('Release data received:', release)
    
    // Обновляем версию и дату
    latestVersion.value = release.tag_name.replace('app-v', '')
    releaseDate.value = new Date(release.published_at).toLocaleDateString()
    
    // Обрабатываем ассеты
    const newFileInfo: Record<string, FileInfo> = {}
    
    release.assets.forEach(asset => {
      const filename = asset.name
      const sizeInMB = (asset.size / (1024 * 1024)).toFixed(1)
      const size = `${sizeInMB} MB`
      
      // Пропускаем файлы подписей (.sig) и другие служебные файлы
      if (!filename.endsWith('.sig') && !filename.includes('latest.json')) {
        newFileInfo[filename] = {
          size,
          downloadUrl: asset.browser_download_url
        }
        console.log(`Added file info for ${filename}:`, newFileInfo[filename])
      }
    })
    
    fileInfo.value = newFileInfo
    console.log('Final file info:', fileInfo.value)
    
  } catch (error) {
    console.error('Failed to fetch release data:', error)
  } finally {
    isLoading.value = false
  }
}

// Lifecycle
onMounted(() => {
  fetchReleaseData()
})
</script>

<style lang="scss" scoped>
.download-window {
  width: 100%;
  height: 100%;
  background: #f5f5f7;
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Helvetica Neue', sans-serif;
  overflow-y: auto;
  color: #1d1d1f;
}

// Header Section
.download-header {
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  padding: 32px 24px;
  
  .header-content {
    max-width: 1200px;
    margin: 0 auto;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .logo-section {
    display: flex;
    align-items: center;
    
    .logo-icon {
      width: 80px;
      height: 80px;
      display: flex;
      align-items: center;
      justify-content: center;
      margin-right: 24px;
      
      img {
        width: 100%;
        height: 100%;
        object-fit: contain;
      }
    }
    
    .logo-text {
      h1 {
        font-size: 32px;
        font-weight: 600;
        color: #1d1d1f;
        margin: 0 0 8px 0;
        letter-spacing: -0.3px;
      }
      
      .version-badge {
        display: flex;
        align-items: center;
        gap: 8px;
        
        .version-label {
          font-size: 14px;
          color: #86868b;
          font-weight: 400;
        }
        
        .version-number {
          background: #007AFF;
          color: white;
          padding: 4px 12px;
          border-radius: 12px;
          font-size: 12px;
          font-weight: 600;
        }
      }
    }
  }
  
  .release-info {
    .release-date {
      font-size: 14px;
      color: #86868b;
      font-weight: 400;
    }
  }
}

// Content
.download-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 32px 24px;
}

// Loading State
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  
  .loading-animation {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
  }
  
  .loading-spinner {
    width: 32px;
    height: 32px;
    border: 2px solid #e5e5e7;
    border-top: 2px solid #007AFF;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  
  .loading-dots {
    display: flex;
    gap: 6px;
    
    span {
      width: 6px;
      height: 6px;
      background: #007AFF;
      border-radius: 50%;
      animation: dots 1.4s ease-in-out infinite both;
      
      &:nth-child(1) { animation-delay: -0.32s; }
      &:nth-child(2) { animation-delay: -0.16s; }
      &:nth-child(3) { animation-delay: 0s; }
    }
  }
  
  .loading-text {
    font-size: 16px;
    color: #86868b;
    font-weight: 400;
    margin-top: 20px;
  }
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

@keyframes dots {
  0%, 80%, 100% {
    transform: scale(0);
    opacity: 0.5;
  }
  40% {
    transform: scale(1);
    opacity: 1;
  }
}

// Section Headers
.section-header {
  margin-bottom: 32px;
  
  h2 {
    font-size: 24px;
    font-weight: 600;
    color: #1d1d1f;
    margin: 0 0 8px 0;
  }
  
  .section-description {
    font-size: 16px;
    color: #86868b;
    margin: 0;
  }
}

// Main Download Section
.main-download-section {
  margin-bottom: 48px;
}

.primary-download-card {
  background: white;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
  
  &:hover {
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.06);
  }
  
  .card-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 24px;
  }
  
  .platform-badge {
    display: flex;
    align-items: center;
    gap: 12px;
    
    .platform-icon {
      font-size: 28px;
    }
    
    .platform-name {
      font-size: 18px;
      font-weight: 600;
      color: #1d1d1f;
    }
  }
  
  .file-details {
    flex: 1;
    
    .file-info {
      display: flex;
      flex-direction: column;
      gap: 4px;
      
      .file-size {
        font-size: 16px;
        font-weight: 600;
        color: #1d1d1f;
      }
      
      .file-type {
        font-size: 14px;
        color: #86868b;
      }
    }
  }
}

// Download Buttons
.download-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  text-decoration: none;
  
  &.primary {
    background: #007AFF;
    color: white;
    
    &:hover {
      background: #0056CC;
    }
    
    &:active {
      background: #004499;
    }
  }
  
  &.secondary {
    background: #f5f5f7;
    color: #1d1d1f;
    border: 1px solid rgba(0, 0, 0, 0.1);
    
    &:hover {
      background: #e5e5e7;
    }
    
    &:active {
      background: #d1d1d6;
    }
  }
  
  .button-icon {
    font-size: 16px;
  }
  
  .button-text {
    font-weight: 500;
  }
}

// Other Platforms Section
.other-platforms-section {
  margin-bottom: 48px;
}

.platforms-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: 24px;
}

.platform-card {
  background: white;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
  
  &:hover {
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.06);
  }
  
  .card-header {
    padding: 20px 20px 16px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
    
    .platform-badge {
      display: flex;
      align-items: center;
      gap: 12px;
      
      .platform-icon {
        font-size: 20px;
      }
      
      .platform-name {
        font-size: 16px;
        font-weight: 600;
        color: #1d1d1f;
      }
    }
  }
  
  .card-content {
    padding: 16px 20px 20px;
  }
}

.download-options {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.download-option {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  background: #f5f5f7;
  border-radius: 8px;
  transition: all 0.2s ease;
  
  &:hover {
    background: #e5e5e7;
  }
  
  .option-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
    
    .option-name {
      font-size: 14px;
      font-weight: 500;
      color: #1d1d1f;
    }
    
    .option-size {
      font-size: 12px;
      color: #86868b;
    }
  }
}

// Instructions Section
.instructions-section {
  margin-top: 48px;
}

.instructions-container {
  background: white;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(0, 0, 0, 0.1);
}

.instructions-tabs {
  display: flex;
  background: #f5f5f7;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  
  .tab-button {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 16px;
    background: none;
    border: none;
    font-size: 14px;
    font-weight: 500;
    color: #86868b;
    cursor: pointer;
    transition: all 0.2s ease;
    position: relative;
    
    &:hover {
      background: rgba(0, 0, 0, 0.02);
      color: #1d1d1f;
    }
    
    &.active {
      color: #007AFF;
      background: white;
      
      &::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        height: 2px;
        background: #007AFF;
      }
    }
    
    .tab-icon {
      font-size: 16px;
    }
    
    .tab-text {
      font-weight: 500;
    }
  }
}

.instructions-content {
  padding: 32px;
}

.instruction-panel {
  .instruction-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
    
    h3 {
      font-size: 16px;
      font-weight: 700;
      color: #1d1d1f;
      margin: 0;
    }
    
    .difficulty-badge {
      padding: 3px 10px;
      border-radius: 12px;
      font-size: 10px;
      font-weight: 600;
      
      &.easy {
        background: #e8f5e8;
        color: #2d5a2d;
      }
      
      &.medium {
        background: #fff3cd;
        color: #856404;
      }
    }
  }
}

.instruction-steps {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.step {
  display: flex;
  gap: 10px;
  
  .step-number {
    width: 24px;
    height: 24px;
    background: #007AFF;
    color: white;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    font-weight: 600;
    flex-shrink: 0;
  }
  
  .step-content {
    flex: 1;
    
    h4 {
      font-size: 14px;
      font-weight: 600;
      color: #1d1d1f;
      margin: 0 0 4px 0;
    }
    
    p {
      font-size: 12px;
      color: #86868b;
      line-height: 1.3;
      margin: 0 0 8px 0;
    }
  }
}

.code-block {
  background: #f5f5f7;
  border-radius: 6px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  overflow: hidden;
  
  .code-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    background: rgba(0, 0, 0, 0.05);
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    
    .code-label {
      font-size: 11px;
      font-weight: 500;
      color: #86868b;
      text-transform: uppercase;
      letter-spacing: 0.5px;
    }
    
    .copy-button {
      display: flex;
      align-items: center;
      gap: 4px;
      padding: 4px 8px;
      background: rgba(0, 122, 255, 0.1);
      border: none;
      border-radius: 4px;
      font-size: 10px;
      font-weight: 500;
      color: #007AFF;
      cursor: pointer;
      transition: all 0.2s ease;
      
      &:hover {
        background: rgba(0, 122, 255, 0.2);
      }
      
      .copy-icon {
        font-size: 10px;
      }
      
      .copy-text {
        font-size: 10px;
      }
    }
  }
  
  .code-content {
    padding: 8px 12px;
    margin: 0;
    
    code {
      font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
      font-size: 11px;
      color: #1d1d1f;
      line-height: 1.3;
      white-space: pre-wrap;
      word-break: break-all;
    }
  }
}

// Instructions Modal
.instructions-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  
  .modal-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(8px);
  }
  
  .modal-content {
    position: relative;
    background: white;
    border-radius: 12px;
    max-width: 550px;
    width: 100%;
    max-height: 72vh;
    overflow: hidden;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
    border: 1px solid rgba(0, 0, 0, 0.1);
    display: flex;
    flex-direction: column;
  }
  
  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 16px 12px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    
    h2 {
      font-size: 16px;
      font-weight: 600;
      color: #1d1d1f;
      margin: 0;
    }
    
    .close-button {
      width: 24px;
      height: 24px;
      border: none;
      background: #f5f5f7;
      border-radius: 50%;
      font-size: 14px;
      color: #86868b;
      cursor: pointer;
      transition: all 0.2s ease;
      display: flex;
      align-items: center;
      justify-content: center;
      
      &:hover {
        background: #e5e5e7;
        color: #1d1d1f;
      }
    }
  }
  
  .modal-body {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
  }
  
  .modal-footer {
    padding: 12px 16px 16px;
    border-top: 1px solid rgba(0, 0, 0, 0.1);
    display: flex;
    justify-content: center;
  }
}

// Responsive Design
@media (max-width: 768px) {
  .download-header {
    padding: 32px 16px;
    
    .header-content {
      flex-direction: column;
      gap: 20px;
      text-align: center;
    }
    
    .logo-section {
      .logo-icon {
        width: 60px;
        height: 60px;
        margin-right: 0;
        margin-bottom: 16px;
      }
      
      .logo-text h1 {
        font-size: 32px;
      }
    }
  }
  
  .download-content {
    padding: 24px 16px;
  }
  
  .primary-download-card .card-content {
    flex-direction: column;
    text-align: center;
  }
  
  .platforms-grid {
    grid-template-columns: 1fr;
  }
  
  .instructions-modal {
    padding: 8px;
    
    .modal-content {
      max-height: 75vh;
      max-width: 480px;
    }
    
    .modal-header {
      padding: 12px 12px 8px;
      
      h2 {
        font-size: 14px;
      }
    }
    
    .modal-body {
      padding: 12px;
    }
    
    .modal-footer {
      padding: 8px 12px 12px;
    }
  }
  
  .instructions-tabs {
    flex-direction: column;
    
    .tab-button {
      padding: 16px;
    }
  }
  
  .instructions-content {
    padding: 24px;
  }
  
  .step {
    flex-direction: column;
    text-align: center;
    
    .step-number {
      align-self: center;
    }
  }
}

@media (max-width: 480px) {
  .download-header {
    padding: 24px 12px;
    
    .logo-section {
      .logo-icon {
        width: 50px;
        height: 50px;
      }
      
      .logo-text h1 {
        font-size: 28px;
      }
    }
  }
  
  .download-content {
    padding: 20px 12px;
  }
  
  .section-header h2 {
    font-size: 24px;
  }
  
  .primary-download-card {
    padding: 24px;
  }
  
  .download-button {
    padding: 14px 24px;
    font-size: 14px;
  }
  
  .instructions-modal {
    padding: 12px;
    
    .modal-header {
      padding: 20px 20px 16px;
      
      h2 {
        font-size: 18px;
      }
    }
    
    .modal-body {
      padding: 20px;
    }
    
    .modal-footer {
      padding: 16px 20px 20px;
    }
  }
}
</style> 