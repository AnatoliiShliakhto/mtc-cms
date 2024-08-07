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
            Table,
            TableToolbar,
            TableProperties,
            TableCellProperties,
            TableColumnResize,
            PasteFromOffice,
            Image, ImageInsert,
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
            Table,
            TableToolbar,
            TableProperties,
            TableCellProperties,
            TableColumnResize,
            PasteFromOffice,

            FileRepository,
            ImageUploadAdapterPlugin,
            Image, ImageInsert,
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
            ]
        },
        htmlSupport: {
            allow: [
                { name: /^(div|p|span|article)$/, classes: true },
                { name: 'img', styles: true, attributes:true },
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
                    dangerous_inner_html: props.value.as_str(),
                }
            }
        }
        script { r#type: "module", { script } }
    }
}
