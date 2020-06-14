// package: types
// file: types.proto

import * as jspb from "google-protobuf";

export class TilePosition extends jspb.Message {
  getX(): number;
  setX(value: number): void;

  getY(): number;
  setY(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TilePosition.AsObject;
  static toObject(includeInstance: boolean, msg: TilePosition): TilePosition.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: TilePosition, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TilePosition;
  static deserializeBinaryFromReader(message: TilePosition, reader: jspb.BinaryReader): TilePosition;
}

export namespace TilePosition {
  export type AsObject = {
    x: number,
    y: number,
  }
}

export class MapPosition extends jspb.Message {
  getX(): number;
  setX(value: number): void;

  getY(): number;
  setY(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MapPosition.AsObject;
  static toObject(includeInstance: boolean, msg: MapPosition): MapPosition.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: MapPosition, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MapPosition;
  static deserializeBinaryFromReader(message: MapPosition, reader: jspb.BinaryReader): MapPosition;
}

export namespace MapPosition {
  export type AsObject = {
    x: number,
    y: number,
  }
}

export class Tile extends jspb.Message {
  clearSquaresList(): void;
  getSquaresList(): Array<Square>;
  setSquaresList(value: Array<Square>): void;
  addSquares(value?: Square, index?: number): Square;

  hasPosition(): boolean;
  clearPosition(): void;
  getPosition(): MapPosition | undefined;
  setPosition(value?: MapPosition): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Tile.AsObject;
  static toObject(includeInstance: boolean, msg: Tile): Tile.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Tile, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Tile;
  static deserializeBinaryFromReader(message: Tile, reader: jspb.BinaryReader): Tile;
}

export namespace Tile {
  export type AsObject = {
    squaresList: Array<Square.AsObject>,
    position?: MapPosition.AsObject,
  }
}

export class Square extends jspb.Message {
  getNorthWall(): WallTypeMap[keyof WallTypeMap];
  setNorthWall(value: WallTypeMap[keyof WallTypeMap]): void;

  getEastWall(): WallTypeMap[keyof WallTypeMap];
  setEastWall(value: WallTypeMap[keyof WallTypeMap]): void;

  getSouthWall(): WallTypeMap[keyof WallTypeMap];
  setSouthWall(value: WallTypeMap[keyof WallTypeMap]): void;

  getWestWall(): WallTypeMap[keyof WallTypeMap];
  setWestWall(value: WallTypeMap[keyof WallTypeMap]): void;

