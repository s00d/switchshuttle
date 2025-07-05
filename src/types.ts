export interface ConfigFile {
  path: string;
  name: string;
}

export interface Command {
  id?: string;
  name: string;
  description?: string;
  command?: string | undefined;
  hotkey?: string | undefined;
  submenu?: Command[] | null;
  commands?: string[] | null;
  inputs?: Record<string, string> | null;
  switch?: string | undefined;
  monitor?: string | undefined;
  icon?: string | null;
}

export interface Template {
  id: string;
  name: string;
  description: string;
  category: string;
  icon: string;
  commands: Command[];
  tags: string[];
}

export interface Config {
  terminal: string;
  launch_in: string;
  theme: string;
  title: string;
  commands: Command[];
  menu_hotkey?: string | null;
  enabled?: boolean;
}
