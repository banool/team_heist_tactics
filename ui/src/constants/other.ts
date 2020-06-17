export const WEBSOCKET_ACTION_PREFIX = "REDUX_WEBSOCKET";
export const WEBSOCKET_ACTION_PREFIX_FULL = "REDUX_WEBSOCKET::";

export const CANVAS_WIDTH = 1600;
export const CANVAS_HEIGHT = 1000;
export const SERVER_WIDTH = 500;
export const SERVER_HEIGHT = 500;

export const TILE_SIZE = 300;
export const SQUARE_SIZE = TILE_SIZE / 4;

export const NUM_MAP_UNITS_WIDE = CANVAS_WIDTH / SQUARE_SIZE;
export const NUM_MAP_UNITS_HIGH = CANVAS_HEIGHT / SQUARE_SIZE;

// Make the heister the size of a square / 2 minus a bit.
export const HEISTER_SIZE = SQUARE_SIZE / 2 - 15;
