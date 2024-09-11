import React from "react";
import TapChart from "./graphs/TapChart";
import MoneyChart from "./graphs/MoneyChart";

export interface IAnalysisBoardProps {
    taps: number[] | null;
    money: number[] | null;
    available: number[] | null;
    total: number[] | null;
}

export default function AnalysisBoard(props: IAnalysisBoardProps) {
    if (props.taps && props.money) {
        return (
            <div>
                <TapChart taps={props.taps} />
                <MoneyChart
                    money={props.money}
                    available={props.available!}
                    total={props.total!}
                />
            </div>
        );
    } else {
        return <div>Now loading...</div>;
    }
}
