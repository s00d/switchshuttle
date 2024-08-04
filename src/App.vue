<template>
  <div id="app">
    <div data-tauri-drag-region class="title-bar">
      <div data-tauri-drag-region class="title">{{ title }}</div>
      <div class="buttons">
        <div @click="onClose"></div>
      </div>
    </div>
    <router-view />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import {getCurrentWindow, cursorPosition} from '@tauri-apps/api/window';
import { useRouter } from 'vue-router';
import { listen, emit } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import {MenuItem} from "@tauri-apps/api/menu/menuItem";
import {Menu} from "@tauri-apps/api/menu/menu";
import { message } from '@tauri-apps/plugin-dialog';
import {Command} from "./types.ts";
import {register} from "@tauri-apps/plugin-global-shortcut";
import {Submenu} from "@tauri-apps/api/menu/submenu";
import {PhysicalPosition} from "@tauri-apps/api/dpi";

const title = ref('Switch Shuttle');
const router = useRouter();

function onClose() {
  router.push('/').catch(() => {});
  getCurrentWindow().hide();
}

listen('navigate', (event: any) => {
  title.value = event.payload[1];
  router.push(event.payload[0]).then(() => {
    // Send a confirmation event back to the backend
    emit('navigation_complete', { route: event.payload[0] });
  }).catch((error) => {
    console.error('Navigation error:', error);
  });
}).catch((error) => {
  console.error('Failed to listen for navigate event:', error);
});

async function createMenuItem(item: Command): Promise<MenuItem | Submenu> {
  if (item.submenu) {
    const submenuItems = await Promise.all(item.submenu.map(createMenuItem));
    return await Submenu.new({
      text: item.name,
      items: submenuItems,
    });
  } else {
    return await MenuItem.new({
      id: item.id,
      text: item.name,
      // action: () => invoke('execute', { command: item.id }), // how it work??? O_o
    });
  }
}

async function showContextMenu(hotkey: string) {
  const pos2 = await cursorPosition();

  const config = await invoke('get_menu_data') as string;
  const menuData = JSON.parse(config) as Record<string, Command[]>;

  if (menuData[hotkey]) {
    const menuItems = await Promise.all(menuData[hotkey].map(createMenuItem));
    const menu = await Menu.new({
      items: menuItems,
    });

    getCurrentWindow().hide().then(() => {
      getCurrentWindow().setPosition(new PhysicalPosition(0, 0)).then(() => {
        menu.popup(new PhysicalPosition(parseInt(pos2.x.toString()), parseInt(pos2.y.toString()))).catch(error => {
          console.error('Failed to show context menu:', error);
        });
      })
    });
  } else {
    console.error(`No menu items found for hotkey: ${hotkey}`);
    await message(`No menu items found for hotkey: ${hotkey}`, { title: 'Error', kind: 'error' });
    await getCurrentWindow().hide();
  }
}


