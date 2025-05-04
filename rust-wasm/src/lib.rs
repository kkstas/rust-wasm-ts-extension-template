use chrome::tabs::ChromeTabsQueryInput;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

mod chrome;

#[macro_use]
mod util;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn test_storage_set_fn(key: String, value: JsValue) -> Promise {
    future_to_promise(async move {
        chrome::storage::local::set(&key, value).await?;

        Ok(JsValue::undefined())
    })
}

#[wasm_bindgen]
pub fn test_storage_get_fn(key: String) -> Promise {
    future_to_promise(async move { chrome::storage::local::get(&key).await })
}

#[wasm_bindgen]
pub fn test_tabs_query_all_fn() -> Promise {
    future_to_promise(async {
        let found_tabs = chrome::tabs::query_all().await?;
        let found_tabs = found_tabs
            .into_iter()
            .map(|el| serde_wasm_bindgen::to_value(&el).unwrap())
            .collect::<js_sys::Array>();

        Ok(found_tabs.into())
    })
}

#[wasm_bindgen]
pub fn test_tabs_query_fn(query_info: JsValue) -> Promise {
    future_to_promise(async {
        let query_info = serde_wasm_bindgen::from_value::<ChromeTabsQueryInput>(query_info)?;
        let found_tabs = chrome::tabs::query(query_info).await?;
        let found_tabs = found_tabs
            .into_iter()
            .map(|el| serde_wasm_bindgen::to_value(&el).unwrap())
            .collect::<js_sys::Array>();

        Ok(found_tabs.into())
    })
}

#[wasm_bindgen]
pub fn test_tabs_get_active_fn() -> Promise {
    future_to_promise(async {
        let query_result = chrome::tabs::get_active().await?;
        match query_result {
            Some(tab) => Ok(serde_wasm_bindgen::to_value(&tab)?),
            None => Ok(JsValue::null()),
        }
    })
}

#[wasm_bindgen]
pub fn test_tabs_update_fn(tab_id: i32, update_properties: JsValue) -> Promise {
    future_to_promise(async move {
        let update_properties = serde_wasm_bindgen::from_value(update_properties)?;
        chrome::tabs::update(tab_id, update_properties).await?;
        Ok(JsValue::undefined())
    })
}

#[wasm_bindgen]
pub fn test_tabs_update_active_fn(update_properties: JsValue) -> Promise {
    future_to_promise(async move {
        let update_properties = serde_wasm_bindgen::from_value(update_properties)?;
        chrome::tabs::update_active(update_properties).await?;
        Ok(JsValue::undefined())
    })
}

#[derive(Serialize, Deserialize)]
struct Message {
    pub msg: String,
}
#[wasm_bindgen]
pub fn test_tabs_send_message_fn(tab_id: i32, message: JsValue) -> Promise {
    future_to_promise(async move {
        let message = serde_wasm_bindgen::from_value::<Message>(message)?;
        chrome::tabs::send_message(tab_id, &message).await
    })
}
