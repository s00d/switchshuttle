<template>
  <div id="app" class="h-screen w-screen bg-slate-50 text-sm font-sans antialiased">
    <AppHeader />
    <main class="pt-16 h-full overflow-auto">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { getCurrentWindow, cursorPosition } from '@tauri-apps/api/window';
import { useRouter } from 'vue-router';
import { listen, emit } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { MenuItem } from "@tauri-apps/api/menu/menuItem";
import { Menu } from "@tauri-apps/api/menu/menu";
import { Submenu } from "@tauri-apps/api/menu/submenu";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification
} from '@tauri-apps/plugin-notification';
import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import { PhysicalPosition } from "@tauri-apps/api/dpi";
import type { Command } from './types';
import AppHeader from './components/AppHeader.vue';

const router = useRouter();

listen('navigate', async (event: any) => {
  try {
    await router.push(event.payload[0]);
    emit('navigation_complete', { route: event.payload[0] });
    const window = getCurrentWindow();
    await window.center();
    await window.setFocus();
    await window.setAlwaysOnTop(true);
    await window.setShadow(true);
  } catch (error) {
    console.error('Navigation error:', error);
  }
}).catch((error) => {
  console.error('Failed to listen for navigate event:', error);
});

async function noti(title: string, body: string) {
  let permissionGranted = await isPermissionGranted();
  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === 'granted';
  }
  if (permissionGranted) {
    sendNotification({ title, body });
  }
}

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
    if (import.meta.env.PROD) {
      noti('Error', `No menu items found for hotkey: ${hotkey}`);
    }
    await getCurrentWindow().hide();
  }
}

async function registerGlobalHotkeys(commands: Command[], uniqueHotkeys: Set<string>) {
  for (const command of commands) {
    if (command.hotkey) {
      if (uniqueHotkeys.has(command.hotkey)) {
        console.error(`Hotkey ${command.hotkey} is already registered for command ${command.name}.`);
        if (import.meta.env.PROD) {
          noti('Warning', `Hotkey ${command.hotkey} is already registered for command ${command.name}.`);
        }
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
          noti('Error', `Failed to register hotkey ${command.hotkey} for command ${command.name}: ${error}`);
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
      if (import.meta.env.PROD) {
        noti('Warning', `Hotkey ${hotkey} is already registered.`);
      }
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
        if (import.meta.env.PROD) {
          noti('Warning', `Failed to register hotkey ${hotkey}: ${error}`);
        }
        await getCurrentWindow().hide();
      });
    }
  }

  for (const items of Object.values(menuData)) {
    await registerGlobalHotkeys(items, uniqueHotkeys);
  }

  if (!await isPermissionGranted()) {
    await requestPermission();
  }
});
</script>