  getSquareType(): SquareTypeMap[keyof SquareTypeMap];
  setSquareType(value: SquareTypeMap[keyof SquareTypeMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Square.AsObject;
  static toObject(includeInstance: boolean, msg: Square): Square.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Square, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Square;
  static deserializeBinaryFromReader(message: Square, reader: jspb.BinaryReader): Square;
}

export namespace Square {
  export type AsObject = {
    northWall: WallTypeMap[keyof WallTypeMap],
    eastWall: WallTypeMap[keyof WallTypeMap],
    southWall: WallTypeMap[keyof WallTypeMap],
    westWall: WallTypeMap[keyof WallTypeMap],
    squareType: SquareTypeMap[keyof SquareTypeMap],
  }
}

export class Heister extends jspb.Message {
  getHeisterColor(): HeisterColorMap[keyof HeisterColorMap];
  setHeisterColor(value: HeisterColorMap[keyof HeisterColorMap]): void;

  hasMapPosition(): boolean;
  clearMapPosition(): void;
  getMapPosition(): MapPosition | undefined;
  setMapPosition(value?: MapPosition): void;

  getHasTakenItem(): boolean;
  setHasTakenItem(value: boolean): void;

  getHasEscaped(): boolean;
  setHasEscaped(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Heister.AsObject;
  static toObject(includeInstance: boolean, msg: Heister): Heister.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Heister, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Heister;
  static deserializeBinaryFromReader(message: Heister, reader: jspb.BinaryReader): Heister;
}

export namespace Heister {
  export type AsObject = {
    heisterColor: HeisterColorMap[keyof HeisterColorMap],
    mapPosition?: MapPosition.AsObject,
    hasTakenItem: boolean,
    hasEscaped: boolean,
  }
}

export class Player extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  clearAbilitiesList(): void;
  getAbilitiesList(): Array<AbilityMap[keyof AbilityMap]>;
  setAbilitiesList(value: Array<AbilityMap[keyof AbilityMap]>): void;
  addAbilities(value: AbilityMap[keyof AbilityMap], index?: number): AbilityMap[keyof AbilityMap];

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Player.AsObject;
  static toObject(includeInstance: boolean, msg: Player): Player.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Player, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Player;
  static deserializeBinaryFromReader(message: Player, reader: jspb.BinaryReader): Player;
}

export namespace Player {
  export type AsObject = {
    name: string,
    abilitiesList: Array<AbilityMap[keyof AbilityMap]>,
  }
}

export class GameState extends jspb.Message {
  getGameName(): string;
  setGameName(value: string): void;

  getGameStarted(): number;
  setGameStarted(value: number): void;

  getTimerRunsOut(): number;
  setTimerRunsOut(value: number): void;

  clearTilesList(): void;
  getTilesList(): Array<Tile>;
  setTilesList(value: Array<Tile>): void;
  addTiles(value?: Tile, index?: number): Tile;

  clearHeistersList(): void;
  getHeistersList(): Array<Heister>;
  setHeistersList(value: Array<Heister>): void;
  addHeisters(value?: Heister, index?: number): Heister;

  getAllItemsTaken(): boolean;
  setAllItemsTaken(value: boolean): void;

  getRemainingTile(): number;
  setRemainingTile(value: number): void;

  getGameStatus(): GameStatusMap[keyof GameStatusMap];
  setGameStatus(value: GameStatusMap[keyof GameStatusMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GameState.AsObject;
  static toObject(includeInstance: boolean, msg: GameState): GameState.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GameState, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GameState;
  static deserializeBinaryFromReader(message: GameState, reader: jspb.BinaryReader): GameState;
}

export namespace GameState {
  export type AsObject = {
    gameName: string,
    gameStarted: number,
    timerRunsOut: number,
    tilesList: Array<Tile.AsObject>,
    heistersList: Array<Heister.AsObject>,
    allItemsTaken: boolean,
    remainingTile: number,
    gameStatus: GameStatusMap[keyof GameStatusMap],
  }
}

export class Move extends jspb.Message {
  hasHeister(): boolean;
  clearHeister(): void;
  getHeister(): Heister | undefined;
  setHeister(value?: Heister): void;

  hasPosition(): boolean;
  clearPosition(): void;
  getPosition(): MapPosition | undefined;
  setPosition(value?: MapPosition): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Move.AsObject;
  static toObject(includeInstance: boolean, msg: Move): Move.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Move, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Move;
  static deserializeBinaryFromReader(message: Move, reader: jspb.BinaryReader): Move;
}

export namespace Move {
  export type AsObject = {
    heister?: Heister.AsObject,
    position?: MapPosition.AsObject,
  }
}

export class InvalidRequest extends jspb.Message {
  getReason(): string;
  setReason(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InvalidRequest.AsObject;
  static toObject(includeInstance: boolean, msg: InvalidRequest): InvalidRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: InvalidRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InvalidRequest;
  static deserializeBinaryFromReader(message: InvalidRequest, reader: jspb.BinaryReader): InvalidRequest;
}

export namespace InvalidRequest {
  export type AsObject = {
    reason: string,
  }
}

export class MainMessage extends jspb.Message {
  hasGameState(): boolean;
  clearGameState(): void;
  getGameState(): GameState | undefined;
  setGameState(value?: GameState): void;

  hasInvalidRequest(): boolean;
  clearInvalidRequest(): void;
  getInvalidRequest(): InvalidRequest | undefined;
  setInvalidRequest(value?: InvalidRequest): void;

  hasMove(): boolean;
  clearMove(): void;
  getMove(): Move | undefined;
  setMove(value?: Move): void;

  getBodyCase(): MainMessage.BodyCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MainMessage.AsObject;
  static toObject(includeInstance: boolean, msg: MainMessage): MainMessage.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: MainMessage, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MainMessage;
  static deserializeBinaryFromReader(message: MainMessage, reader: jspb.BinaryReader): MainMessage;
}

export namespace MainMessage {
  export type AsObject = {
    gameState?: GameState.AsObject,
    invalidRequest?: InvalidRequest.AsObject,
    move?: Move.AsObject,
  }

  export enum BodyCase {
    BODY_NOT_SET = 0,
    GAME_STATE = 1,
    INVALID_REQUEST = 2,
    MOVE = 3,
  }
}

export interface HeisterColorMap {
  YELLOW: 0;
  PURPLE: 1;
  GREEN: 2;
  ORANGE: 3;
}

export const HeisterColor: HeisterColorMap;

export interface HeisterSymbolMap {
  SWORD: 0;
  VIAL: 1;
  BOW: 2;
  AXE: 3;
}

export const HeisterSymbol: HeisterSymbolMap;

export interface HeisterNameMap {
  BARBARIAN: 0;
  MAGE: 1;
  ELF: 2;
  DWARF: 3;
}

export const HeisterName: HeisterNameMap;

export interface SquareTypeMap {
  NORMAL: 0;
  YELLOW_TELEPORT_PAD: 1;
  PURPLE_TELEPORT_PAD: 2;
  GREEN_TELEPORT_PAD: 3;
  ORANGE_TELEPORT_PAD: 4;
  YELLOW_ITEM: 5;
  PURPLE_ITEM: 6;
  GREEN_ITEM: 7;
  ORANGE_ITEM: 8;
  YELLOW_ESCAPE: 9;
  PURPLE_ESCAPE: 10;
  GREEN_ESCAPE: 11;
  ORANGE_ESCAPE: 12;
  ESCALATOR: 13;
  TIMER_FLIP: 14;
  TIMER_FLIP_USED: 15;
  FILLED: 16;
}

export const SquareType: SquareTypeMap;

export interface WallTypeMap {
  CLEAR: 0;
  IMPASSABLE: 1;
  YELLOW_DOOR: 2;
  PURPLE_DOOR: 3;
  GREEN_DOOR: 4;
  ORANGE_DOOR: 5;
}

export const WallType: WallTypeMap;

export interface AbilityMap {
  MOVE_NORTH: 0;
  MOVE_EAST: 1;
  MOVE_SOUTH: 2;
  MOVE_WEST: 3;
  TELEPORT: 4;
  REVEAL_TILES: 5;
  USE_ESCALATOR: 6;
}

export const Ability: AbilityMap;

export interface GameStatusMap {
  STAGING: 0;
  ONGOING: 1;
  VICTORY: 2;
  DEFEAT: 3;
}

export const GameStatus: GameStatusMap;

