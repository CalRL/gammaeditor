import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import IVRadarChart from "./chart/IVRadarChart.tsx";

export default function MonDetails() {
    const { boxId, storageIndex } = useParams();
    const [mon, setMon] = useState<any>(null);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        if (!boxId || !storageIndex) {
            console.warn("Waiting for params to load...");
            return;
        }

        async function fetchMon() {
            try {
                console.log(`ids ${boxId}: ${storageIndex}`);
                const monData = await invoke("get_enriched_by_index", {
                    boxNumber: parseInt(boxId ?? "0"),
                    index: parseInt(storageIndex ?? "0"),
                });
                console.log("Fetched mon:", monData);
                setMon(monData);
            } catch (err) {
                console.error("Failed to fetch mon:", err);
                setError("Failed to fetch mon.");
            }
        }

        fetchMon();
    }, [boxId, storageIndex]);

    async function exportMon() {
        const xp = await invoke("get_xp_by_index", {
            boxNumber: parseInt(boxId ?? "0"),
            index: parseInt(storageIndex ?? "0"),
        })
        console.log(`XP: ${xp}`);
        let ivs = `${mon.ivs.hp},${mon.ivs.attack},${mon.ivs.defense},${mon.ivs.speed},${mon.ivs.special_attack},${mon.ivs.special_defense}`
        console.log(ivs);

        const res = await invoke("run_generator", {
            args: [
                `--species=${mon.name}`,
                `--trainername=placeholder`,
                `--nickname=${mon.name}`,
                `--shiny=${mon.shiny}`,
                `--xp=${xp}`,
                `--gender=0`,
                `--nature=${mon.nature}`,
                `--ball=4`,
                `--ivs=${ivs}`
            ]
        });

        console.log(res);
    }

    if (error) {
        return <div>{error}</div>;
    }

    if (!mon) {
        return <div>Loading...</div>;
    }

    console.log("%j", mon);

    return (
        <div className="p-4">
            <h1 className="text-3xl font-bold mb-4">{mon.name}</h1>
            <button onClick={exportMon}>EXPORT TO PK9</button>
            <p><strong>Level:</strong> {mon.level}</p>
            <p><strong>Gender:</strong> {mon.gender}</p>
            <p><strong>HP:</strong> {mon.current_hp}</p>
            <p><strong>Shiny:</strong> {mon.shiny ? "Yes" : "No"}</p>

            {mon.types && (
                <div>
                    <p><strong>Primary Type:</strong> {mon.types.primary}</p>
                    <p><strong>Secondary Type:</strong> {mon.types.secondary}</p>
                </div>
            )}

            <p><strong>Nature:</strong> {mon.nature}</p>
            <p><strong>Class:</strong> {mon.class}</p>

            {mon.ivs && (
                <div>
                    <IVRadarChart ivs={mon.ivs} />
                </div>
            )}
            {mon.stats && (
                <div>
                    <h2 className="mt-4 font-semibold">Stats:</h2>
                    <p>HP: {mon.stats.hp}</p>
                    <p>Attack: {mon.stats.attack}</p>
                    <p>Defense: {mon.stats.defense}</p>
                    <p>Sp. Atk: {mon.stats.special_attack}</p>
                    <p>Sp. Def: {mon.stats.special_defense}</p>
                    <p>Speed: {mon.stats.speed}</p>
                </div>
            )}

            {mon.moves && (
                <div>
                    <h2 className="mt-4 font-semibold">Moves:</h2>
                    {mon.moves.map((move: any, idx: number) => (
                        <p key={idx}>{move.name} (PP: {move.pp}/{move.max_pp})</p>
                    ))}
                </div>
            )}
        </div>
    );
}
