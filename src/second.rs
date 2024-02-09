use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}
/*
    Creating an iterator of your own involves two steps: creating a struct to hold the iterator’s state,
    and then implementing Iterator for that struct.

    There’s a trait in the standard library for converting something into an iterator: IntoIterator.
        - This trait has one method, into_iter, which converts the thing implementing IntoIterator into an iterator.
*/

/*
    By implementing IntoIterator for a type, you define how it will be "converted to an iterator". -> emphasis on converted
*/
pub struct IntoIterator<T>(List<T>); //-> unit struct , commonly used as a wrapper, holds state which in this case is the list
impl<T> Iterator for IntoIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
        //we are using zero because the List passed to IntoIterator is at index 0
        // self.0 -> List ---> self.0.pop() == List::pop()
    }
}

pub struct Iter<'a, T> {
    /*
        The basic logic we want is to hold a pointer to the current node we want to yield next.
        When we yield an element, we want to proceed to the current node's next node
    */
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    //calling into converts this type into an iterator!
    pub fn into_iter(self) -> IntoIterator<T> {
        //Since into_iter() takes self by value, using a for loop to iterate over a collection consumes that collection.
        IntoIterator(self) //-> here we move self into the IntoIterator
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        /*
            iter borrows self, we need to ensure self lives for as long as Iter is around, reason for the lifetime.
            * -> dereference to get the actual node behind the pointer to heap

            option returns a value /self.head is a option/ we use as_ref to obtain a ref to the underlyng contents
            can we replace above with as_deref()
        */
        Iter {
            next: self.head.as_deref(),
        }
    }

    /*
        Map takes self by value, which would move the Option out of the thing it's in.
        Previously this was fine because we had just taken it out, but now we actually want to leave it where it was.
        The correct way to handle this is with the as_ref method on Option.

        -> It demotes the Option<T> to an Option to a reference to its internals.

        // Error
        pub fn peek(&self) -> Option<&T> {
            self.head.map(|node|{
                &node.elem
            })
        }

    */

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        /*
            mem::replace(&mut option, None) is such an incredibly common idiom that
            Option actually just went ahead and made it a method: take.

            match mem::replace(&mut self.head, None)
        */

        //Take the contents of head, that head points at & replace it with none
        //match on that memory/object -> if None, return None to indicate empty List <- dont pop off an empty List
        //if there is something -> switch pointer to point to the next element, extract the elem out of it
        // We are working directly with memory because self is a mutable reference &mut self

        /*
            match self.head.take() {
                None => None,
                Some(node) => {
                    self.head = node.next;
                    Some(node.elem)
                }
            }
        */

        /*
           match option { None => None, Some(x) => Some(y) } is such an incredibly common idiom that it
           was called map. map takes a function to execute on the x in the Some(x) to produce the y in Some(y).
        */

        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);

        list.push(5);
        list.push(7);
        list.push(8);

        assert_eq!(list.peek(), Some(&8));
        assert_eq!(list.peek_mut(), Some(&mut 8));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
