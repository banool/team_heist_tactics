export interface JoinGameThing {
  name: string;
  handle: string;
}

export interface StagingJoinGameThing {
  name: string;
  handle: string;
}

export enum ConnectionStatus {
  NotConnected,
  Connecting,
  Connected,
  // TODO Do the reconnecting states from here:
  // https://github.com/giantmachines/redux-websocket
}

export enum MoveDirection {
  North,
  East,
  South,
  West,
}

export interface CanvasPosition {
  x: number;
  y: number;
}

export interface SquareCoords {
  x: number;
  y: number;
}

export interface TileCoords {
  x: number;
  y: number;
}
