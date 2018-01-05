
use std::fmt;
use vertex;
use std;
use std::fs::File;
use std::io::Read;
use std::io;
use std::num;
use std::error;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::{ptr, mem};


#[derive(Debug)]
pub enum ImportError {
    Io(io::Error),
    Fmt(fmt::Error),
    ParseF(num::ParseFloatError),
    ParseI(num::ParseIntError),
    FileFormat,
}


impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            ImportError::Io(ref err) => write!(f, "IO error: {}", err),
            ImportError::Fmt(ref err) => write!(f, "Fmt error: {}", err),
            ImportError::ParseF(ref err) => write!(f, "Parse error: {}", err),
            ImportError::ParseI(ref err) => write!(f, "Parse error: {}", err),
            ImportError::FileFormat => {
                write!(f, "Format error: {}", "Issue with import file format")
            }
        }
    }
}

impl error::Error for ImportError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            ImportError::Io(ref err) => err.description(),
            ImportError::Fmt(ref err) => err.description(),
            ImportError::ParseF(ref err) => err.description(),
            ImportError::ParseI(ref err) => err.description(),
            ImportError::FileFormat => "Format error: Issue with import file format",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ImportError::Io(ref err) => Some(err),
            ImportError::Fmt(ref err) => Some(err),
            ImportError::ParseF(ref err) => Some(err),
            ImportError::ParseI(ref err) => Some(err),
            ImportError::FileFormat => None,
        }
    }
}



#[derive(PartialEq)]
pub struct Graph {
    verticies: Vec<vertex::Vertex>,
}
impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "verticies:\n{:?}\n",
            self.verticies,
        )
    }
}

