import {invoke} from "@tauri-apps/api/core";
import React from "react";
import {useOnSaveLoad} from "../utils/useOnSaveLoad.ts";
import MovesTest from "./MovesTest.tsx";

export default function Test() {


    const [getMoney, setMoney] = React.useState(0)


    async function fetchMoney() {
        try {
            const money: number = await invoke("get_player_money");
            setMoney(money);
            console.log("pokeDollars:", money);
        } catch (e) {
            console.error("Failed to get player money:", e);
        }
    }

    async function get() {
        const arr = await invoke<any>("get_pos_array", {boxNumber: 0})
        console.log(arr)
    }

    async function check() {
        const arr = await invoke<any>("check_pos_index", {boxNumber:0, x: 0, y: 0, index:0})

        console.log(arr)
    }

    async function getSize() {
        const res = await invoke<any>("get_box_size", {boxNumber:0})

        console.log(res)
    }

    async function setShiny() { 
        const res = await invoke<any>("set_shiny_by_index", {boxNumber:0, index:0, state: false})

        console.log(res)
    }

    async function getShiny() {
        const res = await invoke<any>("get_shiny_by_index", {boxNumber:0, index:0})

        console.log(res)
    }
    async function getEnriched() {
        const res = await invoke<any>("get_enriched_by_index", {boxNumber:0, index:6 })

        console.log(res)
    }

    async function test() {
        const res = await invoke<any>("test");
        console.log(res);
    }

    async function getInfo() {
        const res = await invoke<any>("get_pokemon_info");
        console.log(res);
    }

    useOnSaveLoad(fetchMoney);

    return(
        <div className="flex space-x-4">
            <button type="button" onClick={fetchMoney}>Money {getMoney}</button>
            <button onClick={get}>pos array</button>
            <button onClick={check}>check</button>
            <button onClick={getSize}>getSize</button>
            <button onClick={setShiny}>setShiny</button>
            <button onClick={getShiny}>getShiny</button>
            <button onClick={getEnriched}>getEnriched</button>
            <button onClick={test}>TEST STORE</button>
            <button onClick={getInfo}>GET INFO</button>
            <MovesTest />
        </div>
    );
}