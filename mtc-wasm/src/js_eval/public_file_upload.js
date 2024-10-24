let api_url = '/api/storage/public/' + window.contentId;
let formData = new FormData();
let fileInput = document.getElementById('fileUpload');
let file = fileInput.files[0];

formData.append('file', file);

let xhr = new XMLHttpRequest();

xhr.upload.addEventListener('upload_progress', function (event) {
    if (event.lengthComputable) {
        let percent = Math.round((event.loaded / event.total) * 100);
        dioxus.send(percent);
    }
});

xhr.addEventListener('load', function (event) {
    dioxus.send(event.target.responseText);
});

xhr.open('POST', api_url, true);
xhr.send(formData);