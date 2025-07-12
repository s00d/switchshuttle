export interface MenuItem {
  name: string
  icon?: string
  hotkey?: string
  checkmark?: boolean
  action: 'terminal' | 'notification' | 'help' | 'config' | 'toggle' | 'showHelp' | 'showAbout' | 'showHomepage' | 'showJsonEditor' | 'showConfigFolder' | 'refresh' | 'separator' | 'hideMenuBar'
  command?: string
  title?: string
  message?: string
  config?: string
  submenu?: MenuItem[]
  usage?: string
  terminalOutput?: string
}

export const userCommands: MenuItem[] = [
  { 
    name: 'npm run dev', 
    hotkey: '⌘D', 
    icon: 'terminal.png',
    action: 'terminal',
    command: 'npm run dev',
    terminalOutput: `> switchshuttle@1.0.0 dev
> nuxt dev

  Nuxt 3.8.0 with Nitro 3.0.0

  ➜ Local:    http://localhost:3000/
  ➜ Network:  http://192.168.1.100:3000/
  ➜ DevTools: http://localhost:3000/__nuxt_devtools__

  ✓ Vite client warmed up in 234ms
  ✓ Nitro built in 1.2s
  ✓ Vite built in 2.1s
  ✓ Server ready in 3.4s

  ℹ Vite 5.0.0
  ℹ ready in 3.4s`
  },
  { 
    name: 'npm run build', 
    hotkey: '⌘B', 
    icon: 'terminal.png',
    action: 'terminal',
    command: 'npm run build',
    terminalOutput: `> switchshuttle@1.0.0 build
> nuxt build

  Nuxt 3.8.0 with Nitro 3.0.0

  ✓ Builder initialized
  ✓ Target: static
  ✓ Builder built in 12.3s
  ✓ Nitro built in 8.7s
  ✓ Generated public files in 1.2s

  📦 Output directory: .output/public
  📦 Static files: 45 files, 2.3 MB`
  },
  { 
    name: 'git status', 
    hotkey: '⌘G', 
    icon: 'terminal.png',
    action: 'terminal',
    command: 'git status',
    terminalOutput: `On branch main
Your branch is up to date with 'origin/main'.

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   pages/[locale].vue
        modified:   components/MenuBar.vue
        new file:   config/menu.ts
        new file:   config/terminal.ts

no changes added to commit (use "git add" and/or "git commit -a")`
  },
  { 
    name: 'docker ps', 
    hotkey: '⌘K', 
    icon: 'terminal.png',
    action: 'terminal',
    command: 'docker ps',
    terminalOutput: `CONTAINER ID   IMAGE     COMMAND   CREATED   STATUS    PORTS     NAMES
No containers are running`
  }
]

export const ramCommands: MenuItem[] = [
  {
    name: 'Show Memory Info',
    icon: 'terminal.png',
    action: 'terminal',
    command: 'free -h',
    terminalOutput: `              total        used        free      shared  buff/cache   available
Mem:           15Gi       8.2Gi       2.1Gi       1.2Gi       4.7Gi       5.8Gi
Swap:         8.0Gi       0.0Ki       8.0Gi`
  },
  {
    name: 'Memory Usage',
    icon: 'terminal.png',
    action: 'terminal',
    command: 'top -l 1 | grep PhysMem',
    terminalOutput: `PhysMem: 15.0G used (8.2G wired), 2.1G unused.`
  },
  {
    name: 'VM Statistics',
    icon: 'terminal.png',
    action: 'terminal',
    command: 'vm_stat',
    terminalOutput: `Mach Virtual Memory Statistics: (page size of 4096 bytes)
Pages free:                          524288.
Pages active:                       2097152.
Pages inactive:                     1048576.
Pages speculative:                    262144.
Pages throttled:                           0.
Pages wired down:                   1048576.
Pages purgeable:                     524288.
"Translation faults":              20971520.
Pages copy-on-write:               10485760.
Pages zero filled:                 20971520.
Pages reactivated:                  5242880.
Pages purged:                       2621440.
File-backed pages:                 2097152.
Anonymous pages:                   1048576.
Pages stored in compressor:        2097152.
Pages occupied by compressor:      1048576.
Decompressions:                    5242880.
Compressions:                      20971520.
Pageins:                           10485760.
Pageouts:                           2621440.
Swapins:                                  0.
Swapouts:                                 0.`
  },
  {
    name: 'Top Processes',
    icon: 'terminal.png',
    action: 'terminal',
    command: 'ps aux | head -10',
    terminalOutput: `USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  0.0  0.0  16844  1076 ?        Ss   Jan01   0:01 /sbin/init
root         2  0.0  0.0      0     0 ?        S    Jan01   0:00 [kthreadd]
root         3  0.0  0.0      0     0 ?        S    Jan01   0:00 [rcu_gp]
root         4  0.0  0.0      0     0 ?        S    Jan01   0:00 [rcu_par_gp]
root         6  0.0  0.0      0     0 ?        S    Jan01   0:00 [kworker/0:0H]
root         8  0.0  0.0      0     0 ?        S    Jan01   0:00 [mm_percpu_wq]
root         9  0.0  0.0      0     0 ?        S    Jan01   0:00 [ksoftirqd/0]
root        10  0.0  0.0      0     0 ?        S    Jan01   0:00 [rcu_sched]
root        11  0.0  0.0      0     0 ?        S    Jan01   0:00 [rcu_bh]`
  },
  {
    name: 'Purge Memory',
    icon: 'terminal.png',
    action: 'terminal',
    command: 'purge',
    terminalOutput: `Memory purged successfully.`
  },
  {
    name: 'Force Purge (sudo)',
    icon: 'terminal.png',
    action: 'terminal',
    command: 'sudo purge',
    terminalOutput: `Password: ********
Memory purged with elevated privileges.`
  }
]

