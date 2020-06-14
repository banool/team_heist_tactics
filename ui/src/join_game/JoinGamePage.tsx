import React, { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Link } from "react-router-dom";

import JoinGameForm from "./JoinGameForm";
import ConnectionStatusComponent from "./ConnectionStatusComponent";


type JoinGamePageProps = {};
const JoinGamePage = ({ }: JoinGamePageProps) => {

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
      <JoinGameForm />
      <ConnectionStatusComponent />
    </div>
  );
};

export default JoinGamePage;
