import React from "react";

import { useDispatch, useSelector } from "react-redux";
import { startGame } from "./api";
import { gameStateSelector, gameHandleSelector } from "./slice";
import { Player } from "../generated/types_pb";

const LobbyForm = () => {
  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);
  const game_handle = useSelector(gameHandleSelector);

  const onSubmit = () => {
    dispatch(startGame(game_handle!));
  };

  const players = game_state!
    .getPlayersList()
    .map((p: Player) => <li>{p.getName()}</li>);

  return (
    <div>
      <h3>Players in lobby:</h3>
      <ul>{players}</ul>
      <input type="button" value="Start Game" onClick={onSubmit} />
    </div>
  );
};

export default LobbyForm;
