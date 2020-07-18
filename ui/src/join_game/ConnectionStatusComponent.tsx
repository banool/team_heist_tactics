import { ConnectionStatus } from "./types";
import React from "react";
import { connectionStatusSelector } from "./slice";
import { useSelector } from "react-redux";

const ConnectionStatusComponent = () => {
  const connection_status = useSelector(connectionStatusSelector);

  return <div><p>Connection Status</p><p>{ConnectionStatus[connection_status]}</p></div>;
};

export default ConnectionStatusComponent;
