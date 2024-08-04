export interface ConfigFile {
  path: string;
  name: string;
}

export interface Command {
  name: string;
  command: string;
  id: string;
  hotkey: string | null;
  subitems: Command[]|null;
  commands: string[];
}

export interface Config {
  terminal: string;
  launch_in: string;
  theme: string;
  title: string;
  commands: Command[];
  menu_hotkey: string;
}
