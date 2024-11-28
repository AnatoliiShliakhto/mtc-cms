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

window.downloadDirectory = '242 цпп';
window.platform = window.platform || 'web';
window.editorInstances = window.editorInstances || [];
window.contentId = window.contentId || '';

const tauri = window.__TAURI__ || undefined;

if (tauri) {
    try {
        window.platform = await tauri.core.invoke('get_platform') || 'web';
        window.downloadDirectory = await tauri.path.downloadDir() + '/' + window.downloadDirectory + '/'
            || './' + window.downloadDirectory + '/';
        await tauri.fs.mkdir(window.downloadDirectory, {recursive: true});
        ScreenAlwaysOn(true);
    } catch (err) {
        console.error(err);
    }
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
                    mode: 'cors',
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

window.ScreenAlwaysOn = function(enable) {
    if (tauri && window.platform === 'android') {
        tauri.core.invoke('plugin:keep-screen-on|keep_screen_on', {
                enable: enable
            }
        );
    }
}

window.linkOpen = function(element) {
    if (tauri) {
        tauri.core.invoke('open_in_browser', { url: decodeURI(element.href) });
    } else {
        const link = document.createElement("a");
        document.body.appendChild(link);
        link.href = element.href;
        link.target = '_blank';
        link.click();
        document.body.removeChild(link);
    }
}

window.openIfExists = async function(file) {
    if (tauri && await tauri.fs.exists(file)) {
        if (window.platform === 'android') {
            //open by Intent for Android
            tauri.core.invoke('plugin:view|view', {payload: {path: file}});
        } else {
            //open by default shell APP
            tauri.shell.open(file);
        }
        return true;
    }
    return false;
}

window.downloadFile = async function(url, path) {
    try {
        await tauri.core.invoke('download', {url: url, path: path});
    } catch (err) {
        console.error( err );
        return false;
    }
    return true;
}

window.linkDownloadThenOpen = async function(element) {
    element.style = 'pointer-events: none;';

    const fileUrl = decodeURI(element.href);
    const fileName = fileUrl.replace(/^.*[\\\/]/, '');
    const filePath = window.downloadDirectory + fileName;

    if (tauri) {
        if (await openIfExists(filePath)) {
            element.style = '';
            return;
        }
    }

    const loader = document.createElement('span');
    loader.className = 'loading loading-spinner loading-xs mr-3 text-primary';
    element.insertBefore(loader, element.firstChild);

    if (tauri) {
        try {
            if (await downloadFile(fileUrl, filePath)) {
                await openIfExists(filePath);
                element.classList.remove('text-error');
            } else {
                element.classList.add('text-error');
            }
        } catch (error) {
            console.error( error );
            element.classList.add('text-error');
        }
    } else {
        try {
            const response = await fetch(fileUrl, {mode: 'cors', credentials: 'include'});
            if (response.ok) {
                const link = document.createElement('a');
                document.body.appendChild(link);
                let blob = await response.blob();
                let urlObj = window.URL.createObjectURL(blob)
                link.href = urlObj;
                link.download = fileName;
                link.click();
                window.URL.revokeObjectURL(urlObj)
                document.body.removeChild(link);
                element.classList.remove('text-error');
            } else {
                element.classList.add('text-error');
            }
        } catch (err) {
            console.error(err);
            element.classList.add('text-error');
        }
    }

    element.removeChild(loader);
    loader.remove();
    element.style = '';
}