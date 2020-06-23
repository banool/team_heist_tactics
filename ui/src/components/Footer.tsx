import React from "react";

const styles: { [key: string]: React.CSSProperties } = {
  root: {
    marginTop: 5,
    textAlign: "right",
    fontSize: 14,
    opacity: 0.8,
  },
};
export default (props) => (
  <div style={styles.root}>
    {"made with love by fatema, kelly, and daniel (c) 2020"}
  </div>
);
