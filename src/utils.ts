import { Config, Command } from './types';

export function handleHotkey(config: Config | Command, key: string, event: KeyboardEvent): void {
  event.preventDefault();
  let hotkey: string[] = [];
  if (event.ctrlKey) hotkey.push('Ctrl');
  if (event.shiftKey) hotkey.push('Shift');
  if (event.altKey) hotkey.push('Alt');
  if (event.metaKey) hotkey.push('Command');
  if (event.key && !['Control', 'Shift', 'Alt', 'Meta'].includes(event.key)) hotkey.push(event.key);
  (config as any)[key] = hotkey.join('+');
}
