use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ({
        let line_number = line!();
        let file_name = file!();
        let comment = format_args!($($t)*).to_string();
        log(&format_args!("{file_name} : {line_number} -> {comment}").to_string())
    })
}
