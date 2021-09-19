use crate::bin_tree::BinTree;

#[allow(dead_code)]
mod bin_tree {
    #[derive(Debug)]
    pub struct BinTree<T> {
        pub left: Option<Box<BinTree<T>>>,
        pub right: Option<Box<BinTree<T>>>,
        pub value: T
    }

    impl<T> BinTree<T> {
        pub fn new(val: T) -> BinTree<T> {
            BinTree{left: None, right: None, value: val}
        }

        // Create and add a new node to the left
        pub fn add_left(&mut self, val: T) {
            self.left = Some(Box::new(BinTree{left: None, right: None, value: val}));
        }

        // Create and add a new node to the right
        pub fn add_right(&mut self, val: T) {
            self.right = Some(Box::new(BinTree{left: None, right: None, value: val}));
        }

        // Append an existing node to the left
        pub fn append_left(&mut self, val: BinTree<T>) {
            self.left = Some(Box::new(val));
        }

        // Append an existing node to the right
        pub fn append_right(&mut self, val: BinTree<T>) {
            self.right = Some(Box::new(val));
        }


        // Remove the node to the left
        pub fn remove_left(&mut self) {
            self.left = None;
        }

        // Remove the node to the right
        pub fn remove_right(&mut self) {
            self.right = None;
        }
    }

}


fn main() {
    let mut a = BinTree::new(0);

    let b = BinTree::new(1);

    a.append_left(b);
    a.left.as_mut().unwrap().add_left(2);
    a.left.as_mut().unwrap().add_right(3);

    println!("{:#?}", a);

    a.left.as_mut().unwrap().remove_left();
    println!("\n\n\n{:#?}", a);

    a.remove_left();
    println!("\n\n\n{:#?}", a);
}
