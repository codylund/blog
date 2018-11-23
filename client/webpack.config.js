const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: "./app/js/index.js",
    output: {
        path: dist,
        filename: "bundle.js",
    },
    devServer: {
        contentBase: "./app/",
    },
    plugins: [
        new HtmlWebpackPlugin({
          template: './app/index.html'
        }),
    
        new WasmPackPlugin({
          crateDirectory: path.resolve(__dirname, "app/wasm")
        }),
    ]
};