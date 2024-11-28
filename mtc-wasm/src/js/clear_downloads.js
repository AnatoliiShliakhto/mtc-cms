try {
    const files = await window.__TAURI__.fs.readDir(downloadDirectory);
    for (let i in files) {
        const path = downloadDirectory + files[i].name;
        if (await window.__TAURI__.fs.exists(path)) {
            await window.__TAURI__.fs.remove(path);
        }
    }
} catch (err) {
    console.log(err);
    dioxus.send(false);
}
dioxus.send(true);