#[derive(Debug)]
pub struct BestPath {
    success: bool,
    distance: i64,
    path: Vec<u64>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { verticies: Vec::new() }
    }

    pub fn add_vertex(&mut self, v: vertex::Vertex) {
        self.verticies.push(v);
    }

    pub fn remove_vertex(&mut self, id: u64) {
        self.verticies.retain(|v| v.id != id)
    }

    fn setup(&mut self, source: u64) {
        for vertex in &mut self.verticies {
            vertex.best_distance = std::i64::MAX;
            vertex.best_verticie = 0;
        }
        self.verticies[source as usize].best_distance = 0;
        //set best distance to i64max
        //reset all best distance and vertex
    }

    pub fn export(&self) -> String {
        return (&self.verticies)
            .into_iter()
            .map(|v| v.export())
            .collect::<Vec<String>>()
            .join("\r\n");
    }

    fn best_path(&self, source: u64, destination: u64, touch_dest: bool) -> BestPath {
        let mut b = BestPath {
            success: touch_dest,
            distance: self.verticies[destination as usize].best_distance,
            path: Vec::new(),
        };
        //you can just fuck this stuff right off if you need to
        let mut vid = destination;
        b.path.push(vid);
        while vid != source {
            vid = self.verticies[vid as usize].best_verticie.clone();
            b.path.push(vid);
        }
        b.path.reverse();
        b
    }

    pub fn shortest(&mut self, source: u64, destination: u64) -> Option<BestPath> {
        //TODO: implement anti looping (on arc direct and on secondary to self.)
        //let mut visiting = LinkedList::new();
        let destination = destination as usize;
        let mut visiting = LinkedList::new(); //BinaryHeap::new();
        let mut touch_dest = false;
        self.setup(source);
        //Need to make own visiting priority heap, or a custom struct to parse in to sort better
        // maybe just to and best_distance
        visiting.push(SmallVertex {
            id: source as usize,
            best_distance: 0,
        });
        while !visiting.is_empty() {
            let visitor = visiting.pop().unwrap();
            //for each arc to other nodes, check if that path is the new best path to it
            //(from our visiting node)
            let mut i = 0;
            if visitor.best_distance > self.verticies[visitor.id].best_distance {
                //Can remove when dupliocates are removed
                continue;
            }
            let l = self.verticies[visitor.id].arcs.len();
            while i < l {
                //arc in arcs {
                let arc = SmallVertex {
                    id: self.verticies[visitor.id].arcs[i].to as usize,
                    best_distance: self.verticies[visitor.id].arcs[i].distance,
                };
                i += 1;
                if arc.id == visitor.id ||
                    visitor.best_distance + arc.best_distance >=
                        self.verticies[arc.id].best_distance
                {
                    continue;
                }
                if arc.id == destination {
                    touch_dest = true;
                }
                //if it is the new best node, make sure we update it's distance and best node, then
                //set it to be (re)visited later
                self.verticies[arc.id].best_distance = visitor.best_distance + arc.best_distance;
                self.verticies[arc.id].best_verticie = visitor.id as u64;
                visiting.push(SmallVertex {
                    id: self.verticies[arc.id].id as usize,
                    best_distance: self.verticies[arc.id].best_distance,
                });
            }
        }
        if !touch_dest {
            return None;
        }
        return Some(self.best_path(source, destination as u64, touch_dest));
    }

    pub fn import(source: &str) -> Result<Graph, ImportError> {
        /*
        NODE to,dist to,dist
        NODE
        NODE
        */
        let mut g = Graph::new();
        for line in source.split("\n") {
            let mut items = line.trim().split(" ");
            let n = items.next();
            if n == None || n.unwrap().trim().is_empty() {
                continue;
            }
            let n = try!(n.unwrap().parse::<u64>().map_err(ImportError::ParseI));
            let mut v = vertex::Vertex::new(n);
            for item in items {
                if item.trim().is_empty() {
                    continue;
                }
                //first item should be consumed by first next()
                let mut arc_raw = item.trim().split(",");
                let to = arc_raw.next();
                let distance = arc_raw.next();
                if (to == None || to.unwrap().trim().is_empty()) ||
                    (distance == None || distance.unwrap().trim().is_empty())
                {
                    return Err(ImportError::FileFormat);
                }
                let t = try!(to.unwrap().parse::<u64>().map_err(ImportError::ParseI));
                let d = try!(distance.unwrap().parse::<i64>().map_err(
                    ImportError::ParseI,
                ));
                v.add_arc(t as u64, d as i64)
            }
            g.add_vertex(v);
        }
        Ok(g)
    }

    pub fn import_file(file: &str) -> Result<Graph, ImportError> {
        let mut data = String::new();
        let mut f = try!(File::open(file).map_err(ImportError::Io));

        try!(f.read_to_string(&mut data).map_err(ImportError::Io));
        return Graph::import(&*data);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmallVertex {
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

/// A LinkedList node.
struct Node<SmallVertex> {
    prev: Raw<SmallVertex>,
    next: Link<SmallVertex>,
    elem: SmallVertex,
}

impl<SmallVertex> Node<SmallVertex> {
    /// Makes a node with the given element.
    #[inline]
    fn new(elem: SmallVertex) -> Self {
        Node {
            prev: Raw::none(),
            next: None,
            elem: elem,
        }
    }

    /// Joins two lists.
    #[inline]
    fn link(&mut self, mut next: Box<Self>) {
        next.prev = Raw::some(self);
        self.next = Some(next);
    }

    /// Makes the given node come after this one, appropriately setting all other links.
    /// Assuming that self has a `next`.
    #[inline]
    fn splice_next(&mut self, mut next: Box<Self>) {
        let mut old_next = self.next.take();
        old_next.as_mut().map(
            |node| node.prev = Raw::some(&mut *next),
        );
        next.prev = Raw::some(self);
        next.next = old_next;
        self.next = Some(next);
    }

    /// Takes the next node from this one, breaking the list into two correctly linked lists.
    #[inline]
    fn take_next(&mut self) -> Option<Box<Self>> {
        let mut next = self.next.take();
        next.as_mut().map(|node| node.prev = Raw::none());
        next
    }
}

/// An owning link.
type Link<SmallVertex> = Option<Box<Node<SmallVertex>>>;

/// A non-owning link, based on a raw ptr.
struct Raw<SmallVertex> {
    ptr: *const Node<SmallVertex>,
}

impl<SmallVertex> Raw<SmallVertex> {
    /// Makes a null reference.
    #[inline]
    fn none() -> Self {
        Raw { ptr: ptr::null_mut() }
    }

    /// Makes a reference to the given node.
    #[inline]
    fn some(ptr: &mut Node<SmallVertex>) -> Self {
        Raw { ptr: ptr }
    }

    /// Converts the ref to an Option containing a reference.
    #[inline]
    fn as_ref(&self) -> Option<&Node<SmallVertex>> {
        unsafe {
            if self.ptr.is_null() {
                None
            } else {
                // 100% legit (no it's not)
                Some(&*self.ptr)
            }
        }
    }

    /// Converts the ref to an Option containing a mutable reference.
    #[inline]
    fn as_mut(&mut self) -> Option<&mut Node<SmallVertex>> {
        unsafe {
            if self.ptr.is_null() {
                None
            } else {
                // 100% legit (no it's not)
                Some(&mut *(self.ptr as *mut Node<SmallVertex>))
            }
        }
    }

    /// SmallVertexakes the reference out and nulls out this one.
    #[inline]
    fn take(&mut self) -> Self {
        mem::replace(self, Raw::none())
    }

    /// Clones this reference. Note that mutability differs from standard clone.
    /// We don't want these to be cloned in the immutable case.
    #[inline]
    fn clone(&mut self) -> Self {
        Raw { ptr: self.ptr }
    }
}

/// An experimental rewrite of LinkedList to provide a more cursor-oriented API.
pub struct LinkedList {
    len: usize,
    head: Link<SmallVertex>,
    tail: Raw<SmallVertex>,
}

impl LinkedList {
    /// Returns an empty `LinkedList`.
    #[inline]
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: Raw::none(),
            len: 0,
        }
    }
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut SmallVertex> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn push(&mut self, elem: SmallVertex) {
        let cursor = &mut self.cursor();
        cursor.reset();
        {
            while cursor.peek_next() != None &&
                cursor.peek_next().unwrap().best_distance < elem.best_distance
            {
                cursor.seek_forward(1)
            }
        }
        cursor.insert(elem);
        //self.push_back(elem);
    }
    pub fn pop(&mut self) -> Option<SmallVertex> {
        self.pop_front()
    }
    /// Appends the given element to the back of the list.
    pub fn push_back(&mut self, elem: SmallVertex) {
        self.len += 1;
        let mut node = Box::new(Node::new(elem));
        // unconditionally make the new node the new tail
        let mut old_tail = mem::replace(&mut self.tail, Raw::some(&mut *node));
        match old_tail.as_mut() {
            // List was empty, so the new node is the new head
            None => self.head = Some(node),
            // List wasn't empty, just need to append this to the old tail
            Some(tail) => tail.link(node),
        }

    }

    /// Appends the given element to the front of the list.
    pub fn push_front(&mut self, elem: SmallVertex) {
        self.len += 1;
        let mut node = Box::new(Node::new(elem));
        match self.head.take() {
            // List was empty, so the new node is the new tail
            None => self.tail = Raw::some(&mut *node),
            // List wasn't empty, append the old head to the new node
            Some(head) => node.link(head),
        }
        // unconditionally make the new node the new head
        self.head = Some(node);
    }

    /// Removes the element at the back of the list and returns it.
    ///
    /// Returns `None` if the list was empty.
    pub fn pop_back(&mut self) -> Option<SmallVertex> {
        // null out the list's tail pointer unconditionally
        self.tail.take().as_mut().and_then(|tail| {
            // tail pointer wasn't null, so decrease the len
            self.len -= 1;
            match tail.prev.take().as_mut() {
                // tail had no previous value, so the list only contained this node.
                // So we have to take this node out by removing the head itself
                None => self.head.take().map(|node| node.elem),
                // tail had a previous value, so we need to make that the new tail
                // and take the node out of its next field
                Some(prev) => {
                    self.tail = Raw::some(prev);
                    prev.next.take().map(|node| node.elem)
                }
            }
        })
    }

    /// Removes the element at the front of the list and returns it.
    ///
    /// Returns `None` if the list was empty.
    pub fn pop_front(&mut self) -> Option<SmallVertex> {
        // null out the list's head pointer unconditionally
        self.head.take().map(|mut head| {
            // head wasn't null, so decrease the len
            self.len -= 1;
            match head.take_next() {
                // head had no next value, so just null out the tail
                None => self.tail = Raw::none(),
                // head had a next value, which should be the new head
                Some(next) => self.head = Some(next),
            }
            head.elem
        })
    }

    /// Returns a reference to the element at the front of the list.
    ///
    /// Returns `None` if the list is empty.
    #[inline]
    pub fn front(&self) -> Option<&SmallVertex> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Returns a reference to the element at the back of the list.
    ///
    /// Returns `None` if the list is empty.
    #[inline]
    pub fn back(&self) -> Option<&SmallVertex> {
        self.tail.as_ref().map(|node| &node.elem)
    }



    /// Inserts the given element into the list at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is greater than the length of the list.
    #[inline]
    pub fn insert(&mut self, index: usize, elem: SmallVertex) {
        assert!(index <= self.len(), "index out of bounds");
        let mut cursor = self.cursor();
        cursor.seek_forward(index);
        cursor.insert(elem);
    }

    /// Removes the element at the given index and returns it.
    ///
    /// Returns `None` if the index is greater than or equal to the length of the list.
    #[inline]
    pub fn remove(&mut self, index: usize) -> Option<SmallVertex> {
        if index >= self.len() {
            None
        } else {
            let mut cursor = self.cursor();
            cursor.seek_forward(index);
            cursor.remove()
        }
    }

    /// Appends the given list to the end of this one. The old list will be empty afterwards.
    pub fn append(&mut self, other: &mut Self) {
        let mut cursor = self.cursor();
        cursor.prev();
        cursor.splice(other);
    }

    /// Inserts the given list at the given index. The old list will be empty afterwards.
    pub fn splice(&mut self, index: usize, other: &mut Self) {
        let mut cursor = self.cursor();
        cursor.seek_forward(index);
        cursor.splice(other);
    }

    /// Returns the number of elements in the list.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks if the list is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Removes all elements from the list.
    #[inline]
    pub fn clear(&mut self) {
        while !self.is_empty() {
            self.pop_front();
        }
    }

    /// Returns a cursor over the list.
    #[inline]
    pub fn cursor(&mut self) -> Cursor {
        Cursor {
            list: self,
            prev: Raw::none(),
            index: 0,
        }
    }
}


