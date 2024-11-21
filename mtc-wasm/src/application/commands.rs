use super::*;

pub fn is_tauri() -> bool {
    js_sys::Reflect::has(
        &JsValue::from(web_sys::window().unwrap()),
        &JsValue::from_str("__TAURI__"))
        .unwrap_or_default()
}

pub async fn set_tauri_session(session: String) {
    #[derive(Serialize)]
    struct SetSessionArgs {
        session: String,
    }

    let _ = tauri_invoke(
        "set_session",
        serde_wasm_bindgen::to_value(&SetSessionArgs { session }).unwrap(),
    ).await;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke, catch)]
    pub async fn tauri_invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke, catch)]
    pub async fn tauri_invoke_without_args(cmd: &str) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "barcodeScanner"], js_name = scan, catch)]
    pub async fn tauri_scan(args: JsValue) -> Result<JsValue, JsValue>;
}