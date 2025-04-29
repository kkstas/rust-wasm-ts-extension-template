use chrome::{
    storage::local::ChromeStorageLocal,
    tabs::{ChromeTabs, ChromeTabsQueryInput},
};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

mod chrome;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn storage_get_from_rust(key: JsValue) -> Promise {
    let store = ChromeStorageLocal::new();
    future_to_promise(async move {
        let key_str = match key.as_string() {
            Some(k) => k,
            None => return Err(JsValue::from_str("Key must be a string")),
        };
        store.get(&key_str).await
    })
}

#[wasm_bindgen]
pub fn storage_set_from_rust(key: JsValue, data: JsValue) -> Promise {
    let store = ChromeStorageLocal::new();
    future_to_promise(async move {
        let key_str = match key.as_string() {
            Some(k) => k,
            None => return Err(JsValue::from_str("Key must be a string")),
        };
        store.set(&key_str, data).await?;
        Ok(JsValue::UNDEFINED)
    })
}

#[wasm_bindgen]
pub fn query_tab_test() -> Promise {
    future_to_promise(async {
        let chrome_tabs = ChromeTabs::new();

        let _found_tabs = chrome_tabs.query_all().await?;
        let found_tab = chrome_tabs
            .query(ChromeTabsQueryInput {
                index: Some(3),
                ..Default::default()
            })
            .await;
        web_sys::console::log_1(
            &serde_wasm_bindgen::to_value(&format!("{:?}", found_tab)).unwrap(),
        );

        Ok(JsValue::undefined())
    })
}

#[wasm_bindgen]
pub fn update_tab_test() -> Promise {
    future_to_promise(async {
        let chrome_tabs = ChromeTabs::new();

        let result = chrome_tabs
            .update_by_id(
                1316221559,
                chrome::tabs::TabUpdateProperties {
                    pinned: Some(true),
                    ..Default::default()
                },
            )
            .await;

        if let Err(e) = result {
            web_sys::console::log_1(&e);
        }

        let result = chrome_tabs
            .update_active(chrome::tabs::TabUpdateProperties {
                muted: Some(true),
                ..Default::default()
            })
            .await;

        if let Err(e) = result {
            web_sys::console::log_1(&e);
        }

        Ok(JsValue::UNDEFINED)
    })
}
