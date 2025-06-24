import { Link, useLocation } from "react-router-dom";

export default function Navbar() {
    const location = useLocation();

    const navLink = (path: string, label: string) => (
        <Link
            to={path}
            className={`px-4 py-2 rounded-md text-white hover:bg-gray-700 transition ${
                location.pathname === path ? "bg-gray-800 font-bold" : ""
            }`}
        >
            {label}
        </Link>
    );

    return (
        <nav className="w-full flex justify-center gap-4 p-4">
            {navLink("/trainer", "Trainer")}
            {navLink("/boxes", "Boxes")}
            {navLink("/party", "Party")}
            {navLink("/test", "Test")}
        </nav>
    );
}
