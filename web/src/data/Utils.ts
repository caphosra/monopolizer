import { IGameInfo, IPlaceInfo, PlaceColor } from "./Interaction";

export function getPlaceInfoList(game: IGameInfo): (IPlaceInfo | null)[] {
    const placeInfoList: (IPlaceInfo | null)[] = [...Array(40)].map(() => null);
    for (const info of game.places) {
        placeInfoList[info.place_id] = info;
    }
    return placeInfoList;
}

export function isProperty(color: PlaceColor): boolean {
    return color !== "None";
}

export function isEstate(color: PlaceColor): boolean {
    return color !== "None" && color !== "Railroad" && color !== "Utilities";
}
