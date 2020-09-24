const common = require("./webpack.common.js");
const path = require("path");
const { merge } = require('webpack-merge');

module.exports = merge(common, {
  devtool: "eval-source-map",
  mode: "development"
});
