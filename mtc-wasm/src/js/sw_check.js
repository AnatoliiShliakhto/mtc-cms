if ("serviceWorker" in navigator) {
    navigator.serviceWorker.ready.then((registration) => {
        if (registration.active && !navigator.serviceWorker.controller) {
            window.location.reload()
        }

        const messageChannel = new MessageChannel()

        messageChannel.port1.onmessage = (event) => {
            if (event.data.type === 'VERSION') {
                console.log('ServiceWorker cache version: ' + event.data.version)
                dioxus.send(true);
            }
        }

        registration.active.postMessage({
            type: 'VERSION',
        }, [messageChannel.port2])
    })
} else {
    dioxus.send(false);
}