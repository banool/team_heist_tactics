import React, { useEffect, useState } from "react";

import { timerRunsOutSelector } from "./slice";
import { useSelector } from "react-redux";

const TimerComponent = () => {
  const timer_runs_out = useSelector(timerRunsOutSelector);

  // useEffect(() => {
  //   // We set recalculateTimer to run every second, but it doesn't actually
  //   // count down a timer, it just recalculates how much time is left based
  //   // on the absolute unix time representing the end of the timer.
  //   let interval = setInterval(recalculateTimer, 1000);
  //   return function cleanup() {
  //     window.clearInterval(interval);
  //   };
  // });

  const secondsLeft = () => {
    return timer_runs_out - Math.floor(Date.now() / 1000);
  };

  // const [seconds_left, set_seconds_left] = useState(secondsLeft());

  // const recalculateTimer = () => {
  //   set_seconds_left(secondsLeft());
  // };

  // const seconds_left_string = seconds_left ? seconds_left : "Not started yet!";

  const kelly_string =
    timer_runs_out > Date.now() + 600 ? "Not started yet!" : secondsLeft();

  return (
    <div>
      <p>Seconds left: {kelly_string} </p>
    </div>
  );
};

export default TimerComponent;
