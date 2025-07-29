use super::*;
use wasm_bindgen_futures::JsFuture;

#[component]
pub fn QrSignIn() -> Element {
    breadcrumbs!();

    let platform = state!(platform);
    if platform.ne("android") {
        return rsx! { NotFound {} }
    }
    let sync = use_coroutine_handle::<SyncAction>();

    use_effect(move || {
        spawn(async move {
            match JsFuture::from(jsFfiStartBarcodeScanner()).await
                .ok()
                .and_then(|jsValue| jsValue.as_string())
            {
                Some(message) => {
                    if message.is_empty() {
                        error_dialog!("error-camera-access");
                        navigator().go_back();
                    } else if !message.starts_with("MTC:000:") {
                        error_dialog!("error-invalid-qr-code");
                        navigator().go_back();
                    } else {
                        let message = message.replace("MTC:000:", "");
                        let mcrypt = new_magic_crypt!(env!("CRYPT_KEY"), 256);
                        if let Ok(decrypted_qr) = mcrypt.decrypt_base64_to_string(&message) {
                            let payload = json!({
                            "api_key": decrypted_qr.as_str(),
                        });

                            if !post_request!(url!(API_AUTH), payload) {
                                navigator().go_back();
                                return;
                            };

                            sync.send(SyncAction::RefreshState());
                        }
                    }
                    navigator().go_back();
                },
                None => error!("failed to invoke jsFfiStartBarcodeScanner"),
            }
        });
    });

    rsx! {
        div {
            class: "modal modal-open grid place-items-center",
            "qr-scanner": true,
            div {
                class: "modal-box bg-transparent",
                Icon {
                    icon: Icons::QrScan,
                    class: "w-full h-full opacity-30 text-slate-100",
                }
            }
        }
    }
}