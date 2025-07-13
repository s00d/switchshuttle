<template>
  <div class="about-content">
    <!-- App Info -->
    <div class="app-info">
      <div class="app-header">
        <div class="app-logo">
          <img src="/logo.svg" alt="SwitchShuttle" class="logo-image" />
        </div>
        <div class="app-details">
          <h2 class="app-title">SwitchShuttle</h2>
          <p class="app-version">{{ $t('demo.about.version') }}</p>
        </div>
      </div>
      <p class="app-description">
        SwitchShuttle is a powerful tool for managing terminal configurations. 
        Create, edit, and switch between different terminal settings with ease.
      </p>
    </div>

    <!-- Features -->
    <div class="features-section">
      <h3 class="section-title">{{ $t('demo.about.features.title') }}</h3>
      <div class="features-grid">
        <div class="feature-item">
          <div class="feature-icon blue">
            <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
            </svg>
          </div>
          <div class="feature-content">
            <h4 class="feature-title">{{ $t('demo.about.features.config.title') }}</h4>
            <p class="feature-description">{{ $t('demo.about.features.config.description') }}</p>
          </div>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon green">
            <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
          </div>
          <div class="feature-content">
            <h4 class="feature-title">{{ $t('demo.about.features.editing.title') }}</h4>
            <p class="feature-description">{{ $t('demo.about.features.editing.description') }}</p>
          </div>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon purple">
            <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
          </div>
          <div class="feature-content">
            <h4 class="feature-title">{{ $t('demo.about.features.quickAccess.title') }}</h4>
            <p class="feature-description">{{ $t('demo.about.features.quickAccess.description') }}</p>
          </div>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon orange">
            <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
          </div>
          <div class="feature-content">
            <h4 class="feature-title">{{ $t('demo.about.features.terminalSupport.title') }}</h4>
            <p class="feature-description">{{ $t('demo.about.features.terminalSupport.description') }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Technical Info -->
    <div class="technical-section">
      <h3 class="section-title">{{ $t('demo.about.technical.title') }}</h3>
      <div class="technical-grid">
        <div class="technical-column">
          <h4 class="technical-subtitle">{{ $t('demo.about.technical.technologies.title') }}</h4>
          <ul class="technical-list">
            <li class="technical-item">
              <span class="bullet blue"></span>
              <span>{{ $t('demo.about.technical.technologies.vue') }}</span>
            </li>
            <li class="technical-item">
              <span class="bullet green"></span>
              <span>{{ $t('demo.about.technical.technologies.nuxt') }}</span>
            </li>
            <li class="technical-item">
              <span class="bullet purple"></span>
              <span>{{ $t('demo.about.technical.technologies.typescript') }}</span>
            </li>
            <li class="technical-item">
              <span class="bullet orange"></span>
              <span>{{ $t('demo.about.technical.technologies.css') }}</span>
            </li>
          </ul>
        </div>
        
        <div class="technical-column">
          <h4 class="technical-subtitle">{{ $t('demo.about.technical.requirements.title') }}</h4>
          <ul class="technical-list">
            <li class="technical-item">
              <span class="bullet blue"></span>
              <span>{{ $t('demo.about.technical.requirements.browser') }}</span>
            </li>
            <li class="technical-item">
              <span class="bullet green"></span>
              <span>{{ $t('demo.about.technical.requirements.memory') }}</span>
            </li>
            <li class="technical-item">
              <span class="bullet purple"></span>
              <span>{{ $t('demo.about.technical.requirements.storage') }}</span>
            </li>
          </ul>
        </div>
      </div>
    </div>

    <!-- Links -->
    <div class="links-section">
      <h3 class="section-title">{{ $t('demo.about.links.title') }}</h3>
      <div class="links-grid">
        <button class="link-button" @click="openLink('https://github.com/s00d/switchshuttle')">
          <svg class="link-icon" fill="currentColor" viewBox="0 0 24 24">
            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
          </svg>
          GitHub
        </button>
        <button class="link-button" @click="openLink('https://s00d.github.io/switchshuttle/')">
          <svg class="link-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9v-9m0-9v9" />
          </svg>
          Website
        </button>
        <button class="link-button" @click="openLink('mailto:virus191288@gmail.com')">
          <svg class="link-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 4.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
          Support
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const openLink = (url: string) => {
  window.open(url, '_blank')
}
</script>

<style scoped>
.about-content {
  padding: 20px;
  font-size: 14px;
  line-height: 1.6;
  color: #1d1d1f;
  height: 100%;
  overflow-y: auto;
}

/* App Info */
.app-info {
  text-align: center;
  margin-bottom: 30px;
}

.app-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 20px;
}

.app-logo {
  margin-bottom: 16px;
}

.logo-image {
  width: 96px;
  height: 96px;
}

.app-title {
  font-size: 24px;
  font-weight: 700;
  color: #1d1d1f;
  margin-bottom: 8px;
}

.app-version {
  font-size: 16px;
  color: #86868b;
  margin-bottom: 16px;
}

.app-description {
  color: #86868b;
  max-width: 500px;
  margin: 0 auto;
  line-height: 1.6;
}

/* Sections */
.section-title {
  font-size: 18px;
  font-weight: 600;
  color: #1d1d1f;
  margin-bottom: 16px;
  border-bottom: 1px solid #f0f0f0;
  padding-bottom: 8px;
}

/* Features */
.features-section {
  margin-bottom: 30px;
}

.features-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
}

.feature-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
}

.feature-icon {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.feature-icon.blue {
  background: #dbeafe;
}

.feature-icon.green {
  background: #dcfce7;
}

.feature-icon.purple {
  background: #f3e8ff;
}

.feature-icon.orange {
  background: #fed7aa;
}

.icon {
  width: 16px;
  height: 16px;
}

.feature-icon.blue .icon {
  color: #2563eb;
}

.feature-icon.green .icon {
  color: #16a34a;
}

.feature-icon.purple .icon {
  color: #9333ea;
}

.feature-icon.orange .icon {
  color: #ea580c;
}

.feature-content {
  flex: 1;
}

.feature-title {
  font-weight: 600;
  color: #1d1d1f;
  margin-bottom: 4px;
  font-size: 14px;
}

.feature-description {
  color: #86868b;
  font-size: 13px;
  line-height: 1.5;
}

/* Technical */
.technical-section {
  margin-bottom: 30px;
}

.technical-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 30px;
}

.technical-subtitle {
  font-weight: 600;
  color: #1d1d1f;
  margin-bottom: 12px;
  font-size: 14px;
}

.technical-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.technical-item {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
  font-size: 13px;
  color: #86868b;
}

.bullet {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.bullet.blue {
  background: #3b82f6;
}

.bullet.green {
  background: #10b981;
}

.bullet.purple {
  background: #8b5cf6;
}

.bullet.orange {
  background: #f59e0b;
}

/* Links */
.links-section {
  margin-bottom: 20px;
}

.links-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.link-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 6px;
  color: #495057;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.link-button:hover {
  background: #e9ecef;
  border-color: #dee2e6;
}

.link-icon {
  width: 16px;
  height: 16px;
}

/* Responsive */
@media (max-width: 768px) {
  .features-grid {
    grid-template-columns: 1fr;
  }
  
  .technical-grid {
    grid-template-columns: 1fr;
  }
  
  .links-grid {
    flex-direction: column;
  }
}
</style> 