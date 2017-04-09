extern crate ds_collection;

use ds_collection::Set;
use ds_collection::basic_binary_tree::BasicBinaryTree;

fn main() {
    let mut set = BasicBinaryTree::new();
    set.insert(1);
    set.insert(5);
    set.insert(2);
    set.insert(4);
    set.insert(3);

    println!("{:?}", set);
    println!("{:?}", set.select(0));
    println!("{:?}", set.select(4));
}
