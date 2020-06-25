export const WEBSOCKET_ACTION_PREFIX = "REDUX_WEBSOCKET";
export const WEBSOCKET_ACTION_PREFIX_FULL = "REDUX_WEBSOCKET::";

export const SERVER_WIDTH = 500;
export const SERVER_HEIGHT = 500;

export const REAL_TILE_SIZE_PX = 1600;
export const REAL_TILE_WALL_SIZE = 150;
export const REAL_TILE_SQUARE_SIZE = 325;

export const TILE_SIZE = 300;
export const MAP_SQUARE_SIZE = TILE_SIZE * 4;
// The square inside the tile is actaully smaller than 1/4 of the tile.
export const INTERNAL_TILE_OFFSET = (REAL_TILE_WALL_SIZE / 1600) * TILE_SIZE;
export const INTERNAL_SQUARE_SIZE = (REAL_TILE_SQUARE_SIZE / 1600) * TILE_SIZE;

// Make the heister the size of a square / 2 minus a bit.
export const HEISTER_SIZE = INTERNAL_SQUARE_SIZE / 2 - 10;
