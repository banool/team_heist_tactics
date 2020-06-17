import { StagingJoinGameThing, ConnectionStatus } from "./types";

import {
  GameState,
  Move,
  MainMessage,
  HeisterColor,
  MapPosition
} from "../generated/types_pb";
import { connect, send } from "@giantmachines/redux-websocket";

import { MoveDirection } from "./types";

export function joinGame(join_game_thing: StagingJoinGameThing) {
  return async dispatch => {
    try {
      var scheme = "ws";
      // If this is an HTTPS connection, we have to use a secure WebSocket
      // connection too, so add another "s" to the scheme.
      if (document.location.protocol === "https:") {
        scheme += "s";
      }

      var urlString =
        scheme +
        "://" +
        document.location.hostname +
        ":" +
        document.location.port +
        "/play_game";

      var serverUrl = new URL(urlString);
      serverUrl.searchParams.set("name", join_game_thing.name);
      serverUrl.searchParams.set("handle", join_game_thing.handle);
      dispatch(connect(serverUrl.toString()));
      console.log("Dispatched action to join game");
    } catch (error) {
      // TODO: dispatch failure.
      console.error("Failed to join game with websocket:", error);
    }
  };
}

export function moveHeister(
  game_state: GameState,
  connection_status: ConnectionStatus,
  move_direction: MoveDirection
) {
  return async dispatch => {
    var hardcoded_color = HeisterColor.GREEN;
    if (
      game_state === null ||
      connection_status !== ConnectionStatus.Connected
    ) {
      console.error("Tried to move heister with no game state / connection");
      return;
    }
    var heisters = game_state.getHeistersList();
    var green_heister = heisters.find(
      h => h.getHeisterColor() == hardcoded_color
    );
    if (green_heister === undefined) {
      console.error("Could not find information for heister");
      return;
    }
    var current_position = green_heister.getMapPosition();
    if (current_position === undefined) {
      console.error("Tried to move heister with no position");
      return;
    }
    var new_position = new MapPosition();
    new_position.setX(current_position.getX());
    new_position.setY(current_position.getY());
    switch (+move_direction) {
      case MoveDirection.North:
        new_position.setY(current_position.getY() + 1);
        break;
      case MoveDirection.East:
        new_position.setX(current_position.getX() + 1);
        break;
      case MoveDirection.South:
        new_position.setY(current_position.getY() - 1);
        break;
      case MoveDirection.West:
        new_position.setX(current_position.getX() - 1);
        break;
      default:
        console.error("Unexpected move direction");
        break;
    }
    var move = new Move();
    console.log(
      "Dispatching action to move GREEN heister (a->b)",
      current_position.toObject(),
      new_position.toObject()
    );
    move.setHeisterColor(hardcoded_color);
    move.setPosition(new_position);
    var main_message = new MainMessage();
    main_message.setMove(move);
    console.log("Dispatching websocket send of Move", move);
    dispatch(send(main_message));
    console.log("Dispatched websocket send of Move");
  };
}

// Take a key input, convert to an enum representing different things
// the user wants to do, then match on that instead.
export function handleKeyInput(
  game_state: GameState | null,
  connection_status: ConnectionStatus,
  key: string
) {
  return async dispatch => {
    var move = getMove(key);
    // Do nothing if the key didn't match anything.
    if (move === null) {
      return;
    }
    if (
      game_state === null ||
      connection_status !== ConnectionStatus.Connected
    ) {
      console.debug("No game state / connection, dropping key input");
      return;
    }
    console.log("Sending move", move);
    switch (move) {
      case MyMove.MoveNorth:
        dispatch(
          moveHeister(game_state, connection_status, MoveDirection.North)
        );
        return;
      case MyMove.MoveEast:
        dispatch(
          moveHeister(game_state, connection_status, MoveDirection.East)
        );
        return;
      case MyMove.MoveSouth:
        dispatch(
          moveHeister(game_state, connection_status, MoveDirection.South)
        );
        return;
      case MyMove.MoveWest:
        dispatch(
          moveHeister(game_state, connection_status, MoveDirection.West)
        );
        return;
      default:
        return null; // Raise error.
    }
  };
}
export function getMove(key: string) {
  switch (key) {
    case "w":
      return MyMove.MoveNorth;
    case "d":
      return MyMove.MoveEast;
    case "s":
      return MyMove.MoveSouth;
    case "a":
      return MyMove.MoveWest;
    default:
      return null;
  }
}

export enum MyMove {
  MoveNorth,
  MoveEast,
  MoveSouth,
  MoveWest
}
