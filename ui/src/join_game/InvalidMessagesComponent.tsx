import React from "react";

import { useSelector } from "react-redux";

import { playerMessageQueueSelector } from "./slice";
import styles from "../components/styles";

const InvalidMessagesComponent = () => {
  const player_message_queue = useSelector(playerMessageQueueSelector);

  return (
    <div style={styles.invalidMessages}>
      {player_message_queue.map((msg) => (
        <p key={msg}>{msg}</p>
      ))}
    </div>
  );
};

export default InvalidMessagesComponent;
