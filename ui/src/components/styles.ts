import * as colors from "../constants/colors";
import { CANVAS_WIDTH, CANVAS_HEIGHT } from "../constants/other";

const styles: { [key: string]: React.CSSProperties } = {
  root: {
    display: "flex",
    flexDirection: "row",
    justifyContent: "center"
  },
  content: {
    marginTop: 20,
    width: 1600,
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
  //https://css-tricks.com/snippets/css/a-guide-to-flexbox
  gameWindowComponent: {
    width: CANVAS_WIDTH,
    height: CANVAS_HEIGHT,
    backgroundColor: "#ffffff",
    position: "relative",
  },
  // Use this to make the overlay appear on top of the main game component window.
  gameWindowComponentWrapper: {
    position: "absolute",
    left: 0,
    right: 0,
    width: "100%",
    height: "100%",
  },
  gameWindowOverlay: {
    display: "flex",
    flexDirection: "row",
    justifyContent: "space-between",
    zIndex: 2,
  },
  resetMapComponent: {
    border: "1px solid black",
  },
};

export default styles;
