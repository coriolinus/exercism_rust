// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

use std::mem;
use std::ptr::NonNull;

type NNMut<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
struct Node<T: std::fmt::Debug> {
    item: T,

    // the pointers below are const because we expect it to be relatively rare
    // for them to change. It doesn't really matter either way; it's possible
    // to cast freely between them.
    /// next steps toward the back of the list
    next: NNMut<T>,

    /// prev steps toward the front of the list
    prev: NNMut<T>,
}

impl<T: std::fmt::Debug> Node<T> {
    fn new(item: T) -> Node<T> {
        Node {
            item,
            next: None,
            prev: None,
        }
    }

    fn len(&self) -> usize {
        dbg!(self);
        1 + self.next.map_or(0, |next| unsafe { next.as_ref().len() })
    }
}

impl<T: std::fmt::Debug> Drop for Node<T> {
    fn drop(&mut self) {
        dbg!("Drop!");
    }
}

#[derive(Debug)]
pub struct LinkedList<T: std::fmt::Debug> {
    // these pointers are mut because we expect them to change relatively frequently
    front: NNMut<T>,
    back: NNMut<T>,
}

impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            front: None,
            back: None,
        }
    }

    pub fn len(&self) -> usize {
        dbg!(self);
        self.front
            .map_or(0, |node| unsafe { dbg!(node.as_ref()).len() })
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<T> {
        Cursor::new(self, self.front)
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<T> {
        Cursor::new(self, self.back)
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        unimplemented!()
    }
}

impl<T: std::fmt::Debug> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // basically the same as in the stdlib implementation of drop
        while let Some(_) = self.pop_front() {}
    }
}

#[derive(Debug)]
pub struct Cursor<'a, T: std::fmt::Debug> {
    ll: &'a mut LinkedList<T>,
    ptr: NNMut<T>,
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T: std::fmt::Debug> Cursor<'_, T> {
    fn new(ll: &mut LinkedList<T>, ptr: NNMut<T>) -> Cursor<T> {
        Cursor { ll, ptr }
    }

    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        dbg!(&self, unsafe{self.ptr.unwrap().as_ref()});
        let mut rv = None;
        let mut next = None;
        if let Some(mut nodep) = self.ptr {
            unsafe {
                dbg!(nodep);
                let node = nodep.as_mut();
                dbg!(&node);
                // select next and update pointers
                if node.next.is_some() {
                    next = node.next;
                } else if node.prev.is_some() {
                    next = node.prev;
                }
                dbg!(next);
                // update external pointers
                if let Some(nnext) = node.next {
                    (*nnext.as_ptr()).prev = node.prev;
                } else {
                    self.ll.back = node.prev;
                }
                if let Some(nprev) = node.prev {
                    (*nprev.as_ptr()).next = node.next;
                } else {
                    self.ll.front = node.next;
                }

                // get return value
                rv = Some(mem::replace(&mut node.item, mem::uninitialized()));
            }
            // update self
            self.ptr = next;
        }
        rv
    }

    pub fn insert_after(&mut self, element: T) {
        let mut new_node = Node::new(element);
        let new_node_ptr: NNMut<T> = NonNull::new(&mut new_node as *mut Node<T>);
        assert!(new_node_ptr.is_some());
        self.ptr = match self.ptr {
            None => {
                self.ll.front = new_node_ptr;
                self.ll.back = new_node_ptr;
                new_node_ptr
            }
            Some(cur_ptr) => {
                unsafe {
                    let cur_node = cur_ptr.as_ptr();
                    if let Some(next) = (*cur_node).next {
                        (*next.as_ptr()).prev = new_node_ptr;
                    } else {
                        self.ll.back = new_node_ptr;
                    }
                    (*cur_node).next = new_node_ptr;
                }

                Some(cur_ptr)
            }
        };
        assert!(self.ll.front.is_some());
        assert!(self.ll.back.is_some());
        assert!(self.ptr.is_some());
        dbg!(&self, unsafe{self.ptr.unwrap().as_ref()});
    }

    pub fn insert_before(&mut self, element: T) {
        let mut new_node = Node::new(element);
        let new_node_ptr: NNMut<T> = NonNull::new(&mut new_node as *mut Node<T>);
        self.ptr = match self.ptr {
            None => {
                self.ll.front = new_node_ptr;
                self.ll.back = new_node_ptr;
                new_node_ptr
            }
            Some(cur_ptr) => {
                unsafe {
                    let cur_node = cur_ptr.as_ptr();
                    if let Some(prev) = (*cur_node).prev {
                        (*prev.as_ptr()).next = new_node_ptr;
                    } else {
                        self.ll.front = new_node_ptr;
                    }
                    (*cur_node).prev = new_node_ptr;
                }

                Some(cur_ptr)
            }
        };
    }
}

pub struct Iter<'a, T>(std::marker::PhantomData<&'a T>);

impl<'a, T: std::fmt::Debug> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unimplemented!()
    }
}
