
function main() {
  if (typeof SharedArrayBuffer !== 'function') {
    alert('this browser does not have SharedArrayBuffer support enabled' + '\n\n');
    return;
  }
  // Test for bulk memory operations with passive data segments
  //  (module (memory 1) (data passive ""))
  const buf = new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    0x05, 0x03, 0x01, 0x00, 0x01, 0x0b, 0x03, 0x01, 0x01, 0x00]);
  if (!WebAssembly.validate(buf)) {
    alert('this browser does not support passive wasm memory, demo does not work' + '\n\n');
    return;
  }
  wasm_bindgen().then(onWasmLoad).catch((e) => {
    console.error('WASM init error:', e);
  });
}

main();

const {
  wasm_mutex_test_init,
  main_test,
} = wasm_bindgen;

async function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function onWasmLoad() {
  console.log('WASM Loaded');
  await wasm_mutex_test_init();
  await sleep(100);
  await main_test();
}
