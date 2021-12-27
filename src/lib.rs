//! # Linked List Set
//! A crate that provides a data structure for compactly storing multiple 
//! linked lists with O(1) allocations in adjacency list-based scene graphs
//! with high cache coherence.
//! 
//! ## Introduction
//! **list_set** is a crate providing a set data structure for storing multiple 
//! linked lists in one contiguous container. The main use case for this crate is 
//! implementing fast adjacency-list based graph data structures. By storing the
//! linked lists in a linear container, we can implement the linked lists in a 
//! compact, cache-coherent, and memory-efficient way. The nodes of each list are 
//! stored in a disordered way in an array-based container to ensure maximum 
//! cache-coherence. Another factor that improves performance of linked lists stored 
//! in a linear container is that we can reduce the number of allocations 
//! from **O(n)** to **O(1)**, where **n**.
//! 
//! ## Getting Started
//! Add **list_set** as a dependency in your project by adding the following line to
//! you `Cargo.toml` file
//! ```text
//! [dependencies]
//! list_set = "0.1.0"
//! ```
//! or if using the **list_set** crate directly from the source tree
//! ```text
//! [dependencies.list_set]
//! path = "/path/to/source/list_set/crate"
//! version = "0.1.0"
//! ```
//! After that, place the crate declaration in your `lib.rs` or `main.rs`
//! file
//! ```rust
//! extern crate list_set;
//! ```
//! 
//! ## Usage
//! For examples of how to use the crate, there are ample examples in the linked 
//! list set module documentation.
//!
extern crate fnv;


use fnv::{
    FnvHashMap,
};
use std::fmt;
use std::marker::{
    PhantomData,
};


/// An internal index describing the location of a linked list node inside the
/// underlying storage of a linked list set.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct NodeIndex(usize);

impl NodeIndex {
    fn new(index: usize) -> Self {
        Self {
            0: index,
        }
    }

    const fn end() -> Self {
        Self {
            0: usize::MAX
        }
    }
}

/// A container that holds an element in a linked list.
#[derive(Clone, Debug)]
struct Node<T> {
    /// The item in the linked list.
    item: T,
    /// The index of the linked list the node is a member of.
    list: ListIndex,
    /// The position of the previous child list node inside the scene graph's 
    /// contiguous child list node storage.
    previous: NodeIndex,
    /// The position of the next child list element inside the scene graph's
    /// contiguous child list node storage.
    next: NodeIndex,
}

impl<T> Node<T> {
    /// Construct a new linked list node.
    fn new(list_index: ListIndex, item: T) -> Self {
        Self {
            item: item,
            list: list_index,
            previous: NodeIndex::end(),
            next: NodeIndex::end(),
        }
    }

    /// Returns an immutable reference to the item stored in the node.
    #[inline]
    fn item(&self) -> &T {
        &self.item
    }

    /// Returns an immutable reference to the item stored in the node.
    #[inline]
    fn item_mut(&mut self) -> &mut T {
        &mut self.item
    }

    /// Get the index of the previous item in the linked list.
    #[inline]
    fn previous(&self) -> NodeIndex {
        self.previous
    }

    /// Get the index of the next item in the linked list.
    #[inline]
    fn next(&self) -> NodeIndex {
        self.next
    }
}

/// A doubly linked list whose nodes are owned by a contiguous backing store.
#[doc(hidden)]
#[derive(Debug)]
pub struct LinkedList<T> {
    /// The index of the first node in the linked list.
    front: NodeIndex,
    /// The index of the last node in the linked list.
    back: NodeIndex,
    /// The number of elements in the linked list.
    length: usize,
    /// A marker indicating the type of them elements storage in the list.
    /// We store this data inside the nodes owned by the backing store, and
    /// not directly in the node itself.
    _marker: PhantomData<T>,
}

impl<T> LinkedList<T> {
    /// Create a new empty list.
    pub const fn new() -> Self {
        Self {
            front: NodeIndex::end(),
            back: NodeIndex::end(),
            length: 0,
            _marker: PhantomData,
        }
    }

    /// Determine whether the linked list is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.front == NodeIndex::end() 
            && self.back == NodeIndex::end()
    }

    /// Returns the length of the linked list.
    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        Self {
            front: self.front,
            back: self.back,
            length: self.length,
            _marker: PhantomData,
        }
    }
}

/// A handle to a linked list inside the stored inside of a linked list set.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListIndex(usize);

impl ListIndex {
    /// Construct a new linked list index.
    pub fn new(index: usize) -> Self {
        Self {
            0: index,
        }
    }

    pub const fn end() -> Self {
        Self {
            0: usize::MAX,
        }
    }
}

impl fmt::Display for ListIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct ListIter<'a, T> {
    current_front: NodeIndex,
    current_back: NodeIndex,
    position_front: usize,
    position_back: usize,
    list: LinkedList<T>,
    nodes: &'a [Node<T>],
}

impl<'a, T: 'a> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_front != NodeIndex::end() {
            let node = &self.nodes[self.current_front.0];
            let item = node.item();

            self.current_front = node.next();
            self.position_front += 1;

            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.list.len() - self.position_front;

        (remaining, Some(remaining))
    }
}

impl<'a, T: 'a> DoubleEndedIterator for ListIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_back != NodeIndex::end() {
            let node = &self.nodes[self.current_back.0];
            let item = node.item();

            self.current_back = node.previous();
            self.position_back += 1;

            Some(item)
        } else {
            None
        }
    }
}

impl<'a, T: 'a> ExactSizeIterator for ListIter<'a, T> {}


#[derive(Debug)]
pub struct ListIterMut<'a, T> {
    current_front: NodeIndex,
    current_back: NodeIndex,
    position_front: usize,
    position_back: usize,
    list: LinkedList<T>,
    nodes: &'a mut [Node<T>],
}

