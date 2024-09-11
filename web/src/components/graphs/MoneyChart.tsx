import React from "react";
import {
    BarChart,
    Bar,
    Rectangle,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    Legend,
    ResponsiveContainer,
} from "recharts";
import "../../styles/IndexesChart.css";

export interface IMoneyChartProps {
    money: number[];
    available: number[];
    total: number[];
}

export default function MoneyChart(props: IMoneyChartProps) {
    const data = [...Array(props.money.length)].map((_, idx) => {
        return {
            name: `player${idx}`,
            money: props.money[idx],
            available: props.available[idx],
            total: props.total[idx],
        };
    });

    return (
        <div className="chart">
            <ResponsiveContainer width="100%" height="100%">
                <BarChart
                    width={500}
                    height={300}
                    data={data}
                    margin={{
                        top: 5,
                        right: 30,
                        left: 20,
                        bottom: 5,
                    }}
                >
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="name" />
                    <YAxis />
                    <Tooltip />
                    <Legend />
                    <Bar
                        dataKey="money"
                        fill="red"
                        activeBar={<Rectangle fill="pink" stroke="blue" />}
                    />
                    <Bar
                        dataKey="available"
                        fill="green"
                        activeBar={<Rectangle fill="pink" stroke="blue" />}
                    />
                    <Bar
                        dataKey="total"
                        fill="blue"
                        activeBar={<Rectangle fill="pink" stroke="blue" />}
                    />
                </BarChart>
            </ResponsiveContainer>
        </div>
    );
}
