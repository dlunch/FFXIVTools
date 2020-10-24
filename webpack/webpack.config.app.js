const path = require("path");
const { merge } = require("webpack-merge");
const CopyPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const TsconfigPathsPlugin = require("tsconfig-paths-webpack-plugin");

const root = path.resolve(__dirname, "..");

const config = {
  mode: "development",
  devtool: "source-map",

  context: root,
  output: {
    path: path.resolve(root, "app/dist"),
    filename: "[name].js",
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: [
          {
            loader: "ts-loader",
            options: {
              onlyCompileBundledFiles: true
            }
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".js"],
    plugins: [new TsconfigPathsPlugin()],
    modules: [".", "node_modules"],
  },
  resolveLoader: {
    modules: ["node_modules", "webpack"],
  },
};

module.exports = [
  merge(config,
    {
      target: 'electron-main',
      entry: { 'index': 'app/main.ts' },
      node: false,
      plugins: [
        new CleanWebpackPlugin(),
      ],
    }
  ),
  merge(config,
    {
      target: 'electron-preload',
      entry: { 'preload': 'app/preload.ts' }
    }
  ),
  merge(config,
    {
      target: 'electron-renderer',
      entry: { 'renderer': 'app/renderer.ts' },
      plugins: [
        new CopyPlugin({
          patterns: [{ from: "app/index.html" }],
        }),
      ]
    }
  ),
]
