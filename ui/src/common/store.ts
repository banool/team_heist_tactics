import { getDefaultMiddleware, configureStore } from "@reduxjs/toolkit";
import reduxWebsocket from '@giantmachines/redux-websocket';
import rootReducer from "./reducers";

import * as jspb from "google-protobuf";

// This means we can only send jspb Messages.
const customSerializer = (payload: jspb.Message) => payload.serializeBinary();

// Create the middleware instance.
const reduxWebsocketMiddleware = reduxWebsocket({ serializer: customSerializer });
const middleware = getDefaultMiddleware().concat(reduxWebsocketMiddleware);

// Create the Redux store.
const store = configureStore({
  reducer: rootReducer,
  middleware: middleware,
});

if (process.env.NODE_ENV === "development" && module.hot) {
  module.hot.accept("./reducers.ts", () => {
    const newRootReducer = require("./reducers.ts").default;
    store.replaceReducer(newRootReducer);
  });
}

export default store;
