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