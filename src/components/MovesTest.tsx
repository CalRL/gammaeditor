import { invoke } from "@tauri-apps/api/core";
export default function MovesTest() {


    async function fetchPP() {
        const res = await invoke<JSON>("get_pp", { boxNumber: 0 });
        console.log(JSON.stringify(res));

    }

    async function fetchPPByIndex(index: number) {
        const res = await invoke<JSON>("get_pp_by_index", { boxNumber: 0, index: index });
        console.log(res);
    }

    async function fetchMovesByBox(boxNumber: number) {
        const res = await invoke<JSON>("get_moves", { boxNumber: boxNumber });
        console.log(res);
    }

    async function fetchMovesByIndex(boxNumber: number, index: number) {
        const res = await invoke<JSON>("get_moves_by_index", { boxNumber: boxNumber, index: index });
        console.log(res);
    }

    async function fetchIvsByIndex(boxNumber: number, index: number) {
        const res = await invoke<JSON>("get_ivs", { boxNumber: boxNumber, index: index });
        console.log(res);
    }

    async function fetchGendersByBox(boxNumber: number) {
        const res = await invoke<JSON>("get_genders_by_box", { boxNumber: boxNumber });
        console.log(res);
    }

    async function fetchMovesPP(boxNumber: number, index: number) {
        const res = await invoke<JSON>("get_enriched_moves", { boxNumber: boxNumber, index:index });
        console.log(res);
    }


    return(
        <div className={"space-x-4 flex"}>
            <div>
                <button onClick={fetchPP}>FETCHPP</button>
                <button onClick={() => { fetchPPByIndex(0); }}>FETCHPP BY INDEX</button>
            </div>
            <div>
                <button onClick={() => { fetchMovesByBox(0); }}>FETCHMOVES</button>
                <button onClick={() => { fetchMovesByIndex(0, 4); }}>FETCHMOVES BY INDEX</button>
            </div>
            <div>
                <button onClick={() => { fetchIvsByIndex(0, 0); }}>FETCH IVS</button>
                <button onClick={() => { fetchGendersByBox(0); }}>FETCH GENDERS</button>
                <button onClick={() => { fetchMovesPP(0, 0); }}>FETCH MOVES AND PP</button>
            </div>

        </div>
    )
}