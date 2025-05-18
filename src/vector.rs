use anyhow::Result;
use std::ops::{Add, AddAssign, Deref, Mul};
pub struct Vector<T> {
    data: Vec<T>,
}

// pretend this is a heavy operation, CPU-intensive
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default,
{
    // if Vector impl Deref -> so a.len => a.data.len()
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("Vector size mismatch"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }

    // pub fn len(&self) -> usize {
    //     self.data.len()
    // }

    // pub fn iter(&self) -> std::slice::Iter<T> {
    //     self.data.iter()
    // }
}

// get index for Vector
// impl<T> Index<usize> for Vector<T> {
//     type Output = T;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.data[index]
//     }
// }

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
