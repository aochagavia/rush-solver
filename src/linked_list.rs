use std::rc::Rc;

/// An immutable linked list
pub struct List<T> {
    inner: Rc<ListInner<T>>
}

enum ListInner<T> {
    Nil,
    Cons(T, Rc<ListInner<T>>)
}

// Note: we add the Copy bound to simplify the `into_iter` function
impl<T: Copy> List<T> {
    /// Create an empty `List`
    pub fn new() -> List<T> {
        List { inner: Rc::new(ListInner::Nil) }
    }

    /// Add an element at the front of the `List`
    ///
    /// The `List` is reused in the tail, since we know it will never be mutated
    pub fn push_front(&self, element: T) -> List<T> {
        List { inner: Rc::new(ListInner::Cons(element, self.inner.clone())) }
    }

    /// Transform the `List` into an iterator
    pub fn into_iter(&self) -> impl DoubleEndedIterator<Item = T> {
        let mut next = &self.inner;
        let mut ret = Vec::new();
        while let ListInner::Cons(move_, ref tail) = **next {
            ret.push(move_);
            next = tail;
        }

        ret.into_iter()
    }
}
