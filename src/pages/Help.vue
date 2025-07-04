<template>
  <div class="container mx-auto px-6 py-8 max-w-4xl">
    <div class="bg-white rounded-lg shadow-lg p-8">
      <h1 class="text-3xl font-bold text-slate-900 mb-8">Help & Documentation</h1>
      
      <!-- Table of Contents -->
      <div class="mb-8 p-4 bg-slate-50 rounded-lg">
        <h2 class="text-lg font-semibold text-slate-900 mb-3">Table of Contents</h2>
        <ul class="space-y-1 text-sm">
          <li><a href="#getting-started" class="text-blue-600 hover:text-blue-800">Getting Started</a></li>
          <li><a href="#visual-editor" class="text-blue-600 hover:text-blue-800">Visual Editor Guide</a></li>
          <li><a href="#json-configuration" class="text-blue-600 hover:text-blue-800">JSON Configuration</a></li>
          <li><a href="#command-types" class="text-blue-600 hover:text-blue-800">Command Types</a></li>
          <li><a href="#configuration-management" class="text-blue-600 hover:text-blue-800">Configuration Management</a></li>
          <li><a href="#switch-commands" class="text-blue-600 hover:text-blue-800">Switch Commands</a></li>
          <li><a href="#cli-usage" class="text-blue-600 hover:text-blue-800">CLI Usage</a></li>
          <li><a href="#troubleshooting" class="text-blue-600 hover:text-blue-800">Troubleshooting</a></li>
        </ul>
      </div>

      <!-- Getting Started -->
      <section id="getting-started" class="mb-12">
        <h2 class="text-2xl font-bold text-slate-900 mb-4">Getting Started</h2>
        
        <div class="space-y-6">
          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Installation</h3>
            <div class="bg-slate-50 p-4 rounded-lg">
              <h4 class="font-medium text-slate-700 mb-2">macOS (Recommended)</h4>
              <code class="block bg-slate-200 p-2 rounded text-sm">brew tap s00d/switchshuttle && brew install --cask switchshuttle</code>
              
              <h4 class="font-medium text-slate-700 mt-4 mb-2">Manual Download</h4>
              <p class="text-sm text-slate-600">Download the latest release from <a href="https://github.com/s00d/switchshuttle/releases" class="text-blue-600 hover:text-blue-800">GitHub Releases</a> for your platform.</p>
            </div>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">First Launch</h3>
            <ol class="list-decimal list-inside space-y-2 text-sm text-slate-700">
              <li>Launch SwitchShuttle - it will appear in your system tray</li>
              <li>Right-click the tray icon to access the menu</li>
              <li>Click "Edit Configuration" to open the visual editor</li>
              <li>Add your first command or import an existing configuration</li>
              <li>Save and restart the application</li>
            </ol>
          </div>
        </div>
      </section>

      <!-- Visual Editor Guide -->
      <section id="visual-editor" class="mb-12">
        <h2 class="text-2xl font-bold text-slate-900 mb-4">Visual Editor Guide</h2>
        
        <div class="space-y-6">
          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Opening the Editor</h3>
            <p class="text-sm text-slate-700 mb-4">The visual editor provides an intuitive interface for managing your configurations without editing JSON directly.</p>
            
            <div class="bg-blue-50 border-l-4 border-blue-400 p-4 mb-4">
              <div class="flex">
                <div class="flex-shrink-0">
                  <svg class="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
                  </svg>
                </div>
                <div class="ml-3">
                  <p class="text-sm text-blue-700">
                    <strong>Tip:</strong> You can access the editor by right-clicking the system tray icon and selecting "Edit Configuration"
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Configuration Management</h3>
            <div class="grid md:grid-cols-2 gap-4">
              <div class="bg-slate-50 p-4 rounded-lg">
                <h4 class="font-medium text-slate-700 mb-2">Adding New Configurations</h4>
                <ol class="list-decimal list-inside space-y-1 text-sm text-slate-600">
                  <li>Click "Add Configuration" button</li>
                  <li>Fill in the basic settings (terminal, launch mode)</li>
                  <li>Add commands using the "Add Command" button</li>
                  <li>Configure each command's properties</li>
                  <li>Save the configuration</li>
                </ol>
              </div>
              
              <div class="bg-slate-50 p-4 rounded-lg">
                <h4 class="font-medium text-slate-700 mb-2">Editing Existing Configurations</h4>
                <ol class="list-decimal list-inside space-y-1 text-sm text-slate-600">
                  <li>Select the configuration from the dropdown</li>
                  <li>Modify settings in the form fields</li>
                  <li>Edit commands in the commands list</li>
                  <li>Use the toggle to enable/disable configurations</li>
                  <li>Save changes</li>
                </ol>
              </div>
            </div>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Command Editor</h3>
            <div class="bg-slate-50 p-4 rounded-lg">
              <h4 class="font-medium text-slate-700 mb-2">Adding Commands</h4>
              <div class="space-y-3 text-sm text-slate-600">
                <div>
                  <strong>Basic Command:</strong>
                  <ul class="list-disc list-inside ml-4 mt-1">
                    <li>Enter the command name</li>
                    <li>Add the terminal command</li>
                    <li>Optionally set a hotkey</li>
                  </ul>
                </div>
                
                <div>
                  <strong>Submenu Command:</strong>
                  <ul class="list-disc list-inside ml-4 mt-1">
                    <li>Select "Submenu" type</li>
                    <li>Add child commands</li>
                    <li>Organize in hierarchical structure</li>
                  </ul>
                </div>
                
                <div>
                  <strong>Switch Command:</strong>
                  <ul class="list-disc list-inside ml-4 mt-1">
                    <li>Select "Switch" type</li>
                    <li>Add the toggle command</li>
                    <li>Add the status check command</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- JSON Configuration -->
      <section id="json-configuration" class="mb-12">
        <h2 class="text-2xl font-bold text-slate-900 mb-4">JSON Configuration</h2>
        
        <div class="space-y-6">
          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Configuration Files Location</h3>
            <div class="bg-slate-50 p-4 rounded-lg">
              <div class="grid md:grid-cols-3 gap-4 text-sm">
                <div>
                  <strong class="text-slate-700">macOS/Linux:</strong>
                  <code class="block bg-slate-200 p-2 rounded mt-1">~/.config/switch-shuttle/</code>
                </div>
                <div>
                  <strong class="text-slate-700">Windows:</strong>
                  <code class="block bg-slate-200 p-2 rounded mt-1">C:\Users\&lt;Username&gt;\AppData\Roaming\switch-shuttle\</code>
                </div>
                <div>
                  <strong class="text-slate-700">File Format:</strong>
                  <code class="block bg-slate-200 p-2 rounded mt-1">*.json</code>
                </div>
              </div>
            </div>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Basic Configuration Structure</h3>
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
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Configuration Parameters</h3>
            <div class="overflow-x-auto">
              <table class="min-w-full bg-white border border-slate-200 rounded-lg">
                <thead class="bg-slate-50">
                  <tr>
                    <th class="px-4 py-2 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">Parameter</th>
                    <th class="px-4 py-2 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">Type</th>
                    <th class="px-4 py-2 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">Default</th>
                    <th class="px-4 py-2 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">Description</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-slate-200">
                  <tr>
                    <td class="px-4 py-2 text-sm font-medium text-slate-900">terminal</td>
                    <td class="px-4 py-2 text-sm text-slate-600">String</td>
                    <td class="px-4 py-2 text-sm text-slate-600">"terminal"</td>
                    <td class="px-4 py-2 text-sm text-slate-600">Terminal application to use</td>
                  </tr>
                  <tr>
                    <td class="px-4 py-2 text-sm font-medium text-slate-900">launch_in</td>
                    <td class="px-4 py-2 text-sm text-slate-600">String</td>
                    <td class="px-4 py-2 text-sm text-slate-600">"current"</td>
                    <td class="px-4 py-2 text-sm text-slate-600">Where to launch commands</td>
                  </tr>
                  <tr>
                    <td class="px-4 py-2 text-sm font-medium text-slate-900">title</td>
                    <td class="px-4 py-2 text-sm text-slate-600">String</td>
                    <td class="px-4 py-2 text-sm text-slate-600">-</td>
                    <td class="px-4 py-2 text-sm text-slate-600">Window/tab title</td>
                  </tr>
                  <tr>
                    <td class="px-4 py-2 text-sm font-medium text-slate-900">enabled</td>
                    <td class="px-4 py-2 text-sm text-slate-600">Boolean</td>
                    <td class="px-4 py-2 text-sm text-slate-600">true</td>
                    <td class="px-4 py-2 text-sm text-slate-600">Whether to load this configuration</td>
                  </tr>
                  <tr>
                    <td class="px-4 py-2 text-sm font-medium text-slate-900">commands</td>
                    <td class="px-4 py-2 text-sm text-slate-600">Array</td>
                    <td class="px-4 py-2 text-sm text-slate-600">[]</td>
                    <td class="px-4 py-2 text-sm text-slate-600">List of command configurations</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </section>

      <!-- Command Types -->
      <section id="command-types" class="mb-12">
        <h2 class="text-2xl font-bold text-slate-900 mb-4">Command Types</h2>
        
        <div class="space-y-6">
          <div class="grid md:grid-cols-2 gap-6">
            <div class="bg-slate-50 p-6 rounded-lg">
              <h3 class="text-lg font-semibold text-slate-800 mb-3">Basic Commands</h3>
              <p class="text-sm text-slate-600 mb-4">Simple commands that execute in the terminal.</p>
              <pre class="bg-slate-900 text-slate-100 p-3 rounded text-xs overflow-x-auto"><code>{
  "name": "Start Server",
  "command": "npm run dev",
  "hotkey": "Ctrl+Shift+D"
}</code></pre>
            </div>
            
            <div class="bg-slate-50 p-6 rounded-lg">
              <h3 class="text-lg font-semibold text-slate-800 mb-3">Multiple Commands</h3>
              <p class="text-sm text-slate-600 mb-4">Execute a sequence of commands.</p>
              <pre class="bg-slate-900 text-slate-100 p-3 rounded text-xs overflow-x-auto"><code>{
  "name": "Full Dev Cycle",
  "commands": [
    "git pull origin main",
    "npm install",
    "npm run build"
  ]
}</code></pre>
            </div>
            
            <div class="bg-slate-50 p-6 rounded-lg">
              <h3 class="text-lg font-semibold text-slate-800 mb-3">Submenus</h3>
              <p class="text-sm text-slate-600 mb-4">Organize commands in hierarchical menus.</p>
              <pre class="bg-slate-900 text-slate-100 p-3 rounded text-xs overflow-x-auto"><code>{
  "name": "Docker Operations",
  "submenu": [
    {
      "name": "Start Services",
      "command": "docker-compose up -d"
    }
  ]
}</code></pre>
            </div>
            
            <div class="bg-slate-50 p-6 rounded-lg">
              <h3 class="text-lg font-semibold text-slate-800 mb-3">Dynamic Inputs</h3>
              <p class="text-sm text-slate-600 mb-4">Commands with user input prompts.</p>
              <pre class="bg-slate-900 text-slate-100 p-3 rounded text-xs overflow-x-auto"><code>{
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
        </div>
      </section>

      <!-- Configuration Management -->
      <section id="configuration-management" class="mb-12">
        <h2 class="text-2xl font-bold text-slate-900 mb-4">Configuration Management</h2>
        
        <div class="space-y-6">
          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Enabling/Disabling Configurations</h3>
            <p class="text-sm text-slate-700 mb-4">You can enable or disable individual configuration files to control which commands are available in the system tray menu.</p>
            
            <div class="bg-green-50 border-l-4 border-green-400 p-4 mb-4">
              <div class="flex">
                <div class="flex-shrink-0">
                  <svg class="h-5 w-5 text-green-400" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                  </svg>
                </div>
                <div class="ml-3">
                  <p class="text-sm text-green-700">
                    <strong>Benefits:</strong> Temporarily disable configurations without deletion, test configurations during development, organize multiple configurations.
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Visual Editor Method</h3>
            <ol class="list-decimal list-inside space-y-2 text-sm text-slate-700">
              <li>Open the configuration editor</li>
              <li>Use the toggle switch in the "Configuration Status" section</li>
              <li>Enabled configurations will be loaded and available in the menu</li>
              <li>Disabled configurations will be ignored</li>
            </ol>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">JSON Method</h3>
            <pre class="bg-slate-900 text-slate-100 p-4 rounded-lg overflow-x-auto text-sm"><code>{
  "terminal": "iterm",
  "launch_in": "current",
  "title": "My Commands",
  "enabled": true,  // Set to false to disable
  "commands": [
    {
      "name": "Example Command",
      "command": "echo Hello World"
    }
  ]
}</code></pre>
          </div>
        </div>
      </section>

      <!-- Switch Commands -->
      <section id="switch-commands" class="mb-12">
        <h2 class="text-2xl font-bold text-slate-900 mb-4">Switch Commands</h2>
        
        <div class="space-y-6">
          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Overview</h3>
            <p class="text-sm text-slate-700 mb-4">Switch commands allow you to toggle system functions with background execution. They provide visual feedback in the menu and automatically detect current status.</p>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Features</h3>
            <div class="grid md:grid-cols-2 gap-4">
              <div class="bg-slate-50 p-4 rounded-lg">
                <h4 class="font-medium text-slate-700 mb-2">Background Execution</h4>
                <p class="text-sm text-slate-600">Commands run silently without opening a terminal window.</p>
              </div>
              
              <div class="bg-slate-50 p-4 rounded-lg">
                <h4 class="font-medium text-slate-700 mb-2">Status Detection</h4>
                <p class="text-sm text-slate-600">Automatically detects the current state of the function.</p>
              </div>
              
              <div class="bg-slate-50 p-4 rounded-lg">
                <h4 class="font-medium text-slate-700 mb-2">Visual Feedback</h4>
                <p class="text-sm text-slate-600">Shows enabled/disabled status in the menu.</p>
              </div>
              
              <div class="bg-slate-50 p-4 rounded-lg">
                <h4 class="font-medium text-slate-700 mb-2">Cross-Platform</h4>
                <p class="text-sm text-slate-600">Works on macOS, Windows, and Linux.</p>
              </div>
            </div>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Example Configuration</h3>
            <pre class="bg-slate-900 text-slate-100 p-4 rounded-lg overflow-x-auto text-sm"><code>{
  "name": "🔧 System Controls",
  "submenu": [
    {
      "name": "📶 Toggle WiFi",
      "command": "networksetup -setairportpower en0 toggle",
      "switch": "networksetup -getairportpower en0 | grep -q 'On' && echo 'true' || echo 'false'"
    },
    {
      "name": "🔊 Toggle Bluetooth",
      "command": "blueutil -p toggle",
      "switch": "blueutil -p | grep -q '1' && echo 'true' || echo 'false'"
    },
    {
      "name": "🌙 Toggle Dark Mode",
      "command": "osascript -e 'tell app \"System Events\" to tell appearance preferences to set dark mode to not dark mode'",
      "switch": "osascript -e 'tell app \"System Events\" to tell appearance preferences to get dark mode'"
    }
  ]
}</code></pre>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Creating Switch Commands</h3>
            <div class="bg-slate-50 p-4 rounded-lg">
              <h4 class="font-medium text-slate-700 mb-2">Required Parameters</h4>
              <ul class="space-y-2 text-sm text-slate-600">
                <li><strong>command:</strong> The command to execute when toggling</li>
                <li><strong>switch:</strong> Command that returns "true" or "false" to check current status</li>
                <li><strong>name:</strong> Display name in the menu</li>
              </ul>
            </div>
          </div>
        </div>
      </section>

      <!-- CLI Usage -->
      <section id="cli-usage" class="mb-12">
        <h2 class="text-2xl font-bold text-slate-900 mb-4">CLI Usage</h2>
        
        <div class="space-y-6">
          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Command Line Interface</h3>
            <p class="text-sm text-slate-700 mb-4">SwitchShuttle provides a command-line interface for executing commands directly from the terminal.</p>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Basic Usage</h3>
            <div class="bg-slate-50 p-4 rounded-lg">
              <h4 class="font-medium text-slate-700 mb-2">Execute a command by ID</h4>
              <code class="block bg-slate-200 p-2 rounded text-sm">switch-shuttle execute &lt;command-id&gt;</code>
              
              <h4 class="font-medium text-slate-700 mt-4 mb-2">List all available commands</h4>
              <code class="block bg-slate-200 p-2 rounded text-sm">switch-shuttle list</code>
              
              <h4 class="font-medium text-slate-700 mt-4 mb-2">Show command details</h4>
              <code class="block bg-slate-200 p-2 rounded text-sm">switch-shuttle show &lt;command-id&gt;</code>
            </div>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Advanced Usage</h3>
            <div class="bg-slate-50 p-4 rounded-lg">
              <h4 class="font-medium text-slate-700 mb-2">Execute with parameters</h4>
              <code class="block bg-slate-200 p-2 rounded text-sm">switch-shuttle execute &lt;command-id&gt; --param value</code>
              
              <h4 class="font-medium text-slate-700 mt-4 mb-2">Filter commands</h4>
              <code class="block bg-slate-200 p-2 rounded text-sm">switch-shuttle list --filter "server"</code>
              
              <h4 class="font-medium text-slate-700 mt-4 mb-2">Export configuration</h4>
              <code class="block bg-slate-200 p-2 rounded text-sm">switch-shuttle export &gt; config.json</code>
            </div>
          </div>
        </div>
      </section>

      <!-- Troubleshooting -->
      <section id="troubleshooting" class="mb-12">
        <h2 class="text-2xl font-bold text-slate-900 mb-4">Troubleshooting</h2>
        
        <div class="space-y-6">
          <div class="grid md:grid-cols-2 gap-6">
            <div class="bg-red-50 border-l-4 border-red-400 p-4 rounded-lg">
              <h3 class="text-lg font-semibold text-red-800 mb-2">Common Issues</h3>
              <div class="space-y-3 text-sm text-red-700">
                <div>
                  <strong>Hotkey conflicts:</strong>
                  <p>Ensure no other applications are using the same hotkey combination.</p>
                </div>
                <div>
                  <strong>Terminal not found:</strong>
                  <p>Verify the terminal application is installed and the path is correct.</p>
                </div>
                <div>
                  <strong>Configuration not loading:</strong>
                  <p>Check JSON syntax and ensure the configuration file is in the correct location.</p>
                </div>
              </div>
            </div>
            
            <div class="bg-blue-50 border-l-4 border-blue-400 p-4 rounded-lg">
              <h3 class="text-lg font-semibold text-blue-800 mb-2">Solutions</h3>
              <div class="space-y-3 text-sm text-blue-700">
                <div>
                  <strong>Restart the application:</strong>
                  <p>After making configuration changes, restart SwitchShuttle to apply them.</p>
                </div>
                <div>
                  <strong>Check logs:</strong>
                  <p>Look for error messages in the application logs or system console.</p>
                </div>
                <div>
                  <strong>Validate JSON:</strong>
                  <p>Use a JSON validator to check your configuration file syntax.</p>
                </div>
              </div>
            </div>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-slate-800 mb-2">Getting Help</h3>
            <div class="bg-slate-50 p-4 rounded-lg">
              <ul class="space-y-2 text-sm text-slate-600">
                <li><strong>GitHub Issues:</strong> <a href="https://github.com/s00d/switchshuttle/issues" class="text-blue-600 hover:text-blue-800">Report bugs and request features</a></li>
                <li><strong>Documentation:</strong> <a href="https://github.com/s00d/switchshuttle#readme" class="text-blue-600 hover:text-blue-800">Complete documentation</a></li>
                <li><strong>Discussions:</strong> <a href="https://github.com/s00d/switchshuttle/discussions" class="text-blue-600 hover:text-blue-800">Community discussions</a></li>
              </ul>
            </div>
          </div>
        </div>
      </section>

      <!-- Footer -->
      <div class="mt-12 pt-8 border-t border-slate-200">
        <p class="text-sm text-slate-500 text-center">
          Need more help? Visit our <a href="https://github.com/s00d/switchshuttle" class="text-blue-600 hover:text-blue-800">GitHub repository</a> for additional resources.
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// No additional logic needed for this help page
</script>

<style scoped>
/* Smooth scrolling for anchor links */
html {
  scroll-behavior: smooth;
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