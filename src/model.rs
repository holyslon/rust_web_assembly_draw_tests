use std::{collections::HashMap, fmt::Debug};

use derivative::Derivative;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use log::info;
use sorted_vec::SortedVec;

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
#[derive(Debug, Clone, Copy)]
pub enum ShapeDescription {
    Line { fill: Color, from: Point, to: Point },
    Fill { fill: Color },
}

pub trait NeedToDraw {
    fn need_drawing(&self) -> bool;

    fn changes_drawed(&mut self);
}

#[derive(Debug, Clone)]
pub struct NewShape<T: NeedToDraw + Default + Debug + Clone> {
    holder: T,
    id: String,
    description: ShapeDescription,
    changed: bool,
    z_order: i64,
}

impl<T: NeedToDraw + Default + Debug + Clone> PartialEq for NewShape<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: NeedToDraw + Default + Debug + Clone> PartialOrd for NewShape<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: NeedToDraw + Default + Debug + Clone> Eq for NewShape<T> {}

impl<T: NeedToDraw + Default + Debug + Clone> Ord for NewShape<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z_order
            .cmp(&other.z_order)
            .then(self.id.cmp(&other.id))
    }
}

impl<T: NeedToDraw + Default + Debug + Clone> NewShape<T> {
    pub fn new(id: String, description: ShapeDescription, z_order: i64) -> NewShape<T> {
        NewShape {
            holder: Default::default(),
            id: id,
            description: description,
            changed: true,
            z_order: z_order,
        }
    }

    pub fn change_description<F: Fn(&ShapeDescription) -> ShapeDescription>(&mut self, f: &mut F) {
        self.description = f(&self.description);
        self.changed = true;
    }

    pub fn description(&self) -> &ShapeDescription {
        &self.description
    }
}

impl<T: NeedToDraw + Default + Debug + Clone> NeedToDraw for NewShape<T> {
    fn need_drawing(&self) -> bool {
        if self.changed {
            return true;
        }
        self.holder.need_drawing()
    }

    fn changes_drawed(&mut self) {
        self.changed = false;
        self.holder.changes_drawed();
    }
}

pub struct Shapes<T: NeedToDraw + Default + Debug + Clone> {
    data: SortedVec<NewShape<T>>,
    changed: bool,
}

impl<T: NeedToDraw + Default + Debug + Clone> Shapes<T>
where
    T: NeedToDraw,
{
    pub fn new() -> Shapes<T> {
        Shapes {
            data: SortedVec::<NewShape<T>>::new(),
            changed: false,
        }
    }

    pub fn add(&mut self, shape: NewShape<T>) {
        let index = self.data.insert(shape);
        self.changed = true
    }

    pub fn change<F>(&mut self, key: &String, f: &mut F)
    where
        F: FnMut(&mut NewShape<T>),
    {
        match self.find_shape_by_key(key) {
            Some(index) => {
                let ref mut p = self.data;
                self.changed = p.mutate_vec(|data| {
                    let v = data.get_mut(index);
                    match v {
                        Some(value) => {
                            f(value);
                            return true;
                        }
                        _ => return false,
                    };
                });
            }
            None => info!("No shape with key {}", key),
        }
    }

    fn find_shape_by_key(&self, key: &String) -> Option<usize> {
        for (index, shape) in self.data.iter().enumerate() {
            if shape.id.eq(key) {
                return Some(index);
            }
        }
        return None;
    }

    pub fn remove(&mut self, key: &String) {
        match self.find_shape_by_key(key) {
            Some(_) => return,
            None => info!("Remove not existing key {}", key),
        }
    }

    pub fn generate_id(&self) -> String {
        let size = self.data.len();
        format!("{size:?}")
    }
    pub fn iter(&self) -> core::slice::Iter<'_, NewShape<T>> {
        self.data.iter()
    }

    pub fn need_drawing(&self) -> bool {
        self.changed
    }

    pub fn changes_drawed(&mut self) {
        self.changed = false
    }
}
