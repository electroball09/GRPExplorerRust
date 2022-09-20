use std::ops::Index;
use std::convert::Into;

#[derive(Debug)]
pub struct IndAcc<'a, T, Idx> where T: Index<Idx>, Idx: Sized + Copy {
    indexable: &'a T,
    index: Idx
}

impl <'a, T, Idx> IndAcc<'a, T, Idx> where T: Index<Idx>, Idx: Sized + Copy {
    pub fn new(indexable: &'a T, index: Idx) -> Self {
        IndAcc {
            indexable,
            index
        }
    }

    pub fn get(&self) -> &<T as Index<Idx>>::Output where <T as Index<Idx>>::Output: Sized{
        &self.indexable[self.index]
    }
}