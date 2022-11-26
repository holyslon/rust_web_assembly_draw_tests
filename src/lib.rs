use rand::random;
use tiny_skia::Color;
use tiny_skia::Paint;
use tiny_skia::Pixmap;
use tiny_skia_path::IntSize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    use std::sync::Once;
    static SET_LOGGER: Once = Once::new();
    SET_LOGGER.call_once(|| {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
    });
}

#[wasm_bindgen]
pub struct Board {
    map: Pixmap,
}

#[wasm_bindgen]
impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        let map = Pixmap::new(width, height).unwrap();
        let mut result = Board { map: map };
        result.do_draw();
        result
    }

    pub fn buffer_pointer(&self) -> usize {
        self.map.data().as_ptr() as usize
    }

    pub fn buffer_size(&self) -> usize {
        self.map.data().len()
    }

    pub fn do_draw(&mut self) {
        self.map.fill(Color::from_rgba8(90, 90, 90, 255));
    }
}