export const configFiles: string[] = [
  'switch-shuttle.json',
  'commands.json',
  'hotkeys.json',
  'settings.json'
]

export const configFileContents: Record<string, string> = {
  'switch-shuttle.json': `{
  "app": {
    "name": "SwitchShuttle",
    "version": "1.0.0",
    "description": "A powerful command switcher for developers"
  },
  "features": {
    "terminal": {
      "enabled": true,
      "theme": "dark",
      "fontSize": 14
    },
    "menu": {
      "enabled": true,
      "position": "top",
      "transparent": false
    },
    "notifications": {
      "enabled": true,
      "duration": 3000,
      "position": "top-right"
    }
  },
  "shortcuts": {
    "toggle": "Cmd+Shift+S",
    "menu": "Cmd+M",
    "terminal": "Cmd+T"
  },
  "themes": {
    "current": "dark",
    "available": ["light", "dark", "auto"]
  },
  "language": {
    "current": "en",
    "available": ["en", "ru", "de", "ja", "zh"]
  }
}`,
  'commands.json': `{
  "commands": [
    {
      "name": "Development Server",
      "command": "npm run dev",
      "hotkey": "Cmd+D",
      "description": "Start local development server",
      "category": "development"
    },
    {
      "name": "Build Project",
      "command": "npm run build",
      "hotkey": "Cmd+B",
      "description": "Build project for production",
      "category": "build"
    },
    {
      "name": "Run Tests",
      "command": "npm test",
      "hotkey": "Cmd+T",
      "description": "Run test suite",
      "category": "testing"
    },
    {
      "name": "Git Status",
      "command": "git status",
      "hotkey": "Cmd+G",
      "description": "Check git repository status",
      "category": "git"
    },
    {
      "name": "Docker PS",
      "command": "docker ps",
      "hotkey": "Cmd+K",
      "description": "List running containers",
      "category": "docker"
    }
  ],
  "categories": {
    "development": {
      "color": "#007AFF",
      "icon": "🚀"
    },
    "build": {
      "color": "#34C759",
      "icon": "🔨"
    },
    "testing": {
      "color": "#FF9500",
      "icon": "🧪"
    },
    "git": {
      "color": "#FF3B30",
      "icon": "📝"
    },
    "docker": {
      "color": "#5856D6",
      "icon": "🐳"
    }
  }
}`,
  'hotkeys.json': `{
  "global": {
    "toggle": "Cmd+Shift+S",
    "menu": "Cmd+M",
    "terminal": "Cmd+T",
    "help": "Cmd+?",
    "about": "Cmd+I"
  },
  "commands": {
    "development": {
      "dev": "Cmd+D",
      "build": "Cmd+B",
      "test": "Cmd+T"
    },
    "git": {
      "status": "Cmd+G",
      "commit": "Cmd+Shift+C",
      "push": "Cmd+Shift+P"
    },
    "docker": {
      "ps": "Cmd+K",
      "logs": "Cmd+L",
      "exec": "Cmd+E"
    }
  },
  "custom": {
    "quick_actions": {
      "open_project": "Cmd+O",
      "save_workspace": "Cmd+S",
      "switch_workspace": "Cmd+Shift+W"
    }
  }
}`,
  'settings.json': `{
  "appearance": {
    "theme": "dark",
    "accent_color": "#007AFF",
    "font_size": 14,
    "font_family": "SF Mono",
    "window_transparency": 0.9
  },
  "behavior": {
    "auto_refresh": true,
    "launch_at_login": false,
    "system_monitoring": true,
    "notifications": true,
    "sound_effects": false
  },
  "terminal": {
    "default_shell": "/bin/zsh",
    "font_size": 13,
    "theme": "dark",
    "cursor_blink": true,
    "scrollback_lines": 1000
  },
  "menu": {
    "position": "top",
    "transparent": false,
    "show_hotkeys": true,
    "show_icons": true,
    "animation_speed": 0.2
  },
  "notifications": {
    "duration": 3000,
    "position": "top-right",
    "sound": false,
    "show_icon": true
  }
}`
}

