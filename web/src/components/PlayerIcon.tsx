import { getPlayerColor } from "../data/Color";
import "../styles/PlayerIcon.css";

export interface IPlayerIconProps {
    playerId: number;
}

export default function PlayerIcon(props: IPlayerIconProps) {
    const color = getPlayerColor(props.playerId);

    return (
        <div className="player-icon" style={{ borderColor: color }}>
            {props.playerId}
        </div>
    );
}
