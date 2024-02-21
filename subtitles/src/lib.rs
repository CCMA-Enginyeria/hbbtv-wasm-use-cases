// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
mod subtitles;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
}

lazy_static! {
    static ref SUBTITLES: Mutex<subtitles::Subtitles> = Mutex::new(subtitles::Subtitles::new());
}

#[wasm_bindgen]
pub fn parse(xml: &str) {
    let mut subtitles = SUBTITLES.lock().unwrap();
    subtitles.load(xml);
    return;
}

#[wasm_bindgen]
pub fn updateSubtitlesForTimecode(ms: i32) {
    let subtitles = SUBTITLES.lock().unwrap();
    subtitles.update_subtitles_for_ms(ms);
    return;
}

#[wasm_bindgen]
pub fn setElementHeight(width: i32, height: i32) {
    let mut subtitles = SUBTITLES.lock().unwrap();
    subtitles.set_element_size(width, height);
    return;
}
