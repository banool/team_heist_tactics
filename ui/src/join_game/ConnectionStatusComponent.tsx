import { ConnectionStatus } from "./types";
import React from "react";
import { connectionStatusSelector } from "./slice";
import { useSelector } from "react-redux";

const ConnectionStatusComponent = () => {
  const connection_status = useSelector(connectionStatusSelector);

  return <p>Connection Status: {ConnectionStatus[connection_status]}</p>;
};

export default ConnectionStatusComponent;
