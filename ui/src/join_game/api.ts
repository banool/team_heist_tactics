import { ConnectionStatus, StagingJoinGameThing } from "./types";
import {
  GameState,
  Heister,
  HeisterColor,
  MainMessage,
  MapPosition,
  Move,
  PlaceTile,
  StartGame,
} from "../generated/types_pb";
import { connect, send } from "@giantmachines/redux-websocket";
import { registerPlayerNameGameHandle, selectKeyboardHeister } from "./slice";

import { MoveDirection } from "./types";

export function joinGame(join_game_thing: StagingJoinGameThing) {
  return async (dispatch) => {
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
    dispatch(
      registerPlayerNameGameHandle({
        player_name: join_game_thing.name,
        game_handle: join_game_thing.handle,
      })
    );
    dispatch(connect(serverUrl.toString()));
    console.log("Dispatched action to join game");
  };
}

export function startGame(game_handle: string) {
  return async (dispatch) => {
    var start_game = new StartGame();
    console.log("Dispatching action to start game");
    var main_message = new MainMessage();
    main_message.setStartGame(start_game);
    dispatch(send(main_message));
    console.log("Dispatched action to start game");
  };
}

export function useEscalator(
  game_state: GameState,
  connection_status: ConnectionStatus,
  heister_selected_keyboard: number // HeisterColor
) {
  return async (dispatch) => {
    if (
      game_state === null ||
      connection_status !== ConnectionStatus.Connected
    ) {
      console.error("Tried to move heister with no game state / connection");
      return;
    }
    var heisters = game_state.getHeistersList();
    var heister = heisters.find(
      (h) => h.getHeisterColor() == heister_selected_keyboard
    );
    if (heister === undefined) {
      console.error("Could not find information for heister");
      return;
    }
    var current_position = heister.getMapPosition()!;
    if (current_position === undefined) {
      console.error("Tried to move heister with no position");
      return;
    }

    // prep checks done - now let's get the dest position, then send the move!
    let map = game_state.getPossibleEscalatorsMap();
    var esc_dest = map.get(heister_selected_keyboard);
    if (esc_dest === undefined) {
      console.log(
        `No possible escalator dest for heister ${heister_selected_keyboard}`
      );
      return;
    }
    console.log(
      `Heister ${heister_selected_keyboard} taking escalator to ${esc_dest}`
    );
    dispatch(moveHeisterReal(heister, esc_dest));
  };
}

export function moveHeister(
  game_state: GameState,
  connection_status: ConnectionStatus,
  heister_selected_keyboard: number, // HeisterColor
  move_direction: MoveDirection
) {
  return async (dispatch) => {
    if (
      game_state === null ||
      connection_status !== ConnectionStatus.Connected
    ) {
      console.error("Tried to move heister with no game state / connection");
      return;
    }
    var heisters = game_state.getHeistersList();
    var heister = heisters.find(
      (h) => h.getHeisterColor() == heister_selected_keyboard
    );
    if (heister === undefined) {
      console.error("Could not find information for heister");
      return;
    }
    var current_position = heister.getMapPosition();
    if (current_position === undefined) {
      console.error("Tried to move heister with no position");
      return;
    }
    var new_position = new MapPosition();
    new_position.setX(current_position.getX());
    new_position.setY(current_position.getY());
    // 0,0 is the top left of the map, not the middle,
    // so you have to minus Y to go north.
    switch (+move_direction) {
      case MoveDirection.North:
        new_position.setY(current_position.getY() - 1);
        break;
      case MoveDirection.East:
        new_position.setX(current_position.getX() + 1);
        break;
      case MoveDirection.South:
        new_position.setY(current_position.getY() + 1);
        break;
      case MoveDirection.West:
        new_position.setX(current_position.getX() - 1);
        break;
      default:
        console.error("Unexpected move direction");
        break;
    }
    dispatch(moveHeisterReal(heister, new_position));
  };
}

export function moveHeisterReal(heister: Heister, new_position: MapPosition) {
  return async (dispatch) => {
    var current_position = heister.getMapPosition()!;
    var move = new Move();
    var heister_color = heister.getHeisterColor();
    console.log(
      `Dispatching action to move heister ${heister_color} (0 yellow, 1 purple, 2 green, 3 orange) from ${current_position.toObject()} -> ${new_position.toObject()})`
    );
    move.setHeisterColor(heister_color);
    move.setPosition(new_position);
    var main_message = new MainMessage();
    main_message.setMove(move);
    console.debug("Dispatching websocket send of Move", move);
    dispatch(send(main_message));
    console.debug("Dispatched websocket send of Move");
  };
}

