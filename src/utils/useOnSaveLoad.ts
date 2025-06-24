import { useEffect } from "react";

import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";

export function useOnSaveLoad(callback: () => void) {
    useEffect(() => {

        invoke<boolean>("is_save_loaded").then((loaded) => {
            if (loaded) {
                callback();
            }
        });

        const unlisten = listen("save-loaded", () => {
            callback();
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);
}