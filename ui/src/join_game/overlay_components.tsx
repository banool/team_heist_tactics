import React from "react";
import { Circle } from "react-konva";
import { getColor } from "./api";
import { heisterSelectedSelector } from "./slice";
import { useSelector } from "react-redux";
import { Ability } from "../generated/types_pb";

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

type PlayerAbilitiesProps = {
  name_prefix: string;
  proto_abilities: number[];
};
export const PlayerAbilities = ({
  name_prefix,
  proto_abilities,
}: PlayerAbilitiesProps) => {
  const getAbilityEmoji = (proto_ability): string => {
    switch (proto_ability) {
      case Ability.MOVE_NORTH:
        return "⬆️";
      case Ability.MOVE_EAST:
        return "➡️";
      case Ability.MOVE_SOUTH:
        return "⬇️";
      case Ability.MOVE_WEST:
        return "⬅️";
      case Ability.TELEPORT:
        return "♋️";
      case Ability.REVEAL_TILES:
        return "🔎";
      case Ability.USE_ESCALATOR:
        return "🎢";
      default:
        console.error("Unexpected Ability", proto_ability);
        return "?";
    }
  };

  var abilities_string: string = "";
  for (let i = 0; i < proto_abilities.length; i++) {
    abilities_string = abilities_string.concat(
      " ",
      getAbilityEmoji(proto_abilities[i])
    );
  }

  console.log("FINDFLSKDF", abilities_string);

  return (
    <p>
      {name_prefix} abilities: {abilities_string}
    </p>
  );
};
