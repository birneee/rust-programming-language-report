fn main() {
    let tree = BinaryTree {
        root: Some(Box::new(Node {
            data: Box::new(50),
            left: Some(Box::new(Node {
                data: Box::new(20),
                left: Some(Box::new(Node {
                    data: Box::new(3),
                    left: None,
                    right: Some(Box::new(Node {
                        data: Box::new(9),
                        left: None,
                        right: None,
                    })),
                })),
                right: None,
            })),
            right: Some(Box::new(Node {
                data: Box::new(80),
                left: Some(Box::new(Node {
                    data: Box::new(70),
                    left: None,
                    right: Some(Box::new(Node {
                        data: Box::new(79),
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Node {
                    data: Box::new(100),
                    left: None,
                    right: None,
                })),
            })),
        }))
    };

    println!("\npre-order");
    tree.for_each_pre_order(|n| {
        println!("{}", n);
    });

    println!("\nin-order");
    tree.for_each_in_order(|n| {
        println!("{}", n);
    });

    println!("\npost-order");
    tree.for_each_post_order(|n| {
        println!("{}", n);
    });
}

struct Node<T> {
    data: Box<T>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn for_each_pre_order<F: Fn(&T)>(&self, action: &F) {
        action(&*self.data);
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
        action(&*self.data);
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
        action(&*self.data);
    }
}

struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> BinaryTree<T> {
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