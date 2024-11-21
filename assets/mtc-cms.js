import {
    ClassicEditor,
    Essentials,
    Paragraph,
    Heading,
    HeadingButtonsUI,
    ParagraphButtonUI,
    BlockQuote,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Font,
    Alignment,
    Link,
    List,
    MediaEmbed,
    RemoveFormat,
    ShowBlocks,
    SourceEditing,
    Indent,
    IndentBlock,
    Undo,
    GeneralHtmlSupport,
    Table,
    TableToolbar,
    TableProperties,
    TableCellProperties,
    TableColumnResize,
    PasteFromOffice,
    Image, ImageInsert, ImageToolbar, ImageResize, ImageStyle,
    FileRepository,
} from '/./assets/ckeditor/ckeditor5.js';

window.downloadDirectory = '242цпп';

const tauri = window.__TAURI__ || undefined;

if (tauri) {
    window.downloadDir = window.__TAURI__.path.downloadDir || undefined;
    window.open = window.__TAURI__.shell.open || undefined;
    window.mkdir = window.__TAURI__.fs.mkdir ||  undefined;
    window.exists = window.__TAURI__.fs.exists ||  undefined;
}

class ImageUploadAdapter {
    constructor(loader) {
        this.loader = loader;
    }

    upload() {
        let api_url = 'https://localhost/api/storage/public/' + window.contentId;

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
                            default: '/public/' + window.contentId + '/' + file.name
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

window.editorInstances = window.editorInstances || [];
window.contentId = window.contentId || '';

window.CkEditorCreate= function(element) {
    ClassicEditor
        .create( document.querySelector(element), {
            plugins: [
                Essentials,
                Paragraph,
                Heading,
                HeadingButtonsUI,
                ParagraphButtonUI,
                BlockQuote,
                Bold,
                Italic,
                Underline,
                Strikethrough,
                Font,
                Alignment,
                Link,
                List,
                MediaEmbed,
                RemoveFormat,
                ShowBlocks,
                SourceEditing,
                Indent,
                IndentBlock,
                Undo,
                GeneralHtmlSupport,
                Table,
                TableToolbar,
                TableProperties,
                TableCellProperties,
                TableColumnResize,
                PasteFromOffice,

                FileRepository,
                ImageUploadAdapterPlugin,
                Image, ImageInsert, ImageToolbar, ImageResize, ImageStyle,
            ],
            toolbar: {
                items: [
                    'undo', 'redo',
                    '|',
                    'paragraph', 'heading1', 'heading2', 'heading3', 'heading4',
                    '|',
                    'bold', 'italic', 'underline',
                    '|',
                    'alignment:left', 'alignment:right', 'alignment:center', 'alignment:justify',
                    '|',
                    'fontSize', 'fontFamily', 'fontColor',
                    '-',
                    'bulletedList', 'numberedList', 'blockQuote',
                    '|',
                    'outdent', 'indent',
                    '|',
                    'insertTable', 'link', 'insertImage', 'mediaEmbed',
                    '|',
                    'removeFormat', 'showBlocks', 'sourceEditing',
                ],
                shouldNotGroupWhenFull: true,
            },
            image: {
                toolbar: [
                    'imageStyle:wrapText', 'imageStyle:breakText', '|',
                    'imageStyle:block', 'imageStyle:inline', 'resizeImage'
                ],
                resizeOptions: [
                    {
                        name: 'resizeImage:original',
                        value: null,
                        label: 'Original'
                    },
                    {
                        name: 'resizeImage:custom',
                        label: 'Custom',
                        value: 'custom'
                    },
                    {
                        name: 'resizeImage:25',
                        value: '25',
                        label: '25%'
                    },
                    {
                        name: 'resizeImage:50',
                        value: '50',
                        label: '50%'
                    }
                ],
            },
            language: {
                ui: 'uk',
            },
            heading: {
                options: [
                    { model: 'paragraph', title: 'Paragraph', class: 'ck-heading_paragraph' },
                    { model: 'heading1', view: 'h1', title: 'Heading 1', class: 'ck-heading_heading1' },
                    { model: 'heading2', view: 'h2', title: 'Heading 2', class: 'ck-heading_heading2' },
                    { model: 'heading3', view: 'h3', title: 'Heading 3', class: 'ck-heading_heading3' },
                    { model: 'heading4', view: 'h4', title: 'Heading 4', class: 'ck-heading_heading4' },
                ]
            },
            htmlSupport: {
                allow: [
                    { name: 'div', classes: false, styles: false, attributes: false },
                    { name: /^(p|span|article|table)$/, classes: true, styles: true },
                    { name: 'img', styles: true, attributes: true },
                    { name: 'a', attributes: true },
                    { name: 'iframe', classes: true, styles: true, attributes: true },
                ],
            },
            table: {
                contentToolbar: [
                    'tableColumn', 'tableRow', 'mergeTableCells',
                    'tableProperties', 'tableCellProperties'
                ]
            },
            mediaEmbed: {
                previewsInData: true
            }
        })
        .then( editor => {
            window.editorInstances.push( editor );
        })
        .catch( error => { console.error( error ); } );
}

window.CkEditorDestroy = function() {
    for (let editor of [...window.editorInstances]) {
        editor.destroy().catch( error => {
            console.log( error );
        } );
        window.editorInstances.pop();
    }
}

window.linkOpen = function(element) {
    if (tauri) {
        open(decodeURI(element.href));
    } else {
        const link = document.createElement("a");
        document.body.appendChild(link);
        link.href = element.href;
        link.target = "_blank";
        link.click();
        document.body.removeChild(link);
    }
}

async function openIfExists(file) {
    if (tauri && await exists(file)) {
        open(file);
        return true;
    }
    return false;
}

async function downloadFile(url, path) {
    try {
        return await tauri.core.invoke('download', {url: url, path: path});
    } catch (err) { console.error( err ); }
    return false;
}

window.linkDownloadThenOpen = async function(element, dir = undefined) {
    element.style = "pointer-events: none;";

    const fileUrl = decodeURI(element.href);
    const fileName = fileUrl.replace(/^.*[\\\/]/, '');
    let filePath = '';

    if (tauri) {
        filePath = await downloadDir() +
            '/' + downloadDirectory + '/' +
            (dir ? '/' + dir + '/' : '');
        if (await openIfExists(filePath + fileName)) {
            element.style = "";
            return;
        }
    }

    const loader = document.createElement("span");
    loader.className = "loading loading-spinner loading-xs mr-3 text-primary";
    element.insertBefore(loader, element.firstChild);

    if (tauri) {
        try {
            await mkdir(filePath, {recursive: true});
            await downloadFile(fileUrl, filePath + fileName);
            await openIfExists(filePath + fileName);
        } catch (error) { console.error( error ); }
    } else {
        try {
            const response = await fetch(fileUrl, {mode: 'cors', credentials: 'include'});
            if (response.ok) {
                const link = document.createElement("a");
                document.body.appendChild(link);
                let blob = await response.blob();
                let urlObj = window.URL.createObjectURL(blob)
                link.href = urlObj;
                link.download = fileName;
                link.click();
                window.URL.revokeObjectURL(urlObj)
                document.body.removeChild(link);
            }
        } catch (err) {
            console.error(err);
        }
    }

    element.removeChild(loader);
    loader.remove();
    element.style = "";
}