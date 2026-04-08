import { listen } from '@tauri-apps/api/event';

export type SystemEvent = {
    module: string;
    level: 'info' | 'warn' | 'error';
    message: string;
    progress?: number;
};

export async function listenToSystemEvents(callback: (payload: SystemEvent) => void) {
    return await listen<SystemEvent>('system-event', (event) => {
        callback(event.payload);
    });
}
