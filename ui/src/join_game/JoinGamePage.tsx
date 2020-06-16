import React, { useState } from "react";
import { useSelector } from "react-redux";

import JoinGameForm from "./JoinGameForm";
import ConnectionStatusComponent from "./ConnectionStatusComponent";
import GameComponent from "./GameComponent";
import { useDispatch } from "react-redux";
import { gameStateSelector } from "./slice";

import { connectionStatusSelector } from "./slice";
import { ConnectionStatus } from "./types";

import { handleKeyInput } from "./api";

type JoinGamePageProps = {};
const JoinGamePage = ({ }: JoinGamePageProps) => {
  const connection_status = useSelector(connectionStatusSelector);

  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);

  const [keyPressed, setKeyPressed] = useState(false);

  // This dispatches a function on page load.
  /*
  useEffect(() => {
    dispatch(fetchScents());
  }, [dispatch]);
  */

  // https://stackoverflow.com/questions/5353254/javascript-onkeydown-event-fire-only-once
  const handleKeyDown = (event) => {
    document.removeEventListener('keydown', handleKeyDown);
    console.debug("Key event", event);
    dispatch(handleKeyInput(game_state, event.key));
  };

  const handleKeyUp = (_) => {
    document.addEventListener('keydown', handleKeyDown);
  };

  document.addEventListener('keydown', handleKeyDown);
  document.addEventListener('keyup', handleKeyUp);

  return (
    <div>
      <h1>Team Heist Tactics</h1>
      {connection_status != ConnectionStatus.Connected ? <JoinGameForm /> : null}
      <ConnectionStatusComponent />
      {connection_status == ConnectionStatus.Connected ? <GameComponent /> : null}
    </div>
  );
};

export default JoinGamePage;
