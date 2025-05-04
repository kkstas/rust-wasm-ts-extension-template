use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["chrome", "tabs"], js_name = query)]
    async fn js_query(query_info: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "tabs"], js_name = update)]
    async fn js_update(tab_id: i32, update_properties: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "tabs"], js_name = update )]
    async fn js_update_active(update_properties: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "tabs"], js_name = sendMessage )]
    async fn js_send_message(tab_id: i32, message: JsValue) -> Result<JsValue, JsValue>;
}

pub async fn update(tab_id: i32, update_properties: TabUpdateProperties) -> Result<(), JsValue> {
    let update_properties = serde_wasm_bindgen::to_value(&update_properties)?;
    js_update(tab_id, update_properties).await?;
    Ok(())
}

pub async fn update_active(update_properties: TabUpdateProperties) -> Result<(), JsValue> {
    let input = serde_wasm_bindgen::to_value(&update_properties)?;
    js_update_active(input).await?;
    Ok(())
}

pub async fn query(query_info: ChromeTabsQueryInput) -> Result<Vec<Tab>, JsValue> {
    let input = serde_wasm_bindgen::to_value(&query_info)?;
    let result = js_query(input).await?;
    let tabs = serde_wasm_bindgen::from_value::<Vec<Tab>>(result)?;

    Ok(tabs)
}

pub async fn get_active() -> Result<Option<Tab>, JsValue> {
    let query_info = ChromeTabsQueryInput {
        active: Some(true),
        last_focused_window: Some(true),
        ..Default::default()
    };
    let query_info = serde_wasm_bindgen::to_value(&query_info)?;
    let result = js_query(query_info).await?;
    let mut tabs = serde_wasm_bindgen::from_value::<Vec<Tab>>(result)?;
    if tabs.len() == 0 {
        return Ok(None);
    }
    Ok(Some(tabs.swap_remove(0)))
}

pub async fn query_all() -> Result<Vec<Tab>, JsValue> {
    let result = js_query(js_sys::Object::new().into()).await?;
    let tabs = serde_wasm_bindgen::from_value::<Vec<Tab>>(result)?;
    Ok(tabs)
}

pub async fn send_message<T: Serialize>(tab_id: i32, message: &T) -> Result<JsValue, JsValue> {
    let message = serde_wasm_bindgen::to_value(message)?;
    js_send_message(tab_id, message).await
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabUpdateProperties {
    pub pinned: Option<bool>,
    pub opener_tab_id: Option<i32>,
    pub url: Option<String>,
    pub highlighted: Option<bool>,
    pub active: Option<bool>,
    pub selected: Option<bool>,
    pub muted: Option<bool>,
    pub auto_discardable: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    pub active: bool,
    pub audible: Option<bool>,
    pub auto_discardable: bool,
    pub discarded: bool,
    pub fav_icon_url: Option<String>,
    pub frozen: bool,
    pub group_id: i32,
    pub height: Option<i32>,
    pub highlighted: bool,
    pub id: Option<i32>,
    pub incognito: bool,
    pub index: i32,
    pub last_accessed: Option<f64>,
    pub muted_info: Option<TabMutedInfo>,
    pub opener_tab_id: Option<i32>,
    pub pending_url: Option<String>,
    pub pinned: bool,
    pub selected: bool,
    pub session_id: Option<String>,
    pub status: Option<TabStatusEnum>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub width: Option<i32>,
    pub window_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TabStatusEnum {
    Unloaded,
    Loading,
    Complete,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabMutedInfo {
    pub extension_id: Option<String>,
    pub muted: bool,
    pub reason: Option<TabMutedReason>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TabMutedReason {
    User,
    Capture,
    Extension,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChromeTabsQueryInput {
    pub active: Option<bool>,
    pub audible: Option<bool>,
    pub auto_discardable: Option<bool>,
    pub current_window: Option<bool>,
    pub discarded: Option<bool>,
    pub frozen: Option<bool>,
    pub group_id: Option<i32>,
    pub highlighted: Option<bool>,
    pub index: Option<i32>,
    pub last_focused_window: Option<bool>,
    pub muted: Option<bool>,
    pub pinned: Option<bool>,
    pub status: Option<ChromeTabsQueryInputStatusEnum>,
    pub title: Option<String>,
    pub url: Option<Vec<String>>,
    pub window_id: Option<i32>,
    pub window_type: Option<WindowType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChromeTabsQueryInputStatusEnum {
    Loading,
    Complete,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindowType {
    Normal,
    Popup,
    Panel,
    App,
    Devtools,
}
