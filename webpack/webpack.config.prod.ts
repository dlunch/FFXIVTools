import merge from 'webpack-merge';
import common from './webpack.config.common';

export default merge(common, {
  mode: 'production',
  devtool: undefined,
});
