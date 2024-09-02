import React, { useState } from "react";
import logo from "./logo.svg";
import "../styles/Place.css";
import { IGameInfo, IPlaceInfo, IPlaceProp } from "../data/Interaction";
import { isEstate, isProperty } from "../data/Utils";
import OwnerDropdown from "./OwnerDropdown";
import PlayerIcon from "./PlayerIcon";

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
    placeInfo: IPlaceInfo | null;
    owner_id: number | null;
    playerIds: number[];
    landingPlayers: number[];
    onHouseClicked: (placeId: number, nth: number) => void;
    onMortgagedClicked: (PlaceId: number) => void;
    onOwnerChanged: (placeId: number, owner_id: number | null) => void;
}

export default function Place(props: IPlaceProps) {
    const property = isProperty(props.prop.color);
    const estate = isEstate(props.prop.color);

    let placeNameComponent = (
        <div className="place-name">{props.prop.name}</div>
    );
    let housesComponent = <div></div>;
    let ownerComponent = <div></div>;
    let rentComponent = <div></div>;
    let priceComponent = <div></div>;
    let housePriceComponent = <div></div>;
    const playersComponent = (
        <div className="place-players">
            {props.landingPlayers.map((playerId) => (
                <PlayerIcon playerId={playerId} />
            ))}
        </div>
    );

    if (property && props.placeInfo) {
        const placeInfo = props.placeInfo;

        placeNameComponent = (
            <div
                className={
                    placeInfo.is_mortgaged
                        ? "place-name mortgaged"
                        : "place-name"
                }
                onClick={() => props.onMortgagedClicked(props.prop.place_id)}
            >
                {props.prop.name}
            </div>
        );
        ownerComponent = (
            <OwnerDropdown
                owner={props.owner_id}
                player_ids={props.playerIds}
                onChanged={(id) =>
                    props.onOwnerChanged(props.prop.place_id, id)
                }
            />
        );
        priceComponent = <div>Price: ${props.prop.price}</div>;

        if (estate) {
            const houses = placeInfo.houses ?? 0;

            housesComponent = (
                <div className="houses-container">
                    {[1, 2, 3, 4, 5].map((nth) => (
                        <House
                            houses_num={houses}
                            nth={nth}
                            onClick={(nth) =>
                                props.onHouseClicked(props.prop.place_id, nth)
                            }
                        />
                    ))}
                </div>
            );
            if (props.prop.rent) {
                rentComponent = <div>Rent: ${props.prop.rent}</div>;
            }
            housePriceComponent = (
                <div>House price: ${props.prop.house_price}</div>
            );
        }
    }

    return (
        <div className="place">
            {placeNameComponent}
            {housesComponent}
            {ownerComponent}
            {rentComponent}
            {priceComponent}
            {housePriceComponent}
            {playersComponent}
        </div>
    );
}
