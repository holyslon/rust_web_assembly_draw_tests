use serde::Deserialize;

#[derive(Deserialize)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Into<super::model::Color> for Color {
    fn into(self) -> super::model::Color {
        super::model::Color::new(self.red, self.green, self.blue, self.alpha)
    }
}

#[derive(Deserialize)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Into<super::model::Point> for Point {
    fn into(self) -> super::model::Point {
        super::model::Point::new(self.x, self.y)
    }
}

#[derive(Deserialize)]
pub struct ShapeId {
    id: String,
}

#[derive(Deserialize)]
pub struct ShapeDto {
    id: ShapeId,
    fill: Color,
    from: Point,
    to: Point,
}

impl Into<super::model::Shape> for ShapeDto {
    fn into(self) -> super::model::Shape {
        super::model::Shape::new(
            self.id.id,
            self.fill.into(),
            vec![self.from.into(), self.to.into()],
        )
    }
}

#[derive(Deserialize)]
pub struct ChangeShapeDto {
    id: ShapeId,
    fill: Option<Color>,
    from: Option<Point>,
    to: Option<Point>,
}

#[derive(Deserialize)]
pub struct BatchRequest {
    add: Vec<ShapeDto>,
    remove: Vec<ShapeId>,
    change: Vec<ChangeShapeDto>,
}
