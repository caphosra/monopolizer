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
import PlaceTable from "./PlaceTable";

interface IBoardState {
    game: IGameInfo | null;
    places: IPlaceProp[] | null;
}

function Board() {
    const [state, setState] = useState<IBoardState>({
        game: null,
        places: null,
    });

    function onGameInfoUpdated(game: IGameInfo): void {
        fetchPlaces(game)
            .then((places) => {
                setState({ game, places });
            })
            .catch((_) => {
                alert("Failed to fetch /places.");
            });
    }

    useEffect(() => {
        fetchInit(4)
            .then((game) => {
                onGameInfoUpdated(game);
            })
            .catch((_) => {
                alert("Failed to fetch /init.");
            });
    }, []);

    if (state.game && state.places) {
        const game = state.game;
        return (
            <PlaceTable
                game={game}
                places={state.places}
                onGameInfoChanged={onGameInfoUpdated}
            />
        );
    } else {
        return <div>Loading...</div>;
    }
}

export default Board;
