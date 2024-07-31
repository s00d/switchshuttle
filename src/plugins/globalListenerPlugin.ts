import { App } from 'vue';
import { listen } from '@tauri-apps/api/event';
import eventBus from './eventBus';

export default {
    install(_app: App) {
        listen('input_data', (event) => {
            eventBus.emit('input_data', event.payload);
        }).catch((error) => {
            console.error('Failed to listen for input_data event:', error);
        });
    }
};
