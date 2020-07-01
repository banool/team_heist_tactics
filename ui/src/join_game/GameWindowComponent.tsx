import React, { useState, useEffect } from "react";
import { useDispatch, useSelector, Provider, connect } from "react-redux";
import {
  gameStateSelector,
  numInvalidMoveAttemptsSelector,
  playerNameSelector,
  playerIsSpectatorSelector,
} from "./slice";
import {
  Tile as ProtoTile,
  Heister as ProtoHeister,
  HeisterColor,
  MapPosition,
  Player,
} from "../generated/types_pb";
import { moveHeisterReal, placeTile, getColor } from "./api";
import { Stage, Layer, Circle, Text, Rect } from "react-konva";
import Konva from "konva";
import { Image } from "react-konva";
import useImage from "use-image";
import {
  HEISTER_SIZE,
  TILE_SIZE,
  INTERNAL_SQUARE_SIZE,
  INTERNAL_TILE_OFFSET,
} from "../constants/other";
import {
  mapPositionToCanvasPosition,
  canvasPositionToMapPosition,
  useWindowDimensions,
} from "./helpers";
import store from "../common/store";
import {
  ResetMapComponent,
  ActiveHeisterKeyboardComponent,
  PlayerAbilities,
} from "./overlay_components";
import styles from "../components/styles";
import ConnectionStatusComponent from "./ConnectionStatusComponent";
import { CanvasPosition } from "./types";
import InvalidMessagesComponent from "./InvalidMessagesComponent";
import TimerComponent from "./TimerComponent";

type GetTileShadowSquareProps = {
  proto_tile: ProtoTile;
  width: number;
  height: number;
};

const get_tile_and_shadow_tile = ({
  proto_tile,
  width,
  height,
}: GetTileShadowSquareProps) => {
  const size = TILE_SIZE;
  const offset = size / 2;

  var map_position = proto_tile.getPosition()!;
  var canvas_position = mapPositionToCanvasPosition(
    map_position,
    width,
    height
  );
  var num_rotations = proto_tile.getNumRotations();
  var tile_name = proto_tile.getName();

  console.log(
    `fuck tile at canvas.x/y ${canvas_position.x} ${
      canvas_position.y
    } map ${map_position.getX()} ${map_position.getY()} rotated ${num_rotations} times`
  );

  var t = (
    <Tile
      key={tile_name}
      tile_name={tile_name}
      size={size}
      num_rotations={num_rotations}
      offset={offset}
      canvas_position={canvas_position}
    />
  );

  var st = (
    <Rect
      key={`${tile_name}_square`}
      x={canvas_position.x + 5}
      y={canvas_position.y + 5}
      width={TILE_SIZE - 10}
      height={TILE_SIZE - 10}
      shadowBlur={20}
      shadowColor="black"
      shadowEnabled={true}
      stroke="black"
      strokeWidth={10}
      offsetX={TILE_SIZE / 2}
      offsetY={TILE_SIZE / 2}
    />
  );

  return { tile: t, shadow_tile: st };
};

type TileProps = {
  tile_name: string;
  size: number;
  num_rotations: number;
  offset: number;
  canvas_position: CanvasPosition;
};
// The offset makes the center of the image be the center of the canvas element.
const Tile = ({
  tile_name,
  size,
  num_rotations,
  offset,
  canvas_position,
}: TileProps) => {
  // TODO Consider preloading the next / all images.
  // Probably not necessary becayse images are cached in the client, so the user
  // only ever suffers the slow load time once.
  console.log("tilename", tile_name);
  const url = `static/images/00${tile_name}.jpg`;
  console.log("url", url);
  const [image, status] = useImage(url);

  var comp: JSX.Element;
  if (status === "loaded") {
    comp = (
      <Image
        //shadowBlur={15}
        image={image}
        width={size}
        height={size}
        offsetX={offset}
        offsetY={offset}
        x={canvas_position.x}
        y={canvas_position.y}
        rotation={num_rotations * 90}
        perfectDrawEnabled={false}
      />
    );
  } else if (status === "loading") {
    comp = <Text text={`Loading tile ${name}...`} />;
  } else {
    comp = <Text text={`Failed to load tile ${name}!!!`} />;
  }

  return comp;
};

