use std::mem;

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, el: i32) {
        let new_node = Node {
            elem: el,
            next: mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop_mine(&mut self) -> Option<i32> {
        let link = mem::replace(&mut self.head, Link::Empty);
        let (new_head, el) = match link {
            Link::More(box_el) => (box_el.next, Some(box_el.elem)),
            Link::Empty => (Link::Empty, None),
        };
        self.head = new_head;
        el
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    // pub fn pop_error(&mut self) -> Option<i32> {
    //     let result;
    //     match &self.head {
    //         Link::Empty => {
    //             result = None;
    //         }
    //         Link::More(node) => {
    //             // node is : &Box<Node>
    //             result = Some(node.elem);
    //             // we are extracting next out of node, but we can't do that, because node is behind shared reference
    //             self.head = node.next;
    //         }
    //     };
    //     result
    // }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

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

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
