import React from "react";
import { useDispatch, useSelector } from "react-redux";
import { gameStateSelector } from "./slice";
import { Move, HeisterColor, Heister } from "../generated/types_pb";
import { moveHeister } from "./api"
import { MoveDirection } from "./types"

const MoveHeisterComponent = () => {
  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);

  const onMoveButtonClick = (move_direction: MoveDirection) => {
    var hardcoded_color = HeisterColor.GREEN;
    if (game_state === null) {
      console.error("Tried to move heister with no game state");
      return;
    }
    var heisters = game_state.getHeistersList();
    var green_heister = heisters.find(h => h.getHeisterColor() == hardcoded_color);
    if (green_heister === undefined) {
      console.error("Could not find information for heister");
      return;
    }
    var current_position = green_heister.getMapPosition();
    if (current_position === undefined) {
      console.error("Tried to move heister with no position");
      return;
    }
    var new_position = current_position;
    switch (+move_direction) {
      case MoveDirection.North:
        new_position.setY(current_position.getY()+1);
        break;
      case MoveDirection.East:
        new_position.setX(current_position.getX()+1);
        break;
      case MoveDirection.South:
        new_position.setY(current_position.getY()-1);
        break;
      case MoveDirection.West:
        new_position.setX(current_position.getX()-1);
        break;
      default:
        console.error("Unexpected move direction");
        break;
    }
    var move = new Move();
    console.log("Dispatching action to move GREEN heister from {} to {}", current_position, new_position);
    move.setHeisterColor(hardcoded_color);
    move.setPosition(new_position);
    dispatch(moveHeister(move));
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
