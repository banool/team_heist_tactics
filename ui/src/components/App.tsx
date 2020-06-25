import React from "react";
import Footer from "./Footer";
import MainPage from "../join_game/MainPage";
import styles from "./styles";
import { Provider } from "react-redux";
import store from "../common/store";

// <Footer />

const App = ({}) => {
  return (
    <div style={styles.mainPage}>
      <MainPage />
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
