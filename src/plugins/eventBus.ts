import { reactive } from 'vue';

type Callback = (data: any) => void;

interface Listeners {
    [key: string]: Callback[];
}

interface EventBus {
    listeners: Listeners;
    lastEventData: { [key: string]: any };
    on(event: string, callback: Callback): void;
    off(event: string, callback: Callback): void;
    emit(event: string, data: any): void;
}

const eventBus: EventBus = reactive({
    listeners: {} as Listeners,
    lastEventData: {} as { [key: string]: any },

    on(event: string, callback: Callback) {
        if (!this.listeners[event]) {
            this.listeners[event] = [];
        }
        this.listeners[event].push(callback);

        // If there is already data for this event, immediately call the callback with the last event data
        if (this.lastEventData[event] !== undefined) {
            callback(this.lastEventData[event]);
            // Optionally, clear the last event data after calling the callback
            delete this.lastEventData[event];
        }
    },

    off(event: string, callback: Callback) {
        if (!this.listeners[event]) return;
        this.listeners[event] = this.listeners[event].filter(listener => listener !== callback);
    },

    emit(event: string, data: any) {
        if (!this.listeners[event] || this.listeners[event].length === 0) {
            this.lastEventData[event] = data; // Store the last event data only if there are no listeners
        } else {
            this.listeners[event].forEach(listener => listener(data));
        }
    }
});

export default eventBus;
