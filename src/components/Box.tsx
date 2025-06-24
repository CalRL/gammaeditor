import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { GridMon } from "../model/Pokemon.ts";
import EmptySlot from "./EmptySlot.tsx";
import MonCard from "./Pokemon.tsx";

export default function Box({ number }: { number: number }) {
    const [mons, setMons] = useState<(GridMon | null)[][]>([]);

    useEffect(() => {
        invoke<(GridMon | null)[][]>("get_simple_mon_grid", { boxNumber: number })
            .then(grid => {
                setMons(grid);
                console.log(grid);
            })
            .catch(console.error);
    }, [number]);

    if (!mons) return <div>Loading...</div>;

    return (
        <div className="p-4">
            <h1 className="text-2xl text-white font-bold mb-4">Box #{number}</h1>
            {mons.map((row, rowIndex) => (
                <div key={rowIndex} className="grid grid-cols-7 gap-3 space-y-4">
                    {row.map((mon, slotIndex) =>
                        mon === null
                            ? <EmptySlot key={`empty-${rowIndex}-${slotIndex}`} />
                            : <MonCard key={`mon-${rowIndex}-${slotIndex}`} mon={mon} />
                    )}
                </div>
            ))}
        </div>
    );
}
