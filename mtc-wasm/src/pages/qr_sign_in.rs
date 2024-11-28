use super::*;

#[component]
pub fn QrSignIn() -> Element {
    breadcrumbs!();

    let platform = state!(platform);
    if platform.ne("android") {
        return rsx! { NotFound {} }
    }
    let sync = use_coroutine_handle::<SyncAction>();

    let scan_qr = r#"
        let scan = window.__TAURI__.barcodeScanner.scan;
        let format = window.__TAURI__.barcodeScanner.Format;

        try {
            await window.__TAURI__.barcodeScanner.requestPermissions();
            let scanned = await scan({ windowed: true, formats: [format.QRCode] });
            dioxus.send(scanned.content);
        } catch (error) {
            console.log(error);
            dioxus.send("");
        }
        try {
            await window.__TAURI__.barcodeScanner.cancel();
        } catch (error) {
            console.log(error);
        }
    "#;

    use_effect(move || {
        spawn(async move {
            if let Ok(Value::String(message)) = eval(scan_qr).recv().await {
                if !message.starts_with("MTC:000:") {
                    alert_dialog!("error-invalid-qr-code");
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