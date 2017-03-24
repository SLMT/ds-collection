use Set;

// A sorted dynamic array.
#[derive(Debug)]
pub struct DynamicArraySet {
    values: Vec<i32>
}

impl Set for DynamicArraySet {
    // create a set
    fn new() -> Self {
        DynamicArraySet {
            values: Vec::new()
        }
    }

    // is x in the set
    fn member(&self, x: i32) -> bool {
        for val in &self.values {
            if *val == x {
                return true;
            }
        }
        false
    }

    // the integer in the set that is just smaller than x
    fn predecessor(&self, x: i32) -> Option<i32> {
        let mut answer: Option<i32> = None;
        for val in &self.values {
            if *val == x {
                break;
            }
            answer = Some(*val);
        }
        answer
    }

    // # of integers in the set smaller than or equal to x
    fn rank(&self, x: i32) -> usize {
        let mut count: usize = 0;
        for val in &self.values {
            if *val == x {
                break;
            }
            count += 1;
        }
        count
    }

    // the j-th smallest integer in the set
    fn select(&self, j: usize) -> Option<i32> {
        if j < self.values.len() {
            Some(self.values[j])
        } else {
            None
        }
    }

    // insert x into the set
    fn insert(&mut self, x: i32) {
        for idx in 0..self.values.len() {
            if x == self.values[idx] {
                return;
            } else if x < self.values[idx] {
                self.values.insert(idx, x);
                return;
            }
        }

        // Empty or larger than all values
        self.values.push(x);
    }

    // delete x in the set
    fn delete(&mut self, x: i32) {
        for idx in 0..self.values.len() {
            if self.values[idx] == x {
                self.values.remove(idx);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use Set;
    use dynamic_arr_set::DynamicArraySet;

    #[test]
    fn test_member() {
        let set = create_testing_data();

        assert_eq!(set.member(3), true);
        assert_eq!(set.member(6), false);
    }

    #[test]
    fn test_predecessor() {
        let set = create_testing_data();

        assert_eq!(set.predecessor(1), None);
        assert_eq!(set.predecessor(5), Some(4));
    }

    #[test]
    fn test_rank() {
        let set = create_testing_data();

        assert_eq!(set.rank(1), 0);
        assert_eq!(set.rank(3), 2);
    }

    #[test]
    fn test_select() {
        let set = create_testing_data();

        assert_eq!(set.select(0), Some(1));
        assert_eq!(set.select(3), Some(4));
    }

    #[test]
    fn test_delete1() {
        let mut set = create_testing_data();

        assert_eq!(set.select(2), Some(3));
        set.delete(3);
        assert_eq!(set.select(2), Some(4));
    }

    #[test]
    fn test_delete2() {
        let mut set = create_testing_data();

        assert_eq!(set.select(4), Some(5));
        set.delete(6);
        assert_eq!(set.select(4), Some(5));
    }

    fn create_testing_data() -> DynamicArraySet {
        let mut set = DynamicArraySet::new();

        // insert data
        set.insert(1);
        set.insert(5);
        set.insert(2);
        set.insert(4);
        set.insert(3);

        set
    }
}
