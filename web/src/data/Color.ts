const PLAYER_COLORS = ["red", "blue", "green", "orange"];

export function getPlayerColor(playerId: number): string {
    return PLAYER_COLORS[playerId % PLAYER_COLORS.length];
}
