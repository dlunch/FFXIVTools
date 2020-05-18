const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const TsconfigPathsPlugin = require("tsconfig-paths-webpack-plugin");
const HtmlLoader = require("./html-loader");

const root = path.resolve(__dirname, "..");
const dist = path.resolve(root, "dist");

module.exports = {
  context: root,
  entry: {
    model_viewer: "web/model_viewer/model_viewer.html",
    translation_compare: "web/translation_compare/translation_compare.html",
  },
  output: {
    path: dist,
    filename: "[name].js",
  },
  module: {
    rules: [
      {
        test: /\.(html)$/,
        use: [
          {
            loader: "html-loader",
            options: {
              minimize: true,
            },
          },
        ],
      },
      {
        test: /\.ts$/,
        use: [
          {
            loader: "ts-loader",
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".js"],
    plugins: [new TsconfigPathsPlugin()],
    modules: ["."],
  },
  resolveLoader: {
    modules: ["node_modules", "webpack"],
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new HtmlLoader.EntryExtractPlugin(),

    new WasmPackPlugin({
      crateDirectory: path.resolve(root, "web/model_viewer"),
      outDir: path.resolve(root, "web/model_viewer/pkg"),
      outName: "index",
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(root, "web/translation_compare"),
      outDir: path.resolve(root, "web/translation_compare/pkg"),
      outName: "index",
    }),
    new CopyPlugin({
      patterns: [
        { from: "web/index.html" }
      ]
    }),
    new CleanWebpackPlugin(),
  ],
};
