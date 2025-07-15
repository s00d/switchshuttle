// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  compatibilityDate: '2025-07-13',
  modules: [
    '@nuxtjs/i18n',
    '@pinia/nuxt',
    '@nuxt/image',
    '@nuxt/eslint'
  ],
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
      routes: [
        '/',
        '/ru', 
        '/de', 
        '/ja', 
        '/zh',
        '/api/readme?locale=en',
        '/api/readme?locale=ru',
        '/api/readme?locale=de',
        '/api/readme?locale=ja',
        '/api/readme?locale=zh',
        '/api/changelog'
      ],
      ignore: [
        '/LICENSE',
        '/CHANGELOG.md',
        '/README.md',
        '/README_*.md'
      ]
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