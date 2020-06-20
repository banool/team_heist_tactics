import React from "react";
import { useDispatch, useSelector } from "react-redux";
import { gameStateSelector } from "./slice";
import { Tile as ProtoTile, Heister as ProtoHeister, HeisterColor, HeisterColorMap, MapPosition } from "../generated/types_pb";
import { moveHeister } from "./api";
import { Stage, Layer, Circle, Text } from "react-konva";
import Konva from "konva";
import { Image } from "react-konva";
import useImage from "use-image";
import {
  CANVAS_WIDTH,
  CANVAS_HEIGHT,
  HEISTER_SIZE,
  SERVER_WIDTH,
  SERVER_HEIGHT,
  TILE_SIZE,
  INTERNAL_SQUARE_SIZE,
  MAP_SQUARE_SIZE,
  INTERNAL_TILE_OFFSET,
} from "../constants/other";
import { CanvasPosition } from "./types";

const mapPositionToCanvasPosition = (
  map_position: MapPosition,
  pixel_offset: number,
): CanvasPosition => {
  var map_x_middle = SERVER_WIDTH / 2;
  var map_y_middle = SERVER_HEIGHT / 2;
  var map_x = map_position.getX();
  var map_y = map_position.getY();
  var map_x_offset = map_x - map_x_middle;
  var map_y_offset = map_y - map_y_middle;
  var num_tiles_away_from_center_x = Math.floor(map_x_offset / 4);
  var num_tiles_away_from_center_y = Math.floor(map_y_offset / 4);
  var corner_canvas_x = (((num_tiles_away_from_center_x * 2) + 1) * INTERNAL_TILE_OFFSET) + (map_x_offset * INTERNAL_SQUARE_SIZE);
  var corner_canvas_y = (((num_tiles_away_from_center_y * 2) + 1) * INTERNAL_TILE_OFFSET) + (map_y_offset * INTERNAL_SQUARE_SIZE);
  var adjusted_canvas_x = corner_canvas_x + pixel_offset + (CANVAS_WIDTH / 2);
  var adjusted_canvas_y = corner_canvas_y + pixel_offset + (CANVAS_HEIGHT / 2);
  return { x: adjusted_canvas_x, y: adjusted_canvas_y };
};

const canvasPositionToMapPosition = (
  canvas_position: CanvasPosition,
  pixel_offset: number,
): MapPosition => {
  var acx = canvas_position.x;
  var acy = canvas_position.y;
  var ccxm = acx - pixel_offset - (CANVAS_WIDTH / 2);
  var ccym = acy - pixel_offset - (CANVAS_WIDTH / 2);
  var ccx = ccxm - (mxo * INTERNAL_SQUARE_SIZE);
  var ccy = ccym - (myo * INTERNAL_SQUARE_SIZE);
  var mxo = (((ccx/INTERNAL_TILE_OFFSET) - 1) / 2) * 4;
  var myo = ((ccy/INTERNAL_TILE_OFFSET) - 1) / 2;
  var map_x_middle = SERVER_WIDTH / 2;
  var map_y_middle = SERVER_HEIGHT / 2;
  var mpx = mxo + map_x_middle;
  var mpy = myo + map_y_middle;
  var out = new MapPosition();
  out.setX(mpx);
  out.setY(mpy);
  return out;
}

// Unit test of sorts.
var mp = new MapPosition();
mp.setX(251);
mp.setY(252);
console.log("map x y", mp.getX(), mp.getY());
var a = mapPositionToCanvasPosition(mp, 20);
console.log("canvas x y", a.x, a.y);
var b = canvasPositionToMapPosition(a, 20);
console.log("back to map x y", b.getX(), b.getY());
if (b.getX() != 30 || b.getY() != 40) {
  var msg = "Map -> canvas -> map position is wrong";
  console.error(msg);
  throw msg;
}

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

  console.log(`tile at canvas.x/y ${canvas_position.x} ${canvas_position.y} map ${map_position}`);

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
  const offset = HEISTER_SIZE;
  const pixel_offset = -INTERNAL_SQUARE_SIZE - HEISTER_SIZE * 2 + 3;

  const heister_color = proto_heister.getHeisterColor();
  const map_position = proto_heister.getMapPosition()!;
  const canvas_position = mapPositionToCanvasPosition(map_position, pixel_offset);

  console.log(`${heister_color} (0 yellow, 1 purple, 2 green, 3 orange) heister at canvas.x/y ${canvas_position.x} ${canvas_position.y} map ${map_position}`);

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
  const onDragEnd = (event) => {
    // Pause rendering of this unit until we get information back
    // about whether the move attempt was valid. Otherwise it'll just snap back immediately.
    // Or perhaps until we get new game state back as a stop gap.
    var x = event.target.x();
    var y = event.target.x();
    console.log("Attempted position ", x, y);

  }

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
}

// This uses special <> syntax to return multiple components.
const Tiles = ({ tiles }) => <>{tiles.map((t: any) => t)}</>;
const Heisters = ({ heisters }) => <>{heisters.map((h: any) => h)}</>;

type GameWindowComponentProps = {
  width: number;
  height: number;
};
const GameWindowComponent = ({ width, height }: GameWindowComponentProps) => {
  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);

  const getTiles = () => {
    var proto_tiles = game_state!.getTilesList();
    var tiles: JSX.Element[] = [];
    for (let i = 0; i < proto_tiles.length; i++) {
      var t = <Tile proto_tile={proto_tiles[i]} />;
      tiles.push(t);
    }
    return tiles;
  };

  const getHeisters = () => {
    var proto_heisters = game_state!.getHeistersList();
    var heisters: JSX.Element[] = [];
    for (let i = 0; i < proto_heisters.length; i++) {
      var t = <Heister proto_heister={proto_heisters[i]} />;
      heisters.push(t);
    }
    return heisters;
  };

  // <div style={{ width: "90%", transform: "translate(+5%, 0%)", backgroundColor: "#ffffff" }}>
  return (
    <div style={{ width: "100%", backgroundColor: "#ffffff" }}>
      <Stage width={width} height={height}>
        <Layer>
          <Tiles tiles={getTiles()} />
          <Heisters heisters={getHeisters()} />
        </Layer>
      </Stage>
    </div>
  );
};

export default GameWindowComponent;
