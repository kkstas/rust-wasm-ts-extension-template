import init, {
  test_storage_get_fn,
  test_storage_set_fn,
  test_tabs_get_active_fn,
  test_tabs_query_all_fn,
  test_tabs_update_fn,
  test_tabs_update_active_fn,
  test_tabs_send_message_fn,
  test_tabs_query_fn,
} from '../../rust-wasm/target/web/rust_wasm';

async function loadWasm() {
  try {
    const wasmUrl = chrome.runtime.getURL('wasm/rust_wasm_bg.wasm');
    await init(wasmUrl);
  } catch (err) {
    console.error('WASM initialization failed:', err);
  }
}
loadWasm();

chrome.runtime.onMessage.addListener(
  (
    _message: object,
    _sender: chrome.runtime.MessageSender,
    _sendResponse: (response: object) => void,
  ) => {
    (async () => {
      await test_storage_set_fn('someKey', 'some value');
      console.log(await test_storage_get_fn('someKey'));

      console.log(await test_tabs_query_all_fn());

      console.log(
        await test_tabs_query_fn({ url: ['https://*.github.com/*'] }),
      );

      const activeTab: chrome.tabs.Tab = await test_tabs_get_active_fn();
      console.log('active tab:', activeTab);

      if (activeTab.id === 1316225470) {
        await test_tabs_update_fn(activeTab.id, { muted: false });
        await test_tabs_update_active_fn({ muted: false });
      }

      console.log(
        await test_tabs_send_message_fn(activeTab.id!, { msg: 'some msg' }),
      );
    })();

    return true;
  },
);
