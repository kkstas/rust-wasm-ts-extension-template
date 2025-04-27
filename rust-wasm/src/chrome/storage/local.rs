use std::cell::RefCell;

use wasm_bindgen::prelude::*;

pub struct ChromeStorageLocal {
    local: JsValue,
    get_fn: RefCell<Option<js_sys::Function>>,
    set_fn: RefCell<Option<js_sys::Function>>,
}

impl ChromeStorageLocal {
    pub fn new() -> Self {
        let chrome = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("chrome"))
            .expect("Failed to get chrome object from global");
        let storage = js_sys::Reflect::get(&chrome, &JsValue::from_str("storage"))
            .expect("Failed to get storage object from chrome");
        let local = js_sys::Reflect::get(&storage, &JsValue::from_str("local"))
            .expect("Failed to get local object from storage");

        Self {
            local,
            get_fn: RefCell::new(None),
            set_fn: RefCell::new(None),
        }
    }

    pub async fn get(&self, key: &str) -> Result<JsValue, JsValue> {
        self.init_get_fn()?;

        let promise = self
            .get_fn
            .borrow()
            .as_ref()
            .unwrap()
            .call1(&self.local, &JsValue::from_str(key))?;
        let promise = js_sys::Promise::resolve(&promise);
        let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
        Ok(result)
    }

    pub async fn set<T: Into<JsValue>>(&self, key: &str, value: T) -> Result<JsValue, JsValue> {
        self.init_set_fn()?;

        let items = js_sys::Object::new();
        js_sys::Reflect::set(&items, &JsValue::from_str(key), &value.into())?;

        let promise = self
            .set_fn
            .borrow()
            .as_ref()
            .unwrap()
            .call1(&self.local, &items)?;
        let promise = js_sys::Promise::resolve(&promise);
        wasm_bindgen_futures::JsFuture::from(promise).await?;

        Ok(JsValue::from_bool(true))
    }

    fn init_get_fn(&self) -> Result<(), JsValue> {
        let mut get_fn = self.get_fn.borrow_mut();
        if get_fn.is_none() {
            *get_fn = Some(
                js_sys::Reflect::get(&self.local, &JsValue::from_str("get"))?
                    .dyn_into::<js_sys::Function>()?,
            );
        }
        Ok(())
    }

    fn init_set_fn(&self) -> Result<(), JsValue> {
        let mut set_fn = self.set_fn.borrow_mut();
        if set_fn.is_none() {
            *set_fn = Some(
                js_sys::Reflect::get(&self.local, &JsValue::from_str("set"))?
                    .dyn_into::<js_sys::Function>()?,
            );
        }
        Ok(())
    }
}
