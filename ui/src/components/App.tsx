import React from "react";
import Footer from "./Footer";
import JoinGamePage from "../join_game/JoinGamePage";
import styles from "./styles";
import { Provider } from "react-redux";
import store from "../common/store";

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

const AppWrapper = () => {
  return (
    <Provider store={store}>
      <App />
    </Provider>
  );
};

export default AppWrapper;
