use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["chrome", "storage", "local"], js_name = get)]
    async fn js_get(key: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "storage", "local"], js_name = set)]
    async fn js_set(items: JsValue) -> Result<JsValue, JsValue>;
}

pub async fn get(key: &str) -> Result<JsValue, JsValue> {
    let result = js_get(key).await?;
    js_sys::Reflect::get(&result, &JsValue::from_str(key))
}

pub async fn set<T: Into<JsValue>>(key: &str, value: T) -> Result<(), JsValue> {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &JsValue::from_str(key), &value.into())?;
    js_set(obj.into()).await?;
    Ok(())
}
