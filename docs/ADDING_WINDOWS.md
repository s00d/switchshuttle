# Adding New Windows to SwitchShuttle Demo

This guide explains how to add new windows to the SwitchShuttle demo application.

## Overview

The demo application uses a window management system that allows you to create custom windows with different content types. Each window is a Vue component that can be opened, closed, and positioned on the desktop.

## Step-by-Step Guide

### 1. Create the Window Component

First, create a new Vue component in `docs/components/`. For example, to create a "Changelog" window:

```vue
<template>
  <div class="changelog-container">
    <div class="changelog-content" ref="contentRef">
      <div class="markdown-content" v-html="content"></div>
    </div>
    <div v-if="showToc && toc.length > 0" class="toc-sidebar">
      <div class="toc-header">
        <h3>Table of Contents</h3>
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
    <div class="window-menu-icon" @click="toggleToc" title="Toggle Table of Contents">
      <span class="menu-icon">📋</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, computed, onMounted } from 'vue'

// Types
interface ChangelogResponse {
  success: boolean
  content?: string
  toc?: Array<{id: string, title: string, level: number}>
  error?: string
}

// State
const showToc = ref(false)
const activeSection = ref('')
const contentRef = ref<HTMLElement>()

const { data: changelogData } = await useAsyncData<ChangelogResponse>('changelog', async () => {
  return await $fetch('/api/changelog')
}, {
  deep: true,
  lazy: false,
})

// Computed
const content = computed(() => changelogData.value?.content || '')
const toc = computed(() => changelogData.value?.toc || [])

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
/* Your styles here */
</style>
```

### 2. Add the Component to pages/index.vue

Import your new component in `docs/pages/index.vue`:

```vue
import ChangelogWindow from '~/components/ChangelogWindow.vue'
```

### 3. Add the Window to the Template

Add your window to the template section in `docs/pages/index.vue`:

```vue
<ChangelogWindow v-else-if="win.component === 'changelog-window'" />
```

## Window Configuration Options

### Basic Window Properties

```javascript
{
  id: 'unique-window-id',           // Unique identifier
  component: 'component-name',       // Vue component name
  props: {                          // Props passed to component
    windowId: 'unique-window-id',
    title: 'Window Title'
  },
  position: { x: 100, y: 100 },    // Initial position
  size: { width: 600, height: 400 } // Initial size
}
```

### Advanced Properties

```javascript
{
  id: 'advanced-window',
  component: 'advanced-window',
  props: {
    windowId: 'advanced-window',
    title: 'Advanced Window',
    data: { /* custom data */ },
    config: { /* configuration */ }
  },
  position: { x: 200, y: 200 },
  size: { width: 800, height: 600 },
  minSize: { width: 400, height: 300 },  // Minimum size
  maxSize: { width: 1200, height: 800 }, // Maximum size
  resizable: true,                        // Allow resizing
  movable: true,                          // Allow moving
  closable: true                          // Allow closing
}
```

## Examples

### Simple Text Window

```vue
<template>
  <div class="simple-window">
    <h1>{{ title }}</h1>
    <p>{{ content }}</p>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  content: string
}>()
</script>
```

### Window with API Data

```vue
<template>
  <div class="api-window">
    <div v-if="pending" class="loading">Loading...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else class="content">
      <pre>{{ data }}</pre>
    </div>
  </div>
</template>

<script setup lang="ts">
const { data, pending, error } = await useFetch('/api/endpoint')
</script>
```

### Interactive Window

```vue
<template>
  <div class="interactive-window">
    <input v-model="inputValue" @keyup.enter="handleSubmit" />
    <button @click="handleSubmit">Submit</button>
    <div class="output">{{ output }}</div>
  </div>
</template>

<script setup lang="ts">
const inputValue = ref('')
const output = ref('')

function handleSubmit() {
  output.value = `Processed: ${inputValue.value}`
  inputValue.value = ''
}
</script>
```

## Best Practices

### 1. Use Consistent Styling

Follow the existing design patterns:

```scss
:root {
  --primary: #3b82f6;
  --bg-primary: #fff;
  --text-primary: #0f172a;
  // ... other variables
}

.window-container {
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: 'JetBrains Mono', monospace;
}
```

### 2. Handle Loading States

Always provide loading and error states:

```vue
<template>
  <div class="window">
    <div v-if="pending" class="loading">
      <div class="spinner"></div>
      <p>Loading...</p>
    </div>
    <div v-else-if="error" class="error">
      <p>Error: {{ error }}</p>
      <button @click="retry">Retry</button>
    </div>
    <div v-else class="content">
      <!-- Your content -->
    </div>
  </div>
</template>
```

### 3. Use Responsive Design

Make sure your windows work well at different sizes:

```scss
.window-content {
  overflow-y: auto;
  overflow-x: hidden;
  word-wrap: break-word;
  hyphens: auto;
}
```

### 4. Add Keyboard Shortcuts

Consider adding keyboard shortcuts for common actions:

```javascript
onMounted(() => {
  const handleKeydown = (e: KeyboardEvent) => {
    if (e.ctrlKey && e.key === 's') {
      e.preventDefault()
      saveData()
    }
  }
  
  document.addEventListener('keydown', handleKeydown)
  onUnmounted(() => document.removeEventListener('keydown', handleKeydown))
})
```

## Troubleshooting

### Common Issues

1. **Window not opening**: Check that the component is properly imported and added to the template
2. **Component not found**: Ensure the component name matches the file name
3. **Styling issues**: Make sure to use the CSS variables for consistent theming
4. **Data not loading**: Check that your API endpoints are working correctly

### Debug Tips

1. Use the browser console to check for errors
2. Add `console.log` statements to debug component lifecycle
3. Check the Network tab to see if API calls are working
4. Use Vue DevTools to inspect component state

## API Integration

If your window needs to fetch data, create an API endpoint in `docs/server/api/`:

```typescript
// docs/server/api/my-data.get.ts
export default defineEventHandler(async (event) => {
  try {
    // Your data fetching logic
    const data = await fetchData()
    return {
      success: true,
      data
    }
  } catch (error) {
    return {
      success: false,
      error: error.message
    }
  }
})
```

Then use it in your component:

```javascript
const { data } = await useAsyncData('my-data', () => $fetch('/api/my-data'))
```

This completes the guide for adding new windows to the SwitchShuttle demo application! 