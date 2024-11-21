let msg = await dioxus.recv();

try {
    await navigator
        .clipboard
        .write([new ClipboardItem(
            {
                'text/plain': new Blob([msg], {type: 'text/plain;charset=utf-8'})
            }
        )])
} catch (e) {
    console.error(e);
}