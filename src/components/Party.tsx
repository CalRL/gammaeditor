import { useEffect, useState } from "react";
import {invoke} from "@tauri-apps/api/core";
import MonCard from "./Pokemon.tsx";
import EmptySlot from "./EmptySlot.tsx";
import {GridMon} from "../model/Pokemon.ts";

export interface PartyMon {
    name: string;
    shiny: boolean;
    index: number;
}

export default function Party() {
    const [party, setParty] = useState<PartyMon[] | null>(null);

    useEffect(() => {
        invoke<PartyMon[]>("get_party")
            .then(setParty)
            .catch((err) => console.error("Failed to load party:", err));
    }, []);

    if (!party) return <div className="text-white">Loading party...</div>;

    return (
        <div className="grid grid-cols-6 gap-2 p-4">
            {party.map((mon, i) => {
                const gridMon: GridMon = {
                    ...mon,
                    box_number: -1,
                    storage_index: mon.index,
                    grid_pos: { slot: 0, row: 0 },
                };

                return mon ? (
                    <MonCard key={`mon-${i}`} mon={gridMon} />
                ) : (
                    <EmptySlot key={`slot-${i}`} />
                );
            })}
        </div>
    );
}
