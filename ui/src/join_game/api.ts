import { StagingJoinGameThing } from "./types";

import store from '../common/store';

import { MainMessage } from "../generated/types_pb";

import { connect } from '@giantmachines/redux-websocket';

export function joinGame(scent: StagingJoinGameThing) {
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
      serverUrl.searchParams.set("name", scent.name);
      serverUrl.searchParams.set("handle", scent.handle);
      store.dispatch(connect(serverUrl.toString()));
    } catch (error) {
      // TODO: dispatch failure.
      console.error("Failed to join game with websocket:", error);
    }
  }
}
