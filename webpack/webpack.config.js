const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlLoader = require("./html-loader");

const root = path.resolve(__dirname, "..");
const dist = path.resolve(root, "dist");

module.exports = {
  context: root,
  mode: "development",
  entry: {
    model_viewer: "apps/model_viewer/html/model_viewer.html"
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

    new WasmPackPlugin({
      crateDirectory: "apps/model_viewer",
      outDir: "apps/model_viewer/pkg"
    })
  ]
};
