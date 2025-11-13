export async function initializeServiceWorker(sessionToken) {
    document.getElementById("wasm-preloader")?.remove();

    if ("serviceWorker" in navigator) {
        try {
            const serviceWorkerRegistration = await navigator.serviceWorker
                .register(`./service_worker?session=${sessionToken}`, {
                    scope: "./",
                    updateViaCache: "none"
                });
            if (self.intervalId) {
                clearInterval(self.intervalId);
            }
            self.intervalId = setInterval(() => serviceWorkerRegistration.update(), 300000);
            serviceWorkerRegistration.addEventListener("updatefound", () => {
                serviceWorkerRegistration.waiting?.postMessage({type: "ACTIVATE"});
            });
        } catch (error) {
            console.error(error);
            throw error;
        }
    }
}

export async function checkServiceWorker() {
    if ("serviceWorker" in navigator) {
        try {
            const serviceWorkerRegistration = await navigator.serviceWorker.ready;

            if (serviceWorkerRegistration.active && !navigator.serviceWorker.controller) {
                window.location.reload();
                // We're reloading, so no need to send a message back.
                // The Dioxus app will re-initialize after reload.
                return;
            }

            const messageChannel = new MessageChannel();
            const versionPromise = new Promise((resolve) => {
                messageChannel.port1.onmessage = ({data}) => {
                    if (data.type === 'VERSION') {
                        console.info('Cache version:', data.version);
                        resolve(); // Indicate success (Service Worker is present and sent version)
                    }
                };
            });

            serviceWorkerRegistration.active.postMessage(
                {type: 'VERSION'},
                [messageChannel.port2]
            );
            return await versionPromise;
        } catch (error) {
            console.error(error);
            throw error;
        }
    } else {
        throw new Error("Service worker was not registered");
    }
}

export async function clearCacheServiceWorker() {
    if ("serviceWorker" in navigator) {
        try {
            const serviceWorkerRegistration = await navigator.serviceWorker.ready;

            const messageChannel = new MessageChannel();
            const clearCachePromise = new Promise((resolve) => {
                messageChannel.port1.onmessage = ({data}) => {
                    if (data.type === 'CLEAR_CACHE') {
                        fetch('/api/sync').catch(console.error);
                        resolve();
                    }
                };
            });

            serviceWorkerRegistration.active?.postMessage(
                {type: 'CLEAR_CACHE'},
                [messageChannel.port2]
            );
            return await clearCachePromise;
        } catch (error) {
            console.error(error);
            throw error;
        }
    } else {
        throw new Error('ServiceWorker was not registered');
    }
}

export function initializeScrollUpButton() {
    const scrollUpButton = document.getElementById("scrollUpButton");
    if (scrollUpButton) {
        window.addEventListener("scroll", () => {
            scrollUpButton.style.display = (document.body.scrollTop > 20 || document.documentElement.scrollTop > 20)
                ? "inline-flex"
                : "none";
        });

        scrollUpButton.addEventListener("click", () => {
            window.scrollTo({
                top: 0,
                behavior: "smooth" // Smooth scrolling effect
            });
        });
    } else {
        const error = "Element scrollUpButton not found";
        console.error(error);
        throw new Error(error);
    }
}

// SETTER
export function setContentId(contentId) {
    window.contentId = contentId;
}

export function setTitle(title) {
    document.title = title;
}

// CKEDITOR
export function createCkEditor(name) {
    const element = `#${name}`;
    window.CkEditorCreate(element);
}

export function destroyCkEditor() {
    window.CkEditorDestroy?.();
}

// CLIPBOARD
export async function copyFromClipboard() {
    try {
        return await navigator.clipboard.readText()
    } catch (error) {
        console.error(error);
        throw error;
    }
}

export async function copyToClipboard(text) {
    try {
        return await navigator.clipboard.write([new ClipboardItem(
            {'text/plain': new Blob([text], {type: 'text/plain;charset=utf-8'})}
        )])
    } catch (error) {
        console.error(error);
        throw error;
    }
}

// FILE UPLOAD
export function clearFileInput() {
    return new Promise((resolve, reject) => {
        try {
            const fileInput = document.querySelector('input[type=file]');
            if (fileInput) {
                fileInput.value = '';
                resolve();
            } else {
                reject(new Error('file input element not found'));
            }
        } catch (error) {
            reject(error);
        }
    });
}

export async function exportCsvFile(contentCsvStr, suggestedFileName) {
    const file = new Blob([contentCsvStr], {type: "text/csv"});
    const opts = {
        types: [
            {
                description: 'CSV',
                accept: {'text/csv': ['.csv']},
            },
        ],
        suggestedName: suggestedFileName,
    };
    await exportFile(file, opts)
}

