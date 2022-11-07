const fs = require("fs");

(async function main() {
  const module = await WebAssembly.instantiate(fs.readFileSync("./test.wasm"));
  module.instance.exports["#main"]();
  console.log(module.instance.exports.out.value);
})();