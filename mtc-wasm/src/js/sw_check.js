if ("serviceWorker" in navigator) {
    navigator.serviceWorker.ready
        .then((registration) => {
            if (registration.active && !navigator.serviceWorker.controller) {
                window.location.reload();
                return;
            }

            const messageChannel = new MessageChannel();

            messageChannel.port1.onmessage = ({data}) => {
                if (data.type === 'VERSION') {
                    console.info('Cache version:', data.version);
                    dioxus.send(true);
                }
            };

            registration.active.postMessage(
                {type: 'VERSION'},
                [messageChannel.port2]
            );
        })
        .catch((error) => console.error('Error with ServiceWorker:', error));
} else {
    dioxus.send(false);
}