import init, {
  storage_get_from_rust,
  storage_set_from_rust,
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
      await storage_set_from_rust('SOME_KEY', message);
      const received = await storage_get_from_rust('SOME_KEY');
      console.log(`received from rust:`, received);
    })();

    return true;
  },
);
