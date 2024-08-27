import React, { useState } from "react";
import logo from "./logo.svg";
import "../styles/Place.css";
import { IGameInfo, IPlaceProp } from "../data/Interaction";
import { isEstate, isProperty } from "../data/Utils";

interface IHouseProps {
    houses_num: number;
    nth: number;
    onClick: (nth: number) => void;
}

function House(props: IHouseProps) {
    return (
        <div
            className={
                props.nth <= props.houses_num
                    ? props.houses_num === 5
                        ? "house hotel-active"
                        : "house house-active"
                    : "house house-inactive"
            }
            onClick={() => props.onClick(props.nth)}
        />
    );
}

interface IPlaceProps {
    prop: IPlaceProp;
    game: IGameInfo;
    onChanged: (game: IGameInfo) => void;
}

export default function Place(props: IPlaceProps) {
    const property = isProperty(props.prop.color);
    const estate = isEstate(props.prop.color);

    function mortgagedOnClick() {
        const updatedPlaces = props.game.places.map((place) => {
            if (place.place_id == props.prop.place_id) {
                place.is_mortgaged = !place.is_mortgaged;
                if (estate) {
                    place.houses = 0;
                }
            }
            return place;
        });
        props.onChanged({ ...props.game, places: updatedPlaces });
    }

    function houseOnClick(nth: number) {
        let changed = false;
        const updatedPlaces = props.game.places.map((place) => {
            if (place.place_id == props.prop.place_id) {
                if (!place.is_mortgaged) {
                    if (nth === place.houses) {
                        nth = 0;
                    }
                    changed = true;
                    return { ...place, houses: nth };
                }
            }
            return place;
        });
        if (changed) {
            props.onChanged({ ...props.game, places: updatedPlaces });
        }
    }

    const placeState = props.game.places.find(
        (place) => place.place_id === props.prop.place_id
    );

    return (
        <div className="place">
            {property ? (
                <div
                    className={
                        placeState?.is_mortgaged
                            ? "place-name mortgaged"
                            : "place-name"
                    }
                    onClick={mortgagedOnClick}
                >
                    {props.prop.name}
                </div>
            ) : (
                <div className="place-name">{props.prop.name}</div>
            )}

            {estate ? (
                <div className="houses-container">
                    {[1, 2, 3, 4, 5].map((nth) => (
                        <House
                            houses_num={placeState?.houses ?? 0}
                            nth={nth}
                            onClick={houseOnClick}
                        />
                    ))}
                </div>
            ) : (
                <div></div>
            )}

            {estate && props.prop.rent ? (
                <div>Rent: ${props.prop.rent}</div>
            ) : (
                <div></div>
            )}

            {property ? <div>Price: ${props.prop.price}</div> : <div></div>}
            {estate ? (
                <div>House price: ${props.prop.house_price}</div>
            ) : (
                <div></div>
            )}
        </div>
    );
}
