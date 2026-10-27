import { describe, it, expect } from 'vitest';
import { hotkeyFromEvent } from '../lib/hotkey';

const press = (code: string, mods: Partial<Record<'ctrlKey' | 'altKey' | 'shiftKey' | 'metaKey', boolean>> = {}) =>
  hotkeyFromEvent({ code, ctrlKey: false, altKey: false, shiftKey: false, metaKey: false, ...mods });

describe('hotkeyFromEvent', () => {
  it('waits while only modifiers are pressed', () => {
    expect(press('ControlLeft', { ctrlKey: true })).toBeNull();
    expect(press('ShiftLeft', { ctrlKey: true, shiftKey: true })).toBeNull();
  });

  it('records the full combination', () => {
    expect(press('KeyM', { ctrlKey: true, shiftKey: true })).toBe('ctrl+shift+m');
  });

  it('uses the physical key, not the shifted character', () => {
    expect(press('Digit1', { shiftKey: true })).toBe('shift+1');
  });

  it('maps keys to names the backend parses', () => {
    expect(press('ArrowUp', { altKey: true })).toBe('alt+up');
    expect(press('Escape')).toBe('esc');
    expect(press('F12', { metaKey: true })).toBe('meta+f12');
    expect(press('IntlBackslash')).toBeNull();
  });
});
