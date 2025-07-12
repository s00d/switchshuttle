<template>
  <Window :title="`${$t('demo.help.title')} — SwitchShuttle`" :initial-x="100" :initial-y="100" :z="1500" @close="$emit('close')">
    <template #titlebar>
      <div class="window-title">{{ $t('demo.help.title') }} — SwitchShuttle</div>
    </template>
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
  </Window>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import Window from './Window.vue'

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

defineEmits<{
  close: []
}>()
</script>

<style scoped>

.help-content {
  padding: 20px;
  font-size: 14px;
  line-height: 1.6;
  color: #1d1d1f;
  height: 100%;
  overflow-y: auto;
  pointer-events: auto;
  user-select: text;
}

.help-content::-webkit-scrollbar {
  width: 8px;
}

.help-content::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 4px;
}

.help-content::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
}

.help-content::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

/* Header */
.help-header {
  text-align: center;
  margin-bottom: 30px;
}

.help-header h2 {
  margin: 0 0 8px 0;
  color: #1a1a1a;
  font-size: 24px;
  font-weight: 700;
}

.help-subtitle {
  color: #86868b;
  font-size: 16px;
  margin: 0;
}

/* Sections */
.help-section {
  margin-bottom: 25px;
}

.help-section h3 {
  margin: 0 0 12px 0;
  color: #333;
  font-size: 18px;
  font-weight: 600;
  border-bottom: 1px solid #f0f0f0;
  padding-bottom: 8px;
}

.help-features,
.help-usage {
  margin: 0;
  padding-left: 20px;
}

.help-features li,
.help-usage li {
  margin-bottom: 8px;
  line-height: 1.5;
}

.help-features strong,
.help-usage strong {
  color: #007AFF;
  font-weight: 600;
  pointer-events: auto;
}

/* Numbered list */
.help-usage {
  counter-reset: item;
  list-style: none;
  padding-left: 0;
}

.help-usage li {
  counter-increment: item;
  margin-bottom: 12px;
  padding-left: 30px;
  position: relative;
}

.help-usage li::before {
  content: counter(item);
  position: absolute;
  left: 0;
  top: 0;
  background: #007AFF;
  color: white;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
}



/* Footer */
.help-footer {
  margin-top: 30px;
  padding-top: 20px;
  border-top: 1px solid #f0f0f0;
  text-align: center;
}

.help-footer p {
  margin-bottom: 8px;
  color: #86868b;
}

.help-footer a {
  color: #007AFF;
  text-decoration: none;
  pointer-events: auto;
  cursor: pointer;
}

.help-footer a:hover {
  text-decoration: underline;
}

.help-note {
  font-style: italic;
  color: #86868b;
  font-size: 13px;
}

/* Code elements */
.help-content code {
  background: rgba(0, 0, 0, 0.1);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 14px;
}

.code-block {
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 6px;
  padding: 12px;
  margin: 12px 0;
  overflow-x: auto;
}

.code-block pre {
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
  font-size: 13px;
  line-height: 1.4;
  color: #333;
}

.code-block code {
  background: none;
  padding: 0;
  color: inherit;
}

/* Subheadings */
.help-section h4 {
  margin: 16px 0 8px 0;
  color: #333;
  font-size: 16px;
  font-weight: 600;
}

.help-content kbd {
  background: #f5f5f5;
  border: 1px solid #ccc;
  border-radius: 3px;
  padding: 2px 6px;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 12px;
  box-shadow: 0 1px 0 rgba(0, 0, 0, 0.2);
}
</style> 