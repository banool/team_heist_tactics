import React from "react";
import { useDispatch, useSelector } from "react-redux";
import { moveHeister } from "./api";
import { MoveDirection } from "./types";
import { gameStateSelector } from "./slice";

const MoveHeisterComponent = () => {
  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);

  const onMoveButtonClick = (move_direction: MoveDirection) => {
    // Exclamation mark because we only show this component when we have game state.
    dispatch(moveHeister(game_state!, move_direction));
  };

  return (
    <div>
      <button onClick={() => onMoveButtonClick(MoveDirection.West)}>⬅️</button>
      <button onClick={() => onMoveButtonClick(MoveDirection.South)}>⬇️</button>
      <button onClick={() => onMoveButtonClick(MoveDirection.North)}>⬆️</button>
      <button onClick={() => onMoveButtonClick(MoveDirection.East)}>➡️</button>
    </div>
  );
};

export default MoveHeisterComponent;
