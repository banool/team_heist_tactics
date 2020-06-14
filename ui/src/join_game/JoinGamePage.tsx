import React from "react";
import { useSelector } from "react-redux";

import JoinGameForm from "./JoinGameForm";
import ConnectionStatusComponent from "./ConnectionStatusComponent";
import GameComponent from "./GameComponent";

import { connectionStatusSelector } from "./slice";
import { ConnectionStatus } from "./types";


type JoinGamePageProps = {};
const JoinGamePage = ({ }: JoinGamePageProps) => {
  const connection_status = useSelector(connectionStatusSelector);

  // This dispatches a function on page load.
  /*
  const dispatch = useDispatch();
  useEffect(() => {
    dispatch(fetchScents());
  }, [dispatch]);
  */

  return (
    <div>
      <h1>Team Heist Tactics</h1>
      {connection_status != ConnectionStatus.Connected ? <JoinGameForm /> : null}
      {connection_status == ConnectionStatus.Connected ? <GameComponent /> : null}
    </div>
  );
};

export default JoinGamePage;
