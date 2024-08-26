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
