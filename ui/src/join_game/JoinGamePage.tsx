import React, { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Link } from "react-router-dom";

import { JoinGameThing } from "./types";
import JoinGameForm from "./JoinGameForm";


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
      <h3>Let's goooooo!!!!!!!!!!!!!!!!!!!!!!!!!!</h3>
      <p>Join Game:</p>
      <JoinGameForm />
    </div>
  );
};

export default JoinGamePage;
