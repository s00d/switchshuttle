<template>
  <Window :title="`${$t('readme.title')} — SwitchShuttle`" :initial-x="800" :initial-y="150" :z="1200" :closable="true" :width="800" :height="500" @close="$emit('close')">
    <template #titlebar>
      <div class="window-title">{{ $t('readme.title') }} — SwitchShuttle</div>
    </template>
    <template #titlebar-right>
      <div class="window-menu-icon" @click="toggleToc" title="Toggle Table of Contents">
        <span class="menu-icon">📋</span>
      </div>
    </template>
    <div class="readme-container">
      <div v-if="isDataLoaded" class="readme-content" ref="contentRef">
        <div class="markdown-content" v-html="content"></div>
      </div>
      <div v-else class="loading">
        <div class="spinner"></div>
        <p>{{ $t('readme.loading') }}</p>
      </div>
      <div v-if="showToc && toc.length > 0 && isDataLoaded" class="toc-sidebar">
        <div class="toc-header">
          <h3>{{ $t('readme.toc.title') }}</h3>
        </div>
        <div class="toc-content">
          <ul class="toc-list">
            <li v-for="(item, index) in toc" :key="index" class="toc-item">
              <a 
                :href="`#${item.id}`" 
                @click.prevent="scrollToSection(item.id)"
                :class="{ 'toc-active': activeSection === item.id }"
              >
                {{ item.title }}
              </a>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </Window>
</template>

<script setup lang="ts">
import { ref, nextTick, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import Window from './Window.vue'

// Types
interface ReadmeResponse {
  success: boolean
  content?: string
  toc?: Array<{id: string, title: string, level: number}>
  error?: string
  locale?: string
  fallback?: boolean
}

// Props
interface Props {
  data?: ReadmeResponse | null
}

const props = withDefaults(defineProps<Props>(), {
  data: undefined
})

const emit = defineEmits(['close'])

const { t } = useI18n()

// State
const showToc = ref(false)
const activeSection = ref('')
const contentRef = ref<HTMLElement>()

// Computed
const content = computed(() => props.data?.content || '')
const toc = computed(() => props.data?.toc || [])

// Проверяем что данные загружены
const isDataLoaded = computed(() => props.data && props.data.success)

function toggleToc() {
  showToc.value = !showToc.value
}
function scrollToSection(id: string) {
  const element = document.getElementById(id)
  if (element) {
    element.scrollIntoView({ behavior: 'smooth' })
    activeSection.value = id
  }
}
onMounted(() => {
  nextTick(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach(entry => {
          if (entry.isIntersecting) {
            activeSection.value = entry.target.id
          }
        })
      },
      { threshold: 0.5 }
    )
    const headings = contentRef.value?.querySelectorAll('h1, h2, h3, h4, h5, h6')
    headings?.forEach(heading => observer.observe(heading))
  })
})
</script>

<style lang="scss">
:root {
  --primary: #3b82f6;
  --primary-dark: #2563eb;
  --primary-light: #60a5fa;
  --secondary: #06b6d4;
  --accent: #ef4444;
  --success: #10b981;
  --warning: #f59e0b;
  --error: #ef4444;
  --bg-primary: #fff;
  --bg-secondary: #f8fafc;
  --bg-tertiary: #f1f5f9;
  --bg-dark: #0f172a;
  --bg-dark-secondary: #1e293b;
  --text-primary: #0f172a;
  --text-secondary: #475569;
  --text-muted: #64748b;
  --text-dark: #fff;
  --border-light: #e2e8f0;
  --border-medium: #cbd5e1;
  --border-dark: #94a3b8;
  --shadow-lg: 0 10px 15px -3px rgba(0,0,0,0.1);
  --shadow-md: 0 4px 6px -1px rgba(0,0,0,0.1);
  --shadow-sm: 0 1px 2px 0 rgba(0,0,0,0.05);
  --radius: 8px;
  --space-xs: 0.25rem;
  --space-sm: 0.5rem;
  --space-md: 1rem;
  --space-lg: 1.5rem;
  --space-xl: 2rem;
  --space-2xl: 3rem;
  --transition-fast: 150ms ease;
  --transition-slow: 350ms ease;
}

.readme-container {
  display: flex;
  height: 100%;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', 'Monaco', monospace;
  font-size: 14px;
  line-height: 1.6;
}

.readme-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-lg);
  border-right: 1px solid var(--border-light);
}

.window-menu-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  cursor: pointer;
  border-radius: 4px;
  transition: all var(--transition-fast);
  
  &:hover {
    background: var(--bg-tertiary);
  }
  
  .menu-icon {
    font-size: 14px;
    opacity: 0.7;
  }
}



.toc-sidebar {
  width: 250px;
  background: var(--bg-secondary);
  border-left: 1px solid var(--border-light);
  display: flex;
  flex-direction: column;
  
  .toc-header {
    padding: var(--space-md);
    border-bottom: 1px solid var(--border-light);
    background: var(--bg-tertiary);
    
    h3 {
      margin: 0;
      font-size: 13px;
      color: var(--text-primary);
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.5px;
    }
  }
  
  .toc-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-sm);
    
    .toc-list {
      list-style: none;
      padding: 0;
      margin: 0;
      
      .toc-item {
        margin-bottom: var(--space-xs);
        
        a {
          display: block;
          padding: var(--space-xs) var(--space-sm);
          color: var(--text-secondary);
          text-decoration: none;
          border-radius: 4px;
          font-size: 12px;
          line-height: 1.4;
          transition: all var(--transition-fast);
          position: relative;
          overflow: hidden;
          white-space: nowrap;
          text-overflow: ellipsis;
          
          &:hover {
            background: var(--bg-tertiary);
            color: var(--text-primary);
          }
          
          &.toc-active {
            background: var(--primary);
            color: var(--text-dark);
            font-weight: 600;
          }
        }
      }
    }
  }
}