export const menuStructure: MenuItem[] = [
  // User Commands
  ...userCommands,
  
  // CPU Monitor
  {
    name: '🖥️ CPU: {usage}%',
    action: 'terminal',
    command: 'top -l 1 | grep CPU',
    terminalOutput: 'CPU usage: {usage}%\nUser: 15.2%\nSystem: 8.7%\nIdle: 76.1%\n\nThis is a demo version.',
    usage: '25' // Статичное значение для SSR
  },
  
  // RAM Monitor
  {
    name: '💾 RAM: {usage}',
    action: 'terminal',
    command: 'top -l 1 | grep PhysMem',
    terminalOutput: 'Physical Memory: {usage}\nWired: 4.2 GB\nActive: 6.8 GB\nInactive: 2.1 GB\nFree: 1.9 GB\n\nThis is a demo version.',
    usage: '1500 MB' // Статичное значение для SSR
  },
  
  // RAM Commands
  {
    name: '💾 RAM Commands',
    action: 'notification',
    submenu: ramCommands
  },
  
  // System Monitoring Toggle
  {
    name: '🔧 System Monitoring',
    checkmark: true,
    action: 'toggle'
  },
  
  // Separator
  {
    name: 'separator',
    action: 'separator'
  },
  
  // Edit Config with submenu
  {
    name: '🚀 Edit Config',
    icon: 'edit.png',
    action: 'notification',
    submenu: [
      ...configFiles.map(config => ({
        name: config,
        icon: 'edit.png',
        action: 'showJsonEditor' as const,
        config
      })),
      {
        name: 'Show Config Folder',
        icon: 'folder.png',
        action: 'showConfigFolder'
      },
      {
        name: 'Open Visual Editor',
        icon: 'visual.png',
        action: 'notification',
        title: 'Open Visual Editor',
        message: 'This feature is not available in the demo version.'
      },
      {
        name: 'Refresh Configurations',
        icon: 'refresh_settings.png',
        action: 'refresh'
      }
    ]
  },

   {
    name: 'separator',
    action: 'separator'
  },
  
  
  // Launch at Login
  {
    name: 'Launch at Login',
    icon: 'config.png',
    checkmark: false,
    action: 'toggle'
  },
  
  // Separator
  {
    name: 'separator',
    action: 'separator'
  },
  
  // Settings
  {
    name: 'Settings',
    icon: 'config.png',
    action: 'notification',
    title: 'Settings',
    message: 'Settings window is not available in this demo version.'
  },
  
  // About
  {
    name: 'About',
    icon: 'info.png',
    action: 'showAbout'
  },
  
  // Help
  {
    name: 'Help',
    icon: 'help.png',
    action: 'showHelp'
  },
  
  // Homepage
  {
    name: 'Homepage',
    icon: 'site.png',
    action: 'showHomepage'
  },
  
  // Separator
  {
    name: 'separator',
    action: 'separator'
  },
  
  // Quit
  {
    name: 'Quit SwitchShuttle',
    icon: 'exit.png',
    action: 'hideMenuBar'
  }
] 