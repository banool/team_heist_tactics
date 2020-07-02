import Footer from "./Footer";
import MainPage from "../join_game/MainPage";
import { Provider } from "react-redux";
import React from "react";
import store from "../common/store";
import styles from "./styles";

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