pub struct Cursor<'a> {
    list: &'a mut LinkedList,
    prev: Raw<SmallVertex>,
    // index of `next`, where the ghost is at `len`.
    index: usize,
}

impl<'a> Cursor<'a> {
    /// Resets the cursor to lie between the first and last element in the list.
    #[inline]
    pub fn reset(&mut self) {
        self.prev = Raw::none();
        self.index = 0;
    }

    pub fn peek_next(&mut self) -> Option<&mut SmallVertex> {
        let Cursor {
            ref mut list,
            ref mut prev,
            ..
        } = *self;
        match prev.as_mut() {
            None => list.front_mut(),
            Some(prev) => prev.next.as_mut().map(|next| &mut next.elem),
        }
    }

    /// Gets the next element in the list.
    pub fn next(&mut self) -> Option<&mut SmallVertex> {
        self.index += 1;
        match self.prev.take().as_mut() {
            // We had no previous element; the cursor was sitting at the start position
            // Next element should be the head of the list
            None => {
                match self.list.head {
                    // No head. No elements.
                    None => {
                        self.index = 0;
                        None
                    }
                    // Got the head. Set it as prev and yield its element
                    Some(ref mut head) => {
                        self.prev = Raw::some(&mut **head);
                        Some(&mut head.elem)
                    }
                }
            }
            // We had a previous element, so let's go to its next
            Some(prev) => {
                match prev.next {
                    // No next. We're back at the start point, null the prev and yield None
                    None => {
                        self.index = 0;
                        self.prev = Raw::none();
                        None
                    }
                    // Got a next. Set it as prev and yield its element
                    Some(ref mut next) => {
                        self.prev = Raw::some(&mut **next);
                        unsafe {
                            // upgrade the lifetime
                            Some(mem::transmute(&mut next.elem))
                        }
                    }
                }
            }
        }
    }

