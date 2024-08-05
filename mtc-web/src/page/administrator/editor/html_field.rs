use dioxus::prelude::*;
use crate::page::administrator::editor::FieldProps;

#[component]
pub fn HtmlField(props: FieldProps) -> Element {
    let script = [
        r#"                
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
            HtmlComment,
            Table,
            TableToolbar,
            TableProperties,
            TableCellProperties,
            TableColumnResize,
            PasteFromOffice,
            Image, ImageInsert, ImageToolbar, ImageResize, ImageStyle,
            FileRepository,
        } from 'ckeditor5';

        ClassicEditor
            .create( document.querySelector( '#"#,
        &props.slug,
        r#"' ), {
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
            HtmlComment,
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
                'paragraph', 'heading1', 'heading2', 'heading3', 'heading4', 'heading5', 'heading6',
                '|',
                'bold', 'italic', 'underline',
                '|',
                'alignment:left', 'alignment:right', 'alignment:center', 'alignment:justify',
                '|',
                'fontSize', 'fontFamily', 'fontColor',
                '-',
                'link',
                'bulletedList',
                'numberedList',
                'blockQuote',
                '|',
                'mediaEmbed',
                'removeFormat', 'showBlocks', 'sourceEditing',
                '|',
                'outdent', 'indent',
                'insertTable',
                'insertImage',
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
            ui: 'en',
        },
        heading: {
            options: [
                { model: 'paragraph', title: 'Paragraph', class: 'ck-heading_paragraph' },
                { model: 'heading1', view: 'h1', title: 'Heading 1', class: 'ck-heading_heading1' },
                { model: 'heading2', view: 'h2', title: 'Heading 2', class: 'ck-heading_heading2' },
                { model: 'heading3', view: 'h3', title: 'Heading 3', class: 'ck-heading_heading3' },
                { model: 'heading4', view: 'h4', title: 'Heading 4', class: 'ck-heading_heading4' },
                { model: 'heading5', view: 'h5', title: 'Heading 5', class: 'ck-heading_heading5' },
                { model: 'heading6', view: 'h6', title: 'Heading 6', class: 'ck-heading_heading6' },
            ]
        },
        htmlSupport: {
            allow: [
                { name: /^(div|p|span|article)$/, classes: true },
                { name: 'img', styles: true, attributes:true },
                { name: 'a', attributes:true },
            ],
        },
        table: {
            contentToolbar: [
                'tableColumn', 'tableRow', 'mergeTableCells',
                'tableProperties', 'tableCellProperties'
            ]
        },
        } )
        .catch( error => {
          console.error( error );
        } );
    "#,
    ]
    .concat();

    rsx! {
        label { class: "w-full form-control",
            div { class: "label",
                span { class: "label-text text-primary", { props.title } }
            }
            article { class: "prose prose-sm md:prose-base max-w-full",
                textarea {
                    id: props.slug.clone(),
                    name: props.slug,
                    dangerous_inner_html: props.value,
                }
            }
        }
        script { r#type: "module", { script } }
    }
}
