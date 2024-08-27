import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import Place from "./Place";
import {
    fetchInit,
    fetchPlaces,
    fetchStep,
    IGameInfo,
    IPlaceProp,
} from "../data/Interaction";
import "../styles/PlaceTable.css";

interface IBoardProps {
    game: IGameInfo;
    places: IPlaceProp[];
    onGameInfoChanged: (game: IGameInfo) => void;
}

export default function PlaceTable(props: IBoardProps) {
    if (props.places.length !== 40) {
        return <div>The number of places is invalid.</div>;
    }

    return (
        <div>
            {[...Array(4)].map((_, row) => {
                return (
                    <div className="place-table-row">
                        {[...Array(10)].map((_, column) => {
                            return (
                                <Place
                                    game={props.game}
                                    prop={props.places[row * 10 + column]}
                                    onChanged={props.onGameInfoChanged}
                                />
                            );
                        })}
                    </div>
                );
            })}
        </div>
    );
}