    /// Gets the previous element in the list.
    pub fn prev(&mut self) -> Option<&mut SmallVertex> {
        match self.prev.take().as_mut() {
            // No prev. We're at the start of the list. Yield None and jump to the end.
            None => {
                self.prev = self.list.tail.clone();
                self.index = self.list.len();
                None
            }
            // Have a prev. Yield it and go to the previous element.
            Some(prev) => {
                self.index -= 1;
                self.prev = prev.prev.clone();
                unsafe {
                    // upgrade the lifetime
                    Some(mem::transmute(&mut prev.elem))
                }
            }
        }
    }



    /// Inserts an element at the cursor's location in the list, and moves the cursor head to
    /// lie before it. Therefore, the new element will be yielded by the next call to `next`.
    pub fn insert(&mut self, elem: SmallVertex) {
        // destructure so that we can mutate list while working with prev
        let Cursor {
            ref mut list,
            ref mut prev,
            ..
        } = *self;
        match prev.as_mut() {
            // No prev, we're at the start of the list
            // Also covers empty list
            None => list.push_front(elem),
            Some(node) => {
                if node.next.as_mut().is_none() {
                    // No prev.next, we're at the end of the list
                    list.push_back(elem);
                } else {
                    // We're somewhere in the middle, splice in the new node
                    list.len += 1;
                    node.splice_next(Box::new(Node::new(elem)));
                }
            }
        }
    }

