use log::info;
use tiny_skia::{Color, Paint, PathBuilder, PixmapMut, Stroke, Transform};

pub trait Paintable {
    fn paint(&self, map: &mut PixmapMut);
}

impl super::model::Color {
    pub fn to_tiny_skia(&self) -> Color {
        Color::from_rgba8(*self.red(), *self.green(), *self.blue(), *self.alpha())
    }
}

impl super::model::Point {
    fn x_f32(&self) -> f32 {
        *self.x() as f32
    }
    fn y_f32(&self) -> f32 {
        *self.y() as f32
    }
}

impl Paintable for super::model::Shape {
    fn paint(&self, map: &mut PixmapMut) {
        let mut paint = Paint::default();
        paint.set_color(self.fill().to_tiny_skia());
        paint.anti_alias = true;
        let path = {
            let mut pb = PathBuilder::new();
            let path = self.path();
            let first = path[0];
            pb.move_to(first.x_f32(), first.y_f32());
            for point in &path[1..] {
                pb.line_to(point.x_f32(), point.y_f32());
            }
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
}
