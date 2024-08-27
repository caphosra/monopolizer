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
import "../styles/Board.css";
import Header, { ContentType } from "./Header";

interface IBoardState {
    game: IGameInfo | null;
    places: IPlaceProp[] | null;
    content: ContentType;
}

function Board() {
    const [state, setState] = useState<IBoardState>({
        game: null,
        places: null,
        content: "places",
    });

    function onGameInfoUpdated(game: IGameInfo): void {
        fetchPlaces(game)
            .then((places) => {
                setState((state) => {
                    return { ...state, game, places };
                });
            })
            .catch((_) => {
                alert("Failed to fetch /places.");
            });
    }

    function onContentTypeChanged(content: ContentType): void {
        setState((state) => {
            return { ...state, content };
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
            <div className="root">
                <Header onClick={onContentTypeChanged} />
                <div className="main">
                    {
                        {
                            places: (
                                <PlaceTable
                                    game={game}
                                    places={state.places}
                                    onGameInfoChanged={onGameInfoUpdated}
                                />
                            ),
                            players: <div>Players</div>,
                            analysis: <div>Analysis</div>,
                        }[state.content]
                    }
                </div>
            </div>
        );
    } else {
        return <div>Loading...</div>;
    }
}

export default Board;
