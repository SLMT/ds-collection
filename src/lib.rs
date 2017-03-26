
pub mod array_set;
pub mod tree;

pub trait Set {
    // create a set
    fn new() -> Self;

    // is x in the set
    fn member(&self, x: i32) -> bool;

    // the integer in the set that is just smaller than x
    fn predecessor(&self, x: i32) -> Option<i32>;

    // # of integers in the set smaller than or equal to x
    fn rank(&self, x: i32) -> usize;

    // the j-th smallest integer in the set
    fn select(&self, j: usize) -> Option<i32>;

    // insert x into the set
    fn insert(&mut self, x: i32);

    // delete x in the set
    fn delete(&mut self, x: i32);
}
