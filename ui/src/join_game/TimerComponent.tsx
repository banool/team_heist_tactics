import React, { useEffect, useState } from "react";

import { timerRunsOutSelector } from "./slice";
import { useSelector } from "react-redux";

const TimerComponent = () => {
  const timer_runs_out = useSelector(timerRunsOutSelector);

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
    set_seconds_left(timer_runs_out - Math.floor(Date.now() / 1000));
  };

  const seconds_left_string = seconds_left ? seconds_left : "Not started yet!";

  return (
    <div>
      <p>Seconds left: {seconds_left_string}</p>
    </div>
  );
};

export default TimerComponent;
