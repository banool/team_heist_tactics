import React, { useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import { playerNameSelector } from "./slice";
import { sendChat } from "./api";
import styles from "../components/styles";

const ChatBoxComponent = () => {
  const dispatch = useDispatch();
  const player_name = useSelector(playerNameSelector);
  const [box_content, set_box_content] = useState("");

  const onKeyDown = (event) => {
    if (event.key === "Enter") {
      onSubmit();
    }
  };

  const onSubmit = () => {
    if (box_content.length == 0) {
      return;
    }
    let chat = `${player_name}: ${box_content}`;
    dispatch(sendChat(chat));
    set_box_content("");
  };

  return (
    <div style={styles.chatBox}>
      <form
        onSubmit={(e) => {
          e.preventDefault();
        }}
      >
        <input
          type="text"
          value={box_content}
          onChange={(e) => set_box_content(e.target.value)}
          onKeyDown={onKeyDown}
          size={40}
        />
        <button style={{ marginLeft: 5 }} type="submit" onClick={onSubmit}>
          Send
        </button>
      </form>
    </div>
  );
};

export default ChatBoxComponent;
