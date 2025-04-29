use chrome::{
    storage::local::ChromeStorageLocal,
    tabs::{ChromeTabs, ChromeTabsQueryInput},
};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

mod chrome;

#[macro_use]
mod util;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn example_store_fn(key: JsValue, data: JsValue) -> Promise {
    let store = ChromeStorageLocal::new();

    future_to_promise(async move {
        let key_str = match key.as_string() {
            Some(k) => k,
            None => return Err(JsValue::from_str("Key must be a string")),
        };
        store.set(&key_str, data).await?;
        store.get(&key_str).await
    })
}

#[wasm_bindgen]
pub fn example_chrome_tabs_fn() -> Promise {
    let chrome_tabs = ChromeTabs::new();

    future_to_promise(async move {
        let found_tabs = chrome_tabs
            .query(ChromeTabsQueryInput {
                index: Some(3),
                ..Default::default()
            })
            .await?;
        log!("{:?}", found_tabs);

        let found_tabs = chrome_tabs.query_all().await?;

        if let Some(id) = found_tabs.last().and_then(|tab| tab.id) {
            chrome_tabs
                .update(
                    id,
                    chrome::tabs::TabUpdateProperties {
                        pinned: Some(false),
                        ..Default::default()
                    },
                )
                .await?;
        }

        chrome_tabs
            .update_active(chrome::tabs::TabUpdateProperties {
                muted: Some(true),
                ..Default::default()
            })
            .await?;

        Ok(JsValue::undefined())
    })
}
