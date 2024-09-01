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
import PlayerTable from "./PlayerTable";
import { getPlaceInfoList } from "../data/Utils";

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

    function onHouseClicked(placeId: number, nth: number): void {
        setState((state) => {
            const places = state.game!.places.map((place) => {
                if (place.place_id === placeId && !place.is_mortgaged) {
                    let houses = nth;
                    if (place.houses === nth) {
                        houses = 0;
                    }
                    return { ...place, houses };
                }
                return place;
            });

            const newGame = { ...state.game!, places };
            fetchPlaces(newGame)
                .then((places) => {
                    setState((state) => {
                        return { ...state, places };
                    });
                })
                .catch((_) => {
                    alert("Failed to fetch /places.");
                });
            return { ...state, game: newGame };
        });
    }

    function onMortgagedClicked(placeId: number): void {
        setState((state) => {
            const places = state.game!.places.map((place) => {
                if (place.place_id == placeId) {
                    const is_mortgaged = !place.is_mortgaged;
                    if (place.houses) {
                        return { ...place, is_mortgaged, houses: 0 };
                    } else {
                        return { ...place, is_mortgaged };
                    }
                }
                return place;
            });

            const newGame = { ...state.game!, places };
            fetchPlaces(newGame)
                .then((places) => {
                    setState((state) => {
                        return { ...state, places };
                    });
                })
                .catch((_) => {
                    alert("Failed to fetch /places.");
                });
            return { ...state, game: newGame };
        });
    }

    function onOwnerChanged(placeId: number, owner_id: number | null): void {
        setState((state) => {
            const places = state.game!.places.map((place) => {
                if (place.place_id == placeId) {
                    return { ...place, owner: owner_id ?? undefined };
                }
                return place;
            });

            const newGame = { ...state.game!, places };
            fetchPlaces(newGame)
                .then((places) => {
                    setState((state) => {
                        return { ...state, places };
                    });
                })
                .catch((_) => {
                    alert("Failed to fetch /places.");
                });
            return { ...state, game: newGame };
        });
    }

    if (state.game && state.places) {
        const game = state.game;
        const placeInfos = getPlaceInfoList(game);

        const playerIds = state.game.players
            .filter((player) => !player.is_bankrupted)
            .map((player) => player.player_id);

        return (
            <div className="root">
                <Header onClick={onContentTypeChanged} />
                <div className="main">
                    {
                        {
                            places: (
                                <PlaceTable
                                    places={state.places}
                                    infos={placeInfos}
                                    playerIds={playerIds}
                                    onHouseClicked={onHouseClicked}
                                    onMortgagedClicked={onMortgagedClicked}
                                    onOwnerChanged={onOwnerChanged}
                                />
                            ),
                            players: (
                                <PlayerTable
                                    game={game}
                                    onChanged={onGameInfoUpdated}
                                />
                            ),
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
