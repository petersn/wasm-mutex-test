importScripts('pkg/wasm_mutex_test.js');

// Wait for the main thread to send us the shared module/memory. Once we've got
// it, initialize it all with the `wasm_bindgen` global we imported via
// `importScripts`.
//
// After our first message all subsequent messages are an entry point to run,
// so we just do that.
self.onmessage = (event) => {
  let initialised = wasm_bindgen(...event.data).catch((err) => {
    // Propagate to main `onerror`:
    setTimeout(() => {
      throw err;
    });
    // Rethrow to keep promise rejected and prevent execution of further commands:
    throw err;
  });

  self.onmessage = async (event) => {
    // This will queue further commands up until the module is fully initialised:
    await initialised;
    wasm_bindgen.thread_entry_point(event.data);
  };
};
