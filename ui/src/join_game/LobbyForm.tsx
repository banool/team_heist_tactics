import { gameHandleSelector, gameStateSelector } from "./slice";
import { useDispatch, useSelector } from "react-redux";

import { Player } from "../generated/types_pb";
import React from "react";
import { startGame } from "./api";

const LobbyForm = () => {
  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);
  const game_handle = useSelector(gameHandleSelector);

  const onSubmit = () => {
    dispatch(startGame(game_handle!));
  };

  const players = game_state!
    .getPlayersList()
    .map((p: Player) => <li key={p.getName()}>{p.getName()}</li>);

  return (
    <div>
      <h3>Players in lobby:</h3>
      <ul>{players}</ul>
      <button autoFocus type="submit" onClick={onSubmit}>
        Start Game
      </button>
    </div>
  );
};

export default LobbyForm;
