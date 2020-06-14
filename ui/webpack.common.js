const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  entry: "./src/index.tsx",
  output: {
    library: "App",
    filename: "static/app.js",
    path: path.resolve(__dirname, "dist"),
    pathinfo: true
  },
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        exclude: /(node_modules)/,
        use: {
          loader: "babel-loader",
          options: {
            plugins: ["@babel/plugin-transform-react-jsx"]
          }
        }
      },
      {
        test: /\.tsx?$/,
        exclude: /(node_modules)/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ["@babel/typescript"],
            plugins: [
              "@babel/plugin-transform-react-jsx",
              "@babel/proposal-class-properties",
              "@babel/proposal-object-rest-spread"
            ]
          }
        }
      }
    ]
  },
  resolve: {
    modules: [path.resolve(__dirname, "node_modules")],
    alias: {
      "react-dom": "@hot-loader/react-dom"
    },
    extensions: [".js", ".jsx", ".ts", ".tsx", ".d.ts"]
  },
  plugins: [
    new HtmlWebpackPlugin({
      filename: "index.html",
      template: path.resolve(__dirname, "templates/index.html"),
      base: "/",
      minify: false
    })
  ]
};
