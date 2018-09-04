pub struct LinkedList<T>(std::marker::PhantomData<T>);

pub struct Cursor<'a, T: 'a>(std::marker::PhantomData<&'a mut T>);

pub struct Iter<'a, T: 'a>(std::marker::PhantomData<&'a T>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        unimplemented!()
    }

    pub fn push_back(&mut self, element: T) {
        self.cursor_head().insert_after(element);
    }

    pub fn push_front(&mut self, element: T) {
        self.cursor_tail().insert_before(element);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.cursor_head().take()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.cursor_tail().take()
    }

    /// Return a cursor positioned at the tail node
    pub fn cursor_tail(&mut self) -> Cursor<'_, T> {
        unimplemented!()
    }

    /// Return a cursor positioned at the head node
    pub fn cursor_head(&mut self) -> Cursor<'_, T> {
        unimplemented!()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unimplemented!()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        unimplemented!()
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

// the cursor is expected to act as if it is at the position of a node
impl<'a, T: 'a> Cursor<'a, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Move one position forward and return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Move one position backward and return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Remove the element at the current node from the list and return it.
    pub fn take(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn insert_after(&mut self, _element: T) {
        unimplemented!()
    }

    pub fn insert_before(&mut self, _element: T) {
        unimplemented!()
    }

    pub fn insert_list_after(&mut self, _other: &mut LinkedList<T>) {
        unimplemented!()
    }

    pub fn insert_list_before(&mut self, _other: &mut LinkedList<T>) {
        unimplemented!()
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unimplemented!()
    }
}
