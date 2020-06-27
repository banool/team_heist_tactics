import { useState, useEffect } from "react";
import { MapPosition } from "../generated/types_pb";
import { INTERNAL_SQUARE_SIZE, INTERNAL_TILE_OFFSET, TILE_SIZE, REAL_TILE_SIZE_PX, REAL_TILE_WALL_SIZE, REAL_TILE_SQUARE_SIZE } from "../constants/other";
import { CanvasPosition } from "./types";
import { timingSafeEqual } from "crypto";

export const tileMapPosToCanvasPosSingle = (
  n: number,
  pixel_offset: number,
  canvas_dimension_size_px: number, // CANVAS_WIDTH or CANVAS_HEIGHT
  _tile_offset: number
): number => {
  var center_px = canvas_dimension_size_px / 2;
  var num_tiles_away_from_center = Math.floor(n / 4);
  var tile_no_walls = REAL_TILE_SIZE_PX * 4;
  var wall = REAL_TILE_WALL_SIZE;
  var extra_wall = 0;
  if (n > 0) {
    extra_wall = 1;

  }
  var distance = center_px + (tile_no_walls * num_tiles_away_from_center) + (2 * num_tiles_away_from_center * wall) + (wall * extra_wall);
  return distance;
};

export const mapPositionToCanvasPositionSingle = (
  n: number,
  pixel_offset: number,
  canvas_dimension_size_px: number, // CANVAS_WIDTH or CANVAS_HEIGHT
  _tile_offset: number
): number => {
  var neg = false;
  if (n == 0) {
    center_px = canvas_dimension_size_px / 2;
    return center_px;
  }
  if (n < 0) {
    n = n * -1;
    neg = true;

  }
  var wall = INTERNAL_TILE_OFFSET;
  var center_px = (canvas_dimension_size_px / 2);
  var num_tiles = Math.floor(n / 4);
  var square = INTERNAL_SQUARE_SIZE;
  var distance = 0;
  console.log(n)
  console.log(center_px)
  console.log(wall)
  console.log(square)
  if (neg) {
    if (n < 4) {
      distance = center_px - (square * n) - wall;
    } else {
      distance = center_px - (square * n) - (2 * wall * num_tiles) - wall;
    }
    return distance + wall;

  } else {
    if (n < 4) {
      distance = center_px + (square * n) + wall;
    } else {
      distance = center_px + (square * n) + (2 * wall * num_tiles) + wall;
    }
    return distance - wall;
  }
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

const tileCoordToMapCoord = (
  coord: number,
  pixel_offset: number,
  canvas_dimension_size_px: number // CANVAS_WIDTH or CANVAS_HEIGHT
): number => {
  var zero = canvas_dimension_size_px / 2;
  var wall = REAL_TILE_WALL_SIZE;
  var extra_wall = 0;
  var t = REAL_TILE_WALL_SIZE * 4;
  if (coord > zero) {
    extra_wall = 1;
  }
  var num = (4 * (coord - zero - (wall * extra_wall))) / (t + (2 * wall));

  return num;
};

const canvasCoordToMapCoord = (
  coord: number,
  pixel_offset: number,
  canvas_dimension_size_px: number // CANVAS_WIDTH or CANVAS_HEIGHT
): number => {
  var neg = false;
  var wall = INTERNAL_TILE_OFFSET;
  var center_px = (canvas_dimension_size_px / 2);
  if (center_px != coord) {
    center_px = center_px + wall;
  }
  var away = coord - center_px;
  if (coord < center_px) {
    neg = true;
    away = away * -1;

  }
  var num_tiles = Math.floor(away / TILE_SIZE);
  var square = INTERNAL_SQUARE_SIZE;
  var distance = (away - (num_tiles * 2 * wall) - wall) / square;
  if (away < (TILE_SIZE - wall)) {
    distance = (away - wall) / square;
  }
  if (neg) {
    return distance * -1;
  } else {
    return distance;
  }

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
