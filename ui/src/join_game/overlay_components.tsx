import { getColor, sendChat } from "./api";
import {
  heisterSelectedSelector,
  playerIsSpectatorSelector,
  playerNameSelector,
  playersMaySpeakSelector,
} from "./slice";
import { useDispatch, useSelector } from "react-redux";

import { Ability } from "../generated/types_pb";
import { Circle } from "react-konva";
import React from "react";

// The offset makes the center of the image be the center of the canvas element.
type ResetMapComponentProps = {
  reset_parent_func: () => any;
};
export const ResetMapComponent = ({
  reset_parent_func,
}: ResetMapComponentProps) => {
  return (
    <button style={{ width: 110, height: 40 }} onClick={reset_parent_func}>
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

type TapButtonComponentProps = {
  name_prefix: string;
};
export const TapButtonComponent = ({
  name_prefix,
}: TapButtonComponentProps) => {
  const dispatch = useDispatch();
  const player_name = useSelector(playerNameSelector);

  const onClick = (_event) => {
    let recipient = name_prefix.split("'").slice(0, -1).join("'");
    let msg = `${player_name}: tap ${recipient}`;
    dispatch(sendChat(msg)); // todo
  };

  const is_self = name_prefix === "Your";
  const pointer_events = is_self ? "none" : "auto";
  const text = is_self ? "-" : "Tap";
  const class_name = is_self ? "invisible" : undefined;

  return (
    <button
      className={class_name}
      style={{
        borderRadius: 6,
        fontSize: 16,
        pointerEvents: pointer_events,
        width: 50,
      }}
      onClick={onClick}
    >
      {text}
    </button>
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
  const player_is_spectator = useSelector(playerIsSpectatorSelector);

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

  return (
    <p>
      {player_is_spectator ? null : (
        <TapButtonComponent name_prefix={name_prefix} />
      )}
      &nbsp;&nbsp;
      {name_prefix} abilities: {abilities_string}
    </p>
  );
};

export const MaySpeakComponent = () => {
  let players_may_speak = useSelector(playersMaySpeakSelector);

  let msg: string;
  if (players_may_speak) {
    msg = "🐵 You may speak 🐵";
  } else {
    msg = "🙊 You may not speak 🙊";
  }

  return <p>{msg}</p>;
};
