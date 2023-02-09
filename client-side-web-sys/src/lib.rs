use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let document = window()
        .expect("There should be a window.")
        .document()
        .expect("A document should exist.");
    let body = document.body().expect("The document should have a body");

    let paragraph = document.create_element("p")?;
    paragraph.set_text_content(Some("Hello World!"));

    body.append_child(&paragraph)?;

    Ok(())
}
