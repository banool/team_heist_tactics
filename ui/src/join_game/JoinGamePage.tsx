import React, { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Link } from "react-router-dom";

import { JoinGameThing } from "./types";
import { JoinGameRoute, pathFor } from "../common/routes";
import JoinGameForm from "./JoinGameForm";


type JoinGamePageProps = {};
const JoinGamePage = ({ }: JoinGamePageProps) => {

  // This does something on page load.
  /*
  const dispatch = useDispatch();
  useEffect(() => {
    dispatch(fetchScents());
  }, [dispatch]);
  */

  return (
    <div>
      <h3>Join Game</h3>
      <JoinGameForm />
    </div>
  );
};

export default JoinGamePage;
