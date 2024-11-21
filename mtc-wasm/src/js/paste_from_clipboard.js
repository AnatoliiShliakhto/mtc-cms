try {
    await navigator
        .clipboard
        .readText()
        .then((clipText) => (dioxus.send(clipText)));
} catch (e) {
    console.error(e);
}