export function sendChat(chat: string) {
  return async (dispatch) => {
    var main_message = new MainMessage();
    main_message.setChat(chat);
    dispatch(send(main_message));
  };
}

// Take a key input, convert to an enum representing different things
// the user wants to do, then match on that instead.
export function handleKeyInput(
  game_state: GameState | null,
  connection_status: ConnectionStatus,
  heister_selected_keyboard: number,
  key: string
) {
  return async (dispatch) => {
    var key_action = getKeyAction(key);
    // Do nothing if the key didn't match anything.
    if (key_action === null) {
      return;
    }
    if (
      game_state === null ||
      connection_status !== ConnectionStatus.Connected
    ) {
      console.debug("No game state / connection, dropping key input");
      return;
    }
    switch (key_action) {
      case KeyAction.MoveNorth:
        dispatch(
          moveHeister(
            game_state,
            connection_status,
            heister_selected_keyboard,
            MoveDirection.North
          )
        );
        return;
      case KeyAction.MoveEast:
        dispatch(
          moveHeister(
            game_state,
            connection_status,
            heister_selected_keyboard,
            MoveDirection.East
          )
        );
        return;
      case KeyAction.MoveSouth:
        dispatch(
          moveHeister(
            game_state,
            connection_status,
            heister_selected_keyboard,
            MoveDirection.South
          )
        );
        return;
      case KeyAction.Escalator:
        dispatch(
          useEscalator(game_state, connection_status, heister_selected_keyboard)
        );
        return;
      case KeyAction.MoveWest:
        dispatch(
          moveHeister(
            game_state,
            connection_status,
            heister_selected_keyboard,
            MoveDirection.West
          )
        );
        return;
      case KeyAction.SelectYellowHeister:
        dispatch(selectKeyboardHeister({ heister_color: HeisterColor.YELLOW }));
        return;
      case KeyAction.SelectPurpleHeister:
        dispatch(selectKeyboardHeister({ heister_color: HeisterColor.PURPLE }));
        return;
      case KeyAction.SelectGreenHeister:
        dispatch(selectKeyboardHeister({ heister_color: HeisterColor.GREEN }));
        return;
      case KeyAction.SelectOrangeHeister:
        dispatch(selectKeyboardHeister({ heister_color: HeisterColor.ORANGE }));
        return;
      default:
        return null; // Raise error.
    }
  };
}

export function getKeyAction(key: string) {
  switch (key) {
    case "w":
    case "ArrowUp":
      return KeyAction.MoveNorth;
    case "d":
    case "ArrowRight":
      return KeyAction.MoveEast;
    case "s":
    case "ArrowDown":
      return KeyAction.MoveSouth;
    case "a":
    case "ArrowLeft":
      return KeyAction.MoveWest;
    case "1":
      return KeyAction.SelectYellowHeister;
    case "2":
      return KeyAction.SelectPurpleHeister;
    case "3":
      return KeyAction.SelectGreenHeister;
    case "4":
      return KeyAction.SelectOrangeHeister;
    case "e":
      return KeyAction.Escalator;
    default:
      return null;
  }
}

export enum KeyAction {
  MoveNorth,
  MoveEast,
  MoveSouth,
  MoveWest,
  SelectYellowHeister,
  SelectPurpleHeister,
  SelectGreenHeister,
  SelectOrangeHeister,
  Escalator,
}

export const getColor = (heister_color): string => {
  switch (+heister_color) {
    case HeisterColor.YELLOW:
      return "#f0d249";
    case HeisterColor.PURPLE:
      return "#cb97ef";
    case HeisterColor.GREEN:
      return "#81ae62";
    case HeisterColor.ORANGE:
      return "#e78234";
    default:
      console.error("Unexpected heister color");
      return "#000000";
  }
};

export function placeTile(map_position: MapPosition) {
  return async (dispatch) => {
    var place_tile = new PlaceTile();
    place_tile.setTileEntrance(map_position);
    console.log(
      `Dispatching action to place tile at ${map_position.toObject()}`
    );
    var main_message = new MainMessage();
    main_message.setPlaceTile(place_tile);
    console.debug("Dispatching websocket send of PlaceTile", place_tile);
    dispatch(send(main_message));
    console.debug("Dispatched websocket send of PlaceTile");
  };
}
