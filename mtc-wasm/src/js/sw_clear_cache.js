if ("serviceWorker" in navigator) {
    navigator.serviceWorker.ready.then((sw) => {
        const messageChannel = new MessageChannel();

        messageChannel.port1.onmessage = ({data}) => {
            if (data.type === 'CLEAR_CACHE') {
                fetch('/api/sync').catch(console.error);
                dioxus.send(data.result);
            }
        };

        sw.active?.postMessage(
            {type: 'CLEAR_CACHE'},
            [messageChannel.port2]
        );
    }).catch(console.error);
} else {
    dioxus.send(false);
}