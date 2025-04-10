const CACHE_VERSION = '0.1.0';
const CACHE_NAME = `mtc-cache-${CACHE_VERSION}`;
const PRE_CACHED_RESOURCES = ['/index.html'];

const ONLINE_TIMEOUT = 10000;
let onlineStatus = {value: false, timestamp: new Date().getTime()};

const setOnlineStatus = (isOnline) => {
    onlineStatus = {value: isOnline, timestamp: new Date().getTime()};
};
const addResourcesToCache = async (resources) => {
    caches.open(CACHE_NAME).then((cache) => cache.addAll(resources));
};
const fetchFromNetworkOnly = async (request) => {
    return fetch(request)
        .then((response) => {
            setOnlineStatus(true);
            return response;
        })
        .catch((error) => {
            if (error instanceof TypeError) {
                setOnlineStatus(false);
            }
        });
}
const fetchFromNetworkFirst = async (request) => {
    return caches.open(CACHE_NAME).then((cache) =>
        fetch(request).then((response) =>
            cache.put(request, response.clone()).then(() => {
                setOnlineStatus(true);
                return response;
            })
        ).catch((error) => {
            if (error instanceof TypeError) {
                setOnlineStatus(false);
                return caches.open(CACHE_NAME).then((cache) =>
                    cache
                        .match(request, {ignoreVary: true, ignoreSearch: true})
                        .then((matching) => matching));
            }
        })
    );
}
const fetchFromCacheFirst = async (request) => {
    return caches.open(CACHE_NAME).then((cache) =>
        cache
            .match(request, {ignoreVary: true, ignoreSearch: true})
            .then((matching) => matching ||
                fetch(request).then((response) =>
                    cache.put(request, response.clone()).then(() => {
                        setOnlineStatus(true);
                        return response;
                    })
                ).catch((error) => {
                    if (error instanceof TypeError) {
                        setOnlineStatus(false);
                    }
                })
            ));
};
const healthCheck = async () => {
    const now = new Date().getTime();
    if (now - onlineStatus.timestamp < ONLINE_TIMEOUT) {
        return;
    }

    fetch('/api/health', {headers: {session: '{session}'}, mode: 'cors', credentials: 'include'})
        .then(() => setOnlineStatus(true))
        .catch(() => setOnlineStatus(false));
}

self.addEventListener('install', (event) => event.waitUntil(addResourcesToCache(PRE_CACHED_RESOURCES)));
self.addEventListener('activate', (event) => {
    event.waitUntil(self.registration?.navigationPreload.enable());
    event.waitUntil(self.clients.claim());
    event.waitUntil(caches.keys().then((keys) => {
        return Promise.all(keys.filter((key) => key !== CACHE_NAME)
            .map((nm) => caches.delete(nm)));
    }));
    event.waitUntil(addResourcesToCache(PRE_CACHED_RESOURCES));
})
self.addEventListener('message', (event) => {
    if (event?.data?.type === 'ACTIVATE') {
        self.skipWaiting().then(() => {
            console.log('ServiceWorker reactivated');
        });
    } else if (event?.data?.type === 'VERSION') {
        event.ports[0].postMessage({
            type: 'VERSION', version: CACHE_VERSION,
        });
    } else if (event?.data?.type === 'CLEAR_CACHE') {
        try {
            caches.keys().then((keys) => {
                keys.map((key) => caches.delete(key)
                    .then(() => caches.open(CACHE_NAME)
                        .then((cache) => cache.addAll(PRE_CACHED_RESOURCES))
                        .then(() => {
                            event.ports[0].postMessage({
                                type: 'CLEAR_CACHE', result: true,
                            });
                        })))
            });
        } catch (error) {
            console.log('ServiceWorker clear cache error: ', error);
            event.ports[0].postMessage({
                type: 'CLEAR_CACHE', result: false,
            });
        }
    }
})
self.addEventListener('fetch', (event) => {
    const {request} = event;

    if (!request.url.startsWith(self.location.origin)) {
        return;
    }

    const modifiedHeaders = new Headers(request.headers);
    modifiedHeaders.append('session', '{session}');
    const modifiedRequestInit = {headers: modifiedHeaders, mode: 'cors', credentials: 'include'};
    const modifiedRequest = new Request(request, modifiedRequestInit);

    if (request.method !== 'GET'
        || request.referrer.includes('/administrator/')
        || request.referrer.includes('/editor/')
    ) {
        event.respondWith(fetchFromNetworkOnly(modifiedRequest));
    } else if (onlineStatus.value) {
        event.respondWith(fetchFromNetworkFirst(modifiedRequest));
    } else {
        event.respondWith(fetchFromCacheFirst(modifiedRequest));
        event.waitUntil(healthCheck());
    }
})