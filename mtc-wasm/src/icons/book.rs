use super::*;

pub fn BookIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            "stroke-linejoin": "miter",
            "stroke-width": "0",
            "viewBox": "0 0 24 24",
            "fill": "currentColor",
            "stroke": "none",
            "xmlns": "http://www.w3.org/2000/svg",
            "stroke-linecap": "butt",
            path { 
                "d": "M11,7h2v2h-2V7z M11,11h2v6h-2V11z M12,2C6.48,2,2,6.48,\
                2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20 c-4.41,0-8-3.\
                59-8-8s3.59-8,8-8s8,3.59,8,8S16.41,20,12,20z" 
            }
        }
    }
} 