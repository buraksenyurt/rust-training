const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bundle.js",
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin({
            patterns: [{from: "index.html", to: "index.html"}],
        }),
    ],
    devServer: {
        static: "./dist",
        port: 5555,
        open: true,
    },
    experiments: {
        asyncWebAssembly: true,
    },
};
