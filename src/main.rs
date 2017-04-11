extern crate ds_collection;

use ds_collection::Set;
use ds_collection::red_black_tree::RedBlackTree;

fn main() {
    let mut set = RedBlackTree::new();
    set.insert(1);
    set.insert(5);
    set.insert(2);
    set.insert(4);
    set.insert(3);

    // let set = RedBlackTree::new_test();

    println!("{}", set);
}
