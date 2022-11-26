use std::collections::HashMap;

use derivative::Derivative;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use log::info;

#[derive(Derivative, Getters, Dissolve, new)]
#[derivative(Debug, Copy, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

#[derive(Derivative, Getters, Dissolve, new)]
#[derivative(Debug, Copy, Clone)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Derivative, Getters, Dissolve, new)]
#[derivative(Debug, Clone)]
pub struct Shape {
    id: String,
    fill: Color,
    path: Vec<Point>,
}

impl Shape {
    pub fn set_fill(&mut self, color: Color) {
        self.fill = color
    }
    pub fn reset_path(&mut self, new_path: Vec<Point>) {
        self.path = new_path
    }
}

pub struct Shapes {
    shapes: HashMap<String, Shape>,
    changed: bool,
}

impl IntoIterator for Shapes {
    type Item = Shape;

    type IntoIter = std::collections::hash_map::IntoValues<String, Shape>;

    fn into_iter(self) -> Self::IntoIter {
        self.shapes.into_values()
    }
}

impl Shapes {
    pub fn new() -> Shapes {
        Shapes {
            shapes: HashMap::<String, Shape>::new(),
            changed: false,
        }
    }

    pub fn add(&mut self, shape: Shape) {
        let res = self.shapes.insert(shape.id.clone(), shape);
        if let Some(previous) = res {
            info!("Drop existing shape {}", format!("{previous:?}"));
        }
        self.changed = true
    }

    pub fn change<F>(&mut self, key: &String, f: &mut F)
    where
        F: FnMut(&mut Shape),
    {
        let value = self.shapes.get_mut(key);
        if let Some(actualValue) = value {
            f(actualValue);
            self.changed = true
        } else {
            info!("No shape with key {}", key)
        }
    }

    pub fn generate_id(&self) -> String {
        let size = self.shapes.len();
        format!("{size:?}")
    }
    pub fn iter(&self) -> std::collections::hash_map::Values<String, Shape> {
        self.shapes.values()
    }

    pub fn need_drawing(&self) -> bool {
        self.changed
    }

    pub fn changes_drawed(&mut self) {
        self.changed = false
    }
}
