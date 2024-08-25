import React, { useState } from "react";
import logo from "./logo.svg";
import "../styles/Place.css";

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
                    ? (props.houses_num === 5 ? "house hotel-active" : "house house-active")
                    : "house house-inactive"
            }
            onClick={() => props.onClick(props.nth)}
        />
    );
}

interface IPlaceProps {
    name: string;
    is_mortgaged: boolean;
    houses: number;
}

interface IPlaceState {
    is_mortgaged: boolean;
    houses: number;
}

export default function Place(props: IPlaceProps) {
    let [state, setState] = useState<IPlaceState>({ is_mortgaged: false, houses: props.houses });

    let mortgagedOnClick = () => {
        setState({ is_mortgaged: !state.is_mortgaged, houses: 0 });
    };

    let houseOnClick = (nth: number) => {
        if (!state.is_mortgaged) {
            if (nth === state.houses) {
                nth = 0;
            }
            setState({ ...state, houses: nth });
        }
    };

    return (
        <div className="place">
            <div className={state.is_mortgaged ? "place-name mortgaged" : "place-name"} onClick={mortgagedOnClick}>{props.name}</div>
            <div className="houses-container">
                {[1, 2, 3, 4, 5].map((nth) => (
                    <House
                        houses_num={state.houses}
                        nth={nth}
                        onClick={houseOnClick}
                    />
                ))}
            </div>
        </div>
    );
}
