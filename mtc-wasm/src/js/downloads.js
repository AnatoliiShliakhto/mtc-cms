let result = [];
try {
    const files = await window.__TAURI__.fs.readDir(downloadDirectory);
    for (let i in files) {
        const meta = await window.__TAURI__.fs.stat(downloadDirectory + files[i].name);
        result.push({path: files[i].name, size: meta.size});
    }
} catch (err) {
    console.error( err );
}
dioxus.send(result);