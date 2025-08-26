<template>
  <div class="container mx-auto px-6 py-8 max-w-4xl">
    <div class="bg-white rounded-lg shadow-lg p-8">
      <h1 class="text-3xl font-bold text-slate-900 mb-8">Help & FAQ</h1>

      <!-- Search -->
      <div class="mb-8">
        <div class="relative">
          <input
              v-model="searchQuery"
              type="text"
              placeholder="Search FAQ..."
              class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
          <div class="absolute inset-y-0 right-0 pr-3 flex items-center">
            <SearchIcon class="h-5 w-5 text-slate-400" />
          </div>
        </div>
      </div>

      <!-- FAQ Categories -->
      <div class="space-y-6">
        <!-- Getting Started -->
        <div class="bg-slate-50 rounded-lg p-6">
          <h2 class="text-xl font-bold text-slate-900 mb-4">
            🚀 Getting Started
          </h2>
          <div class="space-y-3">
            <div
                v-for="item in filteredFaqs.gettingStarted"
                :key="item.id"
                class="bg-white rounded-lg border border-slate-200"
            >
              <CustomButton
                  class="w-full px-4 py-3 text-left flex justify-between items-center hover:bg-slate-50 transition-colors"
                  @click="toggleFaq(item.id)"
              >
                <span class="font-medium text-slate-900">{{
                    item.question
                  }}</span>
                <ChevronDownIcon
                    :class="[
                    'w-5 h-5 text-slate-500 transition-transform',
                    { 'rotate-180': openFaqs.includes(item.id) },
                  ]"
                />
              </CustomButton>
              <div v-show="openFaqs.includes(item.id)" class="px-4 pb-4">
                <div
                    class="text-sm text-slate-700 leading-relaxed"
                    v-html="item.answer"
                ></div>
              </div>
            </div>
          </div>
        </div>

        <!-- Configuration -->
        <div class="bg-slate-50 rounded-lg p-6">
          <h2 class="text-xl font-bold text-slate-900 mb-4">
            ⚙️ Configuration
          </h2>
          <div class="space-y-3">
            <div
                v-for="item in filteredFaqs.configuration"
                :key="item.id"
                class="bg-white rounded-lg border border-slate-200"
            >
              <CustomButton
                  class="w-full px-4 py-3 text-left flex justify-between items-center hover:bg-slate-50 transition-colors"
                  @click="toggleFaq(item.id)"
              >
                <span class="font-medium text-slate-900">{{
                    item.question
                  }}</span>
                <ChevronDownIcon
                    :class="[
                    'w-5 h-5 text-slate-500 transition-transform',
                    { 'rotate-180': openFaqs.includes(item.id) },
                  ]"
                />
              </CustomButton>
              <div v-show="openFaqs.includes(item.id)" class="px-4 pb-4">
                <div
                    class="text-sm text-slate-700 leading-relaxed"
                    v-html="item.answer"
                ></div>
              </div>
            </div>
          </div>
        </div>

        <!-- Command Types -->
        <div class="bg-slate-50 rounded-lg p-6">
          <h2 class="text-xl font-bold text-slate-900 mb-4">
            🔧 Command Types
          </h2>
          <div class="space-y-3">
            <div
                v-for="item in filteredFaqs.commandTypes"
                :key="item.id"
                class="bg-white rounded-lg border border-slate-200"
            >
              <CustomButton
                  class="w-full px-4 py-3 text-left flex justify-between items-center hover:bg-slate-50 transition-colors"
                  @click="toggleFaq(item.id)"
              >
                <span class="font-medium text-slate-900">{{
                    item.question
                  }}</span>
                <ChevronDownIcon
                    :class="[
                    'w-5 h-5 text-slate-500 transition-transform',
                    { 'rotate-180': openFaqs.includes(item.id) },
                  ]"
                />
              </CustomButton>
              <div v-show="openFaqs.includes(item.id)" class="px-4 pb-4">
                <div
                    class="text-sm text-slate-700 leading-relaxed"
                    v-html="item.answer"
                ></div>
              </div>
            </div>
          </div>
        </div>

        <!-- Advanced Features -->
        <div class="bg-slate-50 rounded-lg p-6">
          <h2 class="text-xl font-bold text-slate-900 mb-4">
            🚀 Advanced Features
          </h2>
          <div class="space-y-3">
            <div
                v-for="item in filteredFaqs.advancedFeatures"
                :key="item.id"
                class="bg-white rounded-lg border border-slate-200"
            >
              <CustomButton
                  class="w-full px-4 py-3 text-left flex justify-between items-center hover:bg-slate-50 transition-colors"
                  @click="toggleFaq(item.id)"
              >
                <span class="font-medium text-slate-900">{{
                    item.question
                  }}</span>
                <ChevronDownIcon
                    :class="[
                    'w-5 h-5 text-slate-500 transition-transform',
                    { 'rotate-180': openFaqs.includes(item.id) },
                  ]"
                />
              </CustomButton>
              <div v-show="openFaqs.includes(item.id)" class="px-4 pb-4">
                <div
                    class="text-sm text-slate-700 leading-relaxed"
                    v-html="item.answer"
                ></div>
              </div>
            </div>
          </div>
        </div>

        <!-- Troubleshooting -->
        <div class="bg-slate-50 rounded-lg p-6">
          <h2 class="text-xl font-bold text-slate-900 mb-4">
            🔧 Troubleshooting
          </h2>
          <div class="space-y-3">
            <div
                v-for="item in filteredFaqs.troubleshooting"
                :key="item.id"
                class="bg-white rounded-lg border border-slate-200"
            >
              <button
                  class="w-full px-4 py-3 text-left flex justify-between items-center hover:bg-slate-50 transition-colors"
                  @click="toggleFaq(item.id)"
              >
                <span class="font-medium text-slate-900">{{
                    item.question
                  }}</span>
                <ChevronDownIcon
                    :class="[
                    'w-5 h-5 text-slate-500 transition-transform',
                    { 'rotate-180': openFaqs.includes(item.id) },
                  ]"
                />
              </button>
              <div v-show="openFaqs.includes(item.id)" class="px-4 pb-4">
                <div
                    class="text-sm text-slate-700 leading-relaxed"
                    v-html="item.answer"
                ></div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="mt-12 pt-8 border-t border-slate-200">
        <p class="text-sm text-slate-500 text-center">
          Can't find what you're looking for? Visit our
          <a
              href="https://github.com/s00d/switchshuttle"
              class="text-blue-600 hover:text-blue-800"
          >GitHub repository</a
          >
          for additional resources.
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import SearchIcon from '../components/icons/SearchIcon.vue';
import ChevronDownIcon from '../components/icons/ChevronDownIcon.vue';

