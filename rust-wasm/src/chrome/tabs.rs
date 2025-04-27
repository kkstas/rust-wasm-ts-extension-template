use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub struct ChromeTabs {
    update_fn: RefCell<Option<js_sys::Function>>,
    query_fn: RefCell<Option<js_sys::Function>>,
    tabs: JsValue,
}

impl ChromeTabs {
    pub fn new() -> Self {
        let chrome = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("chrome"))
            .expect("Failed to get chrome object from global");
        let tabs = js_sys::Reflect::get(&chrome, &JsValue::from_str("tabs"))
            .expect("Failed to get chrome.tabs");
        Self {
            tabs,
            update_fn: RefCell::new(None),
            query_fn: RefCell::new(None),
        }
    }

    pub async fn update_by_id(
        &self,
        tab_id: i64,
        update_properties: TabUpdateProperties,
    ) -> Result<(), JsValue> {
        self.init_update_fn()?;

        let promise = self.update_fn.borrow().as_ref().unwrap().call2(
            &self.tabs,
            &serde_wasm_bindgen::to_value(&tab_id)?,
            &serde_wasm_bindgen::to_value(&update_properties)?,
        )?;

        let promise = js_sys::Promise::resolve(&promise);
        wasm_bindgen_futures::JsFuture::from(promise).await?;
        Ok(())
    }

    pub async fn update_active(
        &self,
        update_properties: TabUpdateProperties,
    ) -> Result<(), JsValue> {
        self.init_update_fn()?;

        let promise = self.update_fn.borrow().as_ref().unwrap().call1(
            &self.tabs,
            &serde_wasm_bindgen::to_value(&update_properties)?,
        )?;

        let promise = js_sys::Promise::resolve(&promise);
        wasm_bindgen_futures::JsFuture::from(promise).await?;
        Ok(())
    }

    fn init_update_fn(&self) -> Result<(), JsValue> {
        let mut update_fn = self.update_fn.borrow_mut();
        if update_fn.is_none() {
            *update_fn = Some(
                js_sys::Reflect::get(&self.tabs, &JsValue::from_str("update"))?
                    .dyn_into::<js_sys::Function>()?,
            );
        }
        Ok(())
    }

    pub async fn query_all(&self) -> Result<Vec<Tab>, JsValue> {
        self.init_query_fn()?;

        let promise = self
            .query_fn
            .borrow()
            .as_ref()
            .unwrap()
            .call1(&self.tabs, &js_sys::Object::new())?;
        let promise = js_sys::Promise::resolve(&promise);
        let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
        let tabs = serde_wasm_bindgen::from_value::<Vec<Tab>>(result)?;
        Ok(tabs)
    }

    pub async fn query(&self, query_info: ChromeTabsQueryInput) -> Result<Vec<Tab>, JsValue> {
        self.init_query_fn()?;

        let input = serde_wasm_bindgen::to_value(&query_info)?;
        let promise = self
            .query_fn
            .borrow()
            .as_ref()
            .unwrap()
            .call1(&self.tabs, &input)?;
        let promise = js_sys::Promise::resolve(&promise);
        let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
        let tabs = serde_wasm_bindgen::from_value::<Vec<Tab>>(result)?;
        Ok(tabs)
    }

    fn init_query_fn(&self) -> Result<(), JsValue> {
        let mut query_fn = self.query_fn.borrow_mut();
        if query_fn.is_none() {
            *query_fn = Some(
                js_sys::Reflect::get(&self.tabs, &JsValue::from_str("query"))?
                    .dyn_into::<js_sys::Function>()?,
            );
        }
        Ok(())
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabUpdateProperties {
    pub pinned: Option<bool>,
    pub opener_tab_id: Option<i64>,
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
    pub group_id: i64,
    pub height: Option<i64>,
    pub highlighted: bool,
    pub id: Option<i64>,
    pub incognito: bool,
    pub index: i64,
    pub last_accessed: Option<f64>,
    pub muted_info: Option<TabMutedInfo>,
    pub opener_tab_id: Option<i64>,
    pub pending_url: Option<String>,
    pub pinned: bool,
    pub selected: bool,
    pub session_id: Option<String>,
    pub status: Option<TabStatusEnum>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub width: Option<i64>,
    pub window_id: i64,
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
    pub group_id: Option<i64>,
    pub highlighted: Option<bool>,
    pub index: Option<i64>,
    pub last_focused_window: Option<bool>,
    pub muted: Option<bool>,
    pub pinned: Option<bool>,
    pub status: Option<ChromeTabsQueryInputStatusEnum>,
    pub title: Option<String>,
    pub url: Option<Vec<String>>,
    pub window_id: Option<i64>,
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
