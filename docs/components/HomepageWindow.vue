<template>
  <div class="homepage-content">
    <div class="homepage-header">
      <div class="homepage-icon">
        <img src="/switchshuttle.svg" alt="SwitchShuttle" class="icon" />
      </div>
      <h2>{{ $t('demo.homepage.title') }}</h2>
      <p class="homepage-subtitle">{{ $t('demo.homepage.subtitle') }}</p>
    </div>
    <div class="homepage-section">
      <h3>{{ $t('demo.homepage.description.title') }}</h3>
      <p>{{ $t('demo.homepage.description.text') }}</p>
    </div>
    <div class="homepage-section">
      <h3>{{ $t('demo.homepage.features.title') }}</h3>
      <ul class="homepage-features">
        <li>{{ $t('demo.homepage.features.feature1') }}</li>
        <li>{{ $t('demo.homepage.features.feature2') }}</li>
        <li>{{ $t('demo.homepage.features.feature3') }}</li>
        <li>{{ $t('demo.homepage.features.feature4') }}</li>
      </ul>
    </div>
    <div class="homepage-section">
      <h3>{{ $t('demo.homepage.links.title') }}</h3>
      <div class="homepage-links">
        <button class="homepage-link" @click="openLink('https://github.com/s00d/switchshuttle')">
          <svg class="link-icon" fill="currentColor" viewBox="0 0 24 24">
            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
          </svg>
          GitHub Repository
        </button>
        <button class="homepage-link" @click="openLink('https://s00d.github.io/switchshuttle/')">
          <svg class="link-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9v-9m0-9v9" />
          </svg>
          Official Website
        </button>
        <button class="homepage-link" @click="openLink('https://github.com/s00d/switchshuttle/releases')">
          <svg class="link-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
          </svg>
          Latest Releases
        </button>
      </div>
    </div>
    <div class="homepage-footer">
      <p class="homepage-note">{{ $t('demo.homepage.footer.note') }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
const openLink = (url: string) => {
  window.open(url, '_blank')
}
</script>

<style scoped>
.homepage-content {
  padding: 20px;
  font-size: 14px;
  line-height: 1.6;
  color: #1d1d1f;
  height: 100%;
  overflow-y: auto;
}

.homepage-content::-webkit-scrollbar {
  width: 8px;
}

.homepage-content::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 4px;
}

.homepage-content::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
}

.homepage-content::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

/* Header */
.homepage-header {
  text-align: center;
  margin-bottom: 30px;
}

.homepage-icon {
  width: 64px;
  height: 64px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 16px;
}

.homepage-icon .icon {
  width: 64px;
  height: 64px;
  color: white;
}

.homepage-header h2 {
  margin: 0 0 8px 0;
  color: #1a1a1a;
  font-size: 24px;
  font-weight: 700;
}

.homepage-subtitle {
  color: #86868b;
  font-size: 16px;
  margin: 0;
}

/* Sections */
.homepage-section {
  margin-bottom: 25px;
}

.homepage-section h3 {
  margin: 0 0 12px 0;
  color: #333;
  font-size: 18px;
  font-weight: 600;
  border-bottom: 1px solid #f0f0f0;
  padding-bottom: 8px;
}

.homepage-section p {
  margin: 0 0 12px 0;
  color: #86868b;
  line-height: 1.6;
}

/* Features */
.homepage-features {
  margin: 0;
  padding-left: 20px;
}

.homepage-features li {
  margin-bottom: 8px;
  line-height: 1.5;
  color: #86868b;
}

/* Links */
.homepage-links {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.homepage-link {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 8px;
  color: #495057;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

.homepage-link:hover {
  background: #e9ecef;
  border-color: #dee2e6;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.link-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

/* Footer */
.homepage-footer {
  margin-top: 30px;
  padding-top: 20px;
  border-top: 1px solid #f0f0f0;
  text-align: center;
}

.homepage-note {
  font-style: italic;
  color: #86868b;
  font-size: 13px;
  margin: 0;
}
</style> 