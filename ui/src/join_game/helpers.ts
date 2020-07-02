import { useState, useEffect } from "react";
import { MapPosition } from "../generated/types_pb";
import { INTERNAL_SQUARE_SIZE, INTERNAL_TILE_OFFSET } from "../constants/other";
import { CanvasPosition } from "./types";

/**
 * Converts a tile position (R moves right, D moves down) to a grid position.
 *
 * Derivation: each move right gets us a tile that is one grid square lower,
 * and four grid squares + two extra walls to the right.
 * Similarly for moves downwards.
 *
 * @param m thickness of walls as a fraction of the size of one grid square
 */
function tileToGrid(R: number, D: number, m: number): { X: number; Y: number } {
  const X = 2 * m * R + 4 * R - D;
  const Y = 2 * m * D + 4 * D + R;
  return { X, Y };
}

/**
 * Converts a grid position to a tile position (R moves right, D moves down).
 *
 * Derivation: this function is the inverse of tileToGrid(). Solved with WolframAlpha:
 * https://www.wolframalpha.com/input/?i=solve+for+D%2CR%3A+X+%3D+R*%282m+%2B+4%29+-+D%2C+Y+%3D+D*%282m+%2B+4%29+%2B+R
 *
 * If the input X, Y is the position of a tile, the resulting R, D will be integers
 * (or almost integers, depending on floating point rounding).
 *
 * @param m thickness of walls as a fraction of the size of one grid square
 */
function gridToTile(X: number, Y: number, m: number): { R: number; D: number } {
  // https://www.wolframalpha.com/input/?i=solve+for+D%2CR%3A+X+%3D+R*%282m+%2B+4%29+-+D%2C+Y+%3D+D*%282m+%2B+4%29+%2B+R
  const K = 4 * m * m + 16 * m + 17;
  const R = (2 * m * X + 4 * X + Y) / K;
  const D = (2 * m * Y + 4 * Y - X) / K;
  return { R, D };
}

/**
 * Gets the grid coordinates of the tile containing the (possibly not integer)
 * grid position X,Y. May return null, since not all possible grid positions are
 * contained within a tile (e.g. walls, the empty squares at the corners).
 *
 * @param m thickness of walls as a fraction of the size of one grid square
 */
function getTileGridCoords(
  X: number,
  Y: number,
  m: number
): { X: number; Y: number } | null {
  let { R, D } = gridToTile(X, Y, m);
  // There might be a neater way to do this, but here is an algorithm that works:
  // Get the closest integer tile (R,D), and then check it and its 8 neighbours
  // to see if it is the tile that contains (X,Y).
  R = Math.round(R);
  D = Math.round(D);
  for (const r of [-1, 0, 1]) {
    for (const d of [-1, 0, 1]) {
      const { X: x, Y: y } = tileToGrid(R + r, D + d, m);
      if (x >= X - 3.99 && x <= X + 0.01 && y >= Y - 3.99 && y <= Y + 0.01) {
        return { X: x, Y: y };
      }
    }
  }
  return null;
}

/**
 * Converts a grid position on the map (with no walls) to a position in pixels on the canvas (with walls).
 */
export const mapPositionToCanvasPosition = (
  map_position: MapPosition,
  pixel_offset: number,
  canvas_width: number,
  canvas_height: number
): CanvasPosition | null => {
  const pos_no_walls_x = map_position.getX();
  const pos_no_walls_y = map_position.getY();

  // 1. find the tile we are in.
  const tile = getTileGridCoords(pos_no_walls_x, pos_no_walls_y, 0);
  if (tile === null) {
    return null;
  }
  const { X: tile_no_walls_x, Y: tile_no_walls_y } = tile;

  // 2. find its tile coordinates - these are the same whether we have walls or not
  const { R, D } = gridToTile(tile_no_walls_x, tile_no_walls_y, 0);

  // 3. use the tile coordinates to get the grid coordinates with walls
  const m = INTERNAL_TILE_OFFSET / INTERNAL_SQUARE_SIZE;
  const { X: tile_with_walls_x, Y: tile_with_walls_y } = tileToGrid(R, D, m);

  // 4. shift back to get the grid coordinates of the point we want
  const pos_with_walls_x = tile_with_walls_x + pos_no_walls_x - tile_no_walls_x;
  const pos_with_walls_y = tile_with_walls_y + pos_no_walls_y - tile_no_walls_y;

  // 5. convert grid-with-walls to pixels and apply shifts
  const canvas_x =
    INTERNAL_SQUARE_SIZE * pos_with_walls_x + pixel_offset + canvas_width / 2;
  const canvas_y =
    INTERNAL_SQUARE_SIZE * pos_with_walls_y + pixel_offset + canvas_height / 2;
  return { x: canvas_x, y: canvas_y };
};

/**
 * Converts a position in pixels on the canvas (with walls) to a grid position on the map (with no walls).
 */
export const canvasPositionToMapPosition = (
  canvas_position: CanvasPosition,
  pixel_offset: number,
  canvas_width: number,
  canvas_height: number
): MapPosition | null => {
  const canvas_x = canvas_position.x;
  const canvas_y = canvas_position.y;

  // 1. Convert pixels to grid-with-walls coordinate
  const pos_with_walls_x =
    (canvas_x - pixel_offset - canvas_width / 2) / INTERNAL_SQUARE_SIZE;
  const pos_with_walls_y =
    (canvas_y - pixel_offset - canvas_height / 2) / INTERNAL_SQUARE_SIZE;

  // 2. find the tile we are in.
  const m = INTERNAL_TILE_OFFSET / INTERNAL_SQUARE_SIZE;
  const tile = getTileGridCoords(pos_with_walls_x, pos_with_walls_y, m);
  if (tile === null) {
    return null;
  }
  const { X: tile_with_walls_x, Y: tile_with_walls_y } = tile;

  // 3. find its tile coordinates - these are the same whether we have walls or not
  const { R, D } = gridToTile(tile_with_walls_x, tile_with_walls_y, m);

  // 4. use the tile coordinates to get the grid coordinates without walls
  const { X: tile_no_walls_x, Y: tile_no_walls_y } = tileToGrid(R, D, 0);

  // 5. shift back to get the grid coordinates of the point we want, and round down
  const pos_no_walls_x =
    tile_no_walls_x + Math.floor(pos_with_walls_x - tile_with_walls_x);
  const pos_no_walls_y =
    tile_no_walls_y + Math.floor(pos_with_walls_y - tile_with_walls_y);

  const out = new MapPosition();
  out.setX(pos_no_walls_x);
  out.setY(pos_no_walls_y);
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