type HeisterProps = {
  proto_heister: ProtoHeister;
};
const Heister = ({ proto_heister }: HeisterProps) => {
  const dispatch = useDispatch();

  const heister_color = proto_heister.getHeisterColor();
  const map_position = proto_heister.getMapPosition()!;

  // TODO Don't tell the client the walls unless we wanna do client side validation.

  const pixel_offset = HEISTER_SIZE * 1.5 + INTERNAL_SQUARE_SIZE;

  var { width, height } = useWindowDimensions();
  const canvas_position = mapPositionToCanvasPosition(
    map_position,
    width,
    height
  );

  console.debug(
    `${heister_color} (0 yellow, 1 purple, 2 green, 3 orange) heister at canvas.x/y ${canvas_position.x} ${canvas_position.y} map ${map_position}`
  );

  // First, resolve the canvas position into an intended map position.
  // Second, dispatch the move request.
  // Third, turn the map position back into a canvas position (to snap the unit to a square).
  const onDragEnd = (event) => {
    // Pause rendering of this unit until we get information back
    // about whether the move attempt was valid. Otherwise it'll just snap back immediately.
    // Or perhaps until we get new game state back as a stop gap.
    console.info("event okay", event);
    var x = event.target.x();
    var y = event.target.y();
    console.log("Attempted position ", x, y);
    var intended_canvas_position = { x: x, y: y };
    var intended_map_position = canvasPositionToMapPosition(
      intended_canvas_position,
      pixel_offset,
      width,
      height
    );
    console.log(
      `Heister ${heister_color} (0 yellow, 1 purple, 2 green, 3 orange) dropped at ${intended_map_position.getX()} ${intended_map_position.getY()}`
    );
    dispatch(moveHeisterReal(proto_heister, intended_map_position));
  };

  // If x changes but y doesn't, y won't update, only x will.
  // Introducing some jitter makes sure they always change.
  // TODO Do this only when a reset toggle flips.
  var random_x = Math.random() * 0.001 + 0.001;
  var random_y = Math.random() * 0.001 + 0.001;

  return (
    <Circle
      x={canvas_position.x - pixel_offset + random_x}
      y={canvas_position.y - pixel_offset + random_y}
      stroke="black"
      fill={getColor(heister_color)}
      strokeWidth={4}
      radius={HEISTER_SIZE}
      draggable={true}
      onDragEnd={onDragEnd}
      perfectDrawEnabled={false}
    />
  );
};

type PossiblePlacementProps = {
  map_position: MapPosition;
};
const PossiblePlacement = ({ map_position }: PossiblePlacementProps) => {
  const dispatch = useDispatch();

  const pixel_offset = INTERNAL_SQUARE_SIZE * 2;
  console.log("pixel offset", pixel_offset);

  var { width, height } = useWindowDimensions();
  const canvas_position = mapPositionToCanvasPosition(
    map_position,
    width,
    height
  );

  console.log(
    `square at canvas.x/y ${canvas_position.x} ${canvas_position.y} map ${map_position}`
  );

  const onClick = (_event) => {
    dispatch(placeTile(map_position));
  };

  const onMouseEnter = (_event) => {
    setShadowEnabled(true);
  };

  const onMouseLeave = (_event) => {
    setShadowEnabled(false);
  };

  const [shadowEnabled, setShadowEnabled] = useState(false);

  const stroke_width = 4;

  return (
    <Rect
      x={canvas_position.x - pixel_offset}
      y={canvas_position.y - pixel_offset}
      width={INTERNAL_SQUARE_SIZE}
      height={INTERNAL_SQUARE_SIZE}
      stroke="black"
      strokeWidth={stroke_width}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      onClick={onClick}
      shadowBlur={5}
      shadowColor="black"
      shadowEnabled={shadowEnabled}
      perfectDrawEnabled={false}
    />
  );
};

// This uses special <> syntax to return multiple components.
const Tiles = ({ tiles }) => <>{tiles.map((t: any) => t)}</>;
const Heisters = ({ heisters }) => <>{heisters.map((h: any) => h)}</>;
const PossiblePlacements = ({ possible_placements }) => (
  <>{possible_placements.map((p: any) => p)}</>
);
const ShadowTiles = ({ shadow_tiles }) => (
  <>{shadow_tiles.map((st: any) => st)}</>
);
const ListOfPlayerAbilities = ({ player_abilities }) => (
  <>{player_abilities.map((a: any) => a)}</>
);

