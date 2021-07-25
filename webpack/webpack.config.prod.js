const { merge } = require('webpack-merge');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const common = require('./webpack.config.common');

module.exports = merge(common, {
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
  plugins: [new MiniCssExtractPlugin()],
});
