import React, { useEffect, useState } from "react";
import {
    fetchInit,
    fetchMoney,
    fetchPlaces,
    fetchStep,
    fetchSurvival,
    fetchTap,
    IGameInfo,
    IPlaceProp,
} from "../data/Interaction";
import PlaceTable from "./PlaceTable";
import "../styles/Board.css";
import Header, { ContentType } from "./Header";
import PlayerTable from "./PlayerTable";
import { getPlaceInfoList } from "../data/Utils";
import AnalysisBoard from "./AnalysisBoard";

interface IBoardState {
    game: IGameInfo | null;
    places: IPlaceProp[] | null;
    content: ContentType;
    taps: number[] | null;
    money: number[] | null;
    available: number[] | null;
    total: number[] | null;
    survivalRates: number[] | null;
}

function Board() {
    const [state, setState] = useState<IBoardState>({
        game: null,
        places: null,
        content: "places",
        taps: null,
        money: null,
        available: null,
        total: null,
        survivalRates: null,
    });

    function onGameInfoUpdated(game: IGameInfo): void {
        fetchPlaces(game)
            .then((places) => {
                setState((state) => {
                    return { ...state, game, places };
                });
            })
            .catch(() => {
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
            .catch(() => {
                alert("Failed to fetch /init.");
            });
    }, []);

    useEffect(() => {
        if (state.game) {
            fetchTap(state.game)
                .then((taps) => {
                    setState((state) => {
                        return { ...state, taps };
                    });
                })
                .catch(() => {
                    alert("Failed to fetch /tap.");
                });
        }
    }, [state.game]);

    useEffect(() => {
        if (state.game) {
            fetchMoney(state.game)
                .then((money) => {
                    setState((state) => {
                        return {
                            ...state,
                            money: money.money,
                            available: money.available,
                            total: money.total,
                        };
                    });
                })
                .catch(() => {
                    alert("Failed to fetch /money.");
                });
        }
    }, [state.game]);

    useEffect(() => {
        const NUM = 10;
        const DEPTH = 10;
        if (state.game) {
            fetchSurvival(state.game, NUM, DEPTH)
                .then((survivalRates) => {
                    setState((state) => {
                        return { ...state, survivalRates };
                    });
                })
                .catch(() => {
                    alert("Failed to fetch /survival.");
                });
        }
    }, [state.game]);

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
                .catch(() => {
                    alert("Failed to fetch /places.");
                });
            return { ...state, game: newGame };
        });
    }

    function onMortgagedClicked(placeId: number): void {
        setState((state) => {
            const places = state.game!.places.map((place) => {
                if (place.place_id === placeId) {
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
                .catch(() => {
                    alert("Failed to fetch /places.");
                });
            return { ...state, game: newGame };
        });
    }

    function onOwnerChanged(placeId: number, owner_id: number | null): void {
        setState((state) => {
            const places = state.game!.places.map((place) => {
                if (place.place_id === placeId) {
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
                .catch(() => {
                    alert("Failed to fetch /places.");
                });
            return { ...state, game: newGame };
        });
    }

    function onActionInvoked(key: string): void {
        switch (key) {
            case "step1":
                fetchStep(state.game!, 1)
                    .then((game) => {
                        onGameInfoUpdated(game);
                    })
                    .catch(() => {
                        alert("Failed to fetch /step.");
                    });
                break;
            case "step10":
                fetchStep(state.game!, 10)
                    .then((game) => {
                        onGameInfoUpdated(game);
                    })
                    .catch(() => {
                        alert("Failed to fetch /step.");
                    });
                break;
        }
    }

    function onMoneyChanged(player_id: number, money: number): void {
        setState((state) => {
            const players = state.game!.players.map((player) => {
                if (player.player_id === player_id) {
                    return { ...player, money };
                }
                return player;
            });

            const newGame = { ...state.game!, players };
            return { ...state, game: newGame };
        });
    }

    function onPositionChanged(player_id: number, position: number): void {
        setState((state) => {
            const players = state.game!.players.map((player) => {
                if (player.player_id === player_id) {
                    return { ...player, position };
                }
                return player;
            });

            const newGame = { ...state.game!, players };
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
                <Header
                    onClick={onContentTypeChanged}
                    onActionInvoked={onActionInvoked}
                />
                <div className="main">
                    {
                        {
                            places: (
                                <PlaceTable
                                    players={state.game.players}
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
                                    players={state.game.players}
                                    places={state.places}
                                    onMoneyChanged={onMoneyChanged}
                                    onPositionChanged={onPositionChanged}
                                />
                            ),
                            analysis: (
                                <AnalysisBoard
                                    taps={state.taps}
                                    money={state.money}
                                    available={state.available}
                                    total={state.total}
                                    survivalRates={state.survivalRates}
                                />
                            ),
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
