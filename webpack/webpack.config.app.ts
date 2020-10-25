import * as path from 'path';
import * as webpack from 'webpack';
import { merge } from 'webpack-merge';
import * as CopyPlugin from 'copy-webpack-plugin';
import TsconfigPathsPlugin from 'tsconfig-paths-webpack-plugin';
import * as RawGeneratePackageJsonPlugin from 'generate-package-json-webpack-plugin';

const GeneratePackageJsonPlugin = RawGeneratePackageJsonPlugin as unknown as new (basePackageValues: Record<string, unknown>) => webpack.WebpackPluginInstance;

const root = path.resolve(__dirname, '..');

const config: webpack.Configuration = {
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
    modules: ['.', 'node_modules'],
  },
  resolveLoader: {
    modules: ['node_modules', 'webpack'],
  },
};

const packageBase = {
  author: 'Inseok Lee <dlunch@gmail.com>',
  name: 'ffxiv-tools',
  version: '0.1.0',
  main: 'main.js',
};

export default [
  merge(config,
    {
      target: 'electron-main',
      entry: { main: 'app/main.ts' },
      plugins: [
        new GeneratePackageJsonPlugin(packageBase),
      ],
    }),
  merge(config,
    {
      target: 'electron-preload',
      entry: { preload: 'app/preload.ts' },
    }),
  merge(config,
    {
      target: 'electron-renderer',
      entry: { renderer: 'app/renderer.ts' },
      plugins: [
        new CopyPlugin({
          patterns: [{ from: 'app/index.html' }],
        }),
      ],
    }),
];
