import React, { useEffect, useRef, useState } from "react";
import {
  chatBoxActiveSelector,
  playerNameSelector,
  setChatBoxFocus,
} from "./slice";
import { useDispatch, useSelector } from "react-redux";

import { sendChat } from "./api";
import styles from "../components/styles";

export const useFocus = () => {
  const htmlElRef = useRef<HTMLInputElement>(null);
  const setFocus = (should_focus) => {
    const currentEl = htmlElRef.current;
    if (should_focus) {
      currentEl && currentEl.focus();
    } else {
      currentEl && currentEl.blur();
    }
  };
  return [htmlElRef, setFocus] as const;
};

const ChatBoxComponent = () => {
  const dispatch = useDispatch();
  const player_name = useSelector(playerNameSelector);
  const chat_box_active = useSelector(chatBoxActiveSelector);
  const [box_content, set_box_content] = useState("");
  const [inputRef, setInputFocus] = useFocus();
  setInputFocus(chat_box_active);

  const onKeyDown = (event) => {
    if (event.key === "Enter") {
      onSubmit();
    }
  };

  useEffect(() => {
    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("keyup", handleKeyUp);
    return function cleanup() {
      document.removeEventListener("keydown", handleKeyDown);
      document.removeEventListener("keyup", handleKeyUp);
    };
  });

  const handleKeyDown = (event) => {
    if (event.key === "Enter") {
      dispatch(setChatBoxFocus({ focused: !chat_box_active }));
      return;
    }
    document.removeEventListener("keydown", handleKeyDown);
  };

  const handleKeyUp = () => {
    document.addEventListener("keydown", handleKeyDown, { once: true });
  };

  const onFocus = () => {
    dispatch(setChatBoxFocus({ focused: true }));
  };

  const onBlur = () => {
    dispatch(setChatBoxFocus({ focused: false }));
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
          ref={inputRef}
          type="text"
          value={box_content}
          onChange={(e) => set_box_content(e.target.value)}
          onKeyDown={onKeyDown}
          onFocus={onFocus}
          onBlur={onBlur}
          placeholder="Press Enter to focus"
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
