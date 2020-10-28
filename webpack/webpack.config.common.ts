import * as path from 'path';
import * as webpack from 'webpack';
import * as RawWasmPackPlugin from '@wasm-tool/wasm-pack-plugin';
import * as CopyPlugin from 'copy-webpack-plugin';
import { CleanWebpackPlugin } from 'clean-webpack-plugin';
import TsconfigPathsPlugin from 'tsconfig-paths-webpack-plugin';
import * as HtmlEntryLoader from './html-entry-loader';

const WasmPackPlugin = RawWasmPackPlugin as unknown as new (options: RawWasmPackPlugin.WasmPackPluginOptions) => webpack.WebpackPluginInstance;

const root = path.resolve(__dirname, '..');
const dist = path.resolve(root, 'client/dist');

const configuration: webpack.Configuration = {
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
    modules: ['.'],
  },
  resolveLoader: {
    modules: ['node_modules', 'webpack'],
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new HtmlEntryLoader.EntryExtractPlugin(),

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

export default configuration;
