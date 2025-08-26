import { vi, beforeEach, afterEach } from 'vitest';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
  convertFileSrc: vi.fn(),
  isTauri: true,
}));

// Mock Tauri commands
vi.mock('@tauri-apps/api', () => ({
  invoke: vi.fn(),
  convertFileSrc: vi.fn(),
  isTauri: true,
}));

// Mock window object for tests
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(), // deprecated
    removeListener: vi.fn(), // deprecated
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

// Mock ResizeObserver
Object.defineProperty(window, 'ResizeObserver', {
  writable: true,
  value: vi.fn().mockImplementation(() => ({
    observe: vi.fn(),
    unobserve: vi.fn(),
    disconnect: vi.fn(),
  })),
});

// Mock IntersectionObserver
Object.defineProperty(window, 'IntersectionObserver', {
  writable: true,
  value: vi.fn().mockImplementation(() => ({
    observe: vi.fn(),
    unobserve: vi.fn(),
    disconnect: vi.fn(),
  })),
});

// Mock console methods to reduce noise in tests
const originalConsole = { ...console };
beforeEach(() => {
  console.warn = vi.fn();
  console.error = vi.fn();
});

afterEach(() => {
  console.warn = originalConsole.warn;
  console.error = originalConsole.error;
});
