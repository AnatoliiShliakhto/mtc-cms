class ImageUploadAdapter {
    constructor(loader) {
        this.loader = loader;
    }

    upload() {
        let api_url = 'https://' + window.location.host + '/api/storage/' + sessionStorage.getItem('contentId').replaceAll('"', '');

        return this.loader.file
            .then(file => new Promise((resolve, reject) => {
                const data = new FormData();
                data.append('file', file);

                fetch(api_url, {
                    method: 'POST',
                    body: data,
                    credentials: 'include',
                })
                    .then(data => {
                        resolve({
                            default: '/public/' + sessionStorage.getItem('contentId').replaceAll('"', '') + '/' + file.name
                        });
                    })
                    .catch(error => {
                        reject(error);
                    });
            }));
    }

    abort() {
    }
}

function ImageUploadAdapterPlugin(editor) {
    editor.plugins.get('FileRepository').createUploadAdapter = (loader) => {
        return new ImageUploadAdapter(loader);
    };
}