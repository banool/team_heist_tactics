const common = require("./webpack.common.js");
const { merge } = require('webpack-merge');

module.exports = merge(common, {
  mode: "production",
  output: {
    filename: "static/app.[hash].js"
  }
});
