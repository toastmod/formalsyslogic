use std::ops::{Add, Div, Sub, Mul};

pub struct Set<T: Add + Mul + Div + Sub> {
    elems: Box<[T]>,
    constraints: Vec<Box<dyn FnMut(T) -> bool>>

}

impl<T: Add + Mul + Div + Sub> Set<T> {
    pub fn add_constraint(&mut self, f: &mut dyn FnMut(T) -> bool) {
        let mut nullf = Box::new(|x: T|{
            false
        });

        std::mem::swap(nullf.as_mut(), f);
        self.constraints.push(nullf);
    }
}

impl<T: Add + Mul + Div + Sub> Add for Set<T> {
    type Output = Set<T>;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}



// impl<T: Add + Mul + Div + Sub>  for Set<T> {

// }