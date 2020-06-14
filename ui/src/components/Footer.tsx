import React from "react";

const styles: { [key: string]: React.CSSProperties } = {
  root: {
    marginTop: 5,
    textAlign: "right",
    fontSize: 14,
    opacity: 0.8
  }
};
export default props => (
  <div style={styles.root}>{"made with <3 in amaranta house by max and dan (c) 1987"}</div>
);
