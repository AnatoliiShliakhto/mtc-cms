// Configuration
const CONFIG = {
    cache: {
        version: '0.0.1',
        name: 'mtc-cache-0.0.1',
        resources: ['/index.html']
    },
    network: {
        onlineTimeout: 10000,
        healthEndpoint: '/api/health'
    }
};

class NetworkManager {
    constructor() {
        this.status = { value: false, timestamp: 0 };
    }

    setOnlineStatus(isOnline) {
        this.status = { value: isOnline, timestamp: Date.now() };
    }

    handleNetworkError(error) {
        if (error instanceof TypeError) {
            this.setOnlineStatus(false);
        }
    }

    async checkHealth() {
        if (Date.now() - this.status.timestamp < CONFIG.network.onlineTimeout) return;

        try {
            await fetch(CONFIG.network.healthEndpoint, {
                headers: { session: '{session}' }
            });
            this.setOnlineStatus(true);
        } catch (error) {
            this.handleNetworkError(error);
        }
    }
}

class CacheManager {
    static async cacheResources(resources) {
        try {
            const cache = await caches.open(CONFIG.cache.name);
            await Promise.all(resources.map(resource => this.cacheResource(cache, resource)));
            console.info(`Pre-cached ${resources.length} resources for ${CONFIG.cache.name}`);
        } catch (error) {
            console.error('Error caching resources:', error);
        }
    }

    static async cacheResource(cache, resource) {
        const request = new Request(`${self.location.origin}${resource}`, {
            headers: new Headers({ session: '{session}' })
        });
        try {
            const response = await fetch(request);
            if (response.ok) {
                await cache.put(request, response);
            }
        } catch {
            console.log(`Resource not found: ${resource}`);
        }
    }

    static async clearCache() {
        const keys = await caches.keys();
        await Promise.all(keys.map(key => caches.delete(key)));
        await this.cacheResources(CONFIG.cache.resources);
        await clearIndexedDb();
    }

    static async fetchWithStrategy(request, strategy, networkManager) {
        const strategies = {
            networkOnly: async () => {
                const response = await fetch(request);
                networkManager.setOnlineStatus(true);
                return response;
            },
            networkFirst: async () => {
                try {
                    const response = await fetch(request);
                    await this.cacheResponse(request, response.clone());
                    return response;
                } catch (error) {
                    networkManager.handleNetworkError(error);
                    return caches.match(request, { ignoreVary: true, ignoreSearch: true });
                }
            },
            cacheFirst: async () => {
                const cached = await caches.match(request, { ignoreVary: true, ignoreSearch: true });
                if (cached) return cached;
                return this.fetchWithStrategy(request, 'networkFirst', networkManager);
            }
        };
        return strategies[strategy]();
    }

    static async cacheResponse(request, response) {
        if (response.ok) {
            const cache = await caches.open(CONFIG.cache.name);
            await cache.put(request, response.clone());
        }
        return response;
    }
}

const networkManager = new NetworkManager();

// Event Listeners
self.addEventListener('install', event => {
    event.waitUntil(CacheManager.cacheResources(CONFIG.cache.resources));
});

self.addEventListener('activate', event => {
    event.waitUntil(
        (async () => {
            await self.clients.claim();
            await CacheManager.clearCache();
        })()
    );
});

self.addEventListener('message', event => {
    const messageHandlers = {
        ACTIVATE: () => self.skipWaiting(),
        VERSION: async () => {
            await CacheManager.cacheResources(CONFIG.cache.resources);
            event.ports[0]?.postMessage({ type: 'VERSION', version: CONFIG.cache.version });
        },
        CLEAR_CACHE: async () => {
            try {
                await CacheManager.clearCache();
                event.ports[0]?.postMessage({ type: 'CLEAR_CACHE', result: true });
            } catch (error) {
                console.error('ServiceWorker clear cache error:', error);
                event.ports[0]?.postMessage({ type: 'CLEAR_CACHE', result: false });
            }
        }
    };

    const { type } = event?.data || {};
    const handler = messageHandlers[type];
    if (handler) {
        event.waitUntil(handler());
    } else {
        console.warn('Unhandled message type:', type);
    }
});

self.addEventListener('fetch', event => {
    let { request } = event;
    if (!request.url.startsWith(self.location.origin)) return;

    const modifiedRequest = new Request(request, {
        headers: new Headers({
            ...Object.fromEntries(request.headers.entries()),
            session: '{session}'
        })
    });

    const isAdminOrEditor = ['/administrator/', '/editor/']
        .some(path => request.referrer.includes(path));

    if (request.method !== 'GET' || isAdminOrEditor) {
        event.respondWith(CacheManager.fetchWithStrategy(modifiedRequest, 'networkOnly', networkManager));
        return;
    }

    const strategy = networkManager.status.value ? 'networkFirst' : 'cacheFirst';
    if (!networkManager.status.value) {
        event.waitUntil(networkManager.checkHealth());
    }
    event.respondWith(CacheManager.fetchWithStrategy(modifiedRequest, strategy, networkManager));
});

async function clearIndexedDb() {
    if (typeof indexedDB.databases !== 'function') {
        console.warn("Function indexedDB.databases() is not available in browser");
        return;
    }
    const databases = await indexedDB.databases();
    for (const db of databases) {
        const deleteRequest = indexedDB.deleteDatabase(db.name);
        deleteRequest.onsuccess = () => {
            console.log(`Database deleted: '${db.name}'`);
        };
        deleteRequest.onerror = (event) => {
            console.error(`Database delete failed: '${db.name}'`, event.target.error);
        };
        deleteRequest.onblocked = () => {
            console.error(`Database delete blocked (close all open connections): '${db.name}'`);
        };
    }
}