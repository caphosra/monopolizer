import React from "react";
import IndexesChart from "./IndexesChart";

export interface IAnalysisBoardProps {
    taps: number[] | null;
}

export default function AnalysisBoard(props: IAnalysisBoardProps) {
    if (props.taps) {
        return (
            <IndexesChart taps={props.taps} />
        );
    }
    else {
        return <div>Now loading...</div>;
    }
}