    /// Removes the next element in the list, without moving the cursor. Returns None if the list
    /// is empty, or if `next` is the ghost element
    pub fn remove(&mut self) -> Option<SmallVertex> {
        let Cursor {
            ref mut list,
            ref mut prev,
            ..
        } = *self;
        match prev.as_mut() {
            // No prev, we're at the start of the list
            // Also covers empty list
            None => list.pop_front(),
            Some(prev) => {
                match prev.take_next() {
                    // No prev.next, we're at the ghost, yield None
                    None => None,
                    // We're somewhere in the middle, rip out prev.next
                    Some(mut next) => {
                        list.len -= 1;
                        match next.next.take() {
                            // We were actually at the end of the list, so fix the list's tail
                            None => list.tail = Raw::some(prev),
                            // Really in the middle, link the results of removing next
                            Some(next_next) => prev.link(next_next),
                        }
                        Some(next.elem)
                    }
                }
            }
        }
    }

    /// Splits the list into two at the cursor's current position. This will return a new list
    /// consisting of everything after the cursor, with the original list retaining everything
    /// before. SmallVertexhe cursor will then lie between the tail and the ghost.
    pub fn split(&mut self) -> LinkedList {
        let Cursor {
            ref mut list,
            ref mut prev,
            index,
        } = *self;
        let new_tail = prev.clone();
        let len = list.len();
        match prev.as_mut() {
            // We're at index 0. SmallVertexhe new list is the whole list, so just swap
            None => mem::replace(*list, LinkedList::new()),
            // We're not at index 0. SmallVertexhis means we don't have to worry about fixing
            // the old list's head.
            Some(prev) => {
                let next_tail = list.tail.clone();
                list.len = index;
                list.tail = new_tail; // == prev
                let next_head = prev.take_next();

                LinkedList {
                    head: next_head,
                    tail: next_tail,
                    len: len - index,
                }
            }
        }
    }

    /// Inserts the entire list's contents right after the cursor.
    pub fn splice(&mut self, other: &mut LinkedList) {
        if other.is_empty() {
            return;
        }
        let len = other.len;
        other.len = 0;
        let mut head = other.head.take();
        let mut tail = other.tail.take();
        let Cursor {
            ref mut list,
            ref mut prev,
            ..
        } = *self;

        list.len += len;
        match prev.as_mut() {
            // We're at the head of the list
            None => {
                match list.head.take() {
                    // self list is empty, should just be the other list
                    None => {
                        list.head = head;
                        list.tail = tail;
                    }
                    // self list isn't empty
                    Some(self_head) => {
                        list.head = head;
                        tail.as_mut().unwrap().link(self_head);
                    }
                }
            }
            // Middle or end
            Some(prev) => {
                match prev.take_next() {
                    // We're at the end of the list
                    None => {
                        prev.link(head.take().unwrap());
                        list.tail = tail;
                    }
                    // We're strictly in the middle. Self's head and tail won't change
                    Some(next) => {
                        prev.link(head.take().unwrap());
                        tail.as_mut().unwrap().link(next);
                    }
                }
            }
        }
    }

