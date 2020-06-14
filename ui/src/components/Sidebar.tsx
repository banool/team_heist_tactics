import React from "react";

import { Link } from "react-router-dom";

import * as colors from "../constants/colors";

import routes from "../common/routes";

export default props => {
  return (
    <div>
      <h1>AMARANTA CANDLE SUPREME INC. LLC. PTY. LTD.</h1>
      <ul>
        <Link to="/">
          <li>home</li>
        </Link>
        {routes.map(route => {
          if (route.sidebar === false) {
            return null;
          }
          return (
            <Link to={route.path} key={route.path}>
              <li>{route.name}</li>
            </Link>
          );
        })}
      </ul>
    </div>
  );
};