impl<'a, T: 'a> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_front != NodeIndex::end() {
            // # Safety
            // The mutable reference to a node is only used once
            // by the iterator, and nowhere else. This holds because the 
            // node storage owns all of the nodes, and the iterator slice is a
            // mutable reference to the node storage. Therefore, we can safely
            // sidestep the borrow checker to get a mutable reference to each
            // node inside the node storage.
            let node = unsafe {
                #[inline(always)]
                unsafe fn bounded_by<A>(base_ptr: *mut A, len: usize, count: usize) -> bool {
                    let peak_ptr = base_ptr.add(len);
                    let ptr = base_ptr.add(count);

                    (ptr <= peak_ptr) && (ptr >= base_ptr)
                } 

                let base_ptr = self.nodes.as_mut_ptr();
                let count = self.current_front.0;

                assert!(bounded_by(base_ptr, self.nodes.len(), count));

                &mut *base_ptr.add(count)
            };
            let new_current_front = node.next();
            let item = node.item_mut();

            self.current_front = new_current_front;
            self.position_front += 1;

            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.list.len() - self.position_front;

        (remaining, Some(remaining))
    }
}

impl<'a, T: 'a> DoubleEndedIterator for ListIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_back != NodeIndex::end() {
            // # Safety
            // The mutable reference to a node is only used once
            // by the iterator, and nowhere else. This holds because the 
            // node storage owns all of the nodes, and the iterator slice is a
            // mutable reference to the node storage. Therefore, we can safely
            // sidestep the borrow checker to get a mutable reference to each
            // node inside the node storage.
            let node = unsafe {
                #[inline(always)]
                unsafe fn bounded_by<A>(base_ptr: *mut A, len: usize, count: usize) -> bool {
                    let peak_ptr = base_ptr.add(len);
                    let ptr = base_ptr.add(count);

                    (ptr <= peak_ptr) && (ptr >= base_ptr)
                } 

                let base_ptr = self.nodes.as_mut_ptr();
                let count = self.current_back.0;

                assert!(bounded_by(base_ptr, self.nodes.len(), count));

                &mut *base_ptr.add(count)
            };
            let new_current_back = node.previous();
            let item = node.item_mut();

            self.current_back = new_current_back;
            self.position_back += 1;

            Some(item)
        } else {
            None
        }
    }
}

impl<'a, T: 'a> ExactSizeIterator for ListIterMut<'a, T> {}


pub struct ListIndices<'a, T> {
    iter: std::collections::hash_map::Keys<'a, ListIndex, LinkedList<T>>,
}

impl<'a, T> Iterator for ListIndices<'a, T> {
    type Item = ListIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().copied()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> ExactSizeIterator for ListIndices<'a, T> {}


#[derive(Clone, Debug, Default)]
struct ListIndexAllocator {
    current: usize,
}

impl ListIndexAllocator {
    const fn new() -> Self {
        Self {
            current: 0,
        }
    }

