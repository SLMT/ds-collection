extern crate ds_collection;

use ds_collection::Set;
use ds_collection::array_set::ArraySet;

fn main() {
    let mut set = ArraySet::new();
    set.insert(1);
    set.insert(5);
    set.insert(2);
    set.insert(4);
    set.insert(3);
    println!("{:?}", set);
}
