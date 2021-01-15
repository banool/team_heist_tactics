import {
  GameState,
  GameStatus,
  GameStatusMap,
  MainMessage,
} from "../generated/types_pb";
import {
  MAX_PLAYER_MESSAGES,
  WEBSOCKET_ACTION_PREFIX_FULL,
} from "../constants/other";
import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import {
  WEBSOCKET_BROKEN,
  WEBSOCKET_CLOSED,
  WEBSOCKET_CONNECT,
  WEBSOCKET_ERROR,
  WEBSOCKET_MESSAGE,
  WEBSOCKET_OPEN,
  WEBSOCKET_SEND,
} from "@giantmachines/redux-websocket";

import { ConnectionStatus } from "./types";
import { RootState } from "../common/reducers";

const WEBSOCKET_BROKEN_FULL =
  WEBSOCKET_ACTION_PREFIX_FULL.concat(WEBSOCKET_BROKEN);
const WEBSOCKET_CLOSED_FULL =
  WEBSOCKET_ACTION_PREFIX_FULL.concat(WEBSOCKET_CLOSED);
const WEBSOCKET_CONNECT_FULL =
  WEBSOCKET_ACTION_PREFIX_FULL.concat(WEBSOCKET_CONNECT);
const WEBSOCKET_MESSAGE_FULL =
  WEBSOCKET_ACTION_PREFIX_FULL.concat(WEBSOCKET_MESSAGE);
const WEBSOCKET_OPEN_FULL = WEBSOCKET_ACTION_PREFIX_FULL.concat(WEBSOCKET_OPEN);
const WEBSOCKET_SEND_FULL = WEBSOCKET_ACTION_PREFIX_FULL.concat(WEBSOCKET_SEND);
const WEBSOCKET_ERROR_FULL =
  WEBSOCKET_ACTION_PREFIX_FULL.concat(WEBSOCKET_ERROR);

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
  player_is_spectator: boolean;
  chat_box_active: boolean;
}

const pushToPlayerMessageQueue = (queue: string[], msg: string) => {
  const date = new Date();
  let message = `[${date.toLocaleTimeString()}] ${msg}`;
  queue.push(message);
  while (queue.length > MAX_PLAYER_MESSAGES) {
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
  pushToPlayerMessageQueue(queue, "Special thanks to Dmitry and Max!");
};

interface SelectKeyboardHeisterAction {
  // HeisterColor.
  heister_color: number;
}

interface RegisterPlayerNameAction {
  player_name: string;
  game_handle: string;
}

interface SetChatBoxFocusAction {
  focused: boolean;
}

let initialState: GameInfo = {
  connection_status: ConnectionStatus.NotConnected,
  player_name: null,
  game_handle: null,
  game_state: null,
  num_invalid_move_attempts: 0,
  heister_selected_keyboard: null,
  player_message_queue: [],
  player_is_spectator: true,
  chat_box_active: false,
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
    setChatBoxFocus: (state, action: PayloadAction<SetChatBoxFocusAction>) => {
      const { focused } = action.payload;
      if (!state.game_state!.getPlayersMaySpeak()) {
        if (focused) {
          pushToPlayerMessageQueue(
            state.player_message_queue,
            "You cannot speak right now!"
          );
        }
        state.chat_box_active = false;
      } else {
        state.chat_box_active = focused;
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
      if (main_message.hasChat()) {
        var msg = main_message.getChat()!;
        if (msg.includes("tap ")) {
          let sender = msg.split(": ")[0];
          if (sender === state.player_name) {
            sender = "You";
          }
          var recipient = msg.split("tap ").slice(-1)[0];
          if (recipient === state.player_name) {
            recipient = "you";
          }
          msg = `${sender} tapped at ${recipient}!`;
          window["tap_audio_object"].play();
        }
        pushToPlayerMessageQueue(state.player_message_queue, msg);
      }
      state.game_state = game_state;
      if (state.game_state) {
        let players = state.game_state.getPlayersList().map((p) => p.getName());
        state.player_is_spectator = !players.includes(state.player_name!);
      }
      console.log("HEY FRIEND", JSON.stringify(state.player_message_queue));
    },
    [WEBSOCKET_SEND_FULL]: (_state, _action) => {
      console.debug("Sending message over websocket");
    },
    [WEBSOCKET_ERROR_FULL]: (state, _action) => {
      let msg = `Failed to join. Is the game handle valid?`;
      pushToPlayerMessageQueue(state.player_message_queue, msg);
    },
  },
});

export const {
  registerPlayerNameGameHandle,
  selectKeyboardHeister,
  setChatBoxFocus,
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
export const playerIsSpectatorSelector = (state: RootState): boolean =>
  state.joinGame.player_is_spectator;
export const chatBoxActiveSelector = (state: RootState): boolean =>
  state.joinGame.chat_box_active;
export const gameStatusSelector = (
  state: RootState
): GameStatusMap[keyof GameStatusMap] =>
  state.joinGame.game_state!.getGameStatus();
export const playersMaySpeakSelector = (state: RootState): boolean =>
  state.joinGame.game_state!.getPlayersMaySpeak();

export default joinGameSlice.reducer;
