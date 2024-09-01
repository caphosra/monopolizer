import { Table } from "antd";
import type { TableColumnsType } from "antd";
import { IGameInfo, IPlayerInfo } from "../data/Interaction";

export interface IPlayerProps {
    game: IGameInfo;
    onChanged: (game: IGameInfo) => void;
}

interface IPlayerTableContent {
    id: number;
    money: number;
    status: string;
}

const Column = Table.Column<IPlayerTableContent>;

export default function PlayerTable(props: IPlayerProps) {
    const data = props.game.players.map((player) => {
        const status = player.is_bankrupted
            ? "Bankrupted"
            : player.jail_turn
              ? `In Jail (${player.jail_turn})`
              : "Active";

        return {
            id: player.player_id,
            money: player.money,
            status,
        };
    });

    return (
        <Table dataSource={data}>
            <Column title="ID" dataIndex="id" sorter={(a, b) => a.id - b.id} />
            <Column
                title="Money"
                dataIndex="money"
                sorter={(a, b) => a.money - b.money}
            />
            <Column title="Status" dataIndex="status" />
        </Table>
    );
}
