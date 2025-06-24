export type EnrichedMon = {
    name: string;
    level: number;
    hp: number;
    max_hp: number;
    atk: number;
    def: number;
    spatk: number;
    spdef: number;
    speed: number;
    class: string;
    slot: number;
    row: number;
    nature: string;
    primary_type: string;
    secondary_type: string;
    id: number;
    shiny: boolean;
    is_empty: boolean;
};

export interface Stats {
    hp: number;
    atk: number;
    def: number;
    spatk: number;
    spdef: number;
    speed: number;
}

export interface Move {
    name: string;
    pp: number;
    max_pp: number;
}

export interface Types {
    primary: string;
    secondary: string;
}

export interface GridPos {
    slot: number;
    row: number;
}

export interface TotalMon {
    name: string;
    level: number;
    gender: string;
    current_hp: number;
    stats?: Stats;
    ivs?: Stats;
    moves?: Move[];
    types?: Types;
    nature: string;
    class: string;
    grid_pos: GridPos;
    pokemon_id?: number;
    storage_index?: number;
    box_id?: number;
    pokeball?: string;
    shiny: boolean;
    is_empty: boolean;
}

export interface GridMon {
    name: string;
    shiny: boolean;
    grid_pos: GridPos;
    box_number: number;
    storage_index: number;
}