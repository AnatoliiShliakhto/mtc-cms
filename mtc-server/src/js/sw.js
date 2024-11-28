const FRONT_END_URL = '{front_end_url}';
const CACHE_VERSION = "1.0.0";
const CACHE_NAME = `mtc-cache-${CACHE_VERSION}`;
const PRE_CACHED_RESOURCES = {precache};

// Interval in ms before we re-check the connectivity - change to your liking
const ONLINE_TIMEOUT = 10000;
let onlineStatus = { value: false, timestamp: new Date().getTime() };

// ***** Installation *****
self.addEventListener("install", (event) => {
    console.log("ServiceWorker is installed");
    const channel = new BroadcastChannel("sw-messages");
    channel.postMessage({ CACHE_VERSION });

    getOnlineState().then(() => caches.open(CACHE_NAME)
        .then((cache) => cache.addAll(PRE_CACHED_RESOURCES)));
});

// This allows the web app to trigger skipWaiting
self.addEventListener("message", (ev) => {
    if (ev?.data?.type === "SKIP_WAITING")  {
        self.skipWaiting();
    }
    if (ev?.data?.type === "SW_VERSION")  {
        ev.ports[0].postMessage({
            type: 'SW_VERSION',
            version: CACHE_VERSION,
        });
    }
    if (ev?.data?.type === "CLEAR_CACHE") {
        try {
            caches.keys().then((keys) => {
                keys.map((nm) => caches.delete(nm)
                    .then(() =>
                        caches.open(CACHE_NAME)
                            .then((cache) => cache.addAll(PRE_CACHED_RESOURCES))
                            .then(() => {
                                ev.ports[0].postMessage({
                                    type: 'CLEAR_CACHE',
                                    result: true,
                                });
                            })
                    ))
            });
        } catch(err) {
            ev.ports[0].postMessage({
                type: 'CLEAR_CACHE',
                result: false,
            });
        }
    }
});

// ***** Activation *****
self.addEventListener("activate", (ev) => {
    console.log("ServiceWorker is activated");
    // Activate ServiceWorker immediately
    ev.waitUntil(self.clients.claim());
    // Remove old caches if exists
    ev.waitUntil(
        caches.keys().then((keys) => {
            return Promise.all(
                keys.filter((key) => key !== CACHE_NAME)
                    .map((nm) => caches.delete(nm)),
            );
        }).then(() =>
            caches.open(CACHE_NAME)
                .then((cache) => cache.addAll(PRE_CACHED_RESOURCES))
        )
    );
});

self.addEventListener('fetch', event => {
    // Create modified request with custom headers
    const modifiedHeaders = new Headers(event.request.headers);
    modifiedHeaders.append('session', '{session}');
    const modifiedRequestInit = {headers: modifiedHeaders, mode: 'cors', credentials: 'include'};
    const modifiedRequest = new Request(event.request, modifiedRequestInit)

    if (event.request.method !== 'GET'
        || event.request.referrer.includes('/administrator/')
        || event.request.referrer.includes('/editor/')) {
        event.respondWith(networkOnly(event, modifiedRequest));
        return;
    }

    if (onlineStatus.value) {
        event.respondWith(networkRevalidateAndCache(event, modifiedRequest));
    } else {
        event.respondWith(cacheFirst(event, modifiedRequest));
    }
});

// Try cache and fallback on network
async function cacheFirst(event, request) {
    try {
        const timestamp = new Date().getTime();
        getOnlineState();

        // Return the cache response if it is not null
        const cacheResponse = await caches.match(event.request, { ignoreVary: true, ignoreSearch: true});
        if (cacheResponse) return cacheResponse;

        // If no cache, fetch and cache the result and return result
        const fetchResponse = await fetch(request).catch( err => {
            if (err instanceof TypeError) {
                onlineStatus = { value: false, timestamp: timestamp };
            }
        });
        if (fetchResponse && fetchResponse.ok) {
            const cache = await caches.open(CACHE_NAME);
            await cache.put(event.request, fetchResponse.clone());
            onlineStatus = { value: true, timestamp };
        }
        return fetchResponse;
    } catch (err) {
        console.log("Could not return cache or fetch CF: ", err);
    }
}

async function networkOnly(event, request) {
    try {
        const timestamp = new Date().getTime();

        const fetchResponse = await fetch(request).catch( err => {
            if (err instanceof TypeError) {
                onlineStatus = { value: false, timestamp: timestamp };
                getOnlineState();
            }
        });
        if (fetchResponse && fetchResponse.ok) {
            onlineStatus = { value: true, timestamp: timestamp };
        }
        return fetchResponse;
    } catch (err) {
        console.log("Could not return and fetch the asset CF: ", err);
    }
}

// Try to fetch from network then cache
async function networkRevalidateAndCache(event, request) {
    try {
        const timestamp = new Date().getTime();

        const fetchResponse = await fetch(request).catch( err => {
            if (err instanceof TypeError) {
                onlineStatus = { value: false, timestamp: timestamp };
                getOnlineState();
            }
        });
        if (fetchResponse && fetchResponse.status === 200) {
            const cache = await caches.open(CACHE_NAME);
            await cache.put(event.request, fetchResponse.clone());
            onlineStatus = { value: true, timestamp };
            return fetchResponse;
        } else {
            return await caches.match(event.request, { ignoreVary: true, ignoreSearch: true});
        }
    } catch (err) {
        console.log("Could not return cache or fetch NF", err);
    }
}

async function getOnlineState() {
    const now = new Date().getTime();
    const headers = new Headers();
    headers.append('cache-control', 'no-store');

    // If the last online status is recent, return it
    if (now - onlineStatus.timestamp < ONLINE_TIMEOUT) {
        return new Response(
            JSON.stringify(onlineStatus),
            { status: 200, statusText: 'OK', headers }
        );
    }

    // Otherwise, attempt a real fetch to re-check the connection
    else {
        try {
            await fetch(FRONT_END_URL, { method: 'HEAD', headers });
            onlineStatus = { value: true, timestamp: now };
        } catch (error) {
            if (error instanceof TypeError) {
                onlineStatus = { value: false, timestamp: now };
            } else {
                throw error;
            }
        }
    }
    // Recursive call, this time the new status will be returned
    return await getOnlineState();
}