import React, { useState } from "react";

import store from '../common/store';

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

  // TODO Make this generic again some day lol.
  const getInitial =(
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
  const [handle, setHandle] = useState(getInitial("handle", defaults, existing));

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
