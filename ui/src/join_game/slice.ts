import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import { RootState } from "../common/reducers";

import { Candle } from "./types";

interface CandlesDict {
  [id: number]: Candle;
}

interface CandlesSliceState {
  candles: CandlesDict;
}

interface GetCandlesSuccessAction {
  candles: Candle[];
}

interface GetCandleSuccessAction {
  candle: Candle;
}

let initialState: CandlesSliceState = {
  candles: {}
};

const joinGameSlice = createSlice({
  name: "joinGame",
  initialState,
  reducers: {
    getCandlesSuccess: (state, action: PayloadAction<GetCandlesSuccessAction>) => {
      const { candles } = action.payload;
      state.candles = {};
      candles.forEach(candle => (state.candles[candle.id] = candle));
    },
    getCandleSuccess: (state, action: PayloadAction<GetCandleSuccessAction>) => {
      const { candle } = action.payload;
      state.candles[candle.id] = candle;
    }
  }
});

export const { getCandlesSuccess, getCandleSuccess } = joinGameSlice.actions;
export const candlesSelector = (state: RootState): Candle[] => Object.values(state.joinGame.candles);
export const candleSelector = (id: number) => (state: RootState): Candle => state.joinGame.candles[id];

export default joinGameSlice.reducer;
