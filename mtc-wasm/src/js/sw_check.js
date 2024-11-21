if ("serviceWorker" in navigator) {
    navigator.serviceWorker.ready.then((sw) => {
        const messageChannel = new MessageChannel();

        messageChannel.port1.onmessage = (event) => {
            if (event.data.type === 'SW_VERSION') {
                console.log('Cache version: ' + event.data.version);
                dioxus.send(true);
            }
        };

        sw.active.postMessage({
            type: 'SW_VERSION',
        }, [messageChannel.port2])
    });
} else {
    dioxus.send(false);
}