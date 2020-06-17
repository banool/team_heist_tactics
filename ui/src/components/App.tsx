import { hot } from "react-hot-loader/root";
import React from "react";

import { BrowserRouter as Router } from "react-router-dom";

import Footer from "./Footer";
import * as colors from "../constants/colors";

import JoinGamePage from "../join_game/JoinGamePage";

const styles: { [key: string]: React.CSSProperties } = {
  root: {
    display: "flex",
    flexDirection: "row",
    justifyContent: "center"
  },
  content: {
    marginTop: 20,
    width: 1600,
    backgroundColor: colors.backgroundLight
  },
  layoutTable: {
    border: "2px solid #333"
  },
  contentCell: {
    border: "2px solid #333",
    width: "100%",
    padding: "5px 10px",
    verticalAlign: "top"
  }
};

const App = ({}) => {
  return (
    <div style={styles.root}>
      <div>
        <div style={styles.content}>
          <JoinGamePage />
        </div>
        <Footer />
      </div>
    </div>
  );
};

export default hot(App);
