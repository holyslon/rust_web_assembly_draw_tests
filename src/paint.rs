use log::info;
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, PixmapMut, Stroke, Transform};

pub trait Paintable {
    fn paint(&self, map: &mut PixmapMut);
}

impl super::model::Color {
    pub fn to_tiny_skia(&self) -> Color {
        Color::from_rgba8(*self.red(), *self.green(), *self.blue(), *self.alpha())
    }
}

#[derive(Debug, Clone)]
pub struct PixmapPainter {
    pixmap: Pixmap,
}

impl Default for PixmapPainter {
    fn default() -> Self {
        Self {
            pixmap: Pixmap::new(100, 100).unwrap(),
        }
    }
}

impl super::model::NeedToDraw for PixmapPainter {
    fn need_drawing(&self) -> bool {
        return true;
    }

    fn changes_drawed(&mut self) {}
}

impl super::model::Point {
    fn x_f32(&self) -> f32 {
        *self.x() as f32
    }
    fn y_f32(&self) -> f32 {
        *self.y() as f32
    }
}

impl Paintable for super::model::NewShape<PixmapPainter> {
    fn paint(&self, map: &mut PixmapMut) {
        match self.description() {
            crate::model::ShapeDescription::Line { fill, from, to } => {
                let mut paint = Paint::default();
                paint.set_color(fill.to_tiny_skia());
                paint.anti_alias = false;
                let path = {
                    let mut pb = PathBuilder::new();
                    pb.move_to(from.x_f32(), from.y_f32());
                    pb.line_to(to.x_f32(), to.y_f32());
                    pb.finish().unwrap()
                };

                let res = map.stroke_path(
                    &path,
                    &paint,
                    &Stroke::default(),
                    Transform::identity(),
                    None,
                );
                if res.is_none() {
                    info!("Fail to render path")
                }
            }
            crate::model::ShapeDescription::Fill { fill } => map.fill(fill.to_tiny_skia()),
        };
    }
}
