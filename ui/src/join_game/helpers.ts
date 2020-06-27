import { useState, useEffect } from "react";
import { MapPosition } from "../generated/types_pb";
import { INTERNAL_SQUARE_SIZE, INTERNAL_TILE_OFFSET, TILE_SIZE } from "../constants/other";
import { CanvasPosition } from "./types";

export const mapPositionToCanvasPositionSingle = (
  n: number,
  pixel_offset: number,
  canvas_dimension_size_px: number, // CANVAS_WIDTH or CANVAS_HEIGHT
  _tile_offset: number
): number => {
  var num_tiles_away_from_center = Math.floor(n / 4);
  // Corner Canvas - this is the relative distance n from 0,0 in pixels
  var corner_canvas =
    (num_tiles_away_from_center * 2) * INTERNAL_TILE_OFFSET +
    n * INTERNAL_SQUARE_SIZE;
  // Adjusted Canvas - this translates 0,0 to match the center of the canvas, plus pixel offset
  var adjusted_canvas =
    corner_canvas + pixel_offset + canvas_dimension_size_px / 2;
  return adjusted_canvas;
};

export const mapPositionToCanvasPosition = (
  map_position: MapPosition,
  pixel_offset: number,
  tile_offset_x: number,
  tile_offset_y: number,
  canvas_width: number,
  canvas_height: number
): CanvasPosition => {
  var map_x = map_position.getX();
  var map_y = map_position.getY();
  var canvas_x = mapPositionToCanvasPositionSingle(
    map_x,
    pixel_offset,
    canvas_width,
    tile_offset_x
  );
  var canvas_y = mapPositionToCanvasPositionSingle(
    map_y,
    pixel_offset,
    canvas_height,
    tile_offset_y
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
  var canvas_square_offset = corner_canvas_val - (TILE_SIZE * num_tiles_offset);

  // We should repeat the process (square offset = floor of current val (canvas_square_offset) / square_size)
  var num_squares_offset = Math.floor(canvas_square_offset / INTERNAL_SQUARE_SIZE);

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
  }, []);

  return windowDimensions;
}
