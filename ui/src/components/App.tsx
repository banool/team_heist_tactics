import { hot } from "react-hot-loader/root";
import React from "react";

import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import Sidebar from "./Sidebar";
import Footer from "./Footer";
import * as colors from "../constants/colors";

import DefaultPage from "./DefaultPage";

import routes from "../common/routes";

const styles: { [key: string]: React.CSSProperties } = {
  root: {
    display: "flex",
    flexDirection: "row",
    justifyContent: "center"
  },
  content: {
    marginTop: 20,
    width: 900,
    backgroundColor: colors.backgroundLight
  },
  layoutTable: {
    border: "2px solid #333"
  },
  sidebarCell: {
    border: "2px solid #333",
    width: 220,
    padding: 5,
    verticalAlign: "top"
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
    <Router>
      <div style={styles.root}>
        <div>
          <div style={styles.content}>
            <table style={styles.layoutTable}>
              <tbody>
                <tr>
                  <td style={styles.sidebarCell}>
                    <Sidebar />
                  </td>
                  <td style={styles.contentCell}>
                    <Switch>
                      {routes.map(route => {
                        return (
                          <Route
                            key={route.path}
                            path={route.path}
                            children={<route.component />}
                          />
                        );
                      })}
                      <Route path="/">
                        <DefaultPage />
                      </Route>
                    </Switch>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          <Footer />
        </div>
      </div>
    </Router>
  );
};

export default hot(App);
