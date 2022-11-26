use serde::Deserialize;

#[derive(Deserialize)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Into<super::model::Color> for &Color {
    fn into(self) -> super::model::Color {
        super::model::Color::new(self.red, self.green, self.blue, self.alpha)
    }
}

#[derive(Deserialize)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Into<super::model::Point> for &Point {
    fn into(self) -> super::model::Point {
        super::model::Point::new(self.x, self.y)
    }
}

#[derive(Deserialize)]
pub struct ShapeDto {
    id: String,
    fill: Color,
    from: Point,
    to: Point,
}

impl Into<super::model::Shape> for &ShapeDto {
    fn into(self) -> super::model::Shape {
        super::model::Shape::new(
            self.id.clone(),
            (&self.fill).into(),
            vec![(&self.from).into(), (&self.to).into()],
        )
    }
}

#[derive(Deserialize)]
pub struct ChangeShapeDto {
    id: String,
    fill: Option<Color>,
    from: Option<Point>,
    to: Option<Point>,
}

#[derive(Deserialize)]
pub struct BatchRequest {
    add: Vec<ShapeDto>,
    remove: Vec<String>,
    change: Vec<ChangeShapeDto>,
}

impl BatchRequest {
    pub fn apply(&self, shapes: &mut super::model::Shapes) {
        for shape in &self.add {
            shapes.add(shape.into())
        }
        for id in &self.remove {
            shapes.remove(id)
        }
        for change in &self.change {
            shapes.change(&change.id, &mut |shape| {
                if let Some(fill) = &change.fill {
                    shape.set_fill(fill.into())
                }
                if let Some(from) = &change.from {
                    let mut new_path = vec![from.into()];
                    for point in &shape.path()[1..] {
                        new_path.push(*point)
                    }
                    shape.reset_path(new_path);
                }
                if let Some(to) = &change.to {
                    let from = shape.path()[0];
                    shape.reset_path(vec![from, to.into()]);
                }
            })
        }
    }
}
