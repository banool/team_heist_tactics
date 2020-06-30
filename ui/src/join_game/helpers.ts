import { useState, useEffect } from "react";
import { MapPosition, Tile as ProtoTile } from "../generated/types_pb";
import {
  INTERNAL_SQUARE_SIZE,
  INTERNAL_TILE_OFFSET,
  TILE_SIZE,
} from "../constants/other";
import { CanvasPosition } from "./types";

export const howManyWallsTile = (n: number): number => {
  if (n % 4 == 0) {
    return n / 2;
  } else if ((n + 1) % 4 == 0) {
    return (n + 1) / 2;
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

export const isMapPosSame = (a: MapPosition, b: MapPosition): boolean => {
  if (a.getX() == b.getX() && a.getY() == b.getY()) {
    return true;
  } else {
    return false;
  }
}

export const isInMapPosList = (a: (MapPosition | undefined)[], b: MapPosition): boolean => {
  var i;
  for (i = 0; i < a.length; i++) {
    var c = a[i];
    if (c != undefined) {
      console.log(c.getX());
      console.log(b.getX());
      console.log(c.getY());
      console.log(b.getY());
      if (isMapPosSame(c, b)) {
        console.log("they r the same");
        return true;
      }
    }
  };
  return false;
};


export const tileMapPositionToCanvasPosition = (
  x: number,
  y: number,
  center_x: number,
  center_y: number,
  x0: number,
  y0: number,
  map_positions: (MapPosition | undefined)[],
  back: MapPosition
): CanvasPosition => {
  console.log("recurse");
  var n1 = new MapPosition();
  n1.setX(x0 + 1);
  n1.setY(y0 - 4);
  var n2 = new MapPosition();
  n2.setX(x0 - 4);
  n2.setY(y0 - 1);
  var n3 = new MapPosition();
  n3.setX(x0 - 1);
  n3.setY(y0 + 4);
  var n4 = new MapPosition();
  n4.setX(x0 + 4);
  n4.setY(y0 + 1);
  if (x == 0 && y == 0) {
    return { x: center_x, y: center_y };
  } else if (x == n1.getX() && y == n1.getY()) {
    return { x: center_x + INTERNAL_SQUARE_SIZE, y: center_y - TILE_SIZE};
  } else if (x == n2.getX() && y == n2.getY()) {
    return { x: center_x - TILE_SIZE, y: center_y - INTERNAL_SQUARE_SIZE};
  } else if (x == n3.getX() && y == n3.getY()) {
    return { x: center_x - INTERNAL_SQUARE_SIZE, y: center_y + TILE_SIZE};
  } else if (x == n4.getX() && y == n4.getY()) {
    return { x: center_x + TILE_SIZE, y: center_y + INTERNAL_SQUARE_SIZE};
  } else {
    var imposs = { x: 1000, y: 1000};
    var back1 = new MapPosition();
    back1.setX(x0);
    back1.setY(y0);
    if (isInMapPosList(map_positions, n1) && !(isMapPosSame(n1, back))) {
      var a = tileMapPositionToCanvasPosition(
        x,
        y,
        center_x + INTERNAL_SQUARE_SIZE,
        center_y - TILE_SIZE,
        n1.getX(),
        n1.getY(),
        map_positions,
        back1
      )
      if (a != imposs) {
        return a;
      }
    }
    //console.log(n2.getX())
    //console.log(n2.getY())
    //map_positions.forEach(element => {
      //if (element != undefined) {
        //console.log(element.getX());
        //console.log(element.getY());
      //}
    //});
    if (isInMapPosList(map_positions, n2) && !(isMapPosSame(n2, back))) {
      console.log("hit n2");
      var a = tileMapPositionToCanvasPosition(
        x,
        y,
        center_x - TILE_SIZE,
        center_y - INTERNAL_SQUARE_SIZE,
        n2.getX(),
        n2.getY(),
        map_positions,
        back1
      )
      if (a != imposs) {
        return a;
      }
    }
    if (isInMapPosList(map_positions, n3) && !(isMapPosSame(n3, back))) {
      var a = tileMapPositionToCanvasPosition(
        x,
        y,
        center_x - INTERNAL_SQUARE_SIZE,
        center_y + TILE_SIZE,
        n3.getX(),
        n3.getY(),
        map_positions,
        back1
      )
      if (a != imposs) {
        return a;
      }
    }
    if (isInMapPosList(map_positions, n4) && !(isMapPosSame(n4, back))) {
      var a = tileMapPositionToCanvasPosition(
        x,
        y,
        center_x + TILE_SIZE,
        center_y + INTERNAL_SQUARE_SIZE,
        n4.getX(),
        n4.getY(),
        map_positions,
        back1
      )
      if (a != imposs) {
        return a;
      }
    }
    return imposs;
  }
};

export const mapPositionToCanvasPosition = (
  map_position: MapPosition,
  pixel_offset: number,
  tile_offset_x: number,
  tile_offset_y: number,
  canvas_width: number,
  canvas_height: number,
  tile: number,
  proto_tiles: ProtoTile[]
): CanvasPosition => {
  // This is all the map positions.
  var map_positions = proto_tiles.map((pt) => pt.getPosition());
  var map_x = map_position.getX();
  var map_y = map_position.getY();
  var back = new MapPosition();
    back.setX(0);
    back.setY(0);
  if (tile == 1) {
    return tileMapPositionToCanvasPosition(
      map_x,
      map_y,
      canvas_width / 2,
      canvas_height / 2,
      0,
      0,
      map_positions,
      back
    );
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
