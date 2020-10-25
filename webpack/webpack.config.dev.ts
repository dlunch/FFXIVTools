import * as webpack from 'webpack';
import merge from 'webpack-merge';
import common from './webpack.config.common';

export default merge(common, {
  mode: 'development',
  devtool: 'source-map',
  devServer: {
    contentBase: './dist',
    hot: true,
  },
  plugins: [new webpack.HotModuleReplacementPlugin()],
});
