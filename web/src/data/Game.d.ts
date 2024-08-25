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
