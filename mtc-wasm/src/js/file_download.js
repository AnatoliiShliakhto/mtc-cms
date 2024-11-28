try {
    const tauri = window.__TAURI__ || undefined;
    const fileUrl = window.origin + '{url}';
    const fileSize = {size};
    const fileName = fileUrl.replace(/^.*[\\\/]/, '');
    const filePath = window.downloadDirectory + fileName;

    if (tauri) {
        if (await tauri.fs.exists(filePath)) {
            const meta = await tauri.fs.stat(filePath);
            if (meta && meta.size === fileSize) {
                dioxus.send(true);
            } else {
                dioxus.send(await downloadFile(fileUrl, filePath));
            }
        } else {
            dioxus.send(await downloadFile(fileUrl, filePath));
        }
    } else {
        const response = await fetch(fileUrl);
        if (response.ok) {
            dioxus.send(true)
        }
    }
} catch (err) {
    console.log(err);
    dioxus.send(false);
}