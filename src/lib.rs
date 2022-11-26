use log::info;
use tiny_skia::Pixmap;
use wasm_bindgen::prelude::*;

mod contracts;
mod model;
mod paint;

use model::{Color, Point, Shape};
use paint::Paintable;

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
    shapes: model::Shapes,
}

#[wasm_bindgen]
impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        let map = Pixmap::new(width, height).unwrap();
        let mut result = Board {
            map: map,
            shapes: model::Shapes::new(),
        };
        result.do_draw();
        result
    }

    pub fn batch(&mut self, data: &str) {
        let result: Result<contracts::BatchRequest, serde_json::Error> = serde_json::from_str(data);
        match result {
            Ok(req) => req.apply(&mut self.shapes),
            Err(error) => info!("Fail to read batch {}, Error {}", data, error),
        };
    }

    pub fn buffer_pointer(&self) -> usize {
        self.map.data().as_ptr() as usize
    }

    pub fn buffer_size(&self) -> usize {
        self.map.data().len()
    }

    pub fn put_line(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
    ) -> String {
        let id = self.shapes.generate_id();
        let result = id.clone();

        self.shapes.add(Shape::new(
            id,
            Color::new(red, green, blue, 255),
            vec![Point::new(start_x, start_y), Point::new(end_x, end_y)],
        ));
        return result;
    }

    pub fn change_line(
        &mut self,
        id: String,
        red: u8,
        green: u8,
        blue: u8,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
    ) {
        self.shapes.change(&id, &mut |shape| {
            shape.set_fill(Color::new(red, green, blue, 255));
            shape.reset_path(vec![Point::new(start_x, start_y), Point::new(end_x, end_y)]);
        });
    }

    pub fn do_draw(&mut self) {
        if self.shapes.need_drawing() {
            self.map
                .fill(tiny_skia::Color::from_rgba8(255, 40, 255, 100));
            for shape in self.shapes.iter() {
                shape.paint(&mut self.map.as_mut());
            }
            self.shapes.changes_drawed()
        }
    }
}
