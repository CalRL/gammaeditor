import { Link } from "react-router-dom";
import {GridMon} from "../model/Pokemon.ts";

interface MonCardProps {
    mon: GridMon;
}

export default function MonCard({ mon }: MonCardProps) {
    const {
        name,
        shiny,
        box_number,
        storage_index
    } = mon;

    console.log(name, box_number, storage_index);

    const normalizedName = name.toLowerCase();
    const imagePath = `/sprites/${shiny ? "shiny" : "normal"}/${normalizedName}.png`;

    return (
        <Link to={`/boxes/${box_number}/${storage_index}`}>
            <div
                className={`p-2 rounded shadow text-white w-24 bg-gray-800 h-24 flex flex-col items-center justify-center
                ${shiny ? ' border-yellow-300' : 'border-white'} border-2`}
            >
                <div className="w-16 h-16 flex items-center justify-center">
                    <img
                        src={imagePath}
                        alt={name}
                        className="max-w-full max-h-full object-contain"
                    />
                </div>
                <div className="font-bold text-xs text-center truncate">
                    {name}
                    {shiny && <span className="text-yellow-400 ml-1">★</span>}
                </div>
            </div>
        </Link>
    );
}