    #[inline]
    fn new_index(&mut self) -> ListIndex {
        let next_index = ListIndex::new(self.current);
        self.current += 1;

        next_index
    }
}


/// A collection of linked lists whose nodes are stored in an array-based 
/// container.
///
/// The set allows pushing and popping elements to a particular linked list at 
/// either end in constant time. Storing a set of linked lists in a linked 
/// list set is more memory efficient, and makes better use of the CPU cache. 
/// The ideal case for using a linked list set is in implementing compact 
/// adjacency-list style graph data structures such as scene graphs.
///
/// The linked lists stored in a `LinkedListSet` are accessed using their 
/// `ListIndex` handle. The handle is guaranteed to be stable until the 
/// list is explicitly removed from the set. That is, a `ListIndex` 
/// pointing to a linked list in the set will continue to point to the same 
/// linked list until the list is deleted from the set.
#[derive(Clone, Debug, Default)]
pub struct LinkedListSet<T> {
    /// The allocator for generating new list handles.
    alloc: ListIndexAllocator,
    /// The collection of linked lists stored in the set.
    lists: fnv::FnvHashMap<ListIndex, LinkedList<T>>,
    /// The collection of all the nodes nodes of all the linked lists in the 
    /// set. The nodes themselves can appear in any order inside the underlying 
    /// storage.
    nodes: Vec<Node<T>>,
}

impl<T> LinkedListSet<T> {
    /// Create a new linked list set.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set: LinkedListSet<usize> = LinkedListSet::new();
    ///
    /// assert!(set.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            alloc: ListIndexAllocator::new(),
            lists: FnvHashMap::default(),
            nodes: Vec::new(),
        }
    }

    /// Create a new linked list set with the specified capacity of
    /// linked lists and linked list nodes.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let node_capacity = 3000; 
    /// let mut set: LinkedListSet<usize> = LinkedListSet::with_capacity(
    ///     node_capacity
    /// );
    ///
    /// assert_eq!(set.node_capacity(), node_capacity);
    /// ```
    pub fn with_capacity(node_capacity: usize) -> Self {
        Self {
            alloc: ListIndexAllocator::new(),
            lists: FnvHashMap::default(),
            nodes: Vec::with_capacity(node_capacity),
        }
    }

    /// Create a new empty linked list in a linked list set.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set: LinkedListSet<usize> = LinkedListSet::new();
    ///
    /// assert_eq!(set.list_count(), 0);
    /// let _ = set.new_list();
    /// assert_ne!(set.list_count(), 0);
    /// ```
    pub fn new_list(&mut self) -> ListIndex {
        let new_list_index = self.alloc.new_index();
        let new_list = LinkedList::new();
        self.lists.insert(new_list_index, new_list);

        new_list_index
    }

    /// Determine whether the entire linked list set is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// set.extend(list_index, vec![0, 1, 2, 3, 4, 5]);
    /// 
    /// assert!(!set.is_empty());
    /// assert_ne!(set.list_count(), 0);
    /// assert_ne!(set.node_count(), 0);
    /// assert!(!set.list_is_empty(list_index));
    ///
    /// set.clear_all();
    ///
    /// assert!(set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.lists.is_empty()
    }

    /// Get an immutable reference to a linked list with the given index 
    /// unchecked.
    ///
    /// # Panics
    ///
    /// Panics if a linked list with the list index `list_index` does not
    /// exist in the set.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,
    /// #     ListIndex,
    /// # };
    /// #
    /// let mut set: LinkedListSet<usize> = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// let too_large_list_index = ListIndex::new(usize::MAX - 1);
    /// 
    /// let ok_result = std::panic::catch_unwind(|| {
    ///     set.get_list_unchecked(list_index)
    /// });
    /// assert!(ok_result.is_ok());
    ///
    /// let err_result = std::panic::catch_unwind(|| {
    ///     set.get_list_unchecked(too_large_list_index)
    /// });
    /// assert!(err_result.is_err());
    /// ```
    #[inline]
    pub fn get_list_unchecked(&self, list_index: ListIndex) -> &LinkedList<T> {
        &self.lists[&list_index]
    }

    /// Get an mutable reference to a linked list with the given index 
    /// unchecked.
    ///
    /// # Panics
    ///
    /// Panics if a linked list with the list index `list_index` does not
    /// exist in the set.
    #[inline]
    fn get_list_mut_unchecked(&mut self, list_index: ListIndex) -> &mut LinkedList<T> {
        self.lists.get_mut(&list_index).unwrap()
    }

    /// Get the linked list with the given index unchecked.
    ///
    /// # Panics
    ///
    /// Panics if a linked list with the list index `list_index` does not
    /// exist in the set.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,
    /// #     ListIndex,
    /// # };
    /// #
    /// let mut set: LinkedListSet<usize> = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// let too_large_list_index = ListIndex::new(usize::MAX - 1);
    /// 
    /// let ok_result = std::panic::catch_unwind(|| {
    ///     set.get_list_unchecked(list_index)
    /// });
    /// assert!(ok_result.is_ok());
    ///
    /// let err_result = std::panic::catch_unwind(|| {
    ///     set.get_list_unchecked(too_large_list_index)
    /// });
    /// assert!(err_result.is_err());
    /// ```
    #[inline]
    pub fn get_list(&self, list_index: ListIndex) -> Option<&LinkedList<T>> {
        if self.contains_list(list_index) {
            Some(self.get_list_unchecked(list_index))
        } else {
            None
        }
    }

    /// Get an immutable reference to a specific node from the linked list 
    /// set unchecked.
    ///
    /// # Panics
    ///
    /// Panics if the node index does not exist in the set.
    ///
    /// # Note
    /// Node indices are not stable betwen linked list set mutations.
    #[inline]
    fn get_node_unchecked(&self, node_index: NodeIndex) -> &Node<T> {
        &self.nodes[node_index.0]
    }

    /// Get a mutable reference to a specific node from the linked list set unchecked.
    ///
    /// # Panics
    ///
    /// Panics if the node index does not exist in the set.
    #[inline]
    fn get_node_mut_unchecked(&mut self, node_index: NodeIndex) -> &mut Node<T> {
        &mut self.nodes[node_index.0]
    }

    /// Returns the length of the linked list indexed by `list_index`.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// 
    /// assert_eq!(set.len(list_index), 0);
    ///
    /// set.push_back(list_index, 321);
    ///
    /// assert_ne!(set.len(list_index), 0);
    /// ```
    pub fn len(&self, list_index: ListIndex) -> usize {
        self.get_list_unchecked(list_index).len()
    }

    /// Determine whether a particular linked list in the set is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// 
    /// assert!(set.list_is_empty(list_index));
    ///
    /// set.push_back(list_index, 321);
    ///
    /// assert!(!set.list_is_empty(list_index));
    /// ```
    pub fn list_is_empty(&self, list_index: ListIndex) -> bool {
        self.get_list_unchecked(list_index).is_empty()
    }

    /// Returns the number of linked lists in the set.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set: LinkedListSet<usize> = LinkedListSet::new();
    /// let list_index0 = set.new_list();
    /// let list_index1 = set.new_list();
    /// let list_index2 = set.new_list();
    /// 
    /// assert_eq!(set.list_count(), 3);
    /// ```
    pub fn list_count(&self) -> usize {
        self.lists.len()
    }

    /// Returns the maximum number items across all lists combined that the
    /// set can currently hold.
    ///
    /// The function returns the number of items the collection currently *can* hold,
    /// *not* the number of items the function currently holds. For that, use the
    /// `node_count` function.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let expected_node_capacity = 3000;
    /// let mut set: LinkedListSet<usize> = LinkedListSet::with_capacity(
    ///     expected_node_capacity
    /// );
    /// 
    /// assert_eq!(set.node_capacity(), expected_node_capacity);
    /// ```
    pub fn node_capacity(&self) -> usize {
        self.nodes.capacity()
    }

    /// Returns the number of linked list nodes in the set.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index0 = set.new_list();
    /// let list_index1 = set.new_list();
    /// let list_index2 = set.new_list();
    /// set.extend(list_index0, vec![0, 1, 2, 3, 4, 5]);
    /// set.extend(list_index1, vec![6, 7, 8, 9, 10, 11]);
    /// set.extend(list_index2, vec![12, 13, 14, 15, 16, 17]);
    ///
    /// assert_eq!(
    ///     set.node_count(), 
    ///     set.len(list_index0) + set.len(list_index1) + set.len(list_index2)
    /// );
    /// ```
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Determine whether a linked list contains a particular item.
    ///
    /// Returns `true` if the linked list contains an instance of `item`, and 
    /// `false` if the linked list does not contain and instance of `item`.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,  
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// set.extend(
    ///     list_index, 
    ///     vec![String::from("spam"), String::from("eggs")]
    /// );
    ///
    /// assert!(set.contains(list_index, &String::from("spam")));
    /// assert!(set.contains(list_index, &String::from("eggs")));
    /// assert!(!set.contains(list_index, &String::from("pancakes")));
    /// ```
    ///
    /// If you do not have an instance of `&T`, but just an `&U` such that `T: Borrow<U>`
    /// (for example, `String: Borrow<str>`), you can also use the `any()` function from
    /// the iterator trait.
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,  
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// set.extend(
    ///     list_index, 
    ///     vec![String::from("spam"), String::from("eggs")]
    /// );
    ///
    /// assert!(set.iter(list_index).any(|item| item == "spam"));
    /// assert!(set.iter(list_index).any(|item| item == "eggs"));
    /// assert!(!set.iter(list_index).any(|item| item == "pancakes"));
    /// ```
    pub fn contains(&self, list_index: ListIndex, item: &T) -> bool
    where
        T: PartialEq<T>
    {
        let mut current_index = self.get_list_unchecked(list_index).front;
        while current_index != NodeIndex::end() {
            let current_node = self.get_node_unchecked(current_index);
            if current_node.item() == item {
                return true;
            } else {
                current_index = current_node.next();
            }
        }

        false
    }

    /// Determine whether a linked list set contains a linked list with the 
    /// index given by `list_index`.
    ///
    /// # Example
    /// 
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,
    /// #     ListIndex,
    /// # };
    /// #
    /// let mut set: LinkedListSet<usize> = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// 
    /// assert!(set.contains_list(list_index));
    /// assert!(!set.contains_list(ListIndex::new(usize::MAX - 1)));
    /// ```
    pub fn contains_list(&self, list_index: ListIndex) -> bool {
        self.lists.contains_key(&list_index)
    }

    /// Provide an immutable forward iterator for a linked list with the 
    /// index `list_index` inside the linked list set.
    ///
    /// # Example
    /// 
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// let expected = vec![
    ///     String::from("spam"), 
    ///     String::from("eggs"), 
    ///     String::from("pancakes")
    /// ];
    /// set.extend(list_index, expected.iter().cloned());
    /// 
    /// assert!(set.iter(list_index).enumerate().all(|(i, item_i)| {
    ///     item_i == &expected[i]
    /// }));
    /// ```
    pub fn iter(&self, list_index: ListIndex) -> ListIter<T> {
        ListIter {
            current_front: self.get_list_unchecked(list_index).front,
            current_back: self.get_list_unchecked(list_index).back,
            position_front: 0,
            position_back: 0,
            list: self.get_list_unchecked(list_index).clone(),
            nodes: &self.nodes,
        }
    }

    /// Provide a mutable forward iterator for a linked list with the 
    /// index `list_index` inside the linked list set.
    ///
    /// # Example
    /// 
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// set.extend(list_index, data);
    /// 
    /// for item in set.iter_mut(list_index) {
    ///     *item = 2 * (*item);
    /// }
    ///
    /// let expected = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
    /// let result: Vec<usize> = set.iter(list_index).copied().collect();
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn iter_mut(&mut self, list_index: ListIndex) -> ListIterMut<T> {
        ListIterMut {
            current_front: self.get_list_unchecked(list_index).front,
            current_back: self.get_list_unchecked(list_index).back,
            position_front: 0,
            position_back: 0,
            list: self.get_list_unchecked(list_index).clone(),
            nodes: &mut self.nodes,
        }
    }

    /// Provide an iterator over the handles of the linked lists stored inside
    /// the set.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #    LinkedListSet,    
    /// # };
    /// #
    /// let mut set: LinkedListSet<usize> = LinkedListSet::new();
    /// set.new_list();
    /// set.new_list();
    /// set.new_list();
    /// set.new_list();
    /// set.new_list();
    ///
    /// assert!(set.list_indices().all(|list_index| {
    ///     set.contains_list(list_index)  
    /// }));
    /// ```
    pub fn list_indices(&self) -> ListIndices<T> {
        ListIndices {
            iter: self.lists.keys(),
        }
    }

    /// Clear the entire linked list set.
    ///
    /// This operation deletes all the nodes and all the linked lists contained in the
    /// set.
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,  
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// set.extend(
    ///     list_index, 
    ///     vec![String::from("spam"), String::from("eggs"), String::from("pancakes")]
    /// );
    ///
    /// assert!(!set.is_empty());
    /// set.clear_all();
    /// assert!(set.is_empty());
    /// ```
    pub fn clear_all(&mut self) {
        self.nodes.clear();
        self.lists.clear();
    }

    /// Provides an immutable reference to the front element of a list, or
    /// `None` if the list is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    ///
    /// assert!(set.front(list_index).is_none());
    ///
    /// set.extend(
    ///     list_index, 
    ///     vec![String::from("spam"), String::from("eggs"), String::from("pancakes")]
    /// );
    ///
    /// assert_eq!(set.front(list_index).map(|p| p.as_str()), Some("spam"));
    /// ```
    pub fn front(&self, list_index: ListIndex) -> Option<&T> {
        let front_node_index = {
            let list = self.get_list(list_index)?;
            list.front
        };
        if front_node_index != NodeIndex::end() {
            let node = self.get_node_unchecked(front_node_index);

            Some(node.item())
        } else {
            None
        }
    }

    /// Provides a mutable reference to the front element of a list, or
    /// `None` if the list is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    ///
    /// assert!(set.back_mut(list_index).is_none());
    ///
    /// set.extend(
    ///     list_index, 
    ///     vec![String::from("spam"), String::from("eggs"), String::from("pancakes")]
    /// );
    ///
    /// assert_eq!(set.front_mut(list_index).map(|p| p.as_str()), Some("spam"));
    /// {
    ///     let mut front = set.front_mut(list_index).unwrap();
    ///     *front = String::from("bacon");
    /// }
    /// assert_eq!(set.front(list_index).map(|p| p.as_str()), Some("bacon"));
    /// ```
    pub fn front_mut(&mut self, list_index: ListIndex) -> Option<&mut T> {
        let front_node_index = {
            let list = self.get_list(list_index)?;
            list.front
        };
        if front_node_index != NodeIndex::end() {
            let node = self.get_node_mut_unchecked(front_node_index);

            Some(node.item_mut())
        } else {
            None
        }
    }

    /// Provides an immutable reference to the back element of a list, or
    /// `None` if the list is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    ///
    /// assert!(set.back(list_index).is_none());
    ///
    /// set.extend(
    ///     list_index, 
    ///     vec![String::from("spam"), String::from("eggs"), String::from("pancakes")]
    /// );
    ///
    /// assert_eq!(set.back(list_index).map(|p| p.as_str()), Some("pancakes"));
    /// ```
    pub fn back(&self, list_index: ListIndex) -> Option<&T> {
        let back_node_index = {
            let list = self.get_list(list_index)?;
            list.back
        };
        if back_node_index != NodeIndex::end() {
            let node = self.get_node_unchecked(back_node_index);

            Some(node.item())
        } else {
            None
        }
    }

    /// Provides a mutable reference to the back element of a list, or
    /// `None` if the list is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    ///
    /// assert!(set.back_mut(list_index).is_none());
    ///
    /// set.extend(
    ///     list_index, 
    ///     vec![String::from("spam"), String::from("eggs"), String::from("pancakes")]
    /// );
    ///
    /// assert_eq!(set.back_mut(list_index).map(|p| p.as_str()), Some("pancakes"));
    /// {
    ///     let mut back = set.back_mut(list_index).unwrap();
    ///     *back = String::from("waffles");
    /// }
    /// assert_eq!(set.back(list_index).map(|p| p.as_str()), Some("waffles"));
    /// ```
    pub fn back_mut(&mut self, list_index: ListIndex) -> Option<&mut T> {
        let back_node_index = {
            let list = self.get_list(list_index)?;
            list.back
        };
        if back_node_index != NodeIndex::end() {
            let node = self.get_node_mut_unchecked(back_node_index);

            Some(node.item_mut())
        } else {
            None
        }
    }

    /// Link a new node into a linked list in the set.
    #[inline]
    fn link_list_node(
        &mut self, 
        node_index: NodeIndex, 
        previous_index: NodeIndex, 
        next_index: NodeIndex
    ) {
        {
            let node = self.get_node_mut_unchecked(node_index);
            node.previous = previous_index;
            node.next = next_index;
        }

        if next_index != NodeIndex::end() {
            let next_node = self.get_node_mut_unchecked(next_index);
            next_node.previous = node_index;
        }

        if previous_index != NodeIndex::end() {
            let previous_node = self.get_node_mut_unchecked(previous_index);
            previous_node.next = node_index;
        }
    }

    /// Push an item to the back of a linked list in a linked list set.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,  
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    ///
    /// assert!(set.back(list_index).is_none());
    ///
    /// set.extend(list_index, vec![0, 1, 2, 3, 4, 5]);
    /// 
    /// assert_eq!(set.back(list_index), Some(&5));
    //
    /// set.push_back(list_index, 300);
    ///
    /// assert_eq!(set.back(list_index), Some(&300));
    /// ```
    pub fn push_back(&mut self, list_index: ListIndex, item: T) {
        let new_node = Node::new(list_index, item);
        let new_node_index = NodeIndex::new(self.nodes.len());
        self.nodes.push(new_node);
        if self.get_list_unchecked(list_index).is_empty() {
            let list = self.get_list_mut_unchecked(list_index);
            list.front = new_node_index;
            list.back = new_node_index;
        } else {
            let back = self.get_list_unchecked(list_index).back;
            self.link_list_node(
                new_node_index, 
                back, 
                NodeIndex::end()
            );
            self.get_list_mut_unchecked(list_index).back = new_node_index;
        }

        self.get_list_mut_unchecked(list_index).length += 1;
    }

    /// Push an item to the front of a linked list in a linked list set.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,  
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    ///
    /// assert!(set.front(list_index).is_none());
    ///
    /// set.extend(list_index, vec![0, 1, 2, 3, 4, 5]);
    /// 
    /// assert_eq!(set.front(list_index), Some(&0));
    //
    /// set.push_front(list_index, 300);
    ///
    /// assert_eq!(set.front(list_index), Some(&300));
    /// ```
    pub fn push_front(&mut self, list_index: ListIndex, item: T) {
        let new_node = Node::new(list_index, item);
        let new_node_index = NodeIndex::new(self.nodes.len());
        self.nodes.push(new_node);
        if self.get_list_unchecked(list_index).is_empty() {
            let list = self.get_list_mut_unchecked(list_index);
            list.front = new_node_index;
            list.back = new_node_index;
        } else {
            let front = self.get_list_unchecked(list_index).front;
            self.link_list_node(
                new_node_index, 
                NodeIndex::end(),
                front
            );
            self.get_list_mut_unchecked(list_index).front = new_node_index;
        }

        self.get_list_mut_unchecked(list_index).length += 1;
    }

    /// Unlink a node from a linked list.
    fn unlink_list_node(&mut self, node_index: NodeIndex) {
        let previous_index = {
            let node = self.get_node_unchecked(node_index);
            node.previous
        };
        let next_index = {
            let node = self.get_node_unchecked(node_index);
            node.next
        };

        if previous_index != NodeIndex::end() {
            let previous_node = self.get_node_mut_unchecked(previous_index);
            previous_node.next = next_index;
        }
        
        if next_index != NodeIndex::end() {
            let next_node = self.get_node_mut_unchecked(next_index);
            next_node.previous = previous_index;
        }

        let list_index = {
            let node = self.get_node_unchecked(node_index);
            node.list
        };
        let list = self.get_list_mut_unchecked(list_index);

        if list.front == node_index {
            list.front = next_index;
        }

        if list.back == node_index {
            list.back = previous_index;
        }

        list.length -= 1;
    }

    /// Relink a list node after moving it to a different entry in the 
    /// underlying storage.
    fn relink_list_node(
        &mut self, 
        old_node_index: NodeIndex, 
        new_node_index: NodeIndex
    ) {
        if old_node_index != new_node_index {
            let previous_index = {
                let node = self.get_node_unchecked(old_node_index);
                node.previous
            };
            let next_index = {
                let node = self.get_node_unchecked(old_node_index);
                node.next
            };
        
            if previous_index != NodeIndex::end() {
                let previous_node = self.get_node_mut_unchecked(previous_index);
                previous_node.next = new_node_index;
            }

            if next_index != NodeIndex::end() {
                let next_node = self.get_node_mut_unchecked(next_index);
                next_node.previous = new_node_index;
            }

            // Check front and back of the list of the node index being 
            // moved, old_node_index.
            let list_index = {
                let node = self.get_node_unchecked(old_node_index);
                node.list
            };
            let list = self.get_list_mut_unchecked(list_index);
        
            if list.front == old_node_index {
                list.front = new_node_index;
            }
        
            if list.back == old_node_index {
                list.back = new_node_index;
            } 
        }
    }

    /// Remove a linked list node from the set.
    ///
    /// The function unlinks the node with the input node index from its linked 
    /// list. It may internally modify the layout of other linked lists in the 
    /// process of removing the node to keep the nodes in the underlying storage 
    /// packed.
    ///
    /// # Assumptions
    /// * The list node index exists in the set.
    /// 
    /// # Panics
    ///
    /// This function panics if `node_to_be_removed_index` is out of bounds.
    fn remove_list_node(&mut self, node_to_be_removed_index: NodeIndex) -> T {
        let node_to_be_moved_index = NodeIndex::new(self.nodes.len() - 1);
        self.unlink_list_node(node_to_be_removed_index);
        self.relink_list_node(node_to_be_moved_index, node_to_be_removed_index);

        let removed = self.nodes.swap_remove(node_to_be_removed_index.0);

        removed.item
    }

    /// Remove and return an item at a specific position in a linked list.
    ///
    /// Returns `None` if the list is empty, or `at` falls outside the length 
    /// of the list.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// set.extend(list_index, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    ///
    /// assert!(set.remove(list_index, 5).is_some());
    /// assert!(set.remove(list_index, 11).is_none());
    /// ```
    pub fn remove(&mut self, list_index: ListIndex, at: usize) -> Option<(T, usize)> {
        let mut current_index = self.get_list_unchecked(list_index).front;
        let mut i = 0;
        while current_index != NodeIndex::end() {
            let current_node = self.get_node_unchecked(current_index);
            if i == at {
                let res = self.remove_list_node(current_index);

                return Some((res, i));
            } else {
                current_index = current_node.next;
                i += 1;
            }
        }

        None
    }

    /// Remove and return the first instance of an item from a linked list
    /// in a linked list set.
    ///
    /// Returns `None` if the linked list is empty, or `item` is not present 
    /// in the list.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index0 = set.new_list();
    /// // Insert a collection with exactly one instance of `1`.
    /// set.extend(list_index0, vec![0, 0, 0, 1, 0, 0, 0]);
    /// 
    /// assert!(set.contains(list_index0, &1));
    /// assert!(set.remove_item(list_index0, &1).is_some());
    /// // The set no longer contains the removed item.
    /// assert!(!set.contains(list_index0, &1));
    ///
    /// let list_index1 = set.new_list();
    /// // Insert a collection with more than one instance of `1`.
    /// set.extend(list_index1, vec![0, 0, 0, 1, 0, 1, 0]);
    ///
    /// assert!(set.contains(list_index1, &1));
    /// assert!(set.remove_item(list_index1, &1).is_some());
    /// // The set still contains an instance of the removed item.
    /// assert!(set.contains(list_index1, &1));
    /// ```
    pub fn remove_item(&mut self, list_index: ListIndex, item: &T) -> Option<(T, usize)> 
    where
        T: PartialEq<T>
    {
        let mut current_index = self.get_list_unchecked(list_index).front;
        let mut i = 0;
        while current_index != NodeIndex::end() {
            let current_node = self.get_node_unchecked(current_index);
            if current_node.item() == item {
                let res = self.remove_list_node(current_index);

                return Some((res, i));
            } else {
                current_index = current_node.next();
                i += 1;
            }
        }

        None
    }

    /// Remove and return the first element from a linked list in a linked
    /// list set.
    ///
    /// Returns `None` if the linked list is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// set.push_back(list_index, 1);
    /// set.push_back(list_index, 2);
    /// set.push_back(list_index, 3);
    ///
    /// let result = set.pop_front(list_index);
    /// assert_eq!(result, Some(1));
    /// ```
    pub fn pop_front(&mut self, list_index: ListIndex) -> Option<T> {
        let front_node_index = self.get_list_unchecked(list_index).front;
        if front_node_index != NodeIndex::end() {
            let item = self.remove_list_node(front_node_index);

            return Some(item);
        }

        None
    }

    /// Remove and return the last element from a linked list in a linked
    /// list set.
    ///
    /// Returns `None` if the linked list is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet, 
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// set.push_back(list_index, 1);
    /// set.push_back(list_index, 2);
    /// set.push_back(list_index, 3);
    ///
    /// let result = set.pop_back(list_index);
    /// assert_eq!(result, Some(3));
    /// ```
    pub fn pop_back(&mut self, list_index: ListIndex) -> Option<T> {
        let back_node_index = self.get_list_unchecked(list_index).back;
        if back_node_index != NodeIndex::end() {
            let item = self.remove_list_node(back_node_index);

            return Some(item);
        }

        None
    }

    /// Remove all elements from a linked list in the set.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,  
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// set.extend(
    ///     list_index, 
    ///     vec![String::from("spam"), String::from("eggs")
    /// ]);
    ///
    /// assert!(!set.list_is_empty(list_index));
    ///
    /// set.clear(list_index);
    ///
    /// assert!(set.list_is_empty(list_index));
    /// ```
    pub fn clear(&mut self, list_index: ListIndex) {
        while !self.list_is_empty(list_index) {
            self.pop_front(list_index);
        }
    }

    /// Remove a linked list and all its data from the set.
    ///
    /// Returns `true` if the list removed exists in the 
    /// set prior to calling `remove_list`, and `false` otherwise. The 
    /// `remove_list` function preserves the indices of all the other lists 
    /// in the set. If `remove_list` returns false, no lists were affected.
    ///
    /// # Example
    ///
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,
    /// #     ListIndex, 
    /// # }; 
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_indices = [set.new_list(), set.new_list(), set.new_list()];
    /// for (i, &list_index_i) in list_indices.iter().enumerate() {
    ///     set.extend(list_index_i, vec![i; 5]);  
    /// }
    ///
    /// assert!(list_indices.iter().all(|l| set.contains_list(*l)));
    /// // Remove the list pointed to by `list_indices[0]`.
    /// set.remove_list(list_indices[0]);
    ///
    /// assert!(!set.contains_list(list_indices[0]));
    /// assert!(list_indices[1..].iter().all(|l| set.contains_list(*l)));
    /// ```
    pub fn remove_list(&mut self, list_index: ListIndex) -> bool {
        if self.contains_list(list_index) {
            self.clear(list_index);
            self.lists.remove(&list_index);

            true
        } else {
            false
        }
    }

    /// Extend a linked list with an iterable collection of items.
    ///
    /// # Example
    /// 
    /// ```
    /// # use list_set::{
    /// #     LinkedListSet,
    /// #     
    /// # };
    /// #
    /// let mut set = LinkedListSet::new();
    /// let list_index = set.new_list();
    /// let expected = vec![0, 1, 2, 3, 4, 5];
    /// 
    /// set.extend(list_index, expected.iter().copied());
    /// 
    /// let result: Vec<usize> = set.iter(list_index).copied().collect();
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn extend<I>(&mut self, list_index: ListIndex, items: I)
    where
        I: IntoIterator<Item = T>
    {
        for item in items.into_iter() {
            self.push_back(list_index, item);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_multiple_lists_each_node_in_each_list_has_the_same_list_index() {
        let mut set = LinkedListSet::new();
        let list_indices = [
            set.new_list(),
            set.new_list(),
            set.new_list()   
        ];
        let list_lengths = [10, 8, 30];
        for (list_index, list_length) in list_indices.iter().copied()
            .zip(list_lengths.iter().copied())
        {
            for item in 0..list_length {
                set.push_front(list_index, item);
            }
        }

        for list_index in list_indices.iter().copied() {
            let mut current_index = set.get_list_unchecked(list_index).front;
            let expected = list_index;
            while current_index != NodeIndex::end() {
                let current_node = set.get_node_unchecked(current_index);
                let result = current_node.list;
                current_index = current_node.next();

                assert_eq!(result, expected);
            }
        }
    }
}

#[cfg(test)]
mod iter_mut_tests {
    use super::*;


    struct Test {
        set: LinkedListSet<usize>,
        expected: FnvHashMap<ListIndex, Vec<usize>>,
    }

    fn linked_list_set() -> LinkedListSet<usize> {
        // We hand construct the lists to ensure the nodes in a given list are not 
        // adjacent to each other in the underlying storage.
        let mut lists = FnvHashMap::default();
        lists.insert(ListIndex::new(0), LinkedList {
            front: NodeIndex::new(0),
            back: NodeIndex::new(9),
            length: 4,
            _marker: PhantomData,
        });
        lists.insert(ListIndex::new(1), LinkedList {
            front: NodeIndex::new(1),
            back: NodeIndex::new(10),
            length: 4,
            _marker: PhantomData,
        });
        lists.insert(ListIndex::new(2), LinkedList {
            front: NodeIndex::new(2),
            back: NodeIndex::new(13),
            length: 6,
            _marker: PhantomData,
        });
        let nodes = vec![
            Node {
                item: 10,
                list: ListIndex::new(0),
                previous: NodeIndex::end(),
                next: NodeIndex::new(3),
            },
            Node {
                item: 20,
                list: ListIndex::new(1),
                previous: NodeIndex::end(),
                next: NodeIndex::new(4),
            },
            Node {
                item: 30,
                list: ListIndex::new(2),
                previous: NodeIndex::end(),
                next: NodeIndex::new(5),
            },
            Node {
                item: 11,
                list: ListIndex::new(0),
                previous: NodeIndex::new(0),
                next: NodeIndex::new(6),
            },
            Node {
                item: 21,
                list: ListIndex::new(1),
                previous: NodeIndex::new(1),
                next: NodeIndex::new(7),
            },
            Node {
                item: 31,
                list: ListIndex::new(2),
                previous: NodeIndex::new(2),
                next: NodeIndex::new(8),
            },
            Node {
                item: 12,
                list: ListIndex::new(0),
                previous: NodeIndex::new(3),
                next: NodeIndex::new(9),
            },
            Node {
                item: 22,
                list: ListIndex::new(1),
                previous: NodeIndex::new(4),
                next: NodeIndex::new(10),
            },
            Node {
                item: 32,
                list: ListIndex::new(2),
                previous: NodeIndex::new(5),
                next: NodeIndex::new(11),
            },
            Node {
                item: 13,
                list: ListIndex::new(0),
                previous: NodeIndex::new(6),
                next: NodeIndex::end(),
            },
            Node {
                item: 23,
                list: ListIndex::new(1),
                previous: NodeIndex::new(7),
                next: NodeIndex::end(),
            },
            Node {
                item: 33,
                list: ListIndex::new(2),
                previous: NodeIndex::new(8),
                next: NodeIndex::new(12),
            },
            Node {
                item: 34,
                list: ListIndex::new(2),
                previous: NodeIndex::new(11),
                next: NodeIndex::new(13),
            },
            Node {
                item: 35,
                list: ListIndex::new(2),
                previous: NodeIndex::new(12),
                next: NodeIndex::end(),
            },
        ];
        let alloc = ListIndexAllocator::new();
        
        LinkedListSet {
            alloc: alloc,
            lists: lists,
            nodes: nodes,
        }
    }

    fn test() -> Test {
        let set = linked_list_set();
        let mut expected = FnvHashMap::default();
        expected.insert(ListIndex::new(0), vec![10, 11, 12, 13]);
        expected.insert(ListIndex::new(1), vec![20, 21, 22, 23]);
        expected.insert(ListIndex::new(2), vec![30, 31, 32, 33, 34, 35]);

        Test {
            set: set,
            expected: expected,
        }
    }

    fn test_rev() -> Test {
        let set = linked_list_set();
        let mut expected = FnvHashMap::default();
        expected.insert(ListIndex::new(0), vec![13, 12, 11, 10]);
        expected.insert(ListIndex::new(1), vec![23, 22, 21, 20]);
        expected.insert(ListIndex::new(2), vec![35, 34, 33, 32, 31, 30]);

        Test {
            set: set,
            expected: expected,
        }
    }

    /// A mutable iterator should correctly produce the right linked list nodes
    /// even when the nodes for a linked list are out of order in the underlying 
    /// storage.
    #[test]
    fn test_mutable_iterator_yields_only_nodes_from_the_same_list() {
        let mut test = test();
        let list_indices: Vec<_> = test.set.list_indices().collect();
        for list_index in list_indices.iter().copied() {
            let expected = &test.expected[&list_index];
            let result: Vec<usize> = test.set
                .iter_mut(list_index)
                .map(|v| *v)
                .collect();

            assert_eq!(&result, expected);
        }
    }

    #[test]
    fn test_mutable_iterator_list_lengths() {
        let mut test = test();
        let list_indices: Vec<_> = test.set.list_indices().collect();
        for list_index in list_indices.iter().copied() {
            let expected = &test.expected[&list_index];
            let result: Vec<usize> = test.set
                .iter_mut(list_index)
                .map(|v| *v)
                .collect();

            assert_eq!(result.len(), expected.len());
        }
    }

    /// A mutable iterator in reverse should correctly produce the right 
    /// linked list nodes even when the nodes for a linked list are out of 
    /// order in the underlying storage.
    #[test]
    fn test_mutable_iterator_reverse_yields_only_nodes_from_the_same_list() {
        let mut test = test_rev();
        let list_indices: Vec<_> = test.set.list_indices().collect();
        for list_index in list_indices.iter().copied() {
            let expected = &test.expected[&list_index];
            let result: Vec<usize> = test.set
                .iter_mut(list_index)
                .rev()
                .map(|v| *v)
                .collect();

            assert_eq!(&result, expected);
        }
    }

    #[test]
    fn test_mutable_iterator_reverse_lengths() {
        let mut test = test_rev();
        let list_indices: Vec<_> = test.set.list_indices().collect();
        for list_index in list_indices.iter().copied() {
            let expected = &test.expected[&list_index];
            let result: Vec<usize> = test.set
                .iter_mut(list_index)
                .rev()
                .map(|v| *v)
                .collect();

            assert_eq!(result.len(), expected.len());
        }
    }
}


#[cfg(test)]
mod unlink_tests {
    use super::*;


    #[test]
    fn test_unlink_list_node_single_item_list() {
        let mut set: LinkedListSet<usize> = LinkedListSet::new();
        let list_index = set.new_list();
        set.push_back(list_index, 0xDEAFBEEF);

        assert!(!set.list_is_empty(list_index));
        assert_eq!(set.len(list_index), 1);

        set.unlink_list_node(NodeIndex::new(0));

        assert!(set.list_is_empty(list_index));
        assert_eq!(set.len(list_index), 0);
    }

    #[test]
    fn test_unlink_list_node_front() {
        let mut set: LinkedListSet<usize> = LinkedListSet::new();
        let list_index = set.new_list();
        let items = vec![1, 2, 3];
        for item in items.iter().copied() {
            set.push_back(list_index, item);
        }

        set.unlink_list_node(NodeIndex::new(0));

        let expected = vec![2, 3];
        let result: Vec<usize> = set.iter(list_index).copied().collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_unlink_list_node_back() {
        let mut set: LinkedListSet<usize> = LinkedListSet::new();
        let list_index = set.new_list();
        let items = vec![1, 2, 3];
        for item in items.iter().copied() {
            set.push_back(list_index, item);
        }

        set.unlink_list_node(NodeIndex::new(1));

        let expected = vec![1, 3];
        let result: Vec<usize> = set.iter(list_index).copied().collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_unlink_list_node_middle() {
        let mut set: LinkedListSet<usize> = LinkedListSet::new();
        let list_index = set.new_list();
        let items = vec![1, 2, 3];
        for item in items.iter().copied() {
            set.push_back(list_index, item);
        }

        set.unlink_list_node(NodeIndex::new(2));

        let expected = vec![1, 2];
        let result: Vec<usize> = set.iter(list_index).copied().collect();

        assert_eq!(result, expected);
    }
}

