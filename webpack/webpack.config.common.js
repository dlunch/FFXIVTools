const path = require('path');
const webpack = require('webpack');
const CopyPlugin = require('copy-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const TsconfigPathsPlugin = require('tsconfig-paths-webpack-plugin');
const HtmlEntryLoader = require('html-entry-loader');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const root = path.resolve(__dirname, '..');
const dist = path.resolve(root, 'client/dist');

const configuration = {
  context: root,
  entry: {
    model_viewer: 'client/model_viewer/model_viewer.html',
    translation_compare: 'client/translation_compare/translation_compare.html',
    explorer: 'client/explorer/explorer.html',
  },
  experiments: {
    asyncWebAssembly: true,
  },
  output: {
    path: dist,
    filename: '[name].js',
  },
  module: {
    rules: [
      {
        test: /\.(html)$/,
        use: [
          {
            loader: 'html-entry-loader',
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
            loader: 'ts-loader',
            options: {
              onlyCompileBundledFiles: true,
              compilerOptions: {
                module: 'esnext',
              },
            },
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: ['.ts', '.js'],

    plugins: [new TsconfigPathsPlugin()],
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new HtmlEntryLoader.EntryExtractPlugin(),
    new webpack.DefinePlugin({
      'process.env.IS_LOCALHOST': JSON.stringify(process.env.IS_LOCALHOST),
    }),

    new WasmPackPlugin({
      crateDirectory: path.resolve(root, 'client/model_viewer'),
      outDir: path.resolve(root, 'client/model_viewer/pkg'),
      outName: 'index',
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(root, 'client/translation_compare'),
      outDir: path.resolve(root, 'client/translation_compare/pkg'),
      outName: 'index',
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(root, 'client/explorer'),
      outDir: path.resolve(root, 'client/explorer/pkg'),
      outName: 'index',
    }),
    new CopyPlugin({
      patterns: [{ from: 'client/index.html' }],
    }),
    new CleanWebpackPlugin(),
  ],
};

module.exports = configuration;
