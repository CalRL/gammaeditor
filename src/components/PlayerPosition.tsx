import React from "react";
import { PositionProps } from "../model/PlayerProps.ts";
import { invoke } from "@tauri-apps/api/core";
import { useOnSaveLoad } from "../utils/useOnSaveLoad.ts";

export default function PlayerPosition() {
    const [getTransform, setTransform] = React.useState<PositionProps | null>(null);

    const [x, setX] = React.useState<number>(0);
    const [y, setY] = React.useState<number>(0);
    const [z, setZ] = React.useState<number>(0);

    async function fetchPlayerTransform() {
        try {
            const transform: PositionProps = await invoke("get_player_position");
            setTransform(transform);
            setX(transform.x);
            setY(transform.y);
            setZ(transform.z);
        } catch (e) {
            console.error("Failed to get player transform:", e);
        }
    }

    async function savePosition(e: React.FormEvent) {
        e.preventDefault();
        // TODO: Add your invoke call to save position here
        console.log("Saving position:", { x, y, z });
    }

    useOnSaveLoad(fetchPlayerTransform);

    return (
        <div className="p-4 bg-gray-800 rounded-md max-w-md mx-auto text-white">
            <button
                onClick={fetchPlayerTransform}
                className="mb-4 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded transition"
            >
                Get Position
            </button>

            {getTransform && (
                <div className="mb-6 space-y-1 text-lg font-mono">
                    <div>
                        <span className="font-semibold">X:</span> {getTransform.x}
                    </div>
                    <div>
                        <span className="font-semibold">Y:</span> {getTransform.y}
                    </div>
                    <div>
                        <span className="font-semibold">Z:</span> {getTransform.z.toFixed(4)}
                    </div>
                </div>
            )}

            <form onSubmit={savePosition} className="flex gap-4 items-end">
                <div className="flex flex-col">
                    <label htmlFor="x" className="mb-1 font-semibold">
                        X
                    </label>
                    <input
                        id="x"
                        type="number"
                        value={x}
                        onChange={(e) => setX(parseFloat(e.target.value))}
                        className="rounded px-2 py-1 text-black w-24"
                    />
                </div>
                <div className="flex flex-col">
                    <label htmlFor="y" className="mb-1 font-semibold">
                        Y
                    </label>
                    <input
                        id="y"
                        type="number"
                        value={y}
                        onChange={(e) => setY(parseFloat(e.target.value))}
                        className="rounded px-2 py-1 text-black w-24"
                    />
                </div>
                <div className="flex flex-col">
                    <label htmlFor="z" className="mb-1 font-semibold">
                        Z
                    </label>
                    <input
                        id="z"
                        type="number"
                        value={z}
                        onChange={(e) => setZ(parseFloat(e.target.value))}
                        className="rounded px-2 py-1 text-black w-24"
                        step="0.0001"
                    />
                </div>
                <button
                    type="submit"
                    className="px-4 py-2 bg-green-600 hover:bg-green-700 rounded transition"
                >
                    Set Position
                </button>
            </form>
        </div>
    );
}
