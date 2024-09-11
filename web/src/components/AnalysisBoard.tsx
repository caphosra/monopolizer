import React from "react";
import TapChart from "./graphs/TapChart";
import MoneyChart from "./graphs/MoneyChart";
import SurvivalGraph from "./graphs/SurvivalGraph";
import "../styles/IndexesChart.css";

export interface IAnalysisBoardProps {
    taps: number[] | null;
    money: number[] | null;
    available: number[] | null;
    total: number[] | null;
    survivalRates: number[] | null;
}

export default function AnalysisBoard(props: IAnalysisBoardProps) {
    if (props.taps && props.money && props.survivalRates) {
        return (
            <div className="chart-box">
                <div className="tap-chart-box">
                    <TapChart taps={props.taps} />
                </div>
                <div className="money-chart-box">
                    <MoneyChart
                        money={props.money}
                        available={props.available!}
                        total={props.total!}
                    />
                </div>
                <div className="survival-graph-box">
                    {props.survivalRates.map((rate, idx) => {
                        return (
                            <SurvivalGraph
                                key={`survival-${idx}`}
                                id={idx}
                                survivalRate={rate}
                            />
                        );
                    })}
                </div>
            </div>
        );
    } else {
        return <div>Now loading...</div>;
    }
}