const searchQuery = ref('');
const openFaqs = ref<string[]>([]);

const faqData = {
  gettingStarted: [
    {
      id: 'gs-1',
      question: 'How do I install SwitchShuttle?',
      answer: `
        <p class="mb-3"><strong>macOS (Recommended):</strong></p>
        <code class="block bg-slate-200 p-2 rounded text-sm mb-3">brew tap s00d/switchshuttle && brew install --cask switchshuttle</code>
        
        <p class="mb-3"><strong>Manual Download:</strong></p>
        <p>Download the latest release from <a href="https://github.com/s00d/switchshuttle/releases" class="text-blue-600 hover:text-blue-800">GitHub Releases</a> for your platform.</p>
      `,
    },
    {
      id: 'gs-2',
      question: 'How do I get started after installation?',
      answer: `
        <ol class="list-decimal list-inside space-y-2">
          <li>Launch SwitchShuttle - it will appear in your system tray</li>
          <li>Right-click the tray icon to access the menu</li>
          <li>Click "Edit Configuration" to open the visual editor</li>
          <li>Add your first command or import an existing configuration</li>
          <li>Save and restart the application</li>
        </ol>
      `,
    },
    {
      id: 'gs-3',
      question: 'Where are configuration files stored?',
      answer: `
        <div class="grid md:grid-cols-2 gap-4">
          <div>
            <strong class="text-slate-700">macOS/Linux:</strong>
            <code class="block bg-slate-200 p-2 rounded text-sm mt-1">~/.config/switch-shuttle/</code>
          </div>
          <div>
            <strong class="text-slate-700">Windows:</strong>
            <code class="block bg-slate-200 p-2 rounded text-sm mt-1">C:\\Users\\&lt;Username&gt;\\AppData\\Roaming\\switch-shuttle\\</code>
          </div>
        </div>
      `,
    },
  ],
  configuration: [
    {
      id: 'cfg-1',
      question: 'How do I create a basic configuration?',
      answer: `
        <p class="mb-3">Here's a simple configuration example:</p>
        <pre class="bg-slate-900 text-slate-100 p-4 rounded-lg overflow-x-auto text-sm"><code>{
  "terminal": "iterm",
  "launch_in": "new_tab",
  "title": "My Commands",
  "enabled": true,
  "commands": [
    {
      "name": "🚀 Start Dev Server",
      "command": "npm run dev",
      "hotkey": "Ctrl+Shift+D"
    }
  ]
}</code></pre>
      `,
    },
    {
      id: 'cfg-2',
      question: 'What are the main configuration parameters?',
      answer: `
        <div class="overflow-x-auto">
          <table class="min-w-full bg-white border border-slate-200 rounded-lg">
            <thead class="bg-slate-50">
              <tr>
                <th class="px-4 py-2 text-left text-xs font-medium text-slate-500 uppercase">Parameter</th>
                <th class="px-4 py-2 text-left text-xs font-medium text-slate-500 uppercase">Description</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-200">
              <tr>
                <td class="px-4 py-2 text-sm font-medium text-slate-900">terminal</td>
                <td class="px-4 py-2 text-sm text-slate-600">Terminal application to use (iterm, terminal, alacritty, etc.)</td>
              </tr>
              <tr>
                <td class="px-4 py-2 text-sm font-medium text-slate-900">launch_in</td>
                <td class="px-4 py-2 text-sm text-slate-600">Where to launch commands (current, new_tab, new_window)</td>
              </tr>
              <tr>
                <td class="px-4 py-2 text-sm font-medium text-slate-900">title</td>
                <td class="px-4 py-2 text-sm text-slate-600">Window/tab title for the terminal</td>
              </tr>
              <tr>
                <td class="px-4 py-2 text-sm font-medium text-slate-900">enabled</td>
                <td class="px-4 py-2 text-sm text-slate-600">Whether to load this configuration</td>
              </tr>
            </tbody>
          </table>
        </div>
      `,
    },
    {
      id: 'cfg-3',
      question: 'How do I enable/disable configurations?',
      answer: `
        <p class="mb-3">You can enable or disable individual configuration files to control which commands are available in the system tray menu.</p>
        
        <p class="mb-3"><strong>Visual Editor Method:</strong></p>
        <ol class="list-decimal list-inside space-y-1 mb-3">
          <li>Open the configuration editor</li>
          <li>Use the toggle switch in the "Configuration Status" section</li>
          <li>Enabled configurations will be loaded and available in the menu</li>
        </ol>
        
        <p class="mb-3"><strong>JSON Method:</strong></p>
        <code class="block bg-slate-200 p-2 rounded text-sm">"enabled": true  // Set to false to disable</code>
      `,
    },
  ],
  commandTypes: [
    {
      id: 'ct-1',
      question: 'What types of commands are supported?',
      answer: `
        <div class="grid md:grid-cols-2 gap-4">
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">Basic Commands</h4>
            <p class="text-sm text-slate-600">Simple commands that execute in the terminal.</p>
            <pre class="bg-slate-900 text-slate-100 p-2 rounded text-xs mt-2"><code>{
  "name": "Start Server",
  "command": "npm run dev"
}</code></pre>
          </div>
          
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">Multiple Commands</h4>
            <p class="text-sm text-slate-600">Execute a sequence of commands.</p>
            <pre class="bg-slate-900 text-slate-100 p-2 rounded text-xs mt-2"><code>{
  "name": "Full Dev Cycle",
  "commands": [
    "git pull origin main",
    "npm install",
    "npm run build"
  ]
}</code></pre>
          </div>
          
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">Submenus</h4>
            <p class="text-sm text-slate-600">Organize commands in hierarchical menus.</p>
            <pre class="bg-slate-900 text-slate-100 p-2 rounded text-xs mt-2"><code>{
  "name": "Docker Operations",
  "submenu": [
    {
      "name": "Start Services",
      "command": "docker-compose up -d"
    }
  ]
}</code></pre>
          </div>
          
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">Dynamic Inputs</h4>
            <p class="text-sm text-slate-600">Commands with user input prompts.</p>
            <pre class="bg-slate-900 text-slate-100 p-2 rounded text-xs mt-2"><code>{
  "name": "Create Component",
  "inputs": {
    "componentName": "MyComponent"
  },
  "commands": [
    "mkdir -p src/components/[componentName]"
  ]
}</code></pre>
          </div>
        </div>
      `,
    },
    {
      id: 'ct-2',
      question: 'How do I add hotkeys to commands?',
      answer: `
        <p class="mb-3">You can assign global hotkeys to your commands for quick access:</p>
        <pre class="bg-slate-900 text-slate-100 p-4 rounded-lg overflow-x-auto text-sm"><code>{
  "name": "Start Dev Server",
  "command": "npm run dev",
  "hotkey": "Ctrl+Shift+D"
}</code></pre>
        
        <p class="mb-3"><strong>Supported hotkey formats:</strong></p>
        <ul class="list-disc list-inside space-y-1 text-sm">
          <li><code class="bg-slate-200 px-1 rounded">Ctrl+Shift+D</code> - Standard combination</li>
          <li><code class="bg-slate-200 px-1 rounded">Cmd+Option+S</code> - macOS specific</li>
          <li><code class="bg-slate-200 px-1 rounded">Alt+F1</code> - Function keys</li>
        </ul>
        
        <div class="bg-yellow-50 border-l-4 border-yellow-400 p-4 mt-4">
          <p class="text-sm text-yellow-700">
            <strong>Note:</strong> Ensure no other applications are using the same hotkey combination to avoid conflicts.
          </p>
        </div>
      `,
    },
  ],
  advancedFeatures: [
    {
      id: 'af-1',
      question: 'What are Switch Commands?',
      answer: `
        <p class="mb-3">Switch commands allow you to toggle system functions with background execution. They provide visual feedback in the menu and automatically detect current status.</p>
        
        <p class="mb-3"><strong>Features:</strong></p>
        <ul class="list-disc list-inside space-y-1 mb-3">
          <li>Background execution without opening terminal windows</li>
          <li>Automatic status detection</li>
          <li>Visual feedback showing enabled/disabled status</li>
          <li>Cross-platform support</li>
        </ul>
        
        <p class="mb-3"><strong>Example:</strong></p>
        <pre class="bg-slate-900 text-slate-100 p-4 rounded-lg overflow-x-auto text-sm"><code>{
  "name": "📶 Toggle WiFi",
  "command": "networksetup -setairportpower en0 toggle",
  "switch": "networksetup -getairportpower en0 | grep -q 'On' && echo 'true' || echo 'false'"
}</code></pre>
      `,
    },
    {
      id: 'af-2',
      question: 'What are Monitoring Commands?',
      answer: `
        <p class="mb-3">Monitoring commands display real-time system information directly in the menu. They execute silently in the background and show the results in the menu item name.</p>
        
        <p class="mb-3"><strong>Features:</strong></p>
        <ul class="list-disc list-inside space-y-1 mb-3">
          <li>Real-time display of system metrics</li>
          <li>Background execution without terminal windows</li>
          <li>Click to execute additional commands</li>
          <li>Cross-platform support</li>
        </ul>
        
        <p class="mb-3"><strong>Example:</strong></p>
        <pre class="bg-slate-900 text-slate-100 p-4 rounded-lg overflow-x-auto text-sm"><code>{
  "name": "CPU Usage",
  "monitor": "top -l 1 | grep 'CPU usage' | awk '{print $3}'",
  "command": "top -o cpu"
}</code></pre>
        
        <p class="mb-3"><strong>Best Practices:</strong></p>
        <ul class="list-disc list-inside space-y-1">
          <li>Keep monitor commands fast and lightweight</li>
          <li>Use simple output formats (single values work best)</li>
          <li>Add appropriate units (%, MB, GB, etc.) to the output</li>
          <li>Handle errors gracefully</li>
        </ul>
      `,
    },
    {
      id: 'af-3',
      question: 'How do I use the CLI interface?',
      answer: `
        <p class="mb-3">SwitchShuttle provides a command-line interface for executing commands directly from the terminal.</p>
        
        <p class="mb-3"><strong>Basic Usage:</strong></p>
        <div class="space-y-2">
          <code class="block bg-slate-200 p-2 rounded text-sm">switch-shuttle execute &lt;command-id&gt;</code>
          <code class="block bg-slate-200 p-2 rounded text-sm">switch-shuttle list</code>
          <code class="block bg-slate-200 p-2 rounded text-sm">switch-shuttle show &lt;command-id&gt;</code>
        </div>
        
        <p class="mb-3"><strong>Advanced Usage:</strong></p>
        <div class="space-y-2">
          <code class="block bg-slate-200 p-2 rounded text-sm">switch-shuttle execute &lt;command-id&gt; --param value</code>
          <code class="block bg-slate-200 p-2 rounded text-sm">switch-shuttle list --filter "server"</code>
          <code class="block bg-slate-200 p-2 rounded text-sm">switch-shuttle export &gt; config.json</code>
        </div>
      `,
    },
    {
      id: 'af-4',
      question: 'How do I schedule commands with cron?',
      answer: `
        <p class="mb-3">SwitchShuttle supports scheduled command execution using standard cron expressions. This allows you to automate repetitive tasks.</p>
        
        <p class="mb-3"><strong>Features:</strong></p>
        <ul class="list-disc list-inside space-y-1 mb-3">
          <li>Standard cron format support</li>
          <li>Background execution without terminal windows</li>
          <li>Persistent scheduling (continues when menu is closed)</li>
          <li>Cross-platform compatibility</li>
        </ul>
        
        <p class="mb-3"><strong>Example:</strong></p>
        <pre class="bg-slate-900 text-slate-100 p-4 rounded-lg overflow-x-auto text-sm"><code>{
  "name": "🔄 Auto Backup",
  "commands": [
    "rsync -av /source/ /backup/"
  ],
  "scheduler": "0 2 * * *",
  "background": true,
  "hotkey": "Ctrl+Shift+B"
}</code></pre>
        
        <p class="mb-3"><strong>Common cron patterns:</strong></p>
        <div class="grid md:grid-cols-2 gap-4">
          <div class="bg-slate-50 p-3 rounded-lg">
            <code class="text-sm font-medium">* * * * *</code>
            <p class="text-xs text-slate-600 mt-1">Every minute</p>
          </div>
          <div class="bg-slate-50 p-3 rounded-lg">
            <code class="text-sm font-medium">0 */2 * * *</code>
            <p class="text-xs text-slate-600 mt-1">Every 2 hours</p>
          </div>
          <div class="bg-slate-50 p-3 rounded-lg">
            <code class="text-sm font-medium">0 9 * * 1-5</code>
            <p class="text-xs text-slate-600 mt-1">Weekdays at 9 AM</p>
          </div>
          <div class="bg-slate-50 p-3 rounded-lg">
            <code class="text-sm font-medium">0 0 1 * *</code>
            <p class="text-xs text-slate-600 mt-1">First day of month</p>
          </div>
        </div>
        
        <div class="bg-blue-50 border-l-4 border-blue-400 p-4 mt-4">
          <p class="text-sm text-blue-700">
            <strong>Tip:</strong> Use <code class="bg-blue-100 px-1 rounded">"background": true</code> for scheduled commands to run silently without opening terminal windows.
          </p>
        </div>
      `,
    },
    {
      id: 'af-5',
      question: 'How do I control background execution?',
      answer: `
        <p class="mb-3">SwitchShuttle provides flexible control over how commands are executed - either in the background using ConsolePool or through normal terminal execution.</p>
        
        <p class="mb-3"><strong>Background Execution Options:</strong></p>
        <div class="space-y-3">
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">ConsolePool (Background)</h4>
            <p class="text-sm text-slate-600 mb-2">Commands run silently without opening terminal windows. Perfect for scheduled tasks and system operations.</p>
            <pre class="bg-slate-900 text-slate-100 p-2 rounded text-xs"><code>"background": true</code></pre>
          </div>
          
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">Normal Terminal</h4>
            <p class="text-sm text-slate-600 mb-2">Commands open in visible terminal windows. Best for interactive commands and development work.</p>
            <pre class="bg-slate-900 text-slate-100 p-2 rounded text-xs"><code>"background": false</code></pre>
          </div>
          
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">Auto-Detect</h4>
            <p class="text-sm text-slate-600 mb-2">SwitchShuttle automatically chooses the best execution method based on command type.</p>
            <pre class="bg-slate-900 text-slate-100 p-2 rounded text-xs"><code>"background": null  // or omit the field</code></pre>
          </div>
        </div>
        
        <p class="mb-3"><strong>Example configurations:</strong></p>
        <div class="grid md:grid-cols-2 gap-4">
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">Background Server</h4>
            <pre class="bg-slate-900 text-slate-100 p-2 rounded text-xs"><code>{
  "name": "🚀 Start Server",
  "commands": ["npm run dev"],
  "background": true
}</code></pre>
          </div>
          
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">Interactive Terminal</h4>
            <pre class="bg-slate-900 text-slate-100 p-2 rounded text-xs"><code>{
  "name": "🔧 Debug Mode",
  "commands": ["node --inspect app.js"],
  "background": false
}</code></pre>
          </div>
        </div>
        
        <div class="bg-yellow-50 border-l-4 border-yellow-400 p-4 mt-4">
          <p class="text-sm text-yellow-700">
            <strong>Note:</strong> Switch commands and monitor commands always use background execution regardless of the background setting.
          </p>
        </div>
      `,
    },
  ],
  troubleshooting: [
    {
      id: 'ts-1',
      question: 'My hotkeys are not working. What should I do?',
      answer: `
        <p class="mb-3">Hotkey conflicts are a common issue. Here's how to resolve them:</p>
        
        <ol class="list-decimal list-inside space-y-2 mb-3">
          <li>Check if other applications are using the same hotkey combination</li>
          <li>Try a different hotkey combination (e.g., Ctrl+Shift+Alt+D)</li>
          <li>Restart SwitchShuttle after changing hotkeys</li>
          <li>Check system permissions for accessibility features</li>
        </ol>
        
        <div class="bg-blue-50 border-l-4 border-blue-400 p-4">
          <p class="text-sm text-blue-700">
            <strong>Tip:</strong> On macOS, you may need to grant accessibility permissions to SwitchShuttle in System Preferences > Security & Privacy > Privacy > Accessibility.
          </p>
        </div>
      `,
    },
    {
      id: 'ts-2',
      question: 'My configuration is not loading. How do I fix this?',
      answer: `
        <p class="mb-3">Configuration loading issues can be caused by several factors:</p>
        
        <ul class="list-disc list-inside space-y-2 mb-3">
          <li><strong>JSON Syntax Errors:</strong> Use a JSON validator to check your configuration file syntax</li>
          <li><strong>File Location:</strong> Ensure configuration files are in the correct directory</li>
          <li><strong>File Permissions:</strong> Check that the application can read the configuration files</li>
          <li><strong>Application Restart:</strong> Restart SwitchShuttle after making configuration changes</li>
        </ul>
        
        <p class="mb-3"><strong>Common JSON errors:</strong></p>
        <ul class="list-disc list-inside space-y-1 text-sm">
          <li>Missing commas between objects</li>
          <li>Unclosed brackets or braces</li>
          <li>Invalid string escaping</li>
          <li>Trailing commas in arrays or objects</li>
        </ul>
      `,
    },
    {
      id: 'ts-3',
      question:
          'The terminal is not opening or commands are not executing. What is wrong?',
      answer: `
        <p class="mb-3">Terminal execution issues can be resolved by checking these common causes:</p>
        
        <div class="grid md:grid-cols-2 gap-4 mb-3">
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">Terminal Application</h4>
            <ul class="list-disc list-inside space-y-1 text-sm text-slate-600">
              <li>Verify the terminal application is installed</li>
              <li>Check the terminal path in configuration</li>
              <li>Try different terminal applications</li>
            </ul>
          </div>
          
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">Command Path</h4>
            <ul class="list-disc list-inside space-y-1 text-sm text-slate-600">
              <li>Ensure commands are in system PATH</li>
              <li>Use absolute paths for custom scripts</li>
              <li>Check command syntax and parameters</li>
            </ul>
          </div>
        </div>
        
        <p class="mb-3"><strong>Supported terminals:</strong></p>
        <ul class="list-disc list-inside space-y-1 text-sm">
          <li>macOS: Terminal, iTerm2, Alacritty, Hyper</li>
          <li>Windows: Command Prompt, PowerShell, Windows Terminal</li>
          <li>Linux: gnome-terminal, konsole, xterm, alacritty</li>
        </ul>
      `,
    },
    {
      id: 'ts-4',
      question: 'How do I get help or report issues?',
      answer: `
        <p class="mb-3">There are several ways to get help and support:</p>
        
        <div class="space-y-3">
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">GitHub Resources</h4>
            <ul class="space-y-2 text-sm text-slate-600">
              <li><strong>Issues:</strong> <a href="https://github.com/s00d/switchshuttle/issues" class="text-blue-600 hover:text-blue-800">Report bugs and request features</a></li>
              <li><strong>Documentation:</strong> <a href="https://github.com/s00d/switchshuttle#readme" class="text-blue-600 hover:text-blue-800">Complete documentation</a></li>
              <li><strong>Discussions:</strong> <a href="https://github.com/s00d/switchshuttle/discussions" class="text-blue-600 hover:text-blue-800">Community discussions</a></li>
            </ul>
          </div>
          
          <div class="bg-slate-50 p-4 rounded-lg">
            <h4 class="font-medium text-slate-700 mb-2">Debugging</h4>
            <ul class="space-y-2 text-sm text-slate-600">
              <li>Check application logs for error messages</li>
              <li>Enable debug mode if available</li>
              <li>Test commands manually in terminal</li>
              <li>Validate JSON configuration syntax</li>
            </ul>
          </div>
        </div>
      `,
    },
  ],
};