export async function exportJsonFile(contentJsonStr, suggestedFileName) {
    const file = new Blob([contentJsonStr], {type: "application/json"});
    const opts = {
        types: [
            {
                description: 'JSON',
                accept: {'application/json': ['.json']},
            },
        ],
        suggestedName: suggestedFileName,
    };
    await exportFile(file, opts)
}

async function exportFile(file, opts) {
    if (window.showSaveFilePicker) {
        try {
            const handle = await window.showSaveFilePicker(opts);
            const writable = await handle.createWritable();
            await writable.write(file);
            await writable.close();
        } catch (error) {
            console.error(error);
            throw error;
        }
    } else {
        alert("File save not supported in this browser");
    }
}

export async function uploadFile(storageType) {
    let api_url = '/api/storage/' + storageType + '/' + window.contentId;
    let formData = new FormData();
    let fileInput = document.getElementById('fileUpload');
    let file = fileInput.files[0];

    formData.append('file', file);

    let xhr = new XMLHttpRequest();

    const uploadPromise = new Promise((resolve) => {
        xhr.addEventListener('load', function (event) {
            resolve(event.target.responseText);
        });
    });

    xhr.open('POST', api_url, true);
    xhr.send(formData);

    return await uploadPromise;
}

export async function downloadFile(filePath, fileSize) {
    try {
        const tauri = window.__TAURI__ || undefined;
        const fileUrl = window.origin + filePath;
        const fileName = fileUrl.replace(/^.*[\\\/]/, '');
        const fileDownloadDirectoryPath = window.downloadDirectory + fileName;
        if (tauri) {
            if (await tauri.fs.exists(fileDownloadDirectoryPath)) {
                const meta = await tauri.fs.stat(fileDownloadDirectoryPath);
                if (meta && meta.size !== fileSize) {
                    await window.tauriDownloadFile(fileUrl, fileDownloadDirectoryPath);
                }
            } else {
                await window.tauriDownloadFile(fileUrl, fileDownloadDirectoryPath);
            }
        } else {
            const response = await fetch(fileUrl);
            if (!response.ok) {
                throw new Error(`HTTP error! Status: ${response.status} - ${response.statusText}`);
            }
        }
    } catch (error) {
        console.log(error);
        throw error;
    }
}

export async function listDownloadedFiles() {
    try {
        let result = [];
        const files = await window.__TAURI__.fs.readDir(window.downloadDirectory);
        for (let i in files) {
            const meta = await window.__TAURI__.fs.stat(window.downloadDirectory + files[i].name);
            result.push({path: files[i].name, size: meta.size});
        }
        return result;
    } catch (error) {
        console.error(error);
        throw error;
    }
}

export async function removeDownloadedFiles() {
    try {
        const files = await window.__TAURI__.fs.readDir(window.downloadDirectory);
        for (let i in files) {
            const path = window.downloadDirectory + files[i].name;
            if (await window.__TAURI__.fs.exists(path)) {
                await window.__TAURI__.fs.remove(path);
            }
        }
    } catch (error) {
        console.error(error);
        throw error;
    }
}

export async function removeDownloadedFile(filePath) {
    try {
        const downloadDirectoryFilePath = window.downloadDirectory + filePath;
        if (await window.__TAURI__.fs.exists(downloadDirectoryFilePath)) {
            await window.__TAURI__.fs.remove(downloadDirectoryFilePath);
        }
    } catch (error) {
        console.error(error);
        throw error;
    }
}

export async function openFileIfExist(filePath) {
    try {
        const fileDownloadDirectoryPath = window.downloadDirectory + filePath;
        return await window.openIfExists(fileDownloadDirectoryPath);
    } catch (error) {
        console.error(error);
        throw error;
    }
}

// BARCODE SCANNER
export async function startBarcodeScanner() {
    let scan = window.__TAURI__.barcodeScanner.scan;
    let format = window.__TAURI__.barcodeScanner.Format;
    try {
        await window.__TAURI__.barcodeScanner.requestPermissions();
        let scanned = await scan({windowed: true, formats: [format.QRCode]});
        return scanned.content;
    } catch (error) {
        console.error(error);
        throw error;
    } finally {
        await stopBarcodeScanner();
    }
}

export async function stopBarcodeScanner() {
    try {
        await window.__TAURI__.barcodeScanner.cancel();
    } catch (error) {
        console.error(error);
        throw error;
    }
}

export function openElementLink(element) {
    try {
        window.elementLinkOpen(element);
    } catch (error) {
        console.error(error);
        throw error;
    }
}

export function openLink(url) {
    try {
        window.linkOpen(url);
    } catch (error) {
        console.error(error);
        throw error;
    }
}

