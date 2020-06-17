import React from "react";
import { useDispatch, useSelector } from "react-redux";
import { gameStateSelector } from "./slice";
import { Move } from "../generated/types_pb";
import { moveHeister } from "./api";
import MoveHeisterComponent from "./MoveHeisterComponent";
import GameWindowComponent from "./GameWindowComponent";

const GameComponent = () => {
  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);

  return (
    <div>
      <p>Game State: {game_state? JSON.stringify(game_state.toObject()) : "Waiting to join game and then pull state..."}</p>
      <MoveHeisterComponent />
      <GameWindowComponent />
    </div>
  );
};

export default GameComponent;
