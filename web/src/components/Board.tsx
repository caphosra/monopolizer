import React from "react";
import logo from "./logo.svg";
import Place from "./Place";

function Board() {
    return <Place name="Boardwalk" is_mortgaged={false} houses={0} />;
}

export default Board;
