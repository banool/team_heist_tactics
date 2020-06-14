import React, { useState } from "react";

import { useDispatch } from "react-redux";
import { joinGame } from "./api";
import { JoinGameThing, StagingJoinGameThing } from "./types";

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

  const getInitial = <T extends unknown>(
    field: string,
    defaults: StagingJoinGameThing,
    existing?: JoinGameThing
  ): NonNullable<T> => {
    if (existing !== undefined && existing[field] !== null) {
      return existing[field];
    }
    return defaults[field];
  };

  const [name, setName] = useState(getInitial<typeof defaults.name>("name", defaults, existing));
  const [handle, setHandle] = useState(getInitial<typeof defaults.handle>("handle", defaults, existing));

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
      dispatch(joinGame(stateToStagingJoinGameThing()));
    }
  };

  return (
    <div>
      <form>
        <label>
          <b>Join Game -->&emsp;</b>
          Name:
          <input type="text" placeholder="Your name" value={name} onChange={e => setName(e.target.value)} />
        </label>
        <label>
          Handle:
          <input type="text" placeholder="Game handle" value={handle} onChange={e => setHandle(e.target.value)} />
        </label>
        <input type="button" value="Submit" onClick={onSubmit} />
      </form>
    </div>
  );
};

export default JoinGameForm;
