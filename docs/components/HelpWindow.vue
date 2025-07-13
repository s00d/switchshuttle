<template>
    <div class="help-content">
      <div class="help-header">
        <h2>{{ $t('demo.help.title') }}</h2>
        <p class="help-subtitle">{{ $t('demo.help.subtitle') }}</p>
      </div>
      
      <div class="help-section">
        <h3>{{ $t('demo.help.features.title') }}</h3>
        <ul class="help-features">
          <li><strong>{{ $t('demo.help.features.commandTemplates') }}</strong></li>
          <li><strong>{{ $t('demo.help.features.hotkeySupport') }}</strong></li>
          <li><strong>{{ $t('demo.help.features.crossPlatform') }}</strong></li>
          <li><strong>{{ $t('demo.help.features.visualEditor') }}</strong></li>
          <li><strong>{{ $t('demo.help.features.systemMonitoring') }}</strong></li>
          <li><strong>{{ $t('demo.help.features.autoRefresh') }}</strong></li>
          <li><strong>{{ $t('demo.help.features.launchAtLogin') }}</strong></li>
        </ul>
      </div>
      
      <div class="help-section">
        <h3>{{ $t('demo.help.usage.title') }}</h3>
        <ol class="help-usage">
          <li v-for="(step, index) in usageSteps" :key="index">{{ step }}</li>
        </ol>
      </div>
      
      <div class="help-section">
        <h3>{{ $t('demo.help.installation.title') }}</h3>
        
        <h4>{{ $t('demo.help.installation.homebrew.title') }}</h4>
        <div class="code-block">
          <code>{{ $t('demo.help.installation.homebrew.command') }}</code>
        </div>
        
        <h4>{{ $t('demo.help.installation.manual.title') }}</h4>
        <ol class="help-usage">
          <li v-for="(step, index) in installationSteps" :key="index">{{ step }}</li>
        </ol>
        
        <h4>{{ $t('demo.help.installation.firstConfig.title') }}</h4>
        <ol class="help-usage">
          <li v-for="(step, index) in firstConfigSteps" :key="index">{{ step }}</li>
        </ol>
      </div>
      
      <div class="help-section">
        <h3>{{ $t('demo.help.cli.title') }}</h3>
        <p>{{ $t('demo.help.cli.description') }}</p>
        
        <h4>{{ $t('demo.help.cli.usage.title') }}</h4>
        <ul class="help-features">
          <li><strong>{{ $t('demo.help.cli.usage.execute') }}</strong></li>
          <li><strong>{{ $t('demo.help.cli.usage.executeByName') }}</strong></li>
          <li><strong>{{ $t('demo.help.cli.usage.list') }}</strong></li>
          <li><strong>{{ $t('demo.help.cli.usage.search') }}</strong></li>
        </ul>
        
        <h4>{{ $t('demo.help.cli.examples.title') }}</h4>
        <ul class="help-features">
          <li><strong>{{ $t('demo.help.cli.examples.git') }}</strong></li>
          <li><strong>{{ $t('demo.help.cli.examples.dev') }}</strong></li>
          <li><strong>{{ $t('demo.help.cli.examples.docker') }}</strong></li>
        </ul>
      </div>
      
      <div class="help-section">
        <h3>{{ $t('demo.help.commands.title') }}</h3>
        <ul class="help-features">
          <li><strong>{{ $t('demo.help.commands.devServer') }}</strong></li>
          <li><strong>{{ $t('demo.help.commands.installDeps') }}</strong></li>
          <li><strong>{{ $t('demo.help.commands.runTests') }}</strong></li>
          <li><strong>{{ $t('demo.help.commands.buildProject') }}</strong></li>
          <li><strong>{{ $t('demo.help.commands.gitOps') }}</strong></li>
        </ul>
      </div>
      
      <div class="help-section">
        <h3>{{ $t('demo.help.config.title') }}</h3>
        <p>{{ $t('demo.help.config.description') }}</p>
        <ul class="help-features">
          <li><strong>{{ $t('demo.help.config.macos') }}</strong></li>
          <li><strong>{{ $t('demo.help.config.windows') }}</strong></li>
          <li><strong>{{ $t('demo.help.config.linux') }}</strong></li>
        </ul>
      </div>
      
      <div class="help-section">
        <h3>{{ $t('demo.help.configuration.title') }}</h3>
        
        <h4>{{ $t('demo.help.configuration.basicStructure.title') }}</h4>
        <p>{{ $t('demo.help.configuration.basicStructure.description') }}</p>
        
        <h4>{{ $t('demo.help.configuration.example.title') }}</h4>
        <div class="code-block">
          <pre><code>{{ configExample }}</code></pre>
        </div>
        
        <h4>{{ $t('demo.help.configuration.features.title') }}</h4>
        <ul class="help-features">
          <li><strong>{{ $t('demo.help.configuration.features.dynamicInputs') }}</strong></li>
          <li><strong>{{ $t('demo.help.configuration.features.multipleCommands') }}</strong></li>
          <li><strong>{{ $t('demo.help.configuration.features.nestedSubmenus') }}</strong></li>
          <li><strong>{{ $t('demo.help.configuration.features.switchCommands') }}</strong></li>
          <li><strong>{{ $t('demo.help.configuration.features.monitorCommands') }}</strong></li>
        </ul>
      </div>
      
      <div class="help-section">
        <h3>{{ $t('demo.help.useCases.title') }}</h3>
        
        <h4>{{ $t('demo.help.useCases.developers.title') }}</h4>
        <ul class="help-features">
          <li v-for="(feature, index) in developerFeatures" :key="index"><strong>{{ feature }}</strong></li>
        </ul>
        
        <h4>{{ $t('demo.help.useCases.devops.title') }}</h4>
        <ul class="help-features">
          <li v-for="(feature, index) in devopsFeatures" :key="index"><strong>{{ feature }}</strong></li>
        </ul>
        
        <h4>{{ $t('demo.help.useCases.designers.title') }}</h4>
        <ul class="help-features">
          <li v-for="(feature, index) in designerFeatures" :key="index"><strong>{{ feature }}</strong></li>
        </ul>
      </div>
      
      <div class="help-footer">
        <p>{{ $t('demo.help.footer.info') }}</p>
        <p class="help-note">{{ $t('demo.help.footer.note') }}</p>
      </div>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const usageSteps = computed(() => {
  const steps = t('demo.help.usage.steps')
  return Array.isArray(steps) ? steps : []
})

