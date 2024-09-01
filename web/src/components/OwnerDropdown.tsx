import React, { useState } from "react";
import logo from "./logo.svg";
import "../styles/Place.css";
import { IGameInfo, IPlaceProp } from "../data/Interaction";
import { isEstate, isProperty } from "../data/Utils";
import { Select } from "antd";

interface IOwnerDropdownProps {
    owner: number | null;
    player_ids: number[];
    onChanged: (id: number | null) => void;
}

export default function OwnerDropdown(props: IOwnerDropdownProps) {
    const options: { value: number | null; label: any }[] =
        props.player_ids.map((id) => {
            return { value: id, label: <div>Player{id}</div> };
        });
    options.push({ value: null, label: <div>None</div> });

    return (
        <Select
            defaultValue={props.owner}
            onChange={props.onChanged}
            options={options}
        />
    );
}
