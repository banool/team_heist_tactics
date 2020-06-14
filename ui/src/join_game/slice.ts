import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import { RootState } from "../common/reducers";

import { JoinGameThing, StagingJoinGameThing, ConnectionStatus } from "./types";

import { GameState } from "../generated/types_pb";

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
        // TODO
        return state;

      case WEBSOCKET_SEND:
        // TODO
        return state;

      default:
        console.log("Default websocket action statement");
        return state;
    }
  } else {
    console.log("Got non-websocket action");
    return state;
  }
};

export const {  } = joinGameSlice.actions;
export const connectionStatusSelector = (state: RootState): ConnectionStatus => state.joinGame.connection_status;

//export default joinGameSlice.reducer;
export default reducer;
