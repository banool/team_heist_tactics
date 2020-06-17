import React from "react";
import { useDispatch, useSelector } from "react-redux";
import { gameStateSelector } from "./slice";
import MoveHeisterComponent from "./MoveHeisterComponent";
import GameWindowComponent from "./GameWindowComponent";
import { CANVAS_WIDTH, CANVAS_HEIGHT } from "../constants/other";

const GameComponent = () => {
  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);

  return (
    <div>
      <GameWindowComponent width={CANVAS_WIDTH} height={CANVAS_HEIGHT} />
      <MoveHeisterComponent />
      <p>Game State: {game_state? JSON.stringify(game_state.toObject()) : "Waiting to join game and then pull state..."}</p>
    </div>
  );
};

export default GameComponent;
