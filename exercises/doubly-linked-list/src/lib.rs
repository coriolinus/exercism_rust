// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

use std::ptr::NonNull;

struct Node<T> {
    item: T,

    // the pointers below are const because we expect it to be relatively rare
    // for them to change. It doesn't really matter either way; it's possible
    // to cast freely between them.
    /// next steps toward the back of the list
    next: Option<NonNull<*const Node<T>>>,

    /// prev steps toward the front of the list
    prev: Option<NonNull<*const Node<T>>>,
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
    front: Option<NonNull<*mut Node<T>>>,
    back: Option<NonNull<*mut Node<T>>>,
}

pub struct Cursor<'a, T>(std::marker::PhantomData<&'a T>);

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
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        unimplemented!()
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        unimplemented!()
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
impl<T> Cursor<'_, T> {
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
        unimplemented!()
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
