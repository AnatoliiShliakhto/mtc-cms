if ("serviceWorker" in navigator) {
    try {
        const registration =
            await navigator
                .serviceWorker
                .register("./service_worker?session={session}", {scope: "./", updateViaCache: "none"});

        if (self.intervalId) clearInterval(self.intervalId);
        self.intervalId = setInterval(() => {registration.update();}, 5 * 60 * 1000);

        registration.addEventListener("updatefound", () => {
            registration?.waiting?.postMessage({type: "SKIP_WAITING"});
        });
    } catch (err) {
        console.log("ServiceWorker registration failed", err);
    }
}
