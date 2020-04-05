const merge = require("webpack-merge");
const common = require("./webpack.config.common.js");

module.exports = merge.smart(common, {
  mode: "production",
  devtool: "",
});
