import * as colors from "../constants/colors";
import { CANVAS_WIDTH, CANVAS_HEIGHT } from "../constants/other";

const styles: { [key: string]: React.CSSProperties } = {
  //https://css-tricks.com/snippets/css/a-guide-to-flexbox
  root: {
    display: "flex",
    flexDirection: "row",
    justifyContent: "center"
  },
  content: {
    marginTop: 0,
    width: CANVAS_WIDTH,
    backgroundColor: colors.backgroundLight
  },
  layoutTable: {
    border: "2px solid #333"
  },
  contentCell: {
    border: "2px solid #333",
    width: "100%",
    padding: "5px 10px",
    verticalAlign: "top"
  },
  gameWindowComponent: {
    width: CANVAS_WIDTH,
    height: CANVAS_HEIGHT,
    backgroundColor: "#ffffff",
    position: "relative"
  },
  gameWindowComponentWrapper: {
    position: "absolute",
    left: 0,
    width: "100%",
    height: "100%"
  },
  resetGameWindowOverlay: {
    zIndex: 2,
    position: "absolute",
    //padding: "20px 20px 20px 20px",
    bottom: 20,
    right: 20,
  },
};

export default styles;
