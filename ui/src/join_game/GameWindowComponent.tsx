import React from "react";
import { useDispatch, useSelector } from "react-redux";
import { gameStateSelector } from "./slice";
import { Tile as ProtoTile } from "../generated/types_pb";
import { moveHeister } from "./api";
import { Stage, Layer, Circle, Text } from 'react-konva';
import Konva from 'konva';
import { Image } from 'react-konva';
import useImage from 'use-image';
import { CANVAS_WIDTH, CANVAS_HEIGHT, SERVER_WIDTH, SERVER_HEIGHT } from "../constants/other";

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

type TileProps = {
  proto_tile: ProtoTile;
};

// The offset makes the center of the image be the center of the canvas element.
const Tile = ({ proto_tile }: TileProps) => {
  const url = `static/images/00${proto_tile.getName()}.jpg`;
  const [image, status] = useImage(url);

  const size = 300;
  const offset = size/2;

  var position = proto_tile.getPosition();
  var proto_x = position!.getX();
  var proto_y = position!.getY();
  var x = (proto_x / SERVER_WIDTH) * CANVAS_WIDTH;
  var y = (proto_y / SERVER_HEIGHT) * CANVAS_HEIGHT;

  console.log("x", x);
  console.log("y", y);

  var comp: JSX.Element;
  if (status === "loaded") {
    comp = <Image shadowBlur={10} image={image} width={size} height={size} offsetX={offset} offsetY={offset} x={x} y={y} />;
  } else if (status === "loading") {
    comp = <Text text={`Loading tile ${name}...`}/>
  } else {
    comp = <Text text={`Failed to load tile ${name}!`}/>
  }

  return (comp);
};

// This uses special <> syntax to return multiple components.
const Tiles = ({ tiles }) => (
  <>
    {tiles.map((tile: any) => (
      tile
    ))}
  </>
);


type GameWindowComponentProps = {
  width: number;
  height: number;
};
const GameWindowComponent = ({ width, height }: GameWindowComponentProps) => {
  const dispatch = useDispatch();

  const game_state = useSelector(gameStateSelector);

  const getTiles = () => {
    var proto_tiles = game_state!.getTilesList();
    var tiles: JSX.Element[] = [];
    for (let i = 0; i < proto_tiles.length; i++) {
      var t = <Tile proto_tile={proto_tiles[i]} />;
      tiles.push(t);
    }
    return tiles;
  };

  // <div style={{ width: "90%", transform: "translate(+5%, 0%)", backgroundColor: "#ffffff" }}>
  return (
    <div style={{ width: "100%", backgroundColor: "#ffffff"}}>
      <Stage width={width} height={height}>
        <Layer>
          <Tiles tiles={getTiles()}/>
        </Layer>
      </Stage>
    </div>
  );
};

export default GameWindowComponent;
