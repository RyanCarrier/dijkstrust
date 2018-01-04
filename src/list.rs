
// element is an element of a linked list.
use std::cmp::Ordering;
use std::ptr;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
struct SmallVertex {
    id: usize,
    best_distance: i64,
}

impl fmt::Debug for SmallVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SmallVertex{{{}, {}}}", self.id, self.best_distance)
    }
}

impl SmallVertex {
    pub fn new(id: usize, best_distance: i64) -> SmallVertex {
        SmallVertex {
            id: id,
            best_distance: best_distance,
        }
    }
}

impl Ord for SmallVertex {
    fn cmp(&self, other: &SmallVertex) -> Ordering {
        //As 'max' is given first in bin heap, we give the lowest distance as it has the
        // highest priority
        other.best_distance.cmp(&self.best_distance)
        //self.best_distance.cmp(&other.best_distance)
    }
}
impl PartialOrd for SmallVertex {
    fn partial_cmp(&self, other: &SmallVertex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct LinkedList<T: Ord + Eq> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Clone, Eq, PartialEq)]
struct Node<T: Ord + Eq> {
    elem: T,
    next: Link<T>,
}

impl<T: Ord + Eq + Clone> Ord for Node<T> {
    fn cmp(&self, other: &Node<T>) -> Ordering {
        //As 'max' is given first in bin heap, we give the lowest distance as it has the
        // highest priority
        self.elem.cmp(&other.elem)
        //self.best_distance.cmp(&other.best_distance)
    }
}

impl<T: Ord + Eq + Clone> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl<T: Ord + Eq> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_node = Box::new(Node {
            elem: elem,
            next: None,
        });
        let raw_new: *mut _ = &mut *new_node;

        if self.tail.is_null() {
            self.head = Some(new_node);
        } else {
            unsafe {
                let mut currentr = &self.head;
                //while the next element is smaller than the new one, and the next element isn't
                // the tail check the node after
                let nextr_elem = &currentr.as_ref().unwrap().next.as_ref().unwrap().elem;
                while &new_node.elem > nextr_elem && nextr_elem != &(*self.tail).elem {
                    currentr = &currentr.as_ref().unwrap().next;
                }
                //insert the new node behind the next element (after current)
                new_node.next = currentr.as_mut().unwrap().next;
                currentr.as_mut().unwrap().next = Some(new_node);
            }
        }

        self.tail = raw_new;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
    }

    pub fn is_empty(&self) -> bool {
        return self.head.is_none();
    }
}

impl<T: Ord + Eq> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_empty() {
        let mut l: LinkedList<SmallVertex> = LinkedList::new();
        assert_eq!(l.is_empty(), true);
        l.push(SmallVertex::new(0, 1));
        assert_eq!(l.is_empty(), false);
        l.pop();
        assert_eq!(l.is_empty(), true);
    }

    #[test]
    fn test_stack_push_pop() {
        let mut l: LinkedList<SmallVertex> = LinkedList::new();
        let v = vec![
            SmallVertex::new(0, 1),
            SmallVertex::new(3, 2),
            SmallVertex::new(2, 3),
        ];
        for val in v.clone() {
            l.push(val);
        }
        for val in v {
            assert_eq!(l.pop().unwrap(), val);
        }
    }

    #[test]
    fn test_ordered_push_pop() {
        let mut l: LinkedList<SmallVertex> = LinkedList::new();
        let v = vec![
            SmallVertex::new(0, 0),
            SmallVertex::new(3, 3),
            SmallVertex::new(2, 2),
        ];
        for val in v.clone() {
            l.push(val);
        }
        assert_eq!(l.pop().unwrap(), v[0]);
        assert_eq!(l.pop().unwrap(), v[2]);
        assert_eq!(l.pop().unwrap(), v[1]);

    }
}
