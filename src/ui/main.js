import * as tauriApi from 'https://esm.run/@tauri-apps/api';
import * as tauriEvent from 'https://esm.run/@tauri-apps/api/event';
import * as tauriApiPath from 'https://esm.run/@tauri-apps/api/path';

function registerListenersForMenuItems(items) {
    items.forEach(command => {
        if (command.event) {
            tauriEvent.listen(command.event, (event) => {
                tauriApi.invoke('execute', { command: event.payload }).catch((error) => {
                    console.log('error', error);
                });
            });
        }

        if (command.subitems) {
            registerListenersForMenuItems(command.subitems);
        }
    });
}

async function registerListeners() {
    // Listen to command events
    const config = await tauriApi.invoke('get_menu_data');
    const menuData = JSON.parse(config);

    registerListenersForMenuItems(menuData.items);

    await tauriEvent.listen('menu-did-open', async (event) => {
        console.log(event);

        const config = await tauriApi.invoke('get_menu_data');
        const menuData = JSON.parse(config);

        // show context menu
        tauriApi.invoke('plugin:context_menu|show_context_menu', {
            pos: {
                x: event.payload.x,
                y: event.payload.y
            },
            theme: 'light',
            items: menuData.items
        });
    });
}
registerListeners(); // Register event listeners once

// window.addEventListener('contextmenu', async (e) => {
//     e.preventDefault();
// });
