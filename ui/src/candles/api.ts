import { getCandlesSuccess, getCandleSuccess } from "./slice";
import { Candle } from "./types";

export function fetchCandles() {
  return async dispatch => {
    try {
      const response = await fetch("/api/candle?recursive=true");
      const data = await response.json();

      const candles: Candle[] = data;

      dispatch(getCandlesSuccess({ candles }));
    } catch (error) {
      // TODO: dispatch failure.
      console.error("failed the thing", error);
    }
  };
}

export function fetchCandle(id: number) {
  return async dispatch => {
    try {
      const response = await fetch(`/api/candle/${id}?recursive=true`);
      const data = await response.json();

      const candle: Candle = data;

      dispatch(getCandleSuccess({ candle }));
    } catch (error) {
      // TODO: dispatch failure.
      console.error("failed the thing", error);
    }
  };
}
