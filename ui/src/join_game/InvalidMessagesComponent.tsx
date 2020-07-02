import { MAX_PLAYER_MESSAGES } from "../constants/other";
import React from "react";
import { playerMessageQueueSelector } from "./slice";
import styles from "../components/styles";
import { useSelector } from "react-redux";

const InvalidMessagesComponent = () => {
  const player_message_queue = useSelector(playerMessageQueueSelector);

  const LOWEST_OPACITY = 0.6;

  return (
    <div style={styles.invalidMessages}>
      {player_message_queue.map((msg, i) => (
        <p
          style={{
            opacity:
              LOWEST_OPACITY +
              i * (i - LOWEST_OPACITY) * (1 / MAX_PLAYER_MESSAGES),
          }}
          key={msg}
        >
          {msg}
        </p>
      ))}
    </div>
  );
};

export default InvalidMessagesComponent;
