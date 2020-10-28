import * as MiniCssExtractPlugin from 'mini-css-extract-plugin';
import * as webpack from 'webpack';

import merge from 'webpack-merge';
import common from './webpack.config.common';

export default merge(common, {
  mode: 'production',
  devtool: undefined,
  module: {
    rules: [
      {
        test: /\.less$/,
        use: [
          {
            loader: MiniCssExtractPlugin.loader,
          },
          {
            loader: 'css-loader',
            options: {
              esModule: false,
            },
          },
          {
            loader: 'less-loader',
          },
        ],
      },
    ],
  },
  plugins: [new MiniCssExtractPlugin() as webpack.WebpackPluginInstance],
});
