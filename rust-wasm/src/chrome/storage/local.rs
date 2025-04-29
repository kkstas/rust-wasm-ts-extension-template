use wasm_bindgen::prelude::*;

pub struct ChromeStorageLocal {
    local: JsValue,
    get_fn: js_sys::Function,
    set_fn: js_sys::Function,
}

impl ChromeStorageLocal {
    pub fn new() -> Self {
        let chrome = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("chrome"))
            .expect("Failed to get chrome object from global");

        let storage = js_sys::Reflect::get(&chrome, &JsValue::from_str("storage"))
            .expect("Failed to get storage object from chrome");

        let local = js_sys::Reflect::get(&storage, &JsValue::from_str("local"))
            .expect("Failed to get local object from storage");

        let get_fn = js_sys::Reflect::get(&local, &JsValue::from_str("get"))
            .expect("Failed to get get function")
            .dyn_into::<js_sys::Function>()
            .expect("get is not a function");

        let set_fn = js_sys::Reflect::get(&local, &JsValue::from_str("set"))
            .expect("Failed to get set function")
            .dyn_into::<js_sys::Function>()
            .expect("set is not a function");

        Self {
            local,
            get_fn,
            set_fn,
        }
    }

    pub async fn get(&self, key: &str) -> Result<JsValue, JsValue> {
        let promise = self.get_fn.call1(&self.local, &JsValue::from_str(key))?;
        let promise = js_sys::Promise::resolve(&promise);

        let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
        Ok(result)
    }

    pub async fn set<T: Into<JsValue>>(&self, key: &str, value: T) -> Result<(), JsValue> {
        let items = js_sys::Object::new();
        js_sys::Reflect::set(&items, &JsValue::from_str(key), &value.into())?;

        let promise = self.set_fn.call1(&self.local, &items)?;
        let promise = js_sys::Promise::resolve(&promise);
        wasm_bindgen_futures::JsFuture::from(promise).await?;

        Ok(())
    }
}
