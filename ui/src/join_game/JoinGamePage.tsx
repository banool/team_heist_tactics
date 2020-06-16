import React from "react";
import { useSelector } from "react-redux";

import JoinGameForm from "./JoinGameForm";
import ConnectionStatusComponent from "./ConnectionStatusComponent";
import GameComponent from "./GameComponent";
import { useDispatch } from "react-redux";

import { connectionStatusSelector } from "./slice";
import { ConnectionStatus } from "./types";


type JoinGamePageProps = {};
const JoinGamePage = ({ }: JoinGamePageProps) => {
  const connection_status = useSelector(connectionStatusSelector);

  const dispatch = useDispatch();

  // This dispatches a function on page load.
  /*
  useEffect(() => {
    dispatch(fetchScents());
  }, [dispatch]);
  */

  const handleKeyPress = (event) => {
    console.log("asdasdsa key event ", event);
    //this.props.keyPressAction(event.keyCode);
  };

  document.addEventListener('keypress', handleKeyPress);

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
