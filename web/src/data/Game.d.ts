export interface IPlayer {
    player_id: number;
    money: number;
    is_bankrupted: boolean;
    jail_turn: number | undefined;
    position: number;
}

export interface IPlace {
    place_id: number;
    owner: number | undefined;
    is_mortgaged: boolean;
    houses: number | undefined;
}

export interface IGame {
    turn: number;
    players: IPlayer[];
    places: IPlace[];
}
