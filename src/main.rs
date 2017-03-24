extern crate ds_collection;

use ds_collection::Set;
use ds_collection::dynamic_arr_set::DynamicArraySet;

fn main() {
    let mut set = DynamicArraySet::new();
    set.insert(1);
    set.insert(5);
    set.insert(2);
    set.insert(4);
    set.insert(3);
    println!("{:?}", set);
}
