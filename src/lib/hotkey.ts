// Имена клавиш, которые понимает бэкенд (src-tauri/src/hotkeys.rs, parse_key).
// Берём event.code, а не event.key: он не зависит от раскладки и Shift
// (Shift+1 даёт key "!", а code остаётся "Digit1").
const NAMED_CODES: Record<string, string> = {
  Space: 'space',
  Enter: 'enter',
  Tab: 'tab',
  Escape: 'esc',
  Backspace: 'backspace',
  Delete: 'delete',
  Insert: 'insert',
  Home: 'home',
  End: 'end',
  PageUp: 'pageup',
  PageDown: 'pagedown',
  ArrowUp: 'up',
  ArrowDown: 'down',
  ArrowLeft: 'left',
  ArrowRight: 'right',
};

export function keyNameFromCode(code: string): string | null {
  const letter = /^Key([A-Z])$/.exec(code);
  if (letter) return letter[1].toLowerCase();
  const digit = /^Digit([0-9])$/.exec(code);
  if (digit) return digit[1];
  if (/^F([1-9]|1[0-2])$/.test(code)) return code.toLowerCase();
  return NAMED_CODES[code] ?? null;
}

type KeyEventLike = Pick<KeyboardEvent, 'code' | 'ctrlKey' | 'altKey' | 'shiftKey' | 'metaKey'>;

/**
 * Комбинация для сохранения ("ctrl+shift+m") или null, пока нажаты только
 * модификаторы / клавиша не поддерживается.
 */
export function hotkeyFromEvent(event: KeyEventLike): string | null {
  const key = keyNameFromCode(event.code);
  if (!key) return null;
  const parts: string[] = [];
  if (event.ctrlKey) parts.push('ctrl');
  if (event.altKey) parts.push('alt');
  if (event.shiftKey) parts.push('shift');
  if (event.metaKey) parts.push('meta');
  parts.push(key);
  return parts.join('+');
}
