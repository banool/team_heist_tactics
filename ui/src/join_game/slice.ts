import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import { RootState } from "../common/reducers";

import { JoinGameThing, StagingJoinGameThing, ConnectionStatus } from "./types";

import { GameState, MainMessage } from "../generated/types_pb";

import { WEBSOCKET_ACTION_PREFIX, WEBSOCKET_ACTION_PREFIX_FULL } from "../constants/other";

import {
  WEBSOCKET_BROKEN,
  WEBSOCKET_CLOSED,
  WEBSOCKET_CONNECT,
  WEBSOCKET_DISCONNECT,
  WEBSOCKET_MESSAGE,
  WEBSOCKET_OPEN,
  WEBSOCKET_SEND,
} from '@giantmachines/redux-websocket';

interface GameInfo {
  connection_status: ConnectionStatus,
  game_state: GameState | null,
}

interface GetCandlesSuccessAction {
  candles: JoinGameThing[];
}

interface GetCandleSuccessAction {
  candle: JoinGameThing;
}

let initialState: GameInfo = {
  connection_status: ConnectionStatus.NotConnected,
  game_state: null,
};

// Define things that listen to actions.
const joinGameSlice = createSlice({
  name: "joinGame",
  initialState,
  /*
  reducers: {
    websocketOpen: (state, action: PayloadAction<WEBSOCKET_OPEN>) => {
      const { candles } = action.payload;
      state.candles = {};
      candles.forEach(candle => (state.candles[candle.id] = candle));
    },
    getCandleSuccess: (state, action: PayloadAction<GetCandleSuccessAction>) => {
      const { candle } = action.payload;
      state.candles[candle.id] = candle;
    }
  },*/
  // TODO This should be extraReducers?
  reducers: {}
});

const reducer = (state = initialState, action) => {
  console.log("Previous state and action:", state, action);
  var action_type = action.type;
  if (action_type.startsWith(WEBSOCKET_ACTION_PREFIX)) {
    action_type = action_type.replace(WEBSOCKET_ACTION_PREFIX_FULL, "");
    console.log("Websocket action type suffix:", action_type);
    switch (action_type) {
      case WEBSOCKET_CONNECT:
        console.log("Setting connection status to ", ConnectionStatus[ConnectionStatus.Connecting]);
        return {
          ...state,
          connection_status: ConnectionStatus.Connecting,
        };

      case WEBSOCKET_OPEN:
        console.log("Setting connection status to ", ConnectionStatus[ConnectionStatus.Connected]);
        return {
          ...state,
          connection_status: ConnectionStatus.Connected,
        };

      case WEBSOCKET_BROKEN:
      case WEBSOCKET_CLOSED:
        console.log("Setting connection status to ", ConnectionStatus[ConnectionStatus.NotConnected]);
        return {
          ...state,
          connection_status: ConnectionStatus.NotConnected,
        };

      case WEBSOCKET_MESSAGE:
        console.debug("websocket_message action.payload.message:", action.payload.message);
        var main_message = MainMessage.deserializeBinary(action.payload.message);
        console.debug("Received main message", main_message);
        var game_state = state.game_state;
        if (main_message.hasGameState()) {
          // Excalmation mark because we know it won't be undefined.
          game_state = main_message.getGameState()!;
          console.log("Updating game state to", game_state.toObject());
        }
        if (main_message.hasInvalidRequest()) {
          console.log("Sent an invalid request earlier:", main_message.getInvalidRequest()!);
        }
        return {
          ...state,
          game_state: game_state,
        };

      case WEBSOCKET_SEND:
        // The framework handles sending the message, this just lets us do something
        // when it happens.
        console.debug("Sending message over websocket");
        break;

      default:
        console.warn("Default websocket action statement", action);
        return state;
    }
  } else {
    console.log("Got non-websocket action");
    return state;
  }
};

export const {  } = joinGameSlice.actions;
export const connectionStatusSelector = (state: RootState): ConnectionStatus => state.joinGame.connection_status;
export const gameStateSelector = (state: RootState): GameState | null => state.joinGame.game_state;

//export default joinGameSlice.reducer;
export default reducer;
