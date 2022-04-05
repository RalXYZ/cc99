const { override, addWebpackModuleRule } = require("customize-cra");
const path = require("path");
const wasmExtensionRegExp = /\.wasm$/;

module.exports = override(
  // make the file loader ignore wasm files
  (config) => {
    config.module.rules.find((rule) => {
      return (rule.oneOf || []).find((item) => {
        if (item.loader && item.loader.indexOf("file-loader") >= 0) {
          item.exclude.push(wasmExtensionRegExp); //exclude wasm
          return true; //ignore remaining rules
        }
      });
    });

    return config;
  },

  addWebpackModuleRule({
    test: wasmExtensionRegExp,
    include: path.resolve(__dirname, "src"),
    use: [{ loader: require.resolve("wasm-loader"), options: {} }],
  })

  // //hook up our helloHelper wasm module
  // config => {
  //     config.plugins = (config.plugins || []).concat([
  //         new WasmPackPlugin({
  //             crateDirectory: path.resolve(__dirname, "./helloHelper"),
  //             extraArgs: "--no-typescript",
  //             outDir: path.resolve(__dirname, "./src/helloHelperWasm")
  //         })
  //     ]);
  //
  //     return config;
  // }
);
