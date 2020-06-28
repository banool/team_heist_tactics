import * as colors from "../constants/colors";

const styles: { [key: string]: React.CSSProperties } = {
  //https://css-tricks.com/snippets/css/a-guide-to-flexbox
  mainPage: {
    fontFamily: "'Crimson Text', serif",
  },
  joinGameForm: {
    position: "absolute",
    //width: 800,
    backgroundColor: colors.formBackground,
    padding: 30,
    boxShadow: `10px 10px ${colors.formShadow}`,
    top: "30%",
    left: "50%",
    transform: "translate(-50%, -50%)",
  },
  gameWindowComponent: {
    width: "100%",
    height: "100%",
    position: "relative",
    margin: "auto",
  },
  gameWindowComponentWrapper: {
    position: "absolute",
    left: 0,
    width: "100%",
    height: "100%",
  },
  resetGameWindowOverlay: {
    zIndex: 4,
    position: "absolute",
    right: 30,
    bottom: 30,
  },
  connectionStatusOverlay: {
    zIndex: 5,
    position: "absolute",
    left: 30,
    bottom: 10,
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
    top: 40,
    color: "black",
    fontSize: 17,
  },
  invalidMessages: {},
  invalidMessagesOverlay: {
    zIndex: 5,
    position: "absolute",
    left: 30,
    bottom: 50,
    height: 120,
  },
  timerOverlay: {
    zIndex: 5,
    position: "relative",
    fontSize: 20,
    textAlign: "center",
  },
};

export default styles;
