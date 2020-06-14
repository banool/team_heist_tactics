import { StagingJoinGameThing } from "./types";

import { MainMessage } from "./proto_types.js";


export function joinGame(scent: StagingJoinGameThing) {
  return async dispatch => {
    try {
      // TODO Make websocket
      // Perhaps dispatch function call on root that sets websocket on root
      const response = await fetch("/api/scent", {
        method: "POST",
        headers: {
          "Content-Type": "application/json;charset=utf-8"
        },
        body: JSON.stringify(scent)
      });

      const data = await response.json();
      console.log("CreateScent response", data);
    } catch (error) {
      // TODO: dispatch failure.
      console.error("failed the thing");
    }
  }
}