export async function openDownloadedLink(element) {
    try {
        await window.linkDownloadThenOpen(element);
    } catch (error) {
        console.error(error);
        throw error;
    }
}

export function blurActiveElement() {
    try {
        document.activeElement.blur();
    } catch (error) {
        console.error(error);
        throw error;
    }
}

export function clickElement(elementId) {
    try {
        document.getElementById(elementId).click()
    } catch (error) {
        console.error(error);
        throw error;
    }
}

export function openHtml(html) {
    const newWindow = window.open('', '_blank');
    if (newWindow) {
        newWindow.document.write(html);
        newWindow.document.close();
    } else {
        alert("Please allow pop-ups for this site");
    }
}

// --- Html5 Qr code Scanner ---
let activeHtml5QrcodeScanner = null;

export async function createHtml5QrcodeScanner(elementId, cameraId, torchOn) {
    return new Promise(async (resolve, reject) => {
        try {
            await destroyHtml5QrcodeScanner();
            const config = {
                fps: 10,
                supportedScanTypes: [Html5QrcodeScanType.SCAN_TYPE_CAMERA, Html5QrcodeScanType.SCAN_TYPE_FILE]
            };

            async function onScanSuccess(decodedText, decodedResult) {
                resolve(decodedText);
                await destroyHtml5QrcodeScanner();
            }

            function onScanFailure(errorMessage) {
                throw errorMessage;
            }

            activeHtml5QrcodeScanner = new Html5Qrcode(elementId);
            await activeHtml5QrcodeScanner.start(cameraId, config, onScanSuccess, onScanFailure);
            return toggleHtml5QrcodeScannerTorch(torchOn);
        } catch (error) {
            console.error(error);
            reject(error);
        }
    });
}

export async function toggleHtml5QrcodeScannerTorch(turnOn) {
    return new Promise(async (resolve, reject) => {
        try {
            const torchFeature = activeHtml5QrcodeScanner.getRunningTrackCameraCapabilities().torchFeature();
            if (torchFeature && torchFeature.isSupported()) {
                await torchFeature.apply(turnOn);
            } else {
                console.warn("Torch is not supported")
            }
        } catch (error) {
            console.error(error);
            reject(error);
        }
    });
}

export async function supportedCameraLabelToId() {
    let devices = await Html5Qrcode.getCameras();
    return new Map(devices.map(device => [device.label, device.id]));
}

export async function detectUserEnvironmentHtml5QrcodeCameras() {
    let devices = await Html5Qrcode.getCameras();
    if (!devices || devices.length === 0) {
        return [];
    }

    let defaultCamera = null;
    let userCamera = null;
    let environmentCamera = null;
    let isUserCamera = device =>
        device.label.toLowerCase().includes("front") ||
        device.label.toLowerCase().includes("user");
    let isEnvironmentCamera = device =>
        device.label.toLowerCase().includes("back") ||
        device.label.toLowerCase().includes("environment")

    const tempContainerId = "temp_gate_pass_validation_scanner";
    const tempContainer = document.createElement('div');
    tempContainer.id = tempContainerId;
    tempContainer.style.cssText = "width: 1px; height: 1px; overflow: hidden; position: absolute; left: -9999px;";
    document.body.appendChild(tempContainer);

    try {
        for (const device of devices) {
            let tempHtml5QrcodeScanner = new Html5Qrcode(tempContainerId);
            await tempHtml5QrcodeScanner.start(
                device.id,
                {fps: 1, qrbox: 250},
                () => {
                },
                () => {
                }
            );

            const torchSupported = tempHtml5QrcodeScanner
                .getRunningTrackCameraCapabilities()
                .torchFeature()
                .isSupported();

            await sleep(150);
            await tempHtml5QrcodeScanner.stop();
            await sleep(150);
            tempHtml5QrcodeScanner.clear();

            const camera = {
                label: device.label,
                torch_supported: torchSupported
            };

            // default camera
            if (defaultCamera === null) {
                defaultCamera = camera;
            }

            // first user camera
            if (isUserCamera(device) && userCamera === null) {
                userCamera = camera;
            }

            // environment camera, preferably with torch
            if ((isEnvironmentCamera(device) && userCamera === null) || torchSupported) {
                environmentCamera = camera;
            }
        }
    } catch (error) {
        console.error(error)
    }
    return [userCamera ?? defaultCamera, environmentCamera ?? defaultCamera];
}

export async function destroyHtml5QrcodeScanner() {
    if (activeHtml5QrcodeScanner) {
        await activeHtml5QrcodeScanner.stop();
        activeHtml5QrcodeScanner = null;
    }
}

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

// ---