import React from "react";
import { BarChart, Bar, Rectangle, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from "recharts";
import "../styles/IndexesChart.css";

export interface IIndexesChartProps {
    taps: number[];
}

export default function IndexesChart(props: IIndexesChartProps) {
    const data = props.taps.map((tap, idx) => {
        return {
            name: `player${idx}`,
            tap
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
                <Bar dataKey="tap" fill="#8884d8" activeBar={<Rectangle fill="pink" stroke="blue" />} />
                </BarChart>
            </ResponsiveContainer>
        </div>
    );
}
