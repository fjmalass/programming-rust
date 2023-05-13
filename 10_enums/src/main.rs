use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>), // Using Box to void large size
}

// Binary Tree Example
#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[derive(Debug)]
pub struct Account {
    pub name: String,
    pub lang: String,
    pub id: usize,
    pub balance: f64,
    pub birthday: String,
    pub eye_color: String,
}

// Add node to a TreeNode
impl<T: Ord + std::fmt::Display> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            // Add the first empty node
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }));
            }
            // Recursion to add the left if less than node, else add right
            BinaryTree::NonEmpty(ref mut node) => {
                match node.element.cmp(&value) {
                    Ordering::Less => node.right.add(value),
                    Ordering::Greater => node.left.add(value),
                    Ordering::Equal => eprintln!("{} is already in the tree", &value),
                };
            }
        }
    }
}

fn main() {
    println!("----------");
    println!("-- Json --");
    println!("----------");
    let json = Json::Object(Box::new({
        let mut inner = HashMap::new();
        inner.insert(
            "c".to_string(),
            Json::Array(vec![
                Json::String("hello".to_string()),
                Json::Number(2.0),
                Json::Number(10.),
            ]),
        );
        inner.insert("b".to_string(), Json::String("hello".to_string()));
        inner.insert("a".to_string(), Json::Number(1.0));
        inner.insert("n".to_string(), Json::Null);
        inner.insert("B".to_string(), Json::Bool(true));
        inner
    }));
    println!("{:#?}", json);

    println!("--------------");
    println!("-- Patterns --");
    println!("--------------");
    let jupiter_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));
    let mercury_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));
    let mars_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree,
    }));
    let venus_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Venus",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));
    let uranus_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: BinaryTree::Empty,
        right: venus_tree,
    }));
    let mut saturn_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree,
    }));

    println!("Saturn_Tree: {:#?}", saturn_tree);

    println!("-- Adding --");
    saturn_tree.add("Earth");
    println!("After adding Earth: {:#?}", saturn_tree);
    println!("-- Adding Again --");
    saturn_tree.add("Earth");

    println!("--------------");
    println!("-- Patterns --");
    println!("--------------");
    let account = Account {
        name: "John".to_string(),
        lang: "en".to_string(),
        id: 1,
        balance: 250.0,
        birthday: "1990-01-01".to_string(),
        eye_color: "blue".to_string(),
    };
    println!("Account: {:#?}", account);
    // need reference to borrow name and lang
    match account {
        Account {
            ref name, ref lang, ..
        } => {
            println!("Use in UI Name: {}", name);
            println!("Use in UI Lang: {}", lang);
            println!("Use in UI Ref to account {:#?}", &account);
        }
    };
    // Bindings
    // match account {
    // rect @ Account(..) => {
    // do_something_with(rect);
    // }
    // Some( digit @ 0..=9 ) => {
    //     println!("Digit: {}", digit);
    // }
    // };
}
