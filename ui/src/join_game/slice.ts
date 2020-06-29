import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import { RootState } from "../common/reducers";

import { ConnectionStatus } from "./types";

import { GameState, MainMessage } from "../generated/types_pb";

import {
  WEBSOCKET_ACTION_PREFIX_FULL,
  MAX_PLAYER_MESSAGES,
} from "../constants/other";

import {
  WEBSOCKET_BROKEN,
  WEBSOCKET_CLOSED,
  WEBSOCKET_CONNECT,
  WEBSOCKET_DISCONNECT,
  WEBSOCKET_ERROR,
  WEBSOCKET_MESSAGE,
  WEBSOCKET_OPEN,
  WEBSOCKET_SEND,
} from "@giantmachines/redux-websocket";

const WEBSOCKET_BROKEN_FULL = WEBSOCKET_ACTION_PREFIX_FULL.concat(
  WEBSOCKET_BROKEN
);
const WEBSOCKET_CLOSED_FULL = WEBSOCKET_ACTION_PREFIX_FULL.concat(
  WEBSOCKET_CLOSED
);
const WEBSOCKET_CONNECT_FULL = WEBSOCKET_ACTION_PREFIX_FULL.concat(
  WEBSOCKET_CONNECT
);
const WEBSOCKET_DISCONNECT_FULL = WEBSOCKET_ACTION_PREFIX_FULL.concat(
  WEBSOCKET_DISCONNECT
);
const WEBSOCKET_MESSAGE_FULL = WEBSOCKET_ACTION_PREFIX_FULL.concat(
  WEBSOCKET_MESSAGE
);
const WEBSOCKET_OPEN_FULL = WEBSOCKET_ACTION_PREFIX_FULL.concat(WEBSOCKET_OPEN);
const WEBSOCKET_SEND_FULL = WEBSOCKET_ACTION_PREFIX_FULL.concat(WEBSOCKET_SEND);
const WEBSOCKET_ERROR_FULL = WEBSOCKET_ACTION_PREFIX_FULL.concat(
  WEBSOCKET_ERROR
);

interface GameInfo {
  connection_status: ConnectionStatus;
  player_name: string | null;
  game_handle: string | null;
  game_state: GameState | null;
  num_invalid_move_attempts: number;
  // HeisterColor for whichever is selected, or null if none are.
  heister_selected_keyboard: any | null;
  // A queue containing messages to display to the player.
  player_message_queue: string[];
}

const pushToPlayerMessageQueue = (queue: string[], msg: string) => {
  const date = new Date();
  let message = `[${date.toLocaleTimeString()}] ${msg}`;
  queue.push(message);
  if (queue.length > MAX_PLAYER_MESSAGES) {
    queue.shift();
  }
};

const pushInitialMessages = (queue: string[]) => {
  pushToPlayerMessageQueue(
    queue,
    "Joined game. Welcome to Team Heist Tactics!!!"
  );
  pushToPlayerMessageQueue(
    queue,
    "Made with love by Fatema, Kelly, and Daniel"
  );
  pushToPlayerMessageQueue(queue, "Good luck have fun!");
};

interface SelectKeyboardHeisterAction {
  // HeisterColor.
  heister_color: number;
}

interface RegisterPlayerNameAction {
  player_name: string;
  game_handle: string;
}

let initialState: GameInfo = {
  connection_status: ConnectionStatus.NotConnected,
  player_name: null,
  game_handle: null,
  game_state: null,
  num_invalid_move_attempts: 0,
  heister_selected_keyboard: null,
  player_message_queue: [],
};

const joinGameSlice = createSlice({
  name: "joinGame",
  initialState,
  reducers: {
    registerPlayerNameGameHandle: (
      state,
      action: PayloadAction<RegisterPlayerNameAction>
    ) => {
      const { player_name, game_handle } = action.payload;
      state.player_name = player_name;
      state.game_handle = game_handle;
    },
    selectKeyboardHeister: (
      state,
      action: PayloadAction<SelectKeyboardHeisterAction>
    ) => {
      const { heister_color } = action.payload;
      if (heister_color === state.heister_selected_keyboard) {
        state.heister_selected_keyboard = null;
      } else {
        state.heister_selected_keyboard = heister_color;
      }
    },
  },
  extraReducers: {
    [WEBSOCKET_CONNECT_FULL]: (state, _action) => {
      console.log(
        "Setting connection status to ",
        ConnectionStatus[ConnectionStatus.Connecting]
      );
      state.connection_status = ConnectionStatus.Connecting;
    },
    [WEBSOCKET_OPEN_FULL]: (state, _action) => {
      console.log(
        "Setting connection status to ",
        ConnectionStatus[ConnectionStatus.Connected]
      );
      state.connection_status = ConnectionStatus.Connected;
      pushInitialMessages(state.player_message_queue);
    },
    [WEBSOCKET_BROKEN_FULL]: (state, _action) => {
      console.log(
        "Setting connection status to ",
        ConnectionStatus[ConnectionStatus.NotConnected]
      );
      state.connection_status = ConnectionStatus.NotConnected;
      pushToPlayerMessageQueue(
        state.player_message_queue,
        "Lost connection to server!"
      );
    },
    [WEBSOCKET_CLOSED_FULL]: (state, _action) => {
      console.log(
        "Setting connection status to ",
        ConnectionStatus[ConnectionStatus.NotConnected]
      );
      state.connection_status = ConnectionStatus.NotConnected;
    },
    [WEBSOCKET_MESSAGE_FULL]: (state, action) => {
      var main_message = MainMessage.deserializeBinary(action.payload.message);
      console.debug("Received main message", main_message);
      var game_state = state.game_state;
      if (main_message.hasGameState()) {
        // Excalmation mark because we know it won't be undefined.
        game_state = main_message.getGameState()!;
        console.log("Updating game state to", game_state.toObject());
      }
      if (main_message.hasInvalidRequest()) {
        let msg = main_message.getInvalidRequest()!.getReason();
        console.log("Sent an invalid request earlier:", msg);
        state.num_invalid_move_attempts += 1;
        pushToPlayerMessageQueue(state.player_message_queue, msg);
      }
      state.game_state = game_state;
    },
    [WEBSOCKET_SEND_FULL]: (_state, _action) => {
      console.debug("Sending message over websocket");
    },
    [WEBSOCKET_ERROR_FULL]: (state, action) => {
      let msg = `Failed to join. Is the game handle valid?`;
      pushToPlayerMessageQueue(state.player_message_queue, msg);
    },
  },
});

export const {
  registerPlayerNameGameHandle,
  selectKeyboardHeister,
} = joinGameSlice.actions;

export const connectionStatusSelector = (state: RootState): ConnectionStatus =>
  state.joinGame.connection_status;
export const gameStateSelector = (state: RootState): GameState | null =>
  state.joinGame.game_state;
export const numInvalidMoveAttemptsSelector = (
  state: RootState
): number | null => state.joinGame.num_invalid_move_attempts;
export const heisterSelectedSelector = (state: RootState): any | null =>
  state.joinGame.heister_selected_keyboard;
export const playerMessageQueueSelector = (state: RootState): string[] =>
  state.joinGame.player_message_queue;
export const timerRunsOutSelector = (state: RootState): number =>
  state.joinGame.game_state!.getTimerRunsOut();
export const playerNameSelector = (state: RootState): string | null =>
  state.joinGame.player_name;
export const gameHandleSelector = (state: RootState): string | null =>
  state.joinGame.game_handle;

export default joinGameSlice.reducer;
