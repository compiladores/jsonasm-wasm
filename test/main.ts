export async function run(text: any) {
  const compilerProcess = Deno.run({
    cmd: ["cargo", "run"],
    stdin: "piped",
    stdout: "piped"
  })
  await compilerProcess.stdin?.write(new TextEncoder().encode(JSON.stringify(text)));
  await compilerProcess.stdin.close();
  const output = await compilerProcess.output();
  const compiled = new TextDecoder().decode(output);
  await compilerProcess.close();

  await Deno.writeTextFile("test.wat", compiled)
  const wasmCompileProcess = Deno.run({
    cmd: ["wat2wasm", "test.wat"],
  });
  await wasmCompileProcess.status();
  await wasmCompileProcess.close();

  const f = await Deno.open("./test.wasm")
  const buf = await Deno.readAll(f);
  const wasmModule = new WebAssembly.Module(buf);
  const wasmInstance = new WebAssembly.Instance(wasmModule);

  const rmProcess = Deno.run({
    cmd: ["rm", "test.wat", "test.wasm"],
  });
  await rmProcess.status();
  await rmProcess.close();
}