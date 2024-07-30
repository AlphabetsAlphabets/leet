use std::fmt::Display;

#[derive(Default, Clone, Debug)]
struct Node {
    value: u8,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}

impl Node {
    fn new(value: u8) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
}

impl Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = &self.head;
        let mut items = vec![];

        while let Some(next) = current {
            items.push(next.value);
            current = &next.next;
        }

        write!(f, "{:?}", items)
    }
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn insert(&mut self, value: u8) {
        let mut new_node = Box::new(Node::new(value));
        if self.head.is_none() {
            self.head = Some(new_node);
            self.tail = self.head.clone();
            return;
        }

        let current = &mut self.tail;

        new_node.prev = current.clone();
        if let Some(node) = current {
            node.next = Some(new_node.clone());
        }

        self.head = self.tail.clone();
        self.tail = Some(new_node);
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.insert(4);
}
