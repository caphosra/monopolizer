import React from "react";
import { ResponsiveContainer, PieChart, Pie, Tooltip, Cell } from "recharts";
import "../../styles/IndexesChart.css";

export interface ISurvivalGraphProps {
    id: number;
    survivalRate: number;
}

export default function SurvivalGraph(props: ISurvivalGraphProps) {
    const data = [
        {
            name: "survived",
            value: parseFloat(props.survivalRate.toFixed(2)),
        },
        {
            name: "bankrupted",
            value: parseFloat((1 - props.survivalRate).toFixed(2)),
        },
    ];

    return (
        <div className="chart">
            <ResponsiveContainer width="100%" height="100%">
                <PieChart width={400} height={400}>
                    <Pie
                        dataKey="value"
                        data={data}
                        cx="50%"
                        cy="50%"
                        outerRadius={80}
                        fill="#8884d8"
                        label
                    >
                        <Cell fill="green" />
                        <Cell fill="red" />
                    </Pie>
                    <Tooltip />
                </PieChart>
            </ResponsiveContainer>
            <div>Player {props.id}</div>
        </div>
    );
}
