use crate::bin_tree::BinTree;

#[allow(dead_code)]
#[allow(unused_macros)]
mod bin_tree {
    #[macro_export]
    macro_rules! node {
        (value: $e:expr) => {
            Some(Box::new(BinTree::new($e)))
        };

        (value: $v:expr, left: $n:expr) => {
            Some(Box::new(BinTree{value: $v, left: $n, right: None}))
        };

        (value: $v:expr, right: $n:expr) => {
            Some(Box::new(BinTree{value: $v, left: None, right: $n}))
        };

        (value: $v:expr, left: $ln:expr, right: $rn:expr) => {
            Some(Box::new(BinTree{value: $v, left: $ln, right: $rn}))
        };

        (value: $v:expr, right: $ln:expr, left: $rn:expr) => {
            Some(Box::new(BinTree{value: $v, left: $ln, right: $rn}))
        };

        (append: $n:expr) => {
            Some(Box::new($n))
        }
    }

    #[derive(Debug)]
    pub struct BinTree<T> {
        pub value: T,
        pub left: Option<Box<BinTree<T>>>,
        pub right: Option<Box<BinTree<T>>>
    }

    impl<T> BinTree<T> {
        pub fn new(val: T) -> BinTree<T> {
            BinTree{value: val, left: None, right: None}
        }

        // Create and add a new node to the left
        pub fn add_left(&mut self, val: T) {
            self.left = Some(Box::new(BinTree{value: val, left: None, right: None}));
        }

        // Create and add a new node to the right
        pub fn add_right(&mut self, val: T) {
            self.right = Some(Box::new(BinTree{value: val, left: None, right: None}));
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

    let c = node!{
        value: 4,

        left: node!{
            value: 5
        },

        right: node!{
            value: 6,

            left: node!{
                value: 7
            },

            right: node!{
                value: 8,
                left: node!(append: a) // appending already existing BinTree
            }
        }
    }.unwrap();

    println!("\n\n\n{:#?}", c);
}
