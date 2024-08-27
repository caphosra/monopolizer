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
import { Menu } from "antd";
import { FaHouse } from "react-icons/fa6";
import { FaUser, FaCalculator } from "react-icons/fa";

const items = [
    {
        label: "Places",
        key: "places",
        icon: <FaHouse />,
    },
    {
        label: "Players",
        key: "players",
        icon: <FaUser />,
    },
    {
        label: "Analysis",
        key: "analysis",
        icon: <FaCalculator />,
    },
];

export type ContentType = "places" | "players" | "analysis";

interface IHeaderProps {
    onClick: (key: ContentType) => void;
}

export default function Header(props: IHeaderProps) {
    const [selectedKey, setSelectedKey] = useState("places");

    function onClick(e: { key: string }) {
        setSelectedKey(e.key);
        props.onClick(e.key as ContentType);
    }

    return (
        <Menu
            onClick={onClick}
            selectedKeys={[selectedKey]}
            mode="horizontal"
            items={items}
        />
    );
}
