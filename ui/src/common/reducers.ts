import { combineReducers } from "redux";

// TODO I'm keeping this for a ref for how to get data.
import candlesReducer from "../candles/slice";

const rootReducer = combineReducers({
  candles: candlesReducer,
});

export type RootState = ReturnType<typeof rootReducer>;

export default rootReducer;