const GameWindowComponent = () => {
  const game_state = useSelector(gameStateSelector);
  const player_name = useSelector(playerNameSelector);
  const player_is_spectator = useSelector(playerIsSpectatorSelector);

  // By making this invalid move counter part of the state relevant to this component,
  // the component will get updated whenever there is an invalid move attempt.
  // TODO: Make one of these per heister, to reduce necessary updates.
  useSelector(numInvalidMoveAttemptsSelector);

  var { width, height } = useWindowDimensions();

  // https://reactjs.org/docs/lists-and-keys.html#keys
  const getTiles = () => {
    var proto_tiles = game_state!.getTilesList();
    var tiles: JSX.Element[] = [];
    var shadow_tiles: JSX.Element[] = [];
    for (let i = 0; i < proto_tiles.length; i++) {
      var proto_tile = proto_tiles[i];
      var { tile, shadow_tile } = get_tile_and_shadow_tile({
        proto_tile: proto_tile,
        width: width,
        height: height,
      });
      tiles.push(tile);
      shadow_tiles.push(shadow_tile);
    }
    return { tiles: tiles, shadow_tiles: shadow_tiles };
  };

  const getHeisters = () => {
    var proto_heisters = game_state!.getHeistersList();
    var heisters: JSX.Element[] = [];
    for (let i = 0; i < proto_heisters.length; i++) {
      var t = (
        <Provider key={i} store={store}>
          <Heister key={i + 200} proto_heister={proto_heisters[i]} />
        </Provider>
      );
      heisters.push(t);
    }
    return heisters;
  };

  // Renders squares in positions where the player with the ability
  // to place tiles could place tiles.
  // TODO: Only render this if the player has the ability to place tiles.
  const getPossiblePlacements = () => {
    if (game_state!.getRemainingTiles() == 0) {
      return [];
    }
    var proto_possible_placements = game_state!.getPossiblePlacementsList();
    var possiblePlacements: JSX.Element[] = [];
    for (let i = 0; i < proto_possible_placements.length; i++) {
      var pp = (
        <Provider key={i} store={store}>
          <PossiblePlacement
            key={i + 300}
            map_position={proto_possible_placements[i]}
          />
        </Provider>
      );
      possiblePlacements.push(pp);
    }
    return possiblePlacements;
  };

  const getPlayerAbilities = () => {
    // Make sure their own abilities appear at the top.
    var proto_players = game_state!.getPlayersList();
    var player_abilities: JSX.Element[] = [];
    var self: JSX.Element | null = null;
    for (let i = 0; i < proto_players.length; i++) {
      var proto_player = proto_players[i];
      var name_prefix: string;
      var proto_name = proto_player.getName();
      if (proto_name === player_name) {
        name_prefix = "Your";
      } else {
        name_prefix = `${proto_name}'s`;
      }
      var as = (
        <Provider key={i} store={store}>
          <PlayerAbilities
            key={i + 300}
            name_prefix={name_prefix}
            proto_abilities={proto_player.getAbilitiesList()}
          />
        </Provider>
      );
      if (proto_name === player_name) {
        self = as;
      } else {
        player_abilities.push(as);
      }
    }
    if (self) {
      player_abilities.unshift(self);
    }
    return player_abilities;
  };

  const [stageX, setStageX] = useState(0);
  const [stageY, setStageY] = useState(0);

  // Force the map to re-render in the middle by making the X and Y slightly different.
  const resetMap = () => {
    setStageX(Math.random() * 0.001 + 0.001);
    setStageY(Math.random() * 0.001 + 0.001);
  };

  const KEYBOARD_ITEM_Y = 50;
  const BASE_KEYBOARD_ICON = 30;
  const ORANGE_HEISTER_KEYBOARD_ICON = BASE_KEYBOARD_ICON + 35 * 0;
  const GREEN_HEISTER_KEYBOARD_ICON = BASE_KEYBOARD_ICON + 35 * 1;
  const PURPLE_HEISTER_KEYBOARD_ICON = BASE_KEYBOARD_ICON + 35 * 2;
  const YELLOW_HEISTER_KEYBOARD_ICON = BASE_KEYBOARD_ICON + 35 * 3;

  var { tiles, shadow_tiles } = getTiles();

  // <div style={{ width: "90%", transform: "translate(+5%, 0%)", backgroundColor: "#ffffff" }}>

  // Use position only for transformsEnabled since we don't scale or rotate.
  // For some reason I need to add a provider again for elements inside the
  // konva Stage, even though I shouldn't need to because I have a top level
  // provider wrapping the app.

  // There are two stages. The first here is for things that should move when
  // move "the map". The second is for overlay elements that shouldn't move
  // even when the user drags the map around.

  // We set `listening={false}` on Layers that don't need to receive clicks.
  return (
    <div>
      <div style={styles.gameWindowComponent}>
        <div style={styles.gameWindowComponentWrapper}>
          <Stage
            x={stageX}
            y={stageY}
            width={width}
            height={height}
            draggable={true}
            transformsEnabled={"position"}
          >
            <Layer listening={false}>
              <ShadowTiles shadow_tiles={shadow_tiles} />
              <Tiles tiles={tiles} />
            </Layer>
            <Layer listening={!player_is_spectator}>
              <Heisters heisters={getHeisters()} />
            </Layer>
            <Layer listening={!player_is_spectator}>
              <PossiblePlacements
                possible_placements={getPossiblePlacements()}
              />
            </Layer>
          </Stage>
        </div>
        <div className={player_is_spectator ? "hidden" : ""}>
          <div
            style={{
              ...styles.keyboardHeisterNumber,
              ...{ right: BASE_KEYBOARD_ICON + 5, top: 10 },
            }}
          >
            You're moving
          </div>
          <div
            style={{
              ...styles.keyboardHeisterNumber,
              ...{ right: YELLOW_HEISTER_KEYBOARD_ICON - 4 },
            }}
          >
            1
          </div>
          <div
            style={{
              ...styles.keyboardHeisterNumber,
              ...{ right: PURPLE_HEISTER_KEYBOARD_ICON - 4 },
            }}
          >
            2
          </div>
          <div
            style={{
              ...styles.keyboardHeisterNumber,
              ...{ right: GREEN_HEISTER_KEYBOARD_ICON - 4 },
            }}
          >
            3
          </div>
          <div
            style={{
              ...styles.keyboardHeisterNumber,
              ...{ right: ORANGE_HEISTER_KEYBOARD_ICON - 4 },
            }}
          >
            4
          </div>
        </div>
        <div
          className={player_is_spectator ? "hidden" : ""}
          style={styles.overlayCanvas}
        >
          <Stage
            x={stageX}
            y={stageY}
            width={width}
            height={height}
            draggable={false}
            transformsEnabled={"none"}
          >
            <Layer listening={false}>
              <Provider store={store}>
                <ActiveHeisterKeyboardComponent
                  x={width - YELLOW_HEISTER_KEYBOARD_ICON}
                  y={KEYBOARD_ITEM_Y}
                  heister_color={HeisterColor.YELLOW}
                />
                <ActiveHeisterKeyboardComponent
                  x={width - PURPLE_HEISTER_KEYBOARD_ICON}
                  y={KEYBOARD_ITEM_Y}
                  heister_color={HeisterColor.PURPLE}
                />
                <ActiveHeisterKeyboardComponent
                  x={width - GREEN_HEISTER_KEYBOARD_ICON}
                  y={KEYBOARD_ITEM_Y}
                  heister_color={HeisterColor.GREEN}
                />
                <ActiveHeisterKeyboardComponent
                  x={width - ORANGE_HEISTER_KEYBOARD_ICON}
                  y={KEYBOARD_ITEM_Y}
                  heister_color={HeisterColor.ORANGE}
                />
              </Provider>
            </Layer>
          </Stage>
        </div>
      </div>
      <div style={styles.resetGameWindowOverlay}>
        <ResetMapComponent reset_parent_func={resetMap} />
      </div>
      <div style={styles.invalidMessagesOverlay}>
        <InvalidMessagesComponent />
      </div>
      <div style={styles.timerOverlay}>
        <TimerComponent />
      </div>
      <div style={styles.playerAbilitiesOverlay}>
        <ListOfPlayerAbilities player_abilities={getPlayerAbilities()} />
      </div>
      <div className={player_is_spectator ? "" : "hidden"}>
        <div
          style={{
            ...styles.timerOverlay,
            ...styles.pulse,
            ...{ top: 0 },
          }}
        >
          Spectating
        </div>
      </div>
    </div>
  );
};

export default GameWindowComponent;

// TODO
// Draw 4 circles at the top right and highlight the heister you want to control.
