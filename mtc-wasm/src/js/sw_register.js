const SERVICE_WORKER_UPDATE_INTERVAL_MS = 300000;

document.getElementById("wasm-preloader")?.remove();

if ("serviceWorker" in navigator) {
    navigator.serviceWorker.register("./service_worker?session={session}", {
        scope: "./",
        updateViaCache: "none"
    })
        .then((swRegistration) => {
            if (self.intervalId) {
                clearInterval(self.intervalId);
            }
            self.intervalId = setInterval(() => swRegistration.update(), SERVICE_WORKER_UPDATE_INTERVAL_MS);
            swRegistration.addEventListener("updatefound", () => {
                swRegistration.waiting?.postMessage({ type: "ACTIVATE" });
            });
        })
        .catch((registrationError) => {
            console.error("ServiceWorker registration failed:", registrationError);
        });
}
