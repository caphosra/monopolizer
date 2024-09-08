import React from "react";
import "../styles/Place.css";
import { Select } from "antd";

interface IOwnerDropdownProps {
    owner: number | null;
    player_ids: number[];
    onChanged: (id: number | null) => void;
}

export default function OwnerDropdown(props: IOwnerDropdownProps) {
    const options: { value: number | null; label: JSX.Element }[] =
        props.player_ids.map((id) => {
            return { value: id, label: <div>Player{id}</div> };
        });
    options.push({ value: null, label: <div>None</div> });

    return (
        <Select
            value={props.owner}
            onChange={props.onChanged}
            options={options}
        />
    );
}