.readme-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-2xl);
  background: var(--bg-primary);
  box-shadow: var(--shadow-lg);
  border: 1px solid var(--border-light);
  position: relative;
  overflow-wrap: break-word;
  word-wrap: break-word;
  hyphens: auto;
  max-width: 100%;
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 2px;
    background: linear-gradient(90deg, var(--primary), var(--secondary));
    z-index: 1;
  }
  .markdown-content {
    max-width: 100%;
    overflow-x: hidden;
    img {
      max-width: 100%;
      height: auto;
      border-radius: 8px;
      box-shadow: var(--shadow-md);
    }
    pre {
      max-width: 100%;
      overflow-x: auto;
    }
  }
}

.loading, .error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 200px;
  color: var(--text-muted);
  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--bg-tertiary);
    border-top: 3px solid var(--primary);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: var(--space-lg);
  }
  .retry-btn {
    padding: var(--space-sm) var(--space-lg);
    background: var(--primary);
    color: var(--text-dark);
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    transition: all var(--transition-fast);
    &:hover {
      background: var(--primary-dark);
      transform: translateY(-1px);
    }
  }
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.markdown-content {
  pointer-events: auto;
  user-select: text;
  h1, h2, h3, h4, h5, h6 {
    position: relative;
    margin-top: var(--space-xl);
    margin-bottom: var(--space-md);
    color: var(--text-primary);
    font-weight: 600;
    line-height: 1.3;
    &::before {
      content: '';
      position: absolute;
      left: -24px;
      top: 0.5em;
      width: 4px;
      height: 1em;
      background: var(--primary);
      border-radius: 2px;
    }
  }
  h1 {
    font-size: 2rem;
    font-weight: 700;
    margin-top: 0;
    padding-bottom: var(--space-sm);
    border-bottom: 2px solid var(--border-light);
    &::after {
      content: '';
      position: absolute;
      bottom: -2px;
      left: 0;
      width: 100px;
      height: 2px;
      background: linear-gradient(90deg, var(--primary), var(--secondary));
    }
  }
  h2 {
    font-size: 1.5rem;
    margin-top: var(--space-2xl);
  }
  h3 {
    font-size: 1.25rem;
    color: var(--text-secondary);
  }
  h4 {
    font-size: 1.125rem;
    color: var(--text-secondary);
  }
  p {
    margin-bottom: var(--space-md);
    line-height: 1.7;
    color: var(--text-primary);
  }
  a {
    color: var(--primary);
    text-decoration: none;
    border-bottom: 1px solid transparent;
    transition: all var(--transition-fast);
    &:hover {
      border-bottom-color: var(--primary);
    }
    &[href^="http"]::after {
      content: '↗';
      margin-left: 4px;
      font-size: 12px;
      opacity: 0.7;
    }
  }
  ul, ol {
    margin: var(--space-md) 0;
    padding-left: var(--space-xl);
    li {
      margin-bottom: var(--space-xs);
      line-height: 1.6;
    }
  }
  ul {
    list-style: none;
    li::before {
      content: '▶';
      color: var(--primary);
      font-size: 12px;
      margin-left: -24px;
      margin-right: var(--space-sm);
    }
  }
  ol {
    counter-reset: item;
    li {
      counter-increment: item;
      position: relative;
      &::before {
        content: counter(item);
        position: absolute;
        left: -32px;
        color: var(--primary);
        font-weight: 600;
        font-size: 14px;
        min-width: 20px;
      }
    }
  }
  pre {
    background: var(--bg-dark);
    border: 2px solid var(--border-dark);
    border-radius: 8px;
    padding: var(--space-lg);
    margin: var(--space-lg) 0;
    overflow-x: auto;
    position: relative;
    box-shadow: var(--shadow-lg);
    &::before {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      height: 3px;
      background: linear-gradient(90deg, var(--accent), var(--warning));
      border-radius: 8px 8px 0 0;
    }
    code {
      background: none;
      padding: 0;
      color: var(--text-dark);
      font-size: 13px;
      line-height: 1.4;
      border: none;
      font-family: inherit;
    }
    &.code-block::after {
      content: attr(data-language);
      position: absolute;
      top: var(--space-sm);
      right: var(--space-sm);
      background: var(--bg-dark-secondary);
      color: var(--text-dark);
      padding: 4px 8px;
      border-radius: 4px;
      font-size: 11px;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.5px;
      border: 1px solid var(--border-dark);
    }
  }
  code:not(pre code) {
    background: var(--bg-secondary);
    color: var(--accent);
    padding: 3px 6px;
    border-radius: 4px;
    font-family: inherit;
    font-size: 13px;
    border: 1px solid var(--border-light);
    font-weight: 500;
    box-shadow: var(--shadow-sm);
  }
  table {
    border-collapse: collapse;
    width: 100%;
    margin: var(--space-lg) 0;
    border: 2px solid var(--primary);
    box-shadow: var(--shadow-lg);
    background: var(--bg-primary);
    font-size: 13px;
    th {
      background: var(--primary);
      color: var(--text-dark);
      font-weight: 700;
      padding: var(--space-md) var(--space-lg);
      border: 1px solid var(--primary-dark);
      text-align: left;
      text-transform: uppercase;
      letter-spacing: 0.5px;
      font-size: 12px;
      position: relative;
    }
    td {
      padding: var(--space-md) var(--space-lg);
      border: 1px solid var(--border-light);
      color: var(--text-primary);
      line-height: 1.4;
    }
    tr:nth-child(even) td {
      background: var(--bg-secondary);
    }
    tr:hover td {
      background: var(--bg-tertiary);
      border-color: var(--primary-light);
    }
  }
}
</style>