const installationSteps = computed(() => {
  const steps = t('demo.help.installation.manual.steps')
  return Array.isArray(steps) ? steps : []
})

const firstConfigSteps = computed(() => {
  const steps = t('demo.help.installation.firstConfig.steps')
  return Array.isArray(steps) ? steps : []
})

const developerFeatures = computed(() => {
  const features = t('demo.help.useCases.developers.features')
  return Array.isArray(features) ? features : []
})

const devopsFeatures = computed(() => {
  const features = t('demo.help.useCases.devops.features')
  return Array.isArray(features) ? features : []
})

const designerFeatures = computed(() => {
  const features = t('demo.help.useCases.designers.features')
  return Array.isArray(features) ? features : []
})

const configExample = computed(() => {
  return `{
  "terminal": "iterm",
  "launch_in": "new_tab",
  "title": "${t('demo.help.configuration.example.title')}",
  "commands": [
    {
      "name": "🚀 ${t('demo.help.commands.devServer')}",
      "command": "npm run dev",
      "hotkey": "Ctrl+Shift+D"
    }
  ]
}`
})
</script>

<style scoped>
.help-content {
  padding: 20px;
  max-height: 100%;
  overflow-y: auto;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  line-height: 1.6;
  color: #333;
}

.help-header {
  margin-bottom: 30px;
  text-align: center;
}

.help-header h2 {
  font-size: 28px;
  font-weight: 600;
  color: #2d3748;
  margin-bottom: 8px;
}

.help-subtitle {
  font-size: 16px;
  color: #718096;
  margin: 0;
}

.help-section {
  margin-bottom: 30px;
}

.help-section h3 {
  font-size: 20px;
  font-weight: 600;
  color: #2d3748;
  margin-bottom: 15px;
  padding-bottom: 8px;
  border-bottom: 2px solid #e2e8f0;
}

.help-section h4 {
  font-size: 16px;
  font-weight: 600;
  color: #4a5568;
  margin: 20px 0 10px 0;
}

.help-features {
  list-style: none;
  padding: 0;
  margin: 0;
}

.help-features li {
  padding: 8px 0;
  border-bottom: 1px solid #f7fafc;
  color: #4a5568;
}

.help-features li:last-child {
  border-bottom: none;
}

.help-usage {
  padding-left: 20px;
  color: #4a5568;
}

.help-usage li {
  margin-bottom: 8px;
}

.code-block {
  background: #f7fafc;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  padding: 15px;
  margin: 15px 0;
  overflow-x: auto;
}

.code-block code {
  font-family: 'JetBrains Mono', 'Fira Mono', 'Menlo', 'Consolas', monospace;
  font-size: 13px;
  color: #2d3748;
  background: none;
  padding: 0;
}

.code-block pre {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
}

.help-footer {
  margin-top: 40px;
  padding-top: 20px;
  border-top: 2px solid #e2e8f0;
  text-align: center;
  color: #718096;
}

.help-note {
  font-size: 14px;
  font-style: italic;
  margin-top: 10px;
}

/* Скроллбар */
.help-content::-webkit-scrollbar {
  width: 8px;
}

.help-content::-webkit-scrollbar-track {
  background: #f7fafc;
}

.help-content::-webkit-scrollbar-thumb {
  background: #cbd5e0;
  border-radius: 4px;
}

.help-content::-webkit-scrollbar-thumb:hover {
  background: #a0aec0;
}
</style> 