async function registerGlobalHotkeys(commands: Command[], uniqueHotkeys: Set<string>) {
  for (const command of commands) {
    if (command.hotkey) {
      if (uniqueHotkeys.has(command.hotkey)) {
        console.error(`Hotkey ${command.hotkey} is already registered for command ${command.name}.`);
        await message(`Hotkey ${command.hotkey} is already registered for command ${command.name}.`, { title: 'Error', kind: 'error' });
        await getCurrentWindow().hide();
      } else {
        uniqueHotkeys.add(command.hotkey);
        await register(command.hotkey, async (event) => {
          if (event.state === 'Released') {
            console.log(`Shortcut for command ${command.name} triggered`);
            await invoke('execute', { command: command.id });
          }
        }).catch(async (error) => {
          console.error(`Failed to register hotkey ${command.hotkey} for command ${command.name}:`, error);
          await message(`Failed to register hotkey ${command.hotkey} for command ${command.name}: ${error}`, { title: 'Error', kind: 'error' });
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

  for (const [hotkey, _items] of Object.entries(menuData)) {
    if (uniqueHotkeys.has(hotkey)) {
      console.error(`Hotkey ${hotkey} is already registered.`);
      await message(`Hotkey ${hotkey} is already registered.`, { title: 'Error', kind: 'error' });
      await getCurrentWindow().hide();
    } else {
      uniqueHotkeys.add(hotkey);
      await register(hotkey, async (event) => {
        if (event.state === 'Released') {
          console.log('Shortcut triggered');
          await showContextMenu(hotkey);
        }
      }).catch(async (error) => {
        console.error(`Failed to register hotkey ${hotkey}:`, error);
        await message(`Failed to register hotkey ${hotkey}: ${error}`, { title: 'Error', kind: 'error' });
        await getCurrentWindow().hide();
      });
    }
  }

  for (const [_hotkey, items] of Object.entries(menuData)) {
    await registerGlobalHotkeys(items, uniqueHotkeys);
  }


});


</script>

<style scoped>
#app {
  height: 100vh;
  background-color: #f3f3f3;
  border-radius: 5px;
}

.title-bar {
  width: 100%;
  height: 22px;
  background: #e0e0e0;
  display: flex;
  align-items: center;
  padding: 0 10px;
  box-sizing: border-box;
  -webkit-app-region: drag;
  position: fixed;
  top: 0;
  left: 0;
  z-index: 2;
}

.title-bar .buttons {
  display: flex;
  gap: 8px;
}

.title-bar .buttons div {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background-color: #ff5f56;
  cursor: pointer;
  -webkit-app-region: no-drag;
}

.title-bar .buttons div:nth-child(2) {
  background-color: #ffbd2e;
}

.title-bar .buttons div:nth-child(3) {
  background-color: #27c93f;
}

.title-bar .title {
  flex-grow: 1;
  text-align: center;
  font-size: 12px;
  color: #555;
  -webkit-app-region: drag;
  padding-left: 22px;
}

button {
  padding: 6px 20px;
  font-size: 14px;
  border: 1px solid #ccd0d5;
  border-radius: 6px;
  background: linear-gradient(to bottom, #ffffff, #e0e0e0);
  color: #333;
  cursor: pointer;
  transition: background 0.2s, border-color 0.2s;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  margin: 10px 5px;
}

button:hover {
  background: linear-gradient(to bottom, #e0e0e0, #d0d0d0);
  border-color: #bbb;
}

button:active {
  background: linear-gradient(to bottom, #d0d0d0, #c0c0c0);
  border-color: #aaa;
  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.1);
}

button:focus {
  outline: none;
  box-shadow: 0 0 3px 2px rgba(0, 123, 255, 0.25);
}

.cancel-button {
  background: linear-gradient(to bottom, #dc3545, #c82333);
  color: white;
}

.cancel-button:hover {
  background: linear-gradient(to bottom, #c82333, #a71d2a);
  border-color: #a71d2a;
}

.cancel-button:active {
  background: linear-gradient(to bottom, #a71d2a, #8a1621);
  border-color: #8a1621;
}

.button-blue {
  border: 1px solid #007aff;
  background: linear-gradient(to bottom, #007aff, #005bb5);
  color: white;
}

.button-blue:hover {
  background: linear-gradient(to bottom, #005bb5, #004a99);
  border-color: #005bb5;
}

.button-blue:active {
  background: linear-gradient(to bottom, #004a99, #003d7a);
  border-color: #004a99;
}
</style>
<style>
* {

}
body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  background-color: #f0f0f5;
  overflow: hidden;
  margin: 0;
}

.container {
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  box-sizing: border-box;

  background: #fff;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  text-align: center;
  width: 100%;
  max-width: 400px;
  position: relative;
  height: 100%;
  padding: 30px 0 0 0;
}


button {
  padding: 6px 20px;
  font-size: 14px;
  border: 1px solid #ccd0d5;
  border-radius: 6px;
  background: linear-gradient(to bottom, #ffffff, #e0e0e0);
  color: #333;
  cursor: pointer;
  transition: background 0.2s, border-color 0.2s;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

button:hover {
  background: linear-gradient(to bottom, #e0e0e0, #d0d0d0);
  border-color: #bbb;
}

button:active {
  background: linear-gradient(to bottom, #d0d0d0, #c0c0c0);
  border-color: #aaa;
  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.1);
}

button:focus {
  outline: none;
  box-shadow: 0 0 3px 2px rgba(0, 123, 255, 0.25);
}


.button-blue {
  border: 1px solid #007aff;
  background: linear-gradient(to bottom, #007aff, #005bb5);
  color: white;
}

.button-blue:hover {
  background: linear-gradient(to bottom, #005bb5, #004a99);
  border-color: #005bb5;
}

.button-blue:active {
  background: linear-gradient(to bottom, #004a99, #003d7a);
  border-color: #004a99;
}

.cancel-button {
  background: linear-gradient(to bottom, #dc3545, #c82333);
  color: white;
}

.cancel-button:hover {
  background: linear-gradient(to bottom, #c82333, #a71d2a);
  border-color: #a71d2a;
}

.cancel-button:active {
  background: linear-gradient(to bottom, #a71d2a, #8a1621);
  border-color: #8a1621;
}
</style>
