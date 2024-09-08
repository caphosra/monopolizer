import React from "react";
import { InputNumber, Select, Table } from "antd";
import { IPlaceProp, IPlayerInfo } from "../data/Interaction";
import "../styles/PlayerTable.css";

export interface IPlayerProps {
    players: IPlayerInfo[];
    places: IPlaceProp[];
    onMoneyChanged: (player_id: number, money: number) => void;
    onPositionChanged: (player_id: number, position: number) => void;
}

interface IPlayerTableContent {
    id: number;
    money: number;
    status: string;
    position: number;
}

const Column = Table.Column<IPlayerTableContent>;

export default function PlayerTable(props: IPlayerProps) {
    const data = props.players.map((player) => {
        const status = player.is_bankrupted
            ? "Bankrupted"
            : player.jail_turn
              ? `In Jail (${player.jail_turn})`
              : "Active";

        return {
            id: player.player_id,
            money: player.money,
            status,
            position: player.position,
        };
    });

    function renderMoneyInputField(content: IPlayerTableContent) {
        return (
            <InputNumber
                prefix="$"
                value={content.money}
                disabled={content.status == "Bankrupted"}
                onChange={(val) => props.onMoneyChanged(content.id, val ?? 0)}
            />
        );
    }

    function renderPositionField(content: IPlayerTableContent) {
        const options = props.places.map((place) => {
            return {
                label: place.name,
                value: place.place_id,
            };
        });

        return (
            <Select
                key={`select-position${content.id}`}
                style={{ width: "100%" }}
                value={content.position}
                disabled={content.status == "Bankrupted"}
                options={options}
                onChange={(pos) => props.onPositionChanged(content.id, pos)}
            />
        );
    }

    return (
        <div className="player-table">
            <Table dataSource={[...data]}>
                <Column
                    title="ID"
                    dataIndex="id"
                    sorter={(a, b) => a.id - b.id}
                />
                <Column
                    title="Position"
                    dataIndex="position"
                    sorter={(a, b) => a.position - b.position}
                    render={(_, content) => renderPositionField(content)}
                />
                <Column
                    title="Money"
                    dataIndex="money"
                    sorter={(a, b) => a.money - b.money}
                    render={(_, content) => renderMoneyInputField(content)}
                />
                <Column title="Status" dataIndex="status" />
            </Table>
        </div>
    );
}
