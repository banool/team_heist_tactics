import React from "react";
import { Stage, Layer, Circle, Text } from "react-konva";
import { getColor } from "./api";
import { heisterSelectedSelector } from "./slice";
import { useSelector } from "react-redux";

// The offset makes the center of the image be the center of the canvas element.
type ResetMapComponentProps = {
  reset_parent_func: () => any;
};
export const ResetMapComponent = ({
  reset_parent_func,
}: ResetMapComponentProps) => {
  return (
    <button style={{ width: 100, height: 40 }} onClick={reset_parent_func}>
      Reset Map
    </button>
  );
};

type ActiveHeisterKeyboardComponentProps = {
  heister_color;
  x: number;
  y: number;
};
export const ActiveHeisterKeyboardComponent = ({
  heister_color,
  x,
  y,
}: ActiveHeisterKeyboardComponentProps) => {
  const heister_selected = useSelector(heisterSelectedSelector);

  const color = getColor(heister_color);

  // Pretty sure this is the only component that needs
  // perfectDrawEnabled to be true (which is the default).
  // Without it, the circle looks weird.
  return (
    <Circle
      x={x}
      y={y}
      stroke={color}
      fill={color}
      strokeWidth={2}
      radius={10}
      shadowBlur={8}
      shadowColor="black"
      shadowEnabled={heister_color === heister_selected}
      perfectDrawEnabled={true}
    />
  );
};