    /// Calls `next` the specified number of times.
    pub fn seek_forward(&mut self, by: usize) {
        for _ in 0..by {
            self.next();
        }
    }

    /// Calls `prev` the specified number of times.
    pub fn seek_backward(&mut self, by: usize) {
        for _ in 0..by {
            self.prev();
        }
    }
}


impl Drop for LinkedList {
    fn drop(&mut self) {
        self.clear();
    }
}

impl Default for LinkedList {
    fn default() -> Self {
        Self::new()
    }
}



#[cfg(test)]
mod test_list {
    use super::*;
    #[test]
    fn test_is_empty() {
        let mut l: LinkedList = LinkedList::new();
        assert_eq!(l.is_empty(), true);
        l.push(SmallVertex::new(0, 1));
        assert_eq!(l.is_empty(), false);
        l.pop();
        assert_eq!(l.is_empty(), true);
    }

    #[test]
    fn test_stack_push_pop() {
        let mut l: LinkedList = LinkedList::new();
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
        let mut l: LinkedList = LinkedList::new();
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


#[cfg(test)]
mod test_graph {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Graph::new(), Graph { verticies: Vec::new() });
    }

    #[test]
    fn test_add_remove_vertex() {
        let mut g = Graph::new();
        g.add_vertex(vertex::Vertex::new(1234));
        assert!(g != Graph::new());
        let mut v2 = vertex::Vertex::new(1);
        v2.add_arc(1234, 5);
        g.add_vertex(v2);
        assert_eq!(g.verticies[0], vertex::Vertex::new(1234));
        let arc = vertex::Arc {
            to: 1234,
            distance: 5,
        };
        assert_eq!(g.verticies[1].arcs[0], arc);
        g.remove_vertex(1234);
        assert_eq!(g.verticies.len(), 1);
        assert_eq!(g.verticies[0].arcs[0], arc);
    }

    #[test]
    fn test_import() {
        let a = Graph::import(
            "0 1,1
            1 2,1 4,1000
            2 5,1 4,10
            4 0,5 5,0
            5",
        ).unwrap();
        assert_eq!(a.verticies.len(), 5);
        assert_eq!(a.verticies[0].id, 0);
        assert_eq!(a.verticies[0].arcs.len(), 1);
        assert_eq!(a.verticies[0].arcs[0], vertex::Arc { to: 1, distance: 1 });
        assert_eq!(a.verticies[4].id, 5);
        assert_eq!(a.verticies[4].arcs.len(), 0);
        assert_eq!(a.verticies[2].arcs.len(), 2);
        assert_eq!(
            a.verticies[2].arcs[1],
            vertex::Arc {
                to: 4,
                distance: 10,
            }
        );
    }

    #[test]
    #[should_panic(expected = r#"FileFormat"#)]
    fn test_import_fail1() {
        let fail_string1 = "0 1,
        1 2,1 4,1000
        2 5,1 4,10
        4 0,5 5,0
        5";
        Graph::import(fail_string1).unwrap();
    }

    #[test]
    #[should_panic(expected = r#"FileFormat"#)]
    fn test_import_fail2() {
        let fail_string2 = "0 ,1
        1 2,1 4,1000
        2 5,1 4,10
        4 0,5 5,0
        5";
        Graph::import(fail_string2).unwrap();
    }

    #[test]
    #[should_panic(expected = r#"InvalidDigit"#)]
    fn test_import_fail3() {
        let fail_string3 = "0 1,1
        a 2,1 4,1000
        2 5,1 4,10
        4 0,5 5,0
        5";
        Graph::import(fail_string3).unwrap();
    }



    #[test]
    fn test_shortest_nopath() {}

