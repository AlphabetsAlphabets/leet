use std::fmt::Display;

#[derive(Default, Clone)]
struct Node {
    value: u8,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: u8) -> Self {
        Self {
            value,
            next: None,
        }
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
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
        }
    }

    fn insert(&mut self, value: u8) {
        let mut current = &mut self.head;

        while let Some(next) = current {
            current = &mut next.next;
        }

        // Create a new node
        let new_node = Node::new(value);
        
        // Create a (smart) pointer which points to the new node.
        let ptr_new_node = Box::new(new_node);

        // Set the pointer to the new node to the current node.
        // Rust quirks:
        // I need to wrap ptr_new_node in Some. Because of the type of current.
        // But, the problem is current is a mut reference. The right side can't be
        // &mut Some(ptr_new_node). So, the solution is to deref current with *current.
        // Then set the value inside the reference to Some(ptr_new_node)
        *current = Some(ptr_new_node);
    }

    fn find(self, value: u8) -> bool {
        let mut current = &self.head;

        while let Some(next) = current {
            if next.value == value {
                return true;
            }

            current = &next.next;
        }

        return false;
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    println!("{}", list);
    println!("{}", list.find(4));
}
