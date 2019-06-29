fn main() {
    let mut stack: Stack<u8> = Stack::new();
    for i in 0 .. 4 {
        stack.push(i);
        println!("push {}", i);
    }

    while !stack.is_empty() {
        println!("pop {}", stack.pop().unwrap());
    }
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { head: None }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            data: elem,
            next: self.head.take(),
        }));
    }

    fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        head.map(|mut head| {
            self.head = head.next.take();
            head.data
        })
    }
}