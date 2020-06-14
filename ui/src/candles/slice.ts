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

const candlesSlice = createSlice({
  name: "candles",
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

export const { getCandlesSuccess, getCandleSuccess } = candlesSlice.actions;
export const candlesSelector = (state: RootState): Candle[] => Object.values(state.candles.candles);
export const candleSelector = (id: number) => (state: RootState): Candle => state.candles.candles[id];

export default candlesSlice.reducer;
