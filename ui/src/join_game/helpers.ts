import { useState, useEffect } from "react";
import { MapPosition } from "../generated/types_pb";
import {
  INTERNAL_SQUARE_SIZE,
  INTERNAL_TILE_OFFSET,
  TILE_SIZE,
} from "../constants/other";
import { CanvasPosition, SquareCoords, TileCoords } from "./types";

const inclusive_range = (start: number, end: number): number[] => {
  return [...Array(end + 1 - start).keys()].map((i) => i + start);
};

// The max number of tiles we would have in any direction from the middle.
const TILES_LENGTH = 50;

// This function precomputes a map of square coords (map positions) to tile coords.
// Because you can't really use normal objects as keys, I use a string as a key instead.
// -50 to 50 is total overkill, 25 would be enough, but better safe than sorry.
const precomputeSquareCoordtoTileCoordMap = (): Map<string, TileCoords> => {
  let map: Map<string, TileCoords> = new Map();
  for (let tile_x = -TILES_LENGTH; tile_x <= TILES_LENGTH; tile_x++) {
    for (let tile_y = -TILES_LENGTH; tile_y <= TILES_LENGTH; tile_y++) {
      var min_x = tile_x * 4 - tile_y;
      var max_x = tile_x * 4 - tile_y + 3;
      var min_y = tile_y * 4 + tile_x;
      var max_y = tile_y * 4 + tile_x + 3;
      for (let x of inclusive_range(min_x, max_x)) {
        for (let y of inclusive_range(min_y, max_y)) {
          let key = `${x},${y}`;
          map.set(key, { x: tile_x, y: tile_y });
        }
      }
    }
  }
  return map;
};

// This is a constant, precomputed map of square coords to tile coords.
export const squareCoordToTileCoordMap = precomputeSquareCoordtoTileCoordMap();

// Given square coords, get the tile coords of the tile that square is on.
export const squareCoordstoTileCoords = (x: number, y: number): TileCoords => {
  let key = `${x},${y}`;
  let tile_coords = squareCoordToTileCoordMap.get(key);
  if (tile_coords === undefined) {
    throw `Couldn't lookup tile coord from map coord ${x},${y}`;
  }
  return tile_coords;
};

// Given one of the values in a map position (x or y), get a canvas coord.
export const mapPositionToCanvasPositionSingle = (
  map_n: number,
  tile_n: number,
  canvas_dimension_size_px: number // CANVAS_WIDTH or CANVAS_HEIGHT
): number => {
  var corner_canvas =
    tile_n * 2 * INTERNAL_TILE_OFFSET + map_n * INTERNAL_SQUARE_SIZE;
  // Adjusted Canvas - this translates 0,0 to match the center of the canvas.
  var adjusted_canvas = corner_canvas + canvas_dimension_size_px / 2;
  return adjusted_canvas;
};

// Given a map position, get the canvas coords (canvas pixels).
// These are the final numbers we feed into the tile's x and y.
export const mapPositionToCanvasPosition = (
  map_position: MapPosition,
  canvas_width: number,
  canvas_height: number
): CanvasPosition => {
  var map_x = map_position.getX();
  var map_y = map_position.getY();
  var tile_coords = squareCoordstoTileCoords(map_x, map_y);
  var canvas_x = mapPositionToCanvasPositionSingle(
    map_x,
    tile_coords.x,
    canvas_width
  );
  var canvas_y = mapPositionToCanvasPositionSingle(
    map_y,
    tile_coords.y,
    canvas_height
  );
  return { x: canvas_x, y: canvas_y };
};

const canvasCoordToMapCoord = (
  coord: number,
  pixel_offset: number,
  canvas_dimension_size_px: number // CANVAS_WIDTH or CANVAS_HEIGHT
): number => {
  // Translate the point so that its origin is at 0,0 - not at CANVAS_DIM / 2
  var corner_canvas_val = coord - pixel_offset - canvas_dimension_size_px / 2;

  // We know that corner_canvas_val is the _sum_ of tile_offset and square_offset.
  // We need to get both of those values separately from this value, corner_canvas_val
  var num_tiles_offset = Math.floor(corner_canvas_val / TILE_SIZE);
  var canvas_square_offset = corner_canvas_val - TILE_SIZE * num_tiles_offset;

  // We should repeat the process (square offset = floor of current val (canvas_square_offset) / square_size)
  var num_squares_offset = Math.floor(
    canvas_square_offset / INTERNAL_SQUARE_SIZE
  );

  // For each tile, we're moved 4 away. For each square, it's 1 worth
  var num_squares_away_from_center = num_tiles_offset * 4 + num_squares_offset;
  return num_squares_away_from_center;
};

export const canvasPositionToMapPosition = (
  canvas_position: CanvasPosition,
  pixel_offset: number,
  canvas_width: number,
  canvas_height: number
): MapPosition => {
  var map_x = canvasCoordToMapCoord(
    canvas_position.x,
    pixel_offset,
    canvas_width
  );
  var map_y = canvasCoordToMapCoord(
    canvas_position.y,
    pixel_offset,
    canvas_height
  );
  var out = new MapPosition();
  out.setX(map_x);
  out.setY(map_y);
  return out;
};

function getWindowDimensions() {
  const { innerWidth: width, innerHeight: height } = window;
  return {
    width,
    height,
  };
}

export function useWindowDimensions() {
  const [windowDimensions, setWindowDimensions] = useState(
    getWindowDimensions()
  );

  useEffect(() => {
    function handleResize() {
      setWindowDimensions(getWindowDimensions());
    }

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  });

  return windowDimensions;
}
