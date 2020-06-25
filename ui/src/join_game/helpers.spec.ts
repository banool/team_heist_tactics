import {
  canvasPositionToMapPosition,
  mapPositionToCanvasPosition,
} from "./helpers";
import { MapPosition } from "../generated/types_pb";

test("map_position translation both directions for 0,0 should return 0,0", () => {
  let pixel_offset = 2;
  let center = new MapPosition();
  center.setX(0);
  center.setY(0);
  let a = mapPositionToCanvasPosition(center, pixel_offset, 0, 0);
  let b = canvasPositionToMapPosition(a, pixel_offset);
  expect(b.getX()).toBe(center.getX());
  expect(b.getY()).toBe(center.getY());
});

test("map position translation in both directions at non-center should work, too", () => {
  // NOTE: TODO: This fails at big numbers (251 and 252).
  // Unit test of sorts.
  let pixel_offset = 20;
  var mp = new MapPosition();
  mp.setX(1);
  mp.setY(2);
  var a = mapPositionToCanvasPosition(mp, pixel_offset, 0, 0);
  var b = canvasPositionToMapPosition(a, pixel_offset);
  expect(b.getX()).toBe(mp.getX());
  expect(b.getY()).toBe(mp.getY());
});
