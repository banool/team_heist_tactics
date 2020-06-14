import { StagingJoinGameThing } from "./types";

import { Move, MainMessage } from "../generated/types_pb";
import { connect, send } from '@giantmachines/redux-websocket';

export function joinGame(join_game_thing: StagingJoinGameThing) {
  return async dispatch => {
    try {
      var scheme = "ws";
      // If this is an HTTPS connection, we have to use a secure WebSocket
      // connection too, so add another "s" to the scheme.
      if (document.location.protocol === "https:") {
        scheme += "s";
      }

      var urlString = scheme + "://" + document.location.hostname + ":" + document.location.port + "/play_game";

      var serverUrl = new URL(urlString);
      serverUrl.searchParams.set("name", join_game_thing.name);
      serverUrl.searchParams.set("handle", join_game_thing.handle);
      dispatch(connect(serverUrl.toString()));
      console.log("Dispatched action to join game");
    } catch (error) {
      // TODO: dispatch failure.
      console.error("Failed to join game with websocket:", error);
    }
  }
}

export function moveHeister(move: Move) {
  return async dispatch => {
    var main_message = new MainMessage();
    main_message.setMove(move);
    dispatch(send(main_message));
    console.log("Dispatched websocket send of Move");
  }
}
