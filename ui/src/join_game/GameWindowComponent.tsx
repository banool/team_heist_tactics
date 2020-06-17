import React from "react";
import { useDispatch, useSelector } from "react-redux";
import { gameStateSelector } from "./slice";
import { Tile as ProtoTile, Heister as ProtoHeister, MapPosition } from "../generated/types_pb";
import { moveHeister } from "./api";
import { Stage, Layer, Circle, Text } from "react-konva";
import Konva from "konva";
import { Image } from "react-konva";
import useImage from "use-image";
import {
  CANVAS_WIDTH,
  CANVAS_HEIGHT,
  SERVER_WIDTH,
  SERVER_HEIGHT,
  TILE_SIZE
} from "../constants/other";
import { CanvasPosition } from "./types";

const mapPositionToCanvasPosition = (
  map_position: MapPosition
): CanvasPosition => {
  var map_x = map_position.getX();
  var map_y = map_position.getY();
  var x = (map_x / SERVER_WIDTH) * CANVAS_WIDTH;
  var y = (map_y / SERVER_HEIGHT) * CANVAS_HEIGHT;
  return { x: x, y: y };
};

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

  var map_position = proto_tile.getPosition()!;
  var canvas_position = mapPositionToCanvasPosition(map_position);

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
}

// This uses special <> syntax to return multiple components.
const Tiles = ({ tiles }) => <>{tiles.map((tile: any) => tile)}</>;

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

  // <div style={{ width: "90%", transform: "translate(+5%, 0%)", backgroundColor: "#ffffff" }}>
  return (
    <div style={{ width: "100%", backgroundColor: "#ffffff" }}>
      <Stage width={width} height={height}>
        <Layer>
          <Tiles tiles={getTiles()} />
        </Layer>
      </Stage>
    </div>
  );
};

export default GameWindowComponent;
