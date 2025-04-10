document.getElementById("wasm-preloader").remove()

if ("serviceWorker" in navigator) {
    navigator
        .serviceWorker
        .register("./service_worker?session={session}", {scope: "./", updateViaCache: "none"})
        .then((registration) => {
            if (self.intervalId) clearInterval(self.intervalId)
            self.intervalId = setInterval(async () => {
                await registration.update()
            }, 5 * 60 * 1000)

            registration.addEventListener('updatefound', () => {
                registration?.waiting?.postMessage({type: 'ACTIVATE'})
            })
        })
        .catch((error) => console.log('ServiceWorker registration failed', error))
}