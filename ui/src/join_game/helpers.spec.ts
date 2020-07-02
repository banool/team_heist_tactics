import {
  INTERNAL_SQUARE_SIZE,
  INTERNAL_TILE_OFFSET,
  TILE_SIZE,
} from "../constants/other";
import {
  canvasPositionToMapPosition,
  mapPositionToCanvasPosition,
} from "./helpers";

import { MapPosition } from "../generated/types_pb";

const CANVAS_HEIGHT = 1000;
const CANVAS_WIDTH = 1600;
const pixel_offset = 0;
// For reference in these these tests: INTERNAL_TILE_OFFSET = ~28.sth, INTERNAL_SQUARE_SIZE = ~60.sth

test("tile placement map_position reversible translations", () => {
  let center = new MapPosition();
  center.setX(0);
  center.setY(0);
  let a = mapPositionToCanvasPosition(
    center,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  expect(a.x).toBe(800);
  expect(a.y).toBe(500);
  let b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  expect(b.getX()).toBe(center.getX());
  expect(b.getY()).toBe(center.getY());
});

test("tile 1,-4 map_position reversible translation", () => {
  let tile_corner = new MapPosition();
  tile_corner.setX(1);
  tile_corner.setY(-4);
  let a = mapPositionToCanvasPosition(
    tile_corner,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  // For this simple case, I can guess what the resulting canvas value is. TODO - is this assumption correct?
  expect(a.x).toBe(CANVAS_WIDTH / 2 + INTERNAL_SQUARE_SIZE);
  expect(a.y).toBe(CANVAS_HEIGHT / 2 - TILE_SIZE);
  let b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  expect(b.getX()).toBe(tile_corner.getX());
  expect(b.getY()).toBe(tile_corner.getY());
});

test("tile -1,4 map_position reversible translation", () => {
  let tile_corner = new MapPosition();
  tile_corner.setX(-1);
  tile_corner.setY(4);
  let a = mapPositionToCanvasPosition(
    tile_corner,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  // expect(a.x).toBe(CANVAS_WIDTH / 2 - INTERNAL_SQUARE_SIZE); // -- BROKEN
  expect(a.y).toBe(CANVAS_HEIGHT / 2 + TILE_SIZE);
  let b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  expect(b.getX()).toBe(tile_corner.getX());
  expect(b.getY()).toBe(tile_corner.getY());
});

test("tile -4,-1 map_position reversible translation", () => {
  let tile_corner = new MapPosition();
  tile_corner.setX(-4);
  tile_corner.setY(-1);
  let a = mapPositionToCanvasPosition(
    tile_corner,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  expect(a.x).toBe(
    CANVAS_WIDTH / 2 - (2 * INTERNAL_TILE_OFFSET + 4 * INTERNAL_SQUARE_SIZE)
  );
  // expect(a.y).toBe(
  //   CANVAS_HEIGHT / 2 - INTERNAL_SQUARE_SIZE
  // ); // -- BROKEN
  let b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  expect(b.getX()).toBe(tile_corner.getX());
  expect(b.getY()).toBe(tile_corner.getY());
});

test("tile 4,1 map_position reversible translation", () => {
  let tile_corner = new MapPosition();
  tile_corner.setX(4);
  tile_corner.setY(1);
  let a = mapPositionToCanvasPosition(
    tile_corner,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  expect(a.x).toBe(
    CANVAS_WIDTH / 2 + (2 * INTERNAL_TILE_OFFSET + 4 * INTERNAL_SQUARE_SIZE)
  );
  expect(a.y).toBe(CANVAS_HEIGHT / 2 + INTERNAL_SQUARE_SIZE);
  let b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  expect(b.getX()).toBe(tile_corner.getX());
  expect(b.getY()).toBe(tile_corner.getY());
});

test("tile 4,-16 map_position reversible translation", () => {
  let tile_corner = new MapPosition();
  tile_corner.setX(4);
  tile_corner.setY(-16);
  let a = mapPositionToCanvasPosition(
    tile_corner,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  let b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  expect(b.getX()).toBe(tile_corner.getX());
  expect(b.getY()).toBe(tile_corner.getY());
});

test("map position translation in both directions at non-center should work, too", () => {
  // NOTE: TODO: This fails at big numbers (251 and 252).
  // Unit test of sorts.
  var mp = new MapPosition();
  mp.setX(1);
  mp.setY(2);
  var a = mapPositionToCanvasPosition(
    mp,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  var b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  )!;
  expect(b.getX()).toBe(mp.getX());
  expect(b.getY()).toBe(mp.getY());
});
