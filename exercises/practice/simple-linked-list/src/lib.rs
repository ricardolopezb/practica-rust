use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<T>,
    tail: Option<Box<SimpleLinkedList<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList{head: None, tail: None}
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head == None
    }

    pub fn len(&self) -> usize {
        if self.head != None {
            return 1 + self.tail.len();
        } 
        return 0;
    }

    pub fn push(&mut self, _element: T) {
        if self.head == None {
            self.head = Some(_element);
        } else {
            self.tail.push(_element);
        }

    }

    pub fn pop(&mut self) -> Option<T> {
        self.tail.peek
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let other_list = 
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        unimplemented!()
    }
}
