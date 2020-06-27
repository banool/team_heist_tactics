import {
  canvasPositionToMapPosition,
  mapPositionToCanvasPosition,
} from "./helpers";
import { MapPosition } from "../generated/types_pb";
import { INTERNAL_SQUARE_SIZE, INTERNAL_TILE_OFFSET } from "../constants/other";
let CANVAS_HEIGHT = 1000;
let CANVAS_WIDTH = 1600;
// For reference in these these tests: INTERNAL_TILE_OFFSET = ~28.sth, INTERNAL_SQUARE_SIZE = ~60.sth

// Helper for checking door y-value alignment. (TODO - could be repurposed for x-value, as well)
function get_door_canvas_yval_from_tile_corner(mp, dir_int) {
  // 0 = N, 1 = W, 2 = E, 3 = S
  return mp.y + INTERNAL_TILE_OFFSET + dir_int * INTERNAL_SQUARE_SIZE;
}

test("map_position translation both directions for 0,0 should return 0,0", () => {
  let pixel_offset = 0;
  let center = new MapPosition();
  center.setX(0);
  center.setY(0);
  let a = mapPositionToCanvasPosition(
    center,
    pixel_offset,
    0,
    0,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  let b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  expect(b.getX()).toBe(center.getX());
  expect(b.getY()).toBe(center.getY());
});

test("tile 1,-4 map_position reversible translation", () => {
  let pixel_offset = 0;
  let tile_corner = new MapPosition();
  tile_corner.setX(1);
  tile_corner.setY(-4);
  let a = mapPositionToCanvasPosition(
    tile_corner,
    pixel_offset,
    0,
    0,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  // For this simple case, I can guess what the resulting canvas value is. TODO - is this assumption correct?
  expect(a.x).toBe(CANVAS_WIDTH / 2 + INTERNAL_SQUARE_SIZE);
  expect(a.y).toBe(
    CANVAS_HEIGHT / 2 - 2 * INTERNAL_TILE_OFFSET - 4 * INTERNAL_SQUARE_SIZE
  );
  let b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  expect(b.getX()).toBe(tile_corner.getX());
  expect(b.getY()).toBe(tile_corner.getY());
});

test("tile -1,4 map_position reversible translation", () => {
  let pixel_offset = 0;
  let tile_corner = new MapPosition();
  tile_corner.setX(-1);
  tile_corner.setY(4);
  let a = mapPositionToCanvasPosition(
    tile_corner,
    pixel_offset,
    0,
    0,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  expect(a.x).toBe(
    CANVAS_WIDTH / 2 - (2 * INTERNAL_TILE_OFFSET + INTERNAL_SQUARE_SIZE)
  );
  expect(a.y).toBe(
    CANVAS_HEIGHT / 2 + (2 * INTERNAL_TILE_OFFSET + 4 * INTERNAL_SQUARE_SIZE)
  );
  let b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  expect(b.getX()).toBe(tile_corner.getX());
  expect(b.getY()).toBe(tile_corner.getY());
});

test("tile -4,-1 map_position reversible translation", () => {
  let pixel_offset = 0;
  let tile_corner = new MapPosition();
  tile_corner.setX(-4);
  tile_corner.setY(-1);
  let a = mapPositionToCanvasPosition(
    tile_corner,
    pixel_offset,
    0,
    0,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  expect(a.x).toBe(
    CANVAS_WIDTH / 2 - (2 * INTERNAL_TILE_OFFSET + 4 * INTERNAL_SQUARE_SIZE)
  );
  expect(a.y).toBe(
    CANVAS_HEIGHT / 2 - (2 * INTERNAL_TILE_OFFSET + INTERNAL_SQUARE_SIZE)
  );
  let b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  expect(b.getX()).toBe(tile_corner.getX());
  expect(b.getY()).toBe(tile_corner.getY());
});

test("tile 4,1 map_position reversible translation", () => {
  let pixel_offset = 0;
  let tile_corner = new MapPosition();
  tile_corner.setX(4);
  tile_corner.setY(1);
  let a = mapPositionToCanvasPosition(
    tile_corner,
    pixel_offset,
    0,
    0,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  // expect(a.x).toBe(CANVAS_WIDTH / 2 + (2 * INTERNAL_TILE_OFFSET + 4 * INTERNAL_SQUARE_SIZE))
  // expect(a.y).toBe(CANVAS_HEIGHT / 2 + INTERNAL_SQUARE_SIZE)
  let b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  expect(b.getX()).toBe(tile_corner.getX());
  expect(b.getY()).toBe(tile_corner.getY());
});

test("tile 4,-16 map_position reversible translation", () => {
  let pixel_offset = 0;
  let tile_corner = new MapPosition();
  tile_corner.setX(4);
  tile_corner.setY(-16);
  let a = mapPositionToCanvasPosition(
    tile_corner,
    pixel_offset,
    0,
    0,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  let b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  expect(b.getX()).toBe(tile_corner.getX());
  expect(b.getY()).toBe(tile_corner.getY());
});

test("map position translation in both directions at non-center should work, too", () => {
  // NOTE: TODO: This fails at big numbers (251 and 252).
  // Unit test of sorts.
  let pixel_offset = 20;
  var mp = new MapPosition();
  mp.setX(1);
  mp.setY(2);
  var a = mapPositionToCanvasPosition(
    mp,
    pixel_offset,
    0,
    0,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  var b = canvasPositionToMapPosition(
    a,
    pixel_offset,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  expect(b.getX()).toBe(mp.getX());
  expect(b.getY()).toBe(mp.getY());
});

test("doors align y-axis on western draw", () => {
  var mp = new MapPosition();
  mp.setX(0);
  mp.setY(0);
  var tile00_pos = mapPositionToCanvasPosition(
    mp,
    0,
    0,
    0,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  var tile_center_west_door_val = get_door_canvas_yval_from_tile_corner(
    tile00_pos,
    1
  );
  var tile_center_east_door_yval = get_door_canvas_yval_from_tile_corner(
    tile00_pos,
    2
  );

  var west_door_first_tile = new MapPosition();
  west_door_first_tile.setX(-4);
  west_door_first_tile.setY(-1);
  var tile_west = mapPositionToCanvasPosition(
    west_door_first_tile,
    0,
    0,
    0,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  var tile_west_door_yval = get_door_canvas_yval_from_tile_corner(tile_west, 2); // its east door aligns with center door

  // the distance here, it's off by - that's 2 INTERNAL_TILE_OFFSETs.
  // so, for some cases, we're just adding two extra than we need to (??)
  // expect(tile_center_west_door_val).toBe(tile_west_door_yval); // -- BROKEN

  var east_door_first_tile = new MapPosition();
  east_door_first_tile.setX(4);
  east_door_first_tile.setY(1);
  var tile_east_pos = mapPositionToCanvasPosition(
    east_door_first_tile,
    0,
    0,
    0,
    CANVAS_WIDTH,
    CANVAS_HEIGHT
  );
  var tile_east_door_yval = get_door_canvas_yval_from_tile_corner(
    tile_east_pos,
    1
  ); // west door on this tile

  expect(tile_east_door_yval).toBe(tile_center_east_door_yval);
});
