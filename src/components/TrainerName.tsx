import {invoke} from "@tauri-apps/api/core";
import React from "react";
import {useOnSaveLoad} from "../utils/useOnSaveLoad.ts";


export default function TrainerName() {

    const [name, setName] = React.useState("TrainerNamePlaceholder");

    async function fetchTrainerName() {
        try {
            const trainerName: string = await invoke("get_trainer_name");
            console.log("PlayerPosition:", trainerName);
            setName(trainerName);

        } catch (e) {
            console.error("Failed to get player transform:", e);
        }
    }

    useOnSaveLoad(fetchTrainerName);

    if(name === ""){
        return;
    }

    return(
        <div className="w-full text-center text-xl font-semibold pb-4">
            Trainer Name: {name}
        </div>
    )
}