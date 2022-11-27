use log::info;
use tiny_skia::Pixmap;
use wasm_bindgen::prelude::*;

mod contracts;
mod model;
mod paint;

use model::{Color, NewShape, Point, ShapeDescription, Shapes};
use paint::{Paintable, PixmapPainter};

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
    shapes: Shapes<PixmapPainter>,
}

#[wasm_bindgen]
impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        let map = Pixmap::new(width, height).unwrap();
        let mut result = Board {
            map: map,
            shapes: model::Shapes::new(),
        };
        result.shapes.add(NewShape::<PixmapPainter>::new(
            String::from("background"),
            ShapeDescription::Fill {
                fill: Color::new(255, 40, 255, 100),
            },
            0,
        ));
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

        self.shapes
            .add(model::NewShape::<paint::PixmapPainter>::new(
                id,
                model::ShapeDescription::Line {
                    fill: model::Color::new(red, green, blue, 255),
                    from: model::Point::new(start_x, start_y),
                    to: model::Point::new(end_x, end_y),
                },
                0,
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
            shape.change_description(&mut |d| match d {
                ShapeDescription::Line {
                    fill: _,
                    from: _,
                    to: _,
                } => ShapeDescription::Line {
                    fill: Color::new(red, green, blue, 255),
                    from: Point::new(start_x, start_y),
                    to: Point::new(end_x, end_y),
                },
                _ => *d,
            });
        });
    }

    pub fn do_draw(&mut self) {
        if self.shapes.need_drawing() {
            for shape in self.shapes.iter() {
                shape.paint(&mut self.map.as_mut());
            }
            self.shapes.changes_drawed()
        }
    }
}
