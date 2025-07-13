import { ref, computed } from 'vue'

export interface WindowInfo {
  id: string
  component: any
  props?: Record<string, any>
  zIndex?: number
  minimized?: boolean
  maximized?: boolean
  position?: { x: number, y: number }
  size?: { width: number, height: number }
}

export function useWindowManager() {
  // Используем useState для SSR совместимости
  const windows = useState<WindowInfo[]>('windows', () => [])

  function openWindow(win: WindowInfo) {
    const idx = windows.value.findIndex(w => w.id === win.id)
    if (idx !== -1) {
      // Обновляем props и поднимаем наверх
      const [existing] = windows.value.splice(idx, 1)
      windows.value.push({ ...existing, ...win, minimized: false })
    } else {
      windows.value.push({
        ...win,
        zIndex: Date.now(),
        minimized: false,
        maximized: false,
        position: win.position || { x: 120 + windows.value.length * 40, y: 120 + windows.value.length * 40 },
        size: win.size || { width: 700, height: 480 },
      })
    }
  }

  function closeWindow(id: string) {
    windows.value = windows.value.filter(w => w.id !== id)
  }

  function bringToFront(id: string) {
    const idx = windows.value.findIndex(w => w.id === id)
    if (idx !== -1) {
      const [win] = windows.value.splice(idx, 1)
      win.zIndex = Date.now()
      windows.value.push(win)
    }
  }

  function setWindowState(id: string, patch: Partial<WindowInfo>) {
    const win = windows.value.find(w => w.id === id)
    if (win) Object.assign(win, patch)
  }

  function minimizeWindow(id: string) {
    setWindowState(id, { minimized: true, maximized: false })
  }

  function maximizeWindow(id: string) {
    setWindowState(id, { maximized: true, minimized: false })
  }

  function restoreWindow(id: string) {
    setWindowState(id, { minimized: false, maximized: false })
  }

  function moveWindow(id: string, position: { x: number, y: number }) {
    setWindowState(id, { position })
  }

  function resizeWindow(id: string, size: { width: number, height: number }) {
    setWindowState(id, { size })
  }

  // Computed свойства для TaskBar
  const minimizedWindows = computed(() => {
    const result: Record<string, boolean> = {}
    windows.value.forEach(win => {
      if (win.minimized) {
        result[win.id] = true
      }
    })
    return result
  })

  const maximizedWindows = computed(() => {
    const result: Record<string, boolean> = {}
    windows.value.forEach(win => {
      if (win.maximized) {
        result[win.id] = true
      }
    })
    return result
  })

  const activeWindowId = computed(() => {
    if (windows.value.length === 0) return null
    return windows.value[windows.value.length - 1]?.id || null
  })

  const windowZIndexes = computed(() => {
    const result: Record<string, number> = {}
    windows.value.forEach(win => {
      result[win.id] = win.zIndex || 0
    })
    return result
  })

  function activateWindow(id: string) {
    bringToFront(id)
  }

  return {
    windows,
    openWindow,
    closeWindow,
    bringToFront,
    minimizeWindow,
    maximizeWindow,
    restoreWindow,
    moveWindow,
    resizeWindow,
    setWindowState,
    minimizedWindows,
    maximizedWindows,
    activeWindowId,
    windowZIndexes,
    activateWindow,
  }
} 