import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import vueDevTools from 'vite-plugin-vue-devtools';
import tailwindcss from '@tailwindcss/vite'

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  const isDev = mode === 'development';

  return {
    plugins: [
      vue(),
      tailwindcss(),
      // Only include Vue Devtools plugin in development mode
      isDev && vueDevTools()
    ].filter(Boolean),

    // Поддержка импорта JSON файлов
    json: {
      stringify: true
    },

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
      port: 1420,
      strictPort: true,
      watch: {
        // 3. tell vite to ignore watching `src-tauri`
        ignored: ['**/src-tauri/**'],
      },
    },

    // Test configuration
    test: {
      globals: true,
      environment: 'jsdom',
      setupFiles: ['./src/__tests__/setup.ts'],
      include: ['src/**/*.{test,spec}.{js,ts,vue}'],
      coverage: {
        provider: 'v8',
        reporter: ['text', 'json', 'html'],
        exclude: [
          'node_modules/',
          'src-tauri/',
          'src/__tests__/',
          '**/*.d.ts',
          '**/*.config.*',
          '**/coverage/**'
        ]
      }
    }
  };
});
