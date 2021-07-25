const path = require('path');
const { merge } = require('webpack-merge');
const CopyPlugin = require('copy-webpack-plugin');
const TsconfigPathsPlugin = require('tsconfig-paths-webpack-plugin');
const GeneratePackageJsonPlugin = require('generate-package-json-webpack-plugin');

const root = path.resolve(__dirname, '..');

const config = {
  mode: 'development',
  devtool: 'source-map',

  context: root,
  output: {
    path: path.resolve(root, 'app/dist'),
    filename: '[name].js',
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: [
          {
            loader: 'ts-loader',
            options: {
              onlyCompileBundledFiles: true,
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
};

const packageBase = {
  author: 'Inseok Lee <dlunch@gmail.com>',
  name: 'ffxiv-tools',
  version: '0.1.0',
  main: 'main.js',
};

module.exports = [
  merge(config, {
    target: 'electron-main',
    entry: { main: 'app/electron/main.ts' },
    plugins: [new GeneratePackageJsonPlugin(packageBase)],
  }),
  merge(config, {
    target: 'electron-preload',
    entry: { preload: 'app/electron/preload.ts' },
  }),
  merge(config, {
    target: 'electron-renderer',
    entry: { renderer: 'app/electron/renderer.ts' },
    plugins: [
      new CopyPlugin({
        patterns: [{ from: 'app/electron/index.html' }],
      }),
    ],
  }),
];
