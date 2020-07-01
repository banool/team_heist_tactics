import React, { useState, useEffect } from "react";
import { useSelector } from "react-redux";

import JoinGameForm from "./JoinGameForm";
import GameWindowComponent from "./GameWindowComponent";
import { useDispatch } from "react-redux";
import {
  gameStateSelector,
  heisterSelectedSelector,
  playerIsSpectatorSelector,
} from "./slice";

import { connectionStatusSelector } from "./slice";
import { ConnectionStatus } from "./types";

import { handleKeyInput } from "./api";
import styles from "../components/styles";
import InvalidMessagesComponent from "./InvalidMessagesComponent";
import ConnectionStatusComponent from "./ConnectionStatusComponent";
import { GameStatusMap, GameStatus } from "../generated/types_pb";
import LobbyForm from "./LobbyForm";

type MainGameProps = {};
const MainGame = ({}: MainGameProps) => {
  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);
  const heister_selected_keyboard = useSelector(heisterSelectedSelector);
  const connection_status = useSelector(connectionStatusSelector);
  const player_is_spectator = useSelector(playerIsSpectatorSelector);

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
    if (player_is_spectator) {
      return;
    }
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

  var inner;
  if (
    connection_status == ConnectionStatus.Connected &&
    game_state &&
    game_state.getGameStatus() === GameStatus.ONGOING
  ) {
    inner = <GameWindowComponent />;
  } else {
    var inner_form;
    if (connection_status == ConnectionStatus.Connecting) {
      <p>Joining game...</p>;
    } else if (connection_status == ConnectionStatus.NotConnected) {
      inner_form = <JoinGameForm />;
    } else if (
      game_state &&
      game_state.getGameStatus() === GameStatus.STAGING
    ) {
      inner_form = <LobbyForm />;
    }
    inner = (
      <div style={styles.joinGameForm}>
        <h1 style={{ fontSize: 52, fontFamily: "'Damion', cursive" }}>
          Team Heist Tactics
        </h1>
        {inner_form}
        <InvalidMessagesComponent />
      </div>
    );
  }

  return (
    <div>
      {inner}
      <div style={styles.connectionStatusOverlay}>
        <ConnectionStatusComponent />
      </div>
    </div>
  );
};

// <GameWindowComponent />
// TODO Get rid of GameWindowComponent from here.

export default MainGame;
