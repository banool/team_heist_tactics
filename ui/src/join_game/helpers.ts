import { useState, useEffect } from "react";
import { MapPosition } from "../generated/types_pb";
import {
  INTERNAL_SQUARE_SIZE,
  INTERNAL_TILE_OFFSET,
  TILE_SIZE,
} from "../constants/other";
import { CanvasPosition } from "./types";

export const howManyWallsTile = (n: number): number => {
  if (n % 4 == 0) {
    return n / 2;
  } else if (((n +1)  % 4) == 0) {
    return (n+1)/2;
  } else {
    return 2 * Math.floor(n / 4);
  }
};

export const howManyWallsHeister = (n: number): number => {
  return 2 * Math.floor(n / 4) + 1;
};

export const mapPositionToCanvasPositionSingle = (
  n: number,
  pixel_offset: number,
  canvas_dimension_size_px: number, // CANVAS_WIDTH or CANVAS_HEIGHT
  _tile_offset: number,
  secondary: number,
  tile: number
): number => {
  var neg = false;
  if (n < 0) {
    neg = true;
    n = n * -1;
  }
  var center_px = canvas_dimension_size_px / 2;
  if (n == 0) {
    return center_px;
  }
  var square = INTERNAL_SQUARE_SIZE;
  var wall = INTERNAL_TILE_OFFSET;
  var num_walls = 0;
  if (tile == 1) {
    num_walls = howManyWallsTile(n);
  } else {
    num_walls = howManyWallsHeister(n);
  }
  if (neg) {
    return center_px - (n * square + num_walls * wall);
  } else {
    return center_px + (n * square + num_walls * wall);
  }
};

export const tileMapPositionToCanvasPosition = (
  x: number,
  y: number,
  canvas_width: number,
  canvas_height: number

): CanvasPosition => {
  var center_x = canvas_width / 2;
  var center_y = canvas_height / 2;
  var wall = INTERNAL_TILE_OFFSET;
  var x_px = x + (2*wall * (4*x - y))/17;
  console.log(x_px);
  var y_px = y + (2*wall * (4*y - x))/17;
  console.log(y_px);
  return { x: center_x + x_px, y: center_y + y_px };

}

export const mapPositionToCanvasPosition = (
  map_position: MapPosition,
  pixel_offset: number,
  tile_offset_x: number,
  tile_offset_y: number,
  canvas_width: number,
  canvas_height: number,
  tile: number
): CanvasPosition => {
  var map_x = map_position.getX();
  var map_y = map_position.getY();
  if (tile == 1) {
    return tileMapPositionToCanvasPosition(map_x, map_y, canvas_width, canvas_height);
  }
  var canvas_x = mapPositionToCanvasPositionSingle(
    map_x,
    pixel_offset,
    canvas_width,
    tile_offset_x,
    map_y,
    tile
  );
  var canvas_y = mapPositionToCanvasPositionSingle(
    map_y,
    pixel_offset,
    canvas_height,
    tile_offset_y,
    map_x,
    tile
  );
  return { x: canvas_x, y: canvas_y };
};

const canvasCoordToMapCoord = (
  coord: number,
  pixel_offset: number,
  canvas_dimension_size_px: number // CANVAS_WIDTH or CANVAS_HEIGHT
): number => {
  var center_px = canvas_dimension_size_px / 2;
  var away = 0;
  var neg = false;
  if (coord < center_px) {
    away = center_px - coord;
    neg = true;
  } else {
    away = coord - center_px;
  }
  var num_walls = 2 * Math.floor(away / TILE_SIZE) + 1;
  var dist = (away - num_walls * INTERNAL_TILE_OFFSET) / INTERNAL_SQUARE_SIZE;
  if (neg) {
    return dist * -1;
  } else {
    return dist;
  }
};

export const canvasPositionToMapPosition = (
  canvas_position: CanvasPosition,
  pixel_offset: number,
  canvas_width: number,
  canvas_height: number,
  tile: number
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
