import { PlaceColor } from "./Interaction";

export function isProperty(color: PlaceColor): boolean {
    return color != "None";
}

export function isEstate(color: PlaceColor): boolean {
    return color != "None" && color != "Railroad" && color != "Utilities";
}
