use std::mem;

pub struct List {
    head: Link,
    size: usize,
}

#[derive(PartialEq, Eq)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Eq, PartialEq)]
pub struct Node {
    elem: i32,
    next: Link,
}

impl List {
    fn new() -> Self {
        Self {
            head: Link::Empty,
            size: 0,
        }
    }

    fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => return None,
            Link::More(node) => {
                self.head = node.next;
                self.size -= 1;
                Some(node.elem)
            }
        }
    }

    fn search(&self, value: i32) -> isize {
        let mut i = 0;
        let mut current = &self.head;

        while *current != Link::Empty {
            match &current {
                Link::Empty => return -1,
                Link::More(node) => {
                    if node.elem == value {
                        return i;
                    };
                    i += 1;
                    current = &node.next;
                }
            }
        }
        -1
    }

    fn insert(&mut self, (elem, index): (i32, usize)) {
        todo!();
    }

    fn remove_from(&mut self, index: usize) -> Option<i32> {
        todo!();
    }

    fn remove_elem(&mut self, elem: i32) -> Option<i32> {
        todo!();
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = curr_link {
            curr_link = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check list is empty function
        assert!(list.is_empty());

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Confirm list size
        assert_eq!(list.get_size(), 3);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn traversal_ops() {
        let mut list = List::new();

        assert_eq!(list.search(2), -1);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.search(3), 0);
        assert_eq!(list.search(2), 1);
        assert_eq!(list.search(1), 2);
        assert_eq!(list.search(7), -1);
    }

    #[test]
    fn insertion_op() {}

    #[test]
    fn deletion_op() {}
}
