import React from "react";
import { useDispatch, useSelector } from "react-redux";
import { gameStateSelector, numInvalidMoveAttemptsSelector } from "./slice";
import MoveHeisterComponent from "./MoveHeisterComponent";
import GameWindowComponent from "./GameWindowComponent";

const GameComponent = () => {
  /*
  <MoveHeisterComponent />
  <p>
    Game State:{" "}
    {game_state
      ? JSON.stringify(game_state.toObject())
      : "Waiting to join game and then pull state..."}
  </p>
  */

  return (
    <div>
      <GameWindowComponent />
    </div>
  );
};

export default GameComponent;
