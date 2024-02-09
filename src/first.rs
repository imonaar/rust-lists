/*
List a = Empty | Elem a (List a)
    - A List is either Empty or an Element followed by a List"
    - This is a recursive definition expressed as a sum type
    - Rust calls sum types enums
*/

/*
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}
    - We need to better separate out the idea of having an element from allocating another list.
    - While enums let us declare a type that can contain one of several values, structs let us declare a type that contains many values at once.

*/

use std::mem;

pub struct List {
    head: Link,
}

/*
- There are 3 primary forms that self can take: self, &mut self, and &self
    self - Value
    &mut self - mutable reference
    &self - shared reference
*/

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        //By default, a pattern match will try to move its contents into the new branch,
        // but we can't do this because we don't own self by-value here.

        /*
           What are we doing with mem::replace here?
           we are matching on self.head
           - if it is empty we do nothing, return None
           - if there is something then steal the Node & replace it with Empty <- leave it in a valid state
        */

        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem) // we are moving the memry here thus need for mem::replace
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

/*
    A value represents true ownership. You can do whatever you want with a value: move it, destroy it, mutate it,
    or loan it out via a reference. When you pass something by value, it's moved to the new location.
    The new location now owns the value, and the old location can no longer access it. For this reason most methods
    don't want self -- it would be pretty lame if trying to work with a list made it go away!


    A mutable reference represents temporary exclusive access to a value that you don't own.
    You're allowed to do absolutely anything you want to a value you have a mutable reference to as long you
    leave it in a valid state when you're done (it would be rude to the owner otherwise!).
    This means you can actually completely overwrite the value. A really useful special case of this is
    swapping a value out for another, which we'll be using a lot. The only thing you can't do with an &mut
    is move the value out with no replacement. &mut self is great for methods that want to mutate self.

    A shared reference represents temporary shared access to a value that you don't own.
    Because you have shared access, you're generally not allowed to mutate anything.
    Think of & as putting the value out on display in a museum.
    & is great for methods that only want to observe self.
*/

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

        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
