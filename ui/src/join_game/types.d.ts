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
