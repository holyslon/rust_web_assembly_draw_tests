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
    #[serde(default)]
    fill: Option<Color>,
    #[serde(default)]
    from: Option<Point>,
    #[serde(default)]
    to: Option<Point>,
}

#[cfg(test)]
mod change_shape_dto_tests {
    #[test]
    fn test_id_and_to() {
        let data = r#"
        {
            "id": "some_id",

            "to": {
                "x":1,
                "y":2
            }
        }"#;
        let result: Result<super::ChangeShapeDto, serde_json::Error> = serde_json::from_str(data);

        match result {
            Ok(actual) => {
                assert_eq!(actual.id, "some_id");
                match actual.to {
                    Some(point) => {
                        assert_eq!(point.x, 1);
                        assert_eq!(point.y, 2);
                    }
                    None => panic!("Fail to deserialize 'to'"),
                }
            }
            Err(error) => panic!("Deserialize failed {}", error),
        };
    }

    #[test]
    fn test_id_only() {
        let data = r#"
        {
            "id": "some_id"
        }"#;
        let result: Result<super::ChangeShapeDto, serde_json::Error> = serde_json::from_str(data);

        match result {
            Ok(actual) => assert_eq!(actual.id, "some_id"),
            Err(error) => panic!("Deserialize failed {}", error),
        };
    }
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

#[cfg(test)]
mod batch_request_tests {
    #[test]
    fn test_change_with_id_and_to() {
        let data = r#"{"add":[],"change":[{"id":"0","to":{"x":1,"y":0}}],"remove":[]}"#;
        let result: Result<super::BatchRequest, serde_json::Error> = serde_json::from_str(data);

        match result {
            Ok(actual) => {
                assert_eq!(actual.add.len(), 0);
                assert_eq!(actual.remove.len(), 0);
                assert_eq!(actual.change.len(), 1);
                let change = &actual.change[0];
                assert_eq!(change.id, "0");
                match &change.to {
                    Some(point) => {
                        assert_eq!(point.x, 1);
                        assert_eq!(point.y, 0);
                    }
                    None => panic!("Fail to deserialize 'to'"),
                }
            }
            Err(error) => panic!("Deserialize failed {}", error),
        };
    }

    #[test]
    fn test_id_only() {
        let data = r#"
        {
            "id": "some_id"
        }"#;
        let result: Result<super::ChangeShapeDto, serde_json::Error> = serde_json::from_str(data);

        match result {
            Ok(actual) => assert_eq!(actual.id, "some_id"),
            Err(error) => panic!("Deserialize failed {}", error),
        };
    }
}
