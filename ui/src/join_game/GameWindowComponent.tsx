import React, { useState } from "react";
import { useDispatch, useSelector, Provider, connect } from "react-redux";
import { gameStateSelector, numInvalidMoveAttemptsSelector } from "./slice";
import {
  Tile as ProtoTile,
  Heister as ProtoHeister,
  HeisterColor,
  HeisterColorMap,
  MapPosition
} from "../generated/types_pb";
import { moveHeisterReal } from "./api";
import { Stage, Layer, Circle, Text } from "react-konva";
import Konva from "konva";
import { Image } from "react-konva";
import useImage from "use-image";
import {
  HEISTER_SIZE,
  TILE_SIZE,
  INTERNAL_SQUARE_SIZE,
  INTERNAL_TILE_OFFSET,
  CANVAS_WIDTH,
  CANVAS_HEIGHT
} from "../constants/other";
import {
  mapPositionToCanvasPosition,
  canvasPositionToMapPosition
} from "./helpers";
import { CanvasPosition } from "./types";
import store from "../common/store";
import { ResetMapComponent } from "./overlay_components";
import styles from "../components/styles";

type TileProps = {
  proto_tile: ProtoTile;
};
// The offset makes the center of the image be the center of the canvas element.
const Tile = ({ proto_tile }: TileProps) => {
  // TODO Consider preloading the next / all images.
  // Probably not necessary becayse images are cached in the client, so the user
  // only ever suffers the slow load time once.
  const url = `static/images/00${proto_tile.getName()}.jpg`;
  const [image, status] = useImage(url);

  const size = TILE_SIZE;
  const offset = size / 2;
  const pixel_offset = -INTERNAL_TILE_OFFSET;

  var map_position = proto_tile.getPosition()!;
  var canvas_position = mapPositionToCanvasPosition(map_position, pixel_offset);

  console.log(
    `tile at canvas.x/y ${canvas_position.x} ${canvas_position.y} map ${map_position}`
  );

  var comp: JSX.Element;
  if (status === "loaded") {
    comp = (
      <Image
        shadowBlur={10}
        image={image}
        width={size}
        height={size}
        offsetX={offset}
        offsetY={offset}
        x={canvas_position.x}
        y={canvas_position.y}
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

  const offset = HEISTER_SIZE;
  const pixel_offset = -INTERNAL_SQUARE_SIZE - HEISTER_SIZE * 2 + 3;

  const heister_color = proto_heister.getHeisterColor();
  const map_position = proto_heister.getMapPosition()!;
  const canvas_position = mapPositionToCanvasPosition(
    map_position,
    pixel_offset
  );

  console.log(
    `${heister_color} (0 yellow, 1 purple, 2 green, 3 orange) heister at canvas.x/y ${canvas_position.x} ${canvas_position.y} map ${map_position}`
  );

  const getColor = (heister_color): string => {
    switch (+heister_color) {
      case HeisterColor.YELLOW:
        return "#ffff66";
      case HeisterColor.PURPLE:
        return "#9900cc";
      case HeisterColor.GREEN:
        return "#009933";
      case HeisterColor.ORANGE:
        return "#ff9900";
      default:
        console.error("Unexpected heister color");
        return "#000000";
    }
  };

  // First, resolve the canvas position into an intended map position.
  // Second, dispatch the move request.
  // Third, turn the map position back into a canvas position (to snap the unit to a square).
  const onDragEnd = event => {
    // Pause rendering of this unit until we get information back
    // about whether the move attempt was valid. Otherwise it'll just snap back immediately.
    // Or perhaps until we get new game state back as a stop gap.
    var x = event.target.x();
    var y = event.target.y();
    console.log("Attempted position ", x, y);
    var intended_canvas_position = { x: x, y: y };
    var intended_map_position = canvasPositionToMapPosition(
      intended_canvas_position,
      pixel_offset
    );
    console.log(
      `Heister ${heister_color} (0 yellow, 1 purple, 2 green, 3 orange) dropped at ${intended_map_position.getX()} ${intended_map_position.getY()}`
    );
    dispatch(moveHeisterReal(proto_heister, intended_map_position));
  };

  // If x changes but y doesn't, y won't update, only x will.
  // Introducing some jitter makes sure they always change.
  var random_x = Math.random() * 0.001 + 0.001;
  var random_y = Math.random() * 0.001 + 0.001;

  return (
    <Circle
      shadowBlur={1}
      x={canvas_position.x + random_x}
      y={canvas_position.y + random_y}
      stroke="black"
      fill={getColor(heister_color)}
      strokeWidth={4}
      radius={HEISTER_SIZE}
      offsetX={offset}
      offsetY={offset}
      draggable={true}
      onDragEnd={onDragEnd}
    />
  );
};

// This uses special <> syntax to return multiple components.
const Tiles = ({ tiles }) => <>{tiles.map((t: any) => t)}</>;
const Heisters = ({ heisters }) => <>{heisters.map((h: any) => h)}</>;

const GameWindowComponent = () => {
  const width = CANVAS_WIDTH;
  const height = CANVAS_HEIGHT;

  const game_state = useSelector(gameStateSelector);

  // By making this invalid move counter part of the state relevant to this component,
  // the component will get updated whenever there is an invalid move attempt.
  // TODO: Make one of these per heister, to reduce necessary updates.
  useSelector(numInvalidMoveAttemptsSelector);

  // https://reactjs.org/docs/lists-and-keys.html#keys
  const getTiles = () => {
    var proto_tiles = game_state!.getTilesList();
    var tiles: JSX.Element[] = [];
    for (let i = 0; i < proto_tiles.length; i++) {
      var t = (
        <Provider key={i} store={store}>
          <Tile key={i + 100} proto_tile={proto_tiles[i]} />
        </Provider>
      );
      tiles.push(t);
    }
    return tiles;
  };

  const getHeisters = () => {
    var proto_heisters = game_state!.getHeistersList();
    var heisters: JSX.Element[] = [];
    for (let i = 0; i < proto_heisters.length; i++) {
      var t = (
        <Provider key={i} store={store}>
          <Heister key={i + 100} proto_heister={proto_heisters[i]} />
        </Provider>
      );
      heisters.push(t);
    }
    return heisters;
  };

  const [stageX, setStageX] = useState(0);
  const [stageY, setStageY] = useState(0);

  // Force the map to re-render in the middle by making the X and Y slightly different.
  const resetMap = () => {
    setStageX(Math.random() * 0.001 + 0.001);
    setStageY(Math.random() * 0.001 + 0.001);
  };

  // <div style={{ width: "90%", transform: "translate(+5%, 0%)", backgroundColor: "#ffffff" }}>
  // Use position only for transformsEnabled since we don't scale or rotate.
  return (
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
          <Layer>
            <Tiles tiles={getTiles()} />
            <Heisters heisters={getHeisters()} />
          </Layer>
        </Stage>
      </div>
      <div style={styles.gameWindowComponentWrapper}>
        <div style={styles.gameWindowOverlay}>
          <div style={styles.resetMapComponent}>
            <ResetMapComponent reset_parent_func={resetMap} />
          </div>
          <div style={styles.resetMapComponent}>
            <ResetMapComponent reset_parent_func={resetMap} />
          </div>
        </div>
      </div>
    </div>
  );
};

export default GameWindowComponent;