    #[test]
    fn test_export() {
        let success_strings = [
            "0 1,4\r
1 2,1 4,1000\r
2 5,1 4,10\r
4 0,5 5,0\r
5",
            "0 2,1\r
1 2,1 4,1000\r
2 5,1 4,10\r
4 0,5 5,0\r
5",
            "0 1,1\r
6 2,1 4,1000\r
2 5,1 4,10\r
4 0,5 5,0\r
5",
        ];
        for s in success_strings.iter() {
            assert_eq!(
                Graph::import(s).unwrap().export().trim().replace(" \r", ""),
                s.to_string().trim()
            );
        }
    }
}



#[cfg(all(test, feature = "unstable"))]
mod bench {
    //extern crate test;
    use test::Bencher;

    use graph::Graph;
    use vertex::Vertex;
    use rand;
    use std::fs::File;
    use std::fs;
    use std::io::Write;
    use std::path::Path;
    use std::error::Error;
    use std::io;

    #[bench]
    fn setup_0_node_4(b: &mut Bencher) {
        bench_setup_n(4, b);
    }
    #[bench]
    fn setup_1_node_256(b: &mut Bencher) {
        bench_setup_n(256, b);
    }
    #[bench]
    fn setup_2_node_4096(b: &mut Bencher) {
        bench_setup_n(4096, b);
    }


    #[bench]
    fn bench_0_node_4(b: &mut Bencher) {
        benchn(4, b);
    }
    #[bench]
    fn bench_1_node_16(b: &mut Bencher) {
        benchn(16, b);
    }
    #[bench]
    fn bench_2_node_64(b: &mut Bencher) {
        benchn(64, b);
    }
    #[bench]
    fn bench_3_node_256(b: &mut Bencher) {
        benchn(256, b);
    }
    #[bench]
    fn bench_4_node_1024(b: &mut Bencher) {
        benchn(1024, b);
    }
    #[bench]
    fn bench_5_node_4096(b: &mut Bencher) {
        benchn(4096, b);
    }

    fn bench_setup_n(n: i64, b: &mut Bencher) {
        let base = "./test_files/".to_owned();
        let filename = generate(n, base);
        let mut g = match Graph::import_file(&filename) {
            Ok(r) => r,
            Err(e) => {
                println!("Error importing: {} {}", filename, e);
                return;
            }
        };
        b.iter(|| g.setup(0));
    }

    fn benchn(n: i64, b: &mut Bencher) {
        let base = "./test_files/".to_owned();
        let filename = generate(n, base);
        let mut g = match Graph::import_file(&filename) {
            Ok(r) => r,
            Err(e) => {
                println!("Error importing: {} {}", filename, e);
                return;
            }
        };
        b.iter(|| g.shortest(0, (n - 1) as u64));
    }

    fn generate(nodes: i64, mut path: String) -> String {
        if !path.ends_with("/") {
            path = format!("{}/", path);
        }
        fs::create_dir_all(&path);
        let filename = format!("{}graph{}nodes.txt", path, nodes);
        if Path::new(&filename).exists() {
            //fs::remove_file(Path::new(&filename));
            //println!("File {} already exists, removing.", filename);
            //println!("File {} already exists, leaving.", filename);
            return filename;
        }
        print!("\nGenerating...");
        io::stdout().flush();
        let mut g = Graph::new();
        for i in 0..nodes as i64 {
            let mut v = Vertex::new(i as u64);
            for j in 0..nodes as i64 {
                if j == i {
                    continue;
                }
                //u32 to i64, to ensure >0
                let r = rand::random::<u32>() as i64 % ((nodes * (nodes - j + 1)) as i64);
                v.add_arc(j as u64, ((2 * nodes) - j) as i64 + (r));
            }
            g.add_vertex(v);
        }
        print!("✓\nExporting...");
        io::stdout().flush();
        let x = &*g.export();
        println!("✓");
        write_to_file(x, &filename);
        return filename;
    }

    fn write_to_file(source: &str, file: &str) {
        print!("Creating file...");
        io::stdout().flush();
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut f = match File::create(&file) {
            Err(why) => panic!("couldn't create {}: {}", file, why.description()),
            Ok(f) => f,
        };
        print!("✓\nWriting...");
        io::stdout().flush();
        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match f.write_all(source.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", file, why.description()),
            Ok(_) => println!("✓"),
        }
        io::stdout().flush();
    }
}
