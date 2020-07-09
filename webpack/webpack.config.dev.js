const webpack = require("webpack");
const merge = require("webpack-merge");
const common = require("./webpack.config.common.js");

module.exports = merge.smart(common, {
  mode: "development",
  devtool: "source-map",
  devServer: {
    contentBase: "./dist",
    hot: true,
  },
  plugins: [new webpack.HotModuleReplacementPlugin()],
});
