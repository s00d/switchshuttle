<template>
  <div id="app" class="h-screen w-screen bg-slate-50 text-sm font-sans antialiased">
    <main class="h-full overflow-auto">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { getCurrentWindow, cursorPosition } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';
import { MenuItem } from "@tauri-apps/api/menu/menuItem";
import { Menu } from "@tauri-apps/api/menu/menu";
import { Submenu } from "@tauri-apps/api/menu/submenu";
import SwitchShuttleCommands from './lib/tauri-commands';
import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import { PhysicalPosition } from "@tauri-apps/api/dpi";
import type { Command } from './types';

async function createMenuItem(item: Command): Promise<MenuItem | Submenu> {
  if (item.submenu) {
    const submenuItems = await Promise.all(item.submenu.map(createMenuItem));
    return await Submenu.new({ text: item.name, items: submenuItems });
  } else {
    return await MenuItem.new({
      id: item.id,
      text: item.name
    });
  }
}

async function showContextMenu(hotkey: string) {
  const pos = await cursorPosition();
  const config = await invoke('get_menu_data') as string;
  const menuData = JSON.parse(config) as Record<string, Command[]>;

  if (menuData[hotkey]) {
    const menuItems = await Promise.all(menuData[hotkey].map(createMenuItem));
    const menu = await Menu.new({ items: menuItems });
    const window = getCurrentWindow();
    await window.hide();
    await window.setPosition(new PhysicalPosition(0, 0));
    await menu.popup(new PhysicalPosition(pos.x, pos.y));
  } else {
    console.error(`No menu items found for hotkey: ${hotkey}`);
    SwitchShuttleCommands.show_notification('Error', `No menu items found for hotkey: ${hotkey}`, 'error');
    await getCurrentWindow().hide();
  }
}

async function registerGlobalHotkeys(commands: Command[], uniqueHotkeys: Set<string>) {
  for (const command of commands) {
    if (command.hotkey) {
      if (uniqueHotkeys.has(command.hotkey)) {
        console.error(`Hotkey ${command.hotkey} is already registered for command ${command.name}.`);
        SwitchShuttleCommands.show_notification('Warning', `Hotkey ${command.hotkey} is already registered for command ${command.name}.`, 'warning');
        await getCurrentWindow().hide();
      } else {
        uniqueHotkeys.add(command.hotkey);
        await unregister(command.hotkey);
        await register(command.hotkey, async (event) => {
          if (event.state === 'Released') {
            await invoke('execute', { command: command.id });
          }
        }).catch(async (error) => {
          console.error(`Failed to register hotkey ${command.hotkey} for command ${command.name}:`, error);
          SwitchShuttleCommands.show_notification('Error', `Failed to register hotkey ${command.hotkey} for command ${command.name}: ${error}`, 'error');
          await getCurrentWindow().hide();
        });
      }
    }

    if (command.submenu) {
      await registerGlobalHotkeys(command.submenu, uniqueHotkeys);
    }
  }
}

onMounted(async () => {
  const config = await invoke('get_menu_data') as string;
  const menuData = JSON.parse(config) as Record<string, Command[]>;
  const uniqueHotkeys = new Set<string>();

  for (const [hotkey] of Object.entries(menuData)) {
    if (uniqueHotkeys.has(hotkey)) {
      console.error(`Hotkey ${hotkey} is already registered.`);
      SwitchShuttleCommands.show_notification('Warning', `Hotkey ${hotkey} is already registered.`, 'warning');
      await getCurrentWindow().hide();
    } else {
      uniqueHotkeys.add(hotkey);
      await unregister(hotkey);
      await register(hotkey, async (event) => {
        if (event.state === 'Released') {
          await showContextMenu(hotkey);
        }
      }).catch(async (error) => {
        console.error(`Failed to register hotkey ${hotkey}:`, error);
        SwitchShuttleCommands.show_notification('Warning', `Failed to register hotkey ${hotkey}: ${error}`, 'warning');
        await getCurrentWindow().hide();
      });
    }
  }

  for (const items of Object.values(menuData)) {
    await registerGlobalHotkeys(items, uniqueHotkeys);
  }


});
</script>
