import React, { useState, useEffect } from "react";
import { useSelector } from "react-redux";

import JoinGameForm from "./JoinGameForm";
import ConnectionStatusComponent from "./ConnectionStatusComponent";
import GameComponent from "./GameComponent";
import { useDispatch } from "react-redux";
import { gameStateSelector, heisterSelectedSelector } from "./slice";

import { connectionStatusSelector } from "./slice";
import { ConnectionStatus } from "./types";

import { handleKeyInput } from "./api";
import GameWindowComponent from "./GameWindowComponent";
import styles from "../components/styles";

type MainGameProps = {};
const MainGame = ({}: MainGameProps) => {
  const connection_status = useSelector(connectionStatusSelector);

  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);

  const heister_selected_keyboard = useSelector(heisterSelectedSelector);

  // Bind key listener on mount, and unbind on unmount.
  // See https://reactjs.org/docs/hooks-effect.html.
  // This still does key repeat, but it doesn't bind the event
  // listener more than once so we don't get duplicate key events.
  // So the keys behave as if you were typing into a text area;
  // key repeat kicks in after a user defined OS level delay.
  // https://stackoverflow.com/questions/41693715/react-redux-what-is-the-canonical-way-to-bind-a-keypress-action-to-kick-off-a-r
  useEffect(() => {
    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("keyup", handleKeyUp);
    return function cleanup() {
      document.removeEventListener("keydown", handleKeyDown);
      document.removeEventListener("keyup", handleKeyUp);
    };
  });

  const handleKeyDown = (event) => {
    console.debug("Key event", event);
    dispatch(
      handleKeyInput(
        game_state,
        connection_status,
        heister_selected_keyboard,
        event.key
      )
    );
    document.removeEventListener("keydown", handleKeyDown);
  };

  const handleKeyUp = (event) => {
    document.addEventListener("keydown", handleKeyDown, { once: true });
  };

  // <ConnectionStatusComponent />

  return (
    <div>
      {connection_status != ConnectionStatus.Connected ? (
        <div style={styles.joinGameForm}>
          <h1 style={{ fontSize: 52, fontFamily: "'Damion', cursive" }}>Team Heist Tactics</h1>
          <JoinGameForm />
        </div>
      ) : null}
      {connection_status == ConnectionStatus.Connected && game_state ? (
        <GameComponent />
      ) : null}
    </div>
  );
};

// <GameWindowComponent />
// TODO Get rid of GameWindowComponent from here.

export default MainGame;
