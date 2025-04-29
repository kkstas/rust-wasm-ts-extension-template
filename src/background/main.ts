import init, {
  example_store_fn,
  example_chrome_tabs_fn,
} from '../../rust-wasm/target/web/rust_wasm';

async function loadWasm() {
  try {
    const wasmUrl = chrome.runtime.getURL('wasm/rust_wasm_bg.wasm');
    await init(wasmUrl);
    console.log('WASM loaded successfully');
  } catch (err) {
    console.error('WASM initialization failed:', err);
  }
}
loadWasm();

chrome.runtime.onMessage.addListener(
  (
    message: object,
    sender: chrome.runtime.MessageSender,
    sendResponse: (response: object) => void,
  ) => {
    (async () => {
      const received = await example_store_fn('SOME_KEY', message);
      console.log(`received from rust:`, received);
      const res = await example_chrome_tabs_fn();
      console.log(res);
    })();

    return true;
  },
);
