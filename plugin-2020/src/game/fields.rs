use std::iter::FromIterator;

use serde::{Serialize, Deserialize};
use crate::util::CubeCoords;

use super::Field;

/// The fields of the board. This is a separate
/// struct from Board due to the way it is represented
/// in XML.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fields {
    #[serde(rename = "field")]
    fields: Vec<Field>
}

impl Fields {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    #[inline]
    pub fn contains(&self, coords: impl Into<CubeCoords>) -> bool {
        let cube_coords = coords.into();
        self.fields.iter().any(|f| f.cube_coords() == cube_coords)
    }

    #[inline]
    pub fn get(&self, coords: impl Into<CubeCoords>) -> Option<&Field> {
        let cube_coords = coords.into();
        self.fields.iter().find(|f| f.cube_coords() == cube_coords)
    }

    #[inline]
    pub fn get_mut(&mut self, coords: impl Into<CubeCoords>) -> Option<&mut Field> {
        let cube_coords = coords.into();
        self.fields.iter_mut().find(|f| f.cube_coords() == cube_coords)
    }

    #[inline]
    pub fn insert(&mut self, coords: impl Into<CubeCoords>) {
        if !self.contains(coords) {
            self.fields.push(Field::default());
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=&Field> {
        self.fields.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut Field> {
        self.fields.iter_mut()
    }
}

impl FromIterator<Field> for Fields {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = Field> {
        Self { fields: Vec::from_iter(iter) }
    }
}

