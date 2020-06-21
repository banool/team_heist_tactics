import { hot } from "react-hot-loader/root";
import React from "react";

import Footer from "./Footer";
import JoinGamePage from "../join_game/JoinGamePage";
import styles from "./styles";

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
