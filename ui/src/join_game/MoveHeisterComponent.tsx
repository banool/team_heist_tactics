import { connectionStatusSelector, gameStateSelector } from "./slice";
import { useDispatch, useSelector } from "react-redux";

import { HeisterColor } from "../generated/types_pb";
import { MoveDirection } from "./types";
import React from "react";
import { moveHeister } from "./api";

const MoveHeisterComponent = () => {
  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);
  const connection_status = useSelector(connectionStatusSelector);

  const onMoveButtonClick = (move_direction: MoveDirection) => {
    // Exclamation mark because we only show this component when we have game state.
    // This is hardcoded to the yellow heister.
    dispatch(
      moveHeister(
        game_state!,
        connection_status,
        move_direction,
        HeisterColor.YELLOW
      )
    );
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
