import { combineReducers } from "redux";
// TODO I'm keeping this for a ref for how to get data.
import joinGameReducer from "../join_game/slice";

const rootReducer = combineReducers({
  joinGame: joinGameReducer,
});

export type RootState = ReturnType<typeof rootReducer>;

export default rootReducer;
