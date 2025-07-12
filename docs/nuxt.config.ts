// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: [
    '@nuxtjs/i18n',
    '@pinia/nuxt',
    '@nuxt/image',
    '@nuxt/eslint'
  ],
  hooks: {
    'build:before': async () => {
      const { execSync } = await import('child_process')
      const { join } = await import('path')
      
      try {
        console.log('🔄 Generating README data...')
        execSync('node scripts/generate-readme-data.js', { 
          cwd: join(process.cwd()),
          stdio: 'inherit'
        })
        console.log('✅ README data generated successfully!')
      } catch (error: any) {
        console.error('❌ Error generating README data:', error.message)
      }
    }
  },
  i18n: {
    locales: [
      { code: 'en', iso: 'en-US', name: 'English', file: 'en.json' },
      { code: 'ru', iso: 'ru-RU', name: 'Русский', file: 'ru.json' },
      { code: 'de', iso: 'de-DE', name: 'Deutsch', file: 'de.json' },
      { code: 'ja', iso: 'ja-JP', name: '日本語', file: 'ja.json' },
      { code: 'zh', iso: 'zh-CN', name: '中文', file: 'zh.json' }
    ],
    defaultLocale: 'en',
    strategy: 'prefix_except_default',
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'i18n_redirected',
      redirectOn: 'root'
    },
    langDir: 'locales/',
    lazy: true,
    bundle: {
      optimizeTranslationDirective: false
    }
  },
  ssr: true,
  nitro: {
    prerender: {
      routes: ['/', '/ru', '/de', '/ja', '/zh']
    }
  },
  app: {
    baseURL: process.env.NODE_ENV === 'production' ? '/switchshuttle/' : '/'
  },
  css: ['~/assets/css/main.css'],
  image: {
    quality: 80
  }
})