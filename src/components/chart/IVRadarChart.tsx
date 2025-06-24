import {
    Radar
} from "react-chartjs-2";
import {
    Chart as ChartJS,
    RadialLinearScale,
    PointElement,
    LineElement,
    Filler,
    Tooltip,
    Legend
} from "chart.js";

ChartJS.register(
    RadialLinearScale,
    PointElement,
    LineElement,
    Filler,
    Tooltip,
    Legend
);

interface IVs {
    hp: number;
    attack: number;
    defense: number;
    special_attack: number;
    special_defense: number;
    speed: number;
}

export default function IVRadarChart({ ivs }: { ivs: IVs }) {
    const data = {
        labels: [
            `HP (${ivs.hp})`,
            `Attack (${ivs.attack})`,
            `Defense (${ivs.defense})`,
            `Speed (${ivs.speed})`,
            `Sp. Def (${ivs.special_defense})`,
            `Sp. Atk (${ivs.special_attack})`
        ],
        datasets: [
            {
                label: 'IVs',
                data: [
                    ivs.hp,
                    ivs.attack,
                    ivs.defense,
                    ivs.speed,
                    ivs.special_defense,
                    ivs.special_attack
                ],
                backgroundColor: 'rgba(54, 162, 235, 0.3)',
                borderColor: 'rgb(54, 162, 235)',
                pointBackgroundColor: 'rgb(54, 162, 235)',
                borderWidth: 2,
            },
        ],
    };

    const options = {
        responsive: true,
        plugins: {
            legend: {
                display: false,
            },
            tooltip: {
                enabled: false,
            }
        },
        interaction: {
            mode: null,
        },
        scales: {
            r: {
                beginAtZero: true,
                min: 0,
                max: 31,
                ticks: {
                    display: false
                },
                angleLines: { color: "#444" },
                grid: { color: "#444" },
                pointLabels: { color: "#ddd", font: { size: 14 } }
            }
        }
    };



    return (
        <div className="mx-auto" style={{ width: '384px', height: '384px' }}>
            <div className={"text-2xl text-center font-semibold"}>IVs:</div>
            {/* @ts-ignore */ }
            <Radar data={data} options={options} />
        </div>
    );
}
