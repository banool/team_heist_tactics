import React, { useEffect, useState } from "react";
import { gameStatusSelector, timerRunsOutSelector } from "./slice";

import { GameStatus } from "../generated/types_pb";
import { useSelector } from "react-redux";

const TimerComponent = () => {
  const timer_runs_out = useSelector(timerRunsOutSelector);
  const game_status = useSelector(gameStatusSelector);

  const [seconds_left, set_seconds_left] = useState(300);

  useEffect(() => {
    // We set recalculateTimer to run every second, but it doesn't actually
    // count down a timer, it just recalculates how much time is left based
    // on the absolute unix time representing the end of the timer.
    let interval = setInterval(recalculateTimer, 1000);
    return function cleanup() {
      window.clearInterval(interval);
    };
  });

  const recalculateTimer = () => {
    if (timer_runs_out === 0) {
      return;
    }
    set_seconds_left(timer_runs_out - Math.floor(Date.now() / 1000));
  };

  const getMessage = (): string => {
    let defeat_message = "Defeat! You ran out of time!";
    // We do this check because when the game enters a terminal state,
    // it isn't guaranteed that the server will push this state to
    // the client right now.
    if (seconds_left < 0) {
      return defeat_message;
    }
    switch (+game_status) {
      case GameStatus.STAGING:
        throw "Shouldn't see this component while STAGING";
      case GameStatus.PRE_FIRST_MOVE:
        return "Waiting to make first move";
      case GameStatus.ONGOING:
        return `Seconds left: ${seconds_left}`;
      case GameStatus.VICTORY:
        return "Victory! You made it out with ${seconds_left} to spare, congrats!";
      case GameStatus.DEFEAT:
        return defeat_message;
    }
    throw "Should not be able to get here";
  };

  return (
    <div>
      <p>{getMessage()}</p>
    </div>
  );
};

export default TimerComponent;
