import { JoinGameThing, StagingJoinGameThing } from "./types";
import React, { useState } from "react";

import { joinGame } from "./api";
import { useDispatch } from "react-redux";

const defaults: StagingJoinGameThing = {
  name: "",
  handle: "",
};

type JoinGameFormProps = {
  existing?: JoinGameThing;
};
const JoinGameForm = ({ existing }: JoinGameFormProps) => {
  const updating = existing !== undefined;
  const dispatch = useDispatch();

  // TODO Make this generic again some day lol.
  const getInitial = (
    field: string,
    defaults: StagingJoinGameThing,
    existing?: JoinGameThing
  ): NonNullable<string> => {
    if (existing !== undefined && existing[field] !== null) {
      return existing[field];
    }
    // Read param from URL query params if present.
    var url = new URL(window.location.href);
    var param = url.searchParams.get(field);
    if (param !== null) {
      return param;
    }
    return defaults[field];
  };

  const [name, setName] = useState(getInitial("name", defaults, existing));
  const [handle, setHandle] = useState(
    getInitial("handle", defaults, existing)
  );

  const stateToStagingJoinGameThing = (): StagingJoinGameThing => {
    return {
      name,
      handle,
    };
  };

  const onSubmit = () => {
    if (updating) {
      // TODO: Implement.
      console.error("Not implemented yet :]");
    } else {
      if (name.length <= 0) {
        alert("Please enter a name");
        return;
      }
      if (handle.length <= 0) {
        alert("Please enter a game handle");
        return;
      }
      console.log("Dispatching action to join game");
      dispatch(joinGame(stateToStagingJoinGameThing()));
    }
  };

  return (
    <div>
      <form>
        <br />
        <label>
          <h3>Join Game</h3>
          Name:
          <input
            autoFocus
            type="text"
            placeholder="Your name"
            value={name}
            onChange={(e) => setName(e.target.value)}
            autoComplete="off"
            autoCorrect="off"
            autoCapitalize="off"
            spellCheck="false"
            required={true}
            minLength={1}
            maxLength={100}
          />
        </label>
        <br />
        <label>
          Handle:
          <input
            type="text"
            placeholder="Game handle"
            value={handle}
            onChange={(e) => setHandle(e.target.value)}
            autoComplete="off"
            autoCorrect="off"
            autoCapitalize="off"
            spellCheck="false"
            required={true}
            minLength={1}
          />
        </label>
        <br />
        <button type="submit" onClick={onSubmit}>
          Submit
        </button>
      </form>
    </div>
  );
};

export default JoinGameForm;