const filteredFaqs = computed(() => {
  const query = searchQuery.value.toLowerCase();

  if (!query) {
    return faqData;
  }

  const filterCategory = (category: any[]) => {
    return category.filter(
        item =>
            item.question.toLowerCase().includes(query) ||
            item.answer.toLowerCase().includes(query)
    );
  };

  return {
    gettingStarted: filterCategory(faqData.gettingStarted),
    configuration: filterCategory(faqData.configuration),
    commandTypes: filterCategory(faqData.commandTypes),
    advancedFeatures: filterCategory(faqData.advancedFeatures),
    troubleshooting: filterCategory(faqData.troubleshooting),
  };
});

const toggleFaq = (id: string) => {
  const index = openFaqs.value.indexOf(id);
  if (index > -1) {
    openFaqs.value.splice(index, 1);
  } else {
    openFaqs.value.push(id);
  }
};
</script>

<style scoped>
/* Smooth transitions for FAQ toggles */
.faq-content {
  transition: all 0.3s ease-in-out;
}

/* Custom scrollbar for code blocks */
pre code {
  scrollbar-width: thin;
  scrollbar-color: #64748b #1e293b;
}

pre code::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

pre code::-webkit-scrollbar-track {
  background: #1e293b;
}

pre code::-webkit-scrollbar-thumb {
  background: #64748b;
  border-radius: 3px;
}

pre code::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}
</style>
