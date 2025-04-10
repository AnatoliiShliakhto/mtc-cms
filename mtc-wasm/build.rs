 use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // modify assets/mtc-cms.js
    let js_asset_path = Path::new("../assets").join("mtc-cms.js");
    let js_asset = fs::read_to_string(&js_asset_path).unwrap();
    let js_asset = js_asset.lines()
        .map(|line| {
        if line.contains("window.downloadDirectory = '") {
            format!(r#"window.downloadDirectory = '{}';"#, env!("DOWNLOAD_DIR"))
        } else {
            line.to_string()
        }
    }).collect::<Vec<String>>()
        .join("\r\n");
    fs::write(&js_asset_path, js_asset).unwrap();
}