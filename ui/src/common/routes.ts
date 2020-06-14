import React from "react";
import { generatePath } from "react-router";

import DefaultPage from "../components/DefaultPage";

import CandlePage from "../candles/CandlePage";
import CandlesPage from "../candles/CandlesPage";

interface Route {
  name: string;
  path: string;
  sidebar: boolean;
  component: React.FunctionComponent;
}

export const CandleRoute: Route = {
  name: "candle",
  path: "/candles/:id",
  sidebar: false,
  component: CandlePage,
};
export const CandlesRoute: Route = {
  name: "candles",
  path: "/candles",
  sidebar: true,
  component: CandlesPage,
};

const routes: Route[] = [
  CandleRoute,
  CandlesRoute,
];

export default routes;

export const pathFor = (route: Route, params: {}): string => {
  return generatePath(route.path, params);
};
