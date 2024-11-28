try {
    const path = window.downloadDirectory + '{file}';
    if (await window.__TAURI__.fs.exists(path)) {
        await window.__TAURI__.fs.remove(path);
    }
    dioxus.send(true);
} catch (err) {
    console.log(err);
    dioxus.send(false);
}