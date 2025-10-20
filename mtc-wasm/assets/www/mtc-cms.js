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

window.downloadDirectory = window.location.hostname;
window.platform ??= 'web';
window.editorInstances ??= [];
window.contentId ??= '';

document.addEventListener('contextmenu', (event) => event.preventDefault());

const tauri = window.__TAURI__ || undefined;

if (tauri) {
    (async () => {
        try {
            window.platform = (await tauri.core.invoke('get_platform')) || 'web';
            window.downloadDirectory =
                (await tauri.path.downloadDir()) + '/' + window.downloadDirectory + '/' ||
                './' + window.downloadDirectory + '/';
            await tauri.fs.mkdir(window.downloadDirectory, {recursive: true});
            ScreenAlwaysOn(true);
        } catch (err) {
            console.error(err);
        }
    })();
}

class ImageUploadAdapter {
    constructor(loader) {
        this.loader = loader;
    }

    upload() {
        const api_url = `${window.location.origin}/api/storage/public/${window.contentId}`;

        return this.loader.file.then((file) =>
            new Promise((resolve, reject) => {
                const data = new FormData();
                data.append('file', file);

                fetch(api_url, {
                    method: 'POST',
                    body: data,
                    mode: 'cors',
                    credentials: 'include',
                })
                    .then(() =>
                        resolve({
                            default: `/public/${window.contentId}/${file.name}`,
                        })
                    )
                    .catch(reject);
            })
        );
    }

    abort() {
    }
}

function ImageUploadAdapterPlugin(editor) {
    editor.plugins.get('FileRepository').createUploadAdapter = (loader) =>
        new ImageUploadAdapter(loader);
}

window.CkEditorCreate = (element) => {
    ClassicEditor.create(document.querySelector(element), {
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
            Image,
            ImageInsert,
            ImageToolbar,
            ImageResize,
            ImageStyle,
        ],
        toolbar: {
            items: [
                'undo',
                'redo',
                '|',
                'paragraph',
                'heading1',
                'heading2',
                'heading3',
                'heading4',
                '|',
                'bold',
                'italic',
                'underline',
                '|',
                'alignment:left',
                'alignment:right',
                'alignment:center',
                'alignment:justify',
                '|',
                'fontSize',
                'fontFamily',
                'fontColor',
                '-',
                'bulletedList',
                'numberedList',
                'blockQuote',
                '|',
                'outdent',
                'indent',
                '|',
                'insertTable',
                'link',
                'insertImage',
                'mediaEmbed',
                '|',
                'removeFormat',
                'showBlocks',
                'sourceEditing',
            ],
            shouldNotGroupWhenFull: true,
        },
        image: {
            toolbar: [
                'imageStyle:wrapText',
                'imageStyle:breakText',
                '|',
                'imageStyle:block',
                'imageStyle:inline',
                'resizeImage',
            ],
            resizeOptions: [
                {
                    name: 'resizeImage:original',
                    value: null,
                    label: 'Original',
                },
                {
                    name: 'resizeImage:custom',
                    label: 'Custom',
                    value: 'custom',
                },
                {
                    name: 'resizeImage:25',
                    value: '25',
                    label: '25%',
                },
                {
                    name: 'resizeImage:50',
                    value: '50',
                    label: '50%',
                },
            ],
        },
        language: {
            ui: 'uk',
        },
        heading: {
            options: [
                {model: 'paragraph', title: 'Paragraph', class: 'ck-heading_paragraph'},
                {model: 'heading1', view: 'h1', title: 'Heading 1', class: 'ck-heading_heading1'},
                {model: 'heading2', view: 'h2', title: 'Heading 2', class: 'ck-heading_heading2'},
                {model: 'heading3', view: 'h3', title: 'Heading 3', class: 'ck-heading_heading3'},
                {model: 'heading4', view: 'h4', title: 'Heading 4', class: 'ck-heading_heading4'},
            ],
        },
        htmlSupport: {
            allow: [
                {name: 'div', classes: false, styles: false, attributes: false},
                {name: /^(p|span|article|table)$/, classes: true, styles: true},
                {name: 'img', styles: true, attributes: true},
                {name: 'a', attributes: true},
                {name: 'iframe', classes: true, styles: true, attributes: true},
            ],
        },
        table: {
            contentToolbar: [
                'tableColumn',
                'tableRow',
                'mergeTableCells',
                'tableProperties',
                'tableCellProperties',
            ],
        },
        mediaEmbed: {
            previewsInData: true,
        },
    })
        .then((editor) => {
            window.editorInstances.push(editor);
        })
        .catch(console.error);
};

window.CkEditorDestroy = () => {
    while (window.editorInstances.length) {
        window.editorInstances.pop().destroy().catch(console.error);
    }
};

window.ScreenAlwaysOn = (enable) => {
    if (tauri && window.platform === 'android') {
        tauri.core.invoke('plugin:keep-screen-on|keep_screen_on', {enable});
    }
};

window.elementLinkOpen = (element) => {
    window.linkOpen(element.href);
};

window.linkOpen = (url) => {
    if (tauri) {
        tauri.core.invoke('open_in_browser', {url: decodeURI(url)});
    } else {
        const link = document.createElement('a');
        link.href = url;
        link.target = '_blank';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    }
};

window.openIfExists = async (file) => {
    if (tauri && (await tauri.fs.exists(file))) {
        const action =
            window.platform === 'android'
                ? tauri.core.invoke('plugin:view|view', {payload: {path: file}})
                : tauri.shell.open(file);
        await action;
        return true;
    }
    return false;
};

window.tauriDownloadFile = async (url, path) => {
    try {
        await tauri.core.invoke('download', {url, path});
        return true;
    } catch (err) {
        console.error(err);
        return false;
    }
};

window.linkDownloadThenOpen = async (element) => {
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
            if (await tauriDownloadFile(fileUrl, filePath)) {
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
            const link = document.createElement('a');
            document.body.appendChild(link);
            link.href = fileUrl;
            link.download = fileName;
            link.click();
            document.body.removeChild(link);
            element.classList.remove('text-error');
        } catch (err) {
            console.error(err);
            element.classList.add('text-error');
        }
    }

    element.removeChild(loader);
    loader.remove();
    element.style = '';
};