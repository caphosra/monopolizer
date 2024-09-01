import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import Place from "./Place";
import {
    fetchInit,
    fetchPlaces,
    fetchStep,
    IGameInfo,
    IPlaceInfo,
    IPlaceProp,
} from "../data/Interaction";
import "../styles/PlaceTable.css";

interface IBoardProps {
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
                    <div className="place-table-row">
                        {[...Array(10)].map((_, column) => {
                            const id = row * 10 + column;

                            return (
                                <Place
                                    key={`place${id}`}
                                    prop={props.places[id]}
                                    placeInfo={props.infos[id]}
                                    player_ids={props.playerIds}
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
