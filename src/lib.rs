use brotli::{BrotliDecompress};
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        use wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

cfg_if! {
    if #[cfg(feature = "debug_assertions")] {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            fn log(s: &str);
        }
    }
}



#[wasm_bindgen]
pub fn decompress(buffer: Vec<u8>) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    cfg_if! {
        if #[cfg(feature = "debug_assertions")] {
            log(&format!("`decompress` received: {:?}", buffer));
        }
    }

    let mut output = vec![];
    match BrotliDecompress(&mut &*buffer, &mut output) {
        Ok(_) => Ok(output),
        Err(e) => Err(wasm_bindgen::JsValue::from_str(&format!("{:?}", e))),
    }
}
