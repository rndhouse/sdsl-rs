use anyhow::Result;
pub mod io;
pub mod util;

pub type VoidPtr = *const libc::c_void;

pub trait Ptr {
    fn ptr(&self) -> &VoidPtr;
}

pub trait Id: ParameterValues {
    fn id() -> Result<String>;
}

pub trait Code: ParameterValues {
    fn c_code() -> Result<String>;
}

pub trait ParameterValues {
    fn parameter_values() -> Result<Vec<String>>;
}

pub trait IterGet {
    fn iter_get(&self, index: usize) -> usize;
}

pub struct VectorIterator<'a, T: IterGet> {
    vector: &'a T,
    len: usize,
    index: usize,
}

impl<'a, T: IterGet> VectorIterator<'a, T> {
    pub fn new(vector: &'a T, len: usize) -> Self {
        Self {
            vector,
            len,
            index: 0,
        }
    }
}

impl<'a, T: IterGet> Iterator for VectorIterator<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let result = if self.index < self.len {
            Some(self.vector.iter_get(self.index))
        } else {
            None
        };
        self.index += 1;
        result
    }
}

pub struct VectorIntoIterator<T: IterGet> {
    vector: T,
    len: usize,
    index: usize,
}

impl<T: IterGet> VectorIntoIterator<T> {
    pub fn new(vector: T, len: usize) -> Self {
        Self {
            vector,
            len,
            index: 0,
        }
    }
}

impl<T: IterGet> Iterator for VectorIntoIterator<T> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let result = if self.index < self.len {
            Some(self.vector.iter_get(self.index))
        } else {
            None
        };
        self.index += 1;
        result
    }
}
