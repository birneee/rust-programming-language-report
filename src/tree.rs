fn main() {
    let tree = BinaryTree::new(
        Some(Node::new(
            50,
            Some(Node::new(
                20,
                Some(Node::new(
                    3,
                    None,
                    Some(Node::new(
                        9,
                        None,
                        None)))),
                None)),
            Some(Node::new(
                80,
                Some(Node::new(
                    70,
                    None,
                    Some(Node::new(
                        79,
                        None,
                        None)))),
                Some(Node::new(
                    100,
                    None,
                    None)))))));

    println!("pre-order");
    tree.for_each_pre_order(|n| {
        print!("{} ", n);
    });

    println!("");

    println!("in-order");
    tree.for_each_in_order(|n| {
        print!("{} ", n);
    });

    println!("");

    println!("post-order");
    tree.for_each_post_order(|n| {
        print!("{} ", n);
    });
}

struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, left: Option<Node<T>>, 
                right: Option<Node<T>>) -> Node<T>{
        Node{
            data: data,
            left: left.map(|left|{
                Box::new(left)
            }),
            right: right.map(|right|{
                Box::new(right)
            })
        }
    }

    fn for_each_pre_order<F: Fn(&T)>(&self, action: &F) {
        action(&self.data);
        if let Some(left) = &self.left {
            left.for_each_in_order(action)
        }
        if let Some(right) = &self.right {
            right.for_each_in_order(action)
        }
    }

    fn for_each_in_order<F: Fn(&T)>(&self, action: &F) {
        if let Some(left) = &self.left {
            left.for_each_in_order(action)
        }
        action(&self.data);
        if let Some(right) = &self.right {
            right.for_each_in_order(action)
        }
    }

    fn for_each_post_order<F: Fn(&T)>(&self, action: &F) {
        if let Some(left) = &self.left {
            left.for_each_in_order(action)
        }
        if let Some(right) = &self.right {
            right.for_each_in_order(action)
        }
        action(&self.data);
    }
}

struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> BinaryTree<T> {
    fn new(root: Option<Node<T>>) -> BinaryTree<T>{
        BinaryTree{
            root: root.map(|root|{
                Box::new(root)
            })
        }
    }

    fn for_each_pre_order<F: Fn(&T)>(&self, action: F) {
        if let Some(root) = &self.root {
            root.for_each_pre_order(&action)
        }
    }

    fn for_each_in_order<F: Fn(&T)>(&self, action: F) {
        if let Some(root) = &self.root {
            root.for_each_in_order(&action)
        }
    }

    fn for_each_post_order<F: Fn(&T)>(&self, action: F) {
        if let Some(root) = &self.root {
            root.for_each_post_order(&action)
        }
    }
}