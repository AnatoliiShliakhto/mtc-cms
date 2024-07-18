use dioxus::prelude::*;
use crate::element::editor::FieldProps;

#[component]
pub fn HtmlField(props: FieldProps) -> Element {
    let script = [
        r#"
        import {
            ClassicEditor,
            Essentials,
            Paragraph,
            Heading,
            BlockQuote,
            Bold,
            Italic,
            Font,
            Link,
            List,
            MediaEmbed,
            RemoveFormat,
            ShowBlocks,
            SourceEditing,
            Indent,
            IndentBlock
        } from 'ckeditor5';

        ClassicEditor
            .create( document.querySelector( '#"#,
        &props.slug,
        r#"' ), {
        plugins: [
            Essentials,
            Paragraph,
            Heading,
            BlockQuote,
            Bold,
            Italic,
            Font,
            Link,
            List,
            MediaEmbed,
            RemoveFormat,
            ShowBlocks,
            SourceEditing,
            Indent,
            IndentBlock
        ],
        toolbar: [
            'heading',
            '|',
            'bold',
            'italic',
            'fontSize',
            'fontFamily',
            'fontColor',
            '|',
            'link',
            'bulletedList',
            'numberedList',
            'blockQuote',
            '|',
            'mediaEmbed',
            'removeFormat', 'showBlocks', 'sourceEditing',
            'outdent', 'indent'
            ],
        heading: {
            options: [
                { model: 'paragraph', title: 'Paragraph', class: 'ck-heading_paragraph' },
                { model: 'heading1', view: 'h1', title: 'Heading 1', class: 'ck-heading_heading1' },
                { model: 'heading2', view: 'h2', title: 'Heading 2', class: 'ck-heading_heading2' }
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
            textarea {
                id: props.slug.clone(),
                name: props.slug,
                class: "w-full rounded textarea textarea-bordered",
                dangerous_inner_html: props.value,
            }
        }
        script { r#type: "module", { script } }
    }
}
