const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlLoader = require("./html-loader");

const dist = path.resolve(__dirname, "../../dist");

module.exports = {
  context: path.resolve(__dirname, "../"),
  mode: "development",
  entry: {
    index: "html/index.html"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  module: {
    rules: [
      {
        test: /\.(html)$/,
        use: [
          {
            loader: "html-loader",
            options: {
              minimize: true
            }
          }
        ]
      }
    ]
  },
  resolve: {
    modules: ["."]
  },
  resolveLoader: {
    modules: ["node_modules", "webpack"]
  },
  devServer: {
    contentBase: dist
  },
  plugins: [
    new HtmlLoader.EntryExtractPlugin(),
    new CopyPlugin([path.resolve(__dirname, "../static")]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
      outDir: path.resolve(__dirname, "../pkg")
    })
  ]
};
