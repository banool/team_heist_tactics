import * as colors from "../constants/colors";
import { CANVAS_WIDTH, CANVAS_HEIGHT } from "../constants/other";

const styles: { [key: string]: React.CSSProperties } = {
  //https://css-tricks.com/snippets/css/a-guide-to-flexbox
  root: {
    display: "flex",
    flexDirection: "row",
    justifyContent: "center",
  },
  content: {
    marginTop: 0,
    width: CANVAS_WIDTH,
    backgroundColor: colors.backgroundLight,
  },
  layoutTable: {
    border: "2px solid #333",
  },
  contentCell: {
    border: "2px solid #333",
    width: "100%",
    padding: "5px 10px",
    verticalAlign: "top",
  },
  gameWindowComponent: {
    width: CANVAS_WIDTH,
    height: CANVAS_HEIGHT,
    backgroundColor: colors.background,
    position: "relative",
  },
  gameWindowComponentWrapper: {
    position: "absolute",
    left: 0,
    width: "100%",
    height: "100%",
  },
  resetGameWindowOverlay: {
    zIndex: 2,
    position: "absolute",
    bottom: 20,
    right: 20,
  },
  // CSS for the second canvas we put on top of the first canvas.
  // This makes clicks "pass through" so we can just use it to display stuff.
  // https://stackoverflow.com/questions/1401658/html-overlay-which-allows-clicks-to-fall-through-to-elements-behind-it
  overlayCanvas: {
    pointerEvents: "none",
    height: "0px",
    overflow: "visible",
    background: "none !important",
  },
  keyboardHeisterNumber: {
    zIndex: 2,
    position: "absolute",
    top: 15,
    color: "black",
    fontSize: 18,
  },
};

export default styles;
