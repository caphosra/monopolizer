export interface IPlayerInfo {
    player_id: number;
    money: number;
    is_bankrupted: boolean;
    jail_turn: number | undefined;
    position: number;
}

export interface IPlaceInfo {
    place_id: number;
    owner: number | undefined;
    is_mortgaged: boolean;
    houses: number | undefined;
}

export interface IGameInfo {
    turn: number;
    players: IPlayerInfo[];
    places: IPlaceInfo[];
}

export type PlaceColor =
    | "None"
    | "Railroad"
    | "Utilities"
    | "Brown"
    | "LightBlue"
    | "LightPurple"
    | "Orange"
    | "Red"
    | "Yellow"
    | "Green"
    | "Blue";

export interface IPlaceProp {
    place_id: number;
    name: string;
    color: PlaceColor;
    price: number | undefined;
    house_price: number | undefined;
    rent: number | undefined;
}

const MONOPOLY_SERVER_PORT = 5391;
const API_ROOT =
    process.env.NODE_ENV === "development"
        ? `http://localhost:${MONOPOLY_SERVER_PORT}`
        : "";

export async function fetchInit(num: number): Promise<IGameInfo> {
    let response = await fetch(`${API_ROOT}/init?num=${num}`);
    if (!response.ok) {
        throw "Failed to fetch /init.";
    }
    let info = (await response.json()) as IGameInfo;
    return info;
}

export async function fetchStep(
    game: IGameInfo,
    num: number
): Promise<IGameInfo> {
    let response = await fetch(`${API_ROOT}/step`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ game, num }),
    });
    if (!response.ok) {
        throw "Failed to fetch /step.";
    }
    let info = (await response.json()) as IGameInfo;
    return info;
}

export async function fetchPlaces(game: IGameInfo): Promise<IPlaceProp[]> {
    let response = await fetch(`${API_ROOT}/places`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(game),
    });
    if (!response.ok) {
        throw "Failed to fetch /places.";
    }
    let places = (await response.json()) as { places: IPlaceProp[] };
    return places.places;
}
