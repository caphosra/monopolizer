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
import { VscRunAll } from "react-icons/vsc";

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
        label: "Actions",
        key: "actions",
        icon: <VscRunAll />,
        children: [
            {
                label: "Step 1 turn",
                key: "step1",
            },
            {
                label: "Step 10 turns",
                key: "step10",
            },
        ],
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
    onActionInvoked: (key: string) => void;
}

export default function Header(props: IHeaderProps) {
    const [selectedKey, setSelectedKey] = useState("places");

    function onClick(e: { key: string }) {
        switch (e.key) {
            case "step1":
            case "step10":
                props.onActionInvoked(e.key);
                break;
            default:
                props.onClick(e.key as ContentType);
                setSelectedKey(e.key);
                break;
        }
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
