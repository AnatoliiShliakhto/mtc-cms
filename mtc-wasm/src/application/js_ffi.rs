use crate::prelude::MouseData;
use dioxus::prelude::Event;
use dioxus::web::WebEventExt;
use tracing::error;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/assets/js/js_ffi.js")]
extern "C" {
    #[wasm_bindgen(js_name = initializeServiceWorker)]
    pub fn jsFfiInitializeServiceWorker(sessionToken: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_name = checkServiceWorker)]
    pub fn jsFfiCheckServiceWorker() -> js_sys::Promise;

    #[wasm_bindgen(js_name = clearCacheServiceWorker)]
    pub fn jsFfiClearCacheServiceWorker() -> js_sys::Promise;

    #[wasm_bindgen(js_name = setContentId)]
    pub fn jsFfiSetContentId(contentId: &str);

    #[wasm_bindgen(js_name = setTitle)]
    pub fn jsFfiSetTitle(title: &str);

    #[wasm_bindgen(js_name = initializeScrollUpButton)]
    pub fn jsFfiInitializeScrollUpButton();

    #[wasm_bindgen(js_name = createCkEditor)]
    pub fn jsFfiCreateCkEditor(name: &str);

    #[wasm_bindgen(js_name = destroyCkEditor)]
    pub fn jsFfiDestroyCkEditor();

    #[wasm_bindgen(js_name = copyFromClipboard)]
    pub fn jsFfiCopyFromClipboard() -> js_sys::Promise;

    #[wasm_bindgen(js_name = copyToClipboard)]
    pub fn jsFfiCopyToClipboard(text: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_name = clearFileInput)]
    pub fn jsFfiClearFileInput() -> js_sys::Promise;

    #[wasm_bindgen(js_name = exportJsonFile)]
    pub fn jsFfiExportJsonFile(contentJsonStr: &str, suggestedFileName: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_name = uploadFile)]
    pub fn jsFfiUploadFile(storageType: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_name = downloadFile)]
    pub fn jsFfiDownloadFile(filePath: &str, fileSize: i32) -> js_sys::Promise;

    #[wasm_bindgen(js_name = openFileIfExist)]
    pub fn jsFfiOpenFileIfExist(filePath: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_name = listDownloadedFiles)]
    pub fn jsFfiListDownloadedFiles() -> js_sys::Promise;

    #[wasm_bindgen(js_name = removeDownloadedFiles)]
    pub fn jsFfiRemoveDownloadedFiles() -> js_sys::Promise;

    #[wasm_bindgen(js_name = removeDownloadedFile)]
    pub fn jsFfiRemoveDownloadedFile(filePath: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_name = startBarcodeScanner)]
    pub fn jsFfiStartBarcodeScanner() -> js_sys::Promise;

    #[wasm_bindgen(js_name = stopBarcodeScanner)]
    pub fn jsFfiStopBarcodeScanner() -> js_sys::Promise;

    #[wasm_bindgen(js_name = openLink)]
    pub fn jsFfiOpenLink(html_element: HtmlElement);

    #[wasm_bindgen(js_name = openDownloadedLink)]
    pub fn jsFfiOpenDownloadedLink(html_element: HtmlElement) -> js_sys::Promise;

    #[wasm_bindgen(js_name = blurActiveElement)]
    pub fn jsFfiBlurActiveElement() -> js_sys::Promise;

    #[wasm_bindgen(js_name = uploadPersonnel)]
    pub fn jsFfiUploadPersonnel() -> js_sys::Promise;
}

// Event Handlers
pub async fn jsFfiHandleOpenFileIfExistEvent(event: Event<MouseData>, filePath: &str) {
    event.prevent_default();
    event.stop_propagation();
    if JsFuture::from(jsFfiOpenFileIfExist(filePath)).await.is_err() {
        error!("failed to invoke jsFfiOpenFileIfExist");
    }
}

pub fn jsFfiHandleOpenLinkEvent(event: Event<MouseData>) {
    event.prevent_default();
    event.stop_propagation();
    if let Some(html_element) = target_html_element_opt(event) {
        jsFfiOpenLink(html_element);
    }
}

pub async fn jsFfiHandleOpenDownloadedLinkEvent(event: Event<MouseData>) {
    event.prevent_default();
    event.stop_propagation();
    if let Some(html_element) = target_html_element_opt(event) {
        if JsFuture::from(jsFfiOpenDownloadedLink(html_element)).await.is_err() {
            error!("failed to invoke jsFfiRemoveDownloadedFiles");
        }
    }
}

fn target_html_element_opt(event: Event<MouseData>) -> Option<HtmlElement> {
    match event.as_web_event().current_target() {
        Some(target) => match target.dyn_into::<HtmlElement>() {
            Ok(html_element) => Some(html_element),
            Err(_) => {
                error!("failed to get target as html element");
                None
            }
        },
        None => {
            error!("failed to get event target");
            None
        }
    }
}

pub async fn jsFfiHandleBlurActiveElementEvent(event: Event<MouseData>) {
    event.prevent_default();
    event.stop_propagation();
    if JsFuture::from(jsFfiBlurActiveElement()).await.is_err() {
        error!("failed to invoke jsFfiBlurActiveElement");
    }
}

pub async fn jsFfiHandleUploadPersonnelEvent(event: Event<MouseData>) {
    event.prevent_default();
    event.stop_propagation();
    if JsFuture::from(jsFfiUploadPersonnel()).await.is_err() {
        error!("failed to invoke jsFfiUploadPersonnel");
    }
}