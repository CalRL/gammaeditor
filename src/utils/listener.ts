import { listen } from "@tauri-apps/api/event";

export const listener = {
    on: async (event: string, callback: () => void) => {
        const unlisten = await listen(event, callback);
        return () => {
            unlisten(); // call to remove listener
        };
    },
};