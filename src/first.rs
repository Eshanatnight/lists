use std::mem;

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    // Constructed an empty list
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn new_with_value(_elem: i32) -> Self {
        List {
            head: Link::More(Box::new(Node {
                elem: _elem,
                next: Link::Empty,
            })),
        }
    }

    // Push an Element
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    // Pop an element
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,

            Link::More(_node) => {
                self.head = _node.next;
                Some(_node.elem)
            }
        }
    }
}
