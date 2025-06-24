export default function EmptySlot() {
    return (
        <div className="w-24 h-24 p-2 rounded shadow border-2 border-dashed border-gray-600 bg-gray-800 text-gray-500 flex flex-col items-center justify-center text-xs">
            <span className="text-lg">—</span>
            <span>Empty</span>
        </div>
    );
}
