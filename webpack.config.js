const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  mode: "development",
  entry: {
    index: "./js/index.js",
  },
  devServer: {
    static: {
      directory: path.resolve(__dirname, "dist"),
    },
  },
  devtool: "eval-cheap-module-source-map",
  performance: {
    hints: false,
  },
  experiments: {
    // Support the old WebAssembly like in webpack 4.
    syncWebAssembly: true,
    // Support the new WebAssembly according to the updated specification (https://github.com/WebAssembly/esm-integration), it makes a WebAssembly module an async module.
    // asyncWebAssembly: true,
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        path.resolve(__dirname, "static"),
      ],
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "."),
    }),
  ],
};
