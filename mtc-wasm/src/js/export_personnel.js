const obj = JSON.stringify(await dioxus.recv());
const file = new Blob([obj], {type: "application/json"});

if (window.showSaveFilePicker) {
    const opts = {
        types: [
            {
                description: 'JSON',
                accept: {'application/json': ['.json']},
            },
        ],
        suggestedName: 'mtc-users',
    };
    try {
        const handle = await window.showSaveFilePicker(opts);
        const writable = await handle.createWritable();
        await writable.write(file);
        await writable.close();
    } catch (error) {
        console.error("Error saving file:", error);
    }
} else {
    alert("File save not supported in this browser");
}
