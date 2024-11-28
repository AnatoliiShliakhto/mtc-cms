if ("serviceWorker" in navigator) {
    navigator.serviceWorker.ready.then((sw) => {
        const messageChannel = new MessageChannel();

        messageChannel.port1.onmessage = (event) => {
            if (event.data.type === 'CLEAR_CACHE') {
                try {
                    fetch('/api/sync').then((response) => {});
                } catch (err) {
                    console.error(err);
                }
                dioxus.send(event.data.result);
            }
        };

        sw.active.postMessage({
            type: 'CLEAR_CACHE',
        }, [messageChannel.port2])
    });
} else {
    dioxus.send(false);
}