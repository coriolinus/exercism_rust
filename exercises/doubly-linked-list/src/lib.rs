// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

use std::ptr::NonNull;
use std::mem;

type NNMut<T> = Option<NonNull<*mut Node<T>>>;

struct Node<T> {
    item: T,

    // the pointers below are const because we expect it to be relatively rare
    // for them to change. It doesn't really matter either way; it's possible
    // to cast freely between them.
    /// next steps toward the back of the list
    next: NNMut<T>,

    /// prev steps toward the front of the list
    prev: NNMut<T>,
}

impl<T> Node<T> {
    fn len(&self) -> usize {
        1 + self
            .next
            .map_or(0, |next| unsafe { (**next.as_ref()).len() })
    }
}

pub struct LinkedList<T> {
    // these pointers are mut because we expect them to change relatively frequently
    front: NNMut<T>,
    back: NNMut<T>,
}

pub struct Cursor<T>(NNMut<T>);

pub struct Iter<'a, T>(std::marker::PhantomData<&'a T>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            front: None,
            back: None,
        }
    }

    pub fn len(&self) -> usize {
        self.front
            .map_or(0, |node| unsafe { (**node.as_ref()).len() })
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<T> {
        Cursor(self.front)
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<T> {
        Cursor(self.back)
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        unimplemented!()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // basically the same as in the stdlib implementation of drop
        while let Some(_) = self.pop_front() {}
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<T> {
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
        let mut rv = None;
        let mut next = None;
        if let Cursor(Some(nodep)) = self {
            // select next
            unsafe {
                let node = &mut (**nodep.as_ptr());
                if node.next.is_some() {
                    next = node.next;
                } else if node.prev.is_some() {
                    next = node.prev;
                }
                // get return value
                rv = Some(mem::replace(&mut node.item, mem::uninitialized()));
            }
            // update self
            self.0 = next;
        }
        rv
    }

    pub fn insert_after(&mut self, _element: T) {
        unimplemented!()
    }

    pub fn insert_before(&mut self, _element: T) {
        unimplemented!()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unimplemented!()
    }
}
