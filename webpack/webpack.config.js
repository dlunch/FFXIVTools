const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlLoader = require("./html-loader");

const root = path.resolve(__dirname, "..");
const dist = path.resolve(root, "dist");

module.exports = {
  context: root,
  mode: "development",
  entry: {
    model_viewer: "pages/model_viewer/html/model_viewer.html",
    translation_compare:
      "pages/translation_compare/html/translation_compare.html"
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
      crateDirectory: path.resolve(root, "pages/model_viewer"),
      outDir: path.resolve(root, "pages/model_viewer/pkg")
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(root, "pages/translation_compare"),
      outDir: path.resolve(root, "pages/translation_compare/pkg")
    })
  ]
};
