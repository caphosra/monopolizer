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
    const response = await fetch(`${API_ROOT}/init?num=${num}`);
    if (!response.ok) {
        throw new Error("Failed to fetch /init.");
    }
    const info = (await response.json()) as IGameInfo;
    return info;
}

export async function fetchStep(
    game: IGameInfo,
    num: number
): Promise<IGameInfo> {
    const response = await fetch(`${API_ROOT}/step`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ game, num }),
    });
    if (!response.ok) {
        throw new Error("Failed to fetch /step.");
    }
    const info = (await response.json()) as IGameInfo;
    return info;
}

export async function fetchPlaces(game: IGameInfo): Promise<IPlaceProp[]> {
    const response = await fetch(`${API_ROOT}/places`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(game),
    });
    if (!response.ok) {
        throw new Error("Failed to fetch /places.");
    }
    const places = (await response.json()) as { places: IPlaceProp[] };
    return places.places;
}

export async function fetchTap(game: IGameInfo): Promise<number[]> {
    const response = await fetch(`${API_ROOT}/tap`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(game),
    });
    if (!response.ok) {
        throw new Error("Failed to fetch /tap.");
    }
    const places = (await response.json()) as { taps: number[] };
    return places.taps;
}

export interface IFetchMoneyResponse {
    money: number[];
    available: number[];
    total: number[];
}

export async function fetchMoney(
    game: IGameInfo
): Promise<IFetchMoneyResponse> {
    const response = await fetch(`${API_ROOT}/money`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(game),
    });
    if (!response.ok) {
        throw new Error("Failed to fetch /money.");
    }
    const result = (await response.json()) as IFetchMoneyResponse;
    return result;
}

export async function fetchSurvival(
    game: IGameInfo,
    num: number,
    depth: number
): Promise<number[]> {
    const response = await fetch(`${API_ROOT}/survival`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ game, num, depth }),
    });
    if (!response.ok) {
        throw new Error("Failed to fetch /survival.");
    }
    const result = (await response.json()) as { survival_rates: number[] };
    return result.survival_rates;
}
