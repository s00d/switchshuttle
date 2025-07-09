import { platform } from '@tauri-apps/plugin-os';
export interface TerminalOption {
  value: string;
  label: string;
  icon: string;
}

// macOS терминалы
export const macOSTerminals: TerminalOption[] = [
  { value: 'iterm', label: 'iTerm2', icon: '🖥️' },
  { value: 'terminal', label: 'Terminal.app', icon: '💻' },
  { value: 'alacritty', label: 'Alacritty', icon: '⚡' },
  { value: 'hyper', label: 'Hyper', icon: '🚀' },
  { value: 'warp', label: 'Warp', icon: '⚡' },
  { value: 'vscode-terminal', label: 'VS Code Terminal', icon: '🔧' },
];

// Windows терминалы
export const windowsTerminals: TerminalOption[] = [
  { value: 'hyper', label: 'Hyper', icon: '🚀' },
  { value: 'wsl', label: 'WSL', icon: '🐧' },
  { value: 'powershell', label: 'PowerShell', icon: '💻' },
  { value: 'windows-terminal', label: 'Windows Terminal', icon: '🪟' },
  { value: 'conemu', label: 'ConEmu', icon: '🖥️' },
  { value: 'cmder', label: 'Cmder', icon: '💻' },
  { value: 'git-bash', label: 'Git Bash', icon: '🐧' },
  { value: 'alacritty', label: 'Alacritty', icon: '⚡' },
  { value: 'wezterm', label: 'WezTerm', icon: '🚀' },
  { value: 'tabby', label: 'Tabby', icon: '📝' },
  { value: 'terminus', label: 'Terminus', icon: '🖥️' },
  { value: 'mintty', label: 'MinTTY', icon: '💻' },
  { value: 'putty', label: 'PuTTY', icon: '🔗' },
  { value: 'securecrt', label: 'SecureCRT', icon: '🔒' },
  { value: 'mobaxterm', label: 'MobaXterm', icon: '🖥️' },
  { value: 'royal-tsx', label: 'Royal TSX', icon: '👑' },
  { value: 'vscode-terminal', label: 'VS Code Terminal', icon: '🔧' },
  { value: 'sublime-terminal', label: 'Sublime Text', icon: '📄' },
  { value: 'atom-terminal', label: 'Atom Editor', icon: '⚛️' },
  { value: 'notepad++', label: 'Notepad++', icon: '📝' },
  { value: 'cygwin', label: 'Cygwin', icon: '🐧' },
  { value: 'msys2', label: 'MSYS2', icon: '🐧' },
];

// Linux терминалы
export const linuxTerminals: TerminalOption[] = [
  { value: 'hyper', label: 'Hyper', icon: '🚀' },
  { value: 'gnome-terminal', label: 'GNOME Terminal', icon: '🖥️' },
  { value: 'konsole', label: 'KDE Konsole', icon: '💻' },
  { value: 'xfce4-terminal', label: 'XFCE Terminal', icon: '🖥️' },
  { value: 'lxterminal', label: 'LXTerminal', icon: '💻' },
  { value: 'mate-terminal', label: 'MATE Terminal', icon: '🖥️' },
  { value: 'tilix', label: 'Tilix', icon: '🖥️' },
  { value: 'terminator', label: 'Terminator', icon: '🖥️' },
  { value: 'alacritty', label: 'Alacritty', icon: '⚡' },
  { value: 'wezterm', label: 'WezTerm', icon: '🚀' },
  { value: 'kitty', label: 'Kitty', icon: '🐱' },
  { value: 'tabby', label: 'Tabby', icon: '📝' },
  { value: 'terminology', label: 'Terminology', icon: '🖥️' },
  { value: 'deepin-terminal', label: 'Deepin Terminal', icon: '🖥️' },
  { value: 'cool-retro-term', label: 'Cool Retro Term', icon: '🖥️' },
  { value: 'guake', label: 'Guake', icon: '🖥️' },
  { value: 'yakuake', label: 'Yakuake', icon: '🖥️' },
  { value: 'tilda', label: 'Tilda', icon: '🖥️' },
  { value: 'sakura', label: 'Sakura', icon: '🌸' },
  { value: 'roxterm', label: 'RoxTerm', icon: '🖥️' },
  { value: 'pantheon-terminal', label: 'Pantheon Terminal', icon: '🖥️' },
  { value: 'vscode-terminal', label: 'VS Code Terminal', icon: '🔧' },
  { value: 'sublime-terminal', label: 'Sublime Text', icon: '📄' },
  { value: 'atom-terminal', label: 'Atom Editor', icon: '⚛️' },
];

// Функция для получения списка терминалов в зависимости от ОС
export function getTerminalOptions(): TerminalOption[] {
  const currentPlatform = platform();
  
  switch (currentPlatform) {
    case 'macos':
    case 'ios':
      return macOSTerminals;
    case 'windows':
      return windowsTerminals;
    case 'linux':
    case 'freebsd':
    case 'dragonfly':
    case 'netbsd':
    case 'openbsd':
    case 'solaris':
    case 'android':
      return linuxTerminals;
    default:
      return macOSTerminals; // По умолчанию возвращаем macOS терминалы
  }
}

// Опции запуска (одинаковые для всех ОС)
export const launchOptions = [
  { value: 'current', label: 'Current Window', icon: '📍' },
  { value: 'new_tab', label: 'New Tab', icon: '📑' },
  { value: 'new_window', label: 'New Window', icon: '🪟' }
]; 