import React from "react";
import { useDispatch, useSelector } from "react-redux";
import { gameStateSelector } from "./slice";
import { Move } from "../generated/types_pb";
import { moveHeister } from "./api";
import { Stage, Layer, Circle, Text } from 'react-konva';
import Konva from 'konva';

class ColoredRect extends React.Component {
  state = {
    color: 'green'
  };
  handleClick = () => {
    this.setState({
      color: Konva.Util.getRandomColor()
    });
  };
  render() {
    return (
      <Circle
        x={120}
        y={120}
        radius={30}
        fill={this.state.color}
        shadowBlur={10}
        onClick={this.handleClick}
      />
    );
  }
}

const GameWindowComponent = () => {
  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);

  return (
    <div style={{ width: "90%", transform: "translate(+5%, 0%)", backgroundColor: "#222222" }}>
      <Stage width={1500} height={800}>
        <Layer>
          <ColoredRect />
        </Layer>
      </Stage>
    </div>
  );
};

export default GameWindowComponent;
