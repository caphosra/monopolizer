import React from "react";
import Place from "./Place";
import { IPlaceInfo, IPlaceProp, IPlayerInfo } from "../data/Interaction";
import "../styles/PlaceTable.css";

interface IBoardProps {
    players: IPlayerInfo[];
    places: IPlaceProp[];
    infos: (IPlaceInfo | null)[];
    playerIds: number[];
    onHouseClicked: (placeId: number, nth: number) => void;
    onMortgagedClicked: (placeId: number) => void;
    onOwnerChanged: (placeId: number, owner_id: number | null) => void;
}

export default function PlaceTable(props: IBoardProps) {
    if (props.places.length !== 40 || props.infos.length !== 40) {
        return <div>The number of places is invalid.</div>;
    }

    return (
        <div>
            {[...Array(4)].map((_, row) => {
                return (
                    <div
                        key={`place-table-row-${row}`}
                        className="place-table-row"
                    >
                        {[...Array(10)].map((_, column) => {
                            const id = row * 10 + column;

                            const landingPlayers = props.players
                                .filter(
                                    (player) =>
                                        (player.jail_turn ?? 0) === 0 &&
                                        !player.is_bankrupted &&
                                        player.position === id
                                )
                                .map((player) => player.player_id);

                            return (
                                <Place
                                    key={`place${id}`}
                                    prop={props.places[id]}
                                    placeInfo={props.infos[id]}
                                    playerIds={props.playerIds}
                                    landingPlayers={landingPlayers}
                                    owner_id={props.infos[id]?.owner ?? null}
                                    onHouseClicked={props.onHouseClicked}
                                    onMortgagedClicked={
                                        props.onMortgagedClicked
                                    }
                                    onOwnerChanged={props.onOwnerChanged}
                                />
                            );
                        })}
                    </div>
                );
            })}
        </div>
    );
}
