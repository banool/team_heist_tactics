import { getDefaultMiddleware, configureStore } from "@reduxjs/toolkit";
import reduxWebsocket from "@giantmachines/redux-websocket";
import rootReducer from "./reducers";
import { WEBSOCKET_ACTION_PREFIX } from "../constants/other";

import * as jspb from "google-protobuf";

// This means we can only send jspb Messages.
const customSerializer = (payload: jspb.Message) => payload.serializeBinary();

// Create the middleware instance.
const reduxWebsocketMiddleware = reduxWebsocket({
  serializer: customSerializer,
  prefix: WEBSOCKET_ACTION_PREFIX,
  reconnectOnClose: true,
  reconnectInterval: 1000,
  // Modify the websocket so it returns arraybuffers instead of blobs.
  onOpen: (socket: WebSocket) => (socket.binaryType = "arraybuffer"),
});
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
