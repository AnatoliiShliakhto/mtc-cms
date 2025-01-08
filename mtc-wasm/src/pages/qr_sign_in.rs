use super::*;

/// Displays a QR code scanner for user sign-in on Android devices.
///
/// This component checks if the platform is Android, and if not, redirects to a NotFound page.
/// When run on Android, it uses a barcode scanner to capture a QR code. The QR code should start
/// with "MTC:000:". If the captured code is valid, it decrypts the API key, sends a sign-in request,
/// and refreshes the application state. If the code is invalid or any error occurs, it navigates back.
///
/// # UI Elements
///
/// - A modal dialog containing the QR scanner interface.
/// - An icon indicating the QR scanning process.
///
/// # Behavior
///
/// - If the platform is not Android, redirects to NotFound.
/// - If a valid QR code is scanned, attempts to sign in the user.
/// - Alerts the user and navigates back if the QR code is invalid or on error.
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
            if let Ok(Value::String(message)) = eval(JS_BARCODE_SCAN).recv().await {
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