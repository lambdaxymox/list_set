extern crate list_set;


use list_set::*;
use std::collections::{
    HashMap,
};


/// An empty linked list set should have no elements inside it.
#[test]
fn test_empty_set() {
    let set: LinkedListSet<usize> = LinkedListSet::new();

    assert!(set.is_empty());
    assert_eq!(set.list_count(), 0);
    assert_eq!(set.node_count(), 0);
}

/// A fresh linked list in a linked list set shoudl have no elements inside it.
#[test]
fn test_empty_list() {
    let mut set: LinkedListSet<usize> = LinkedListSet::new();
    let list_index = set.new_list();

    assert!(set.list_is_empty(list_index));
}

/// The length of an empty list should be zero.
#[test]
fn test_empty_list_length() {
    let mut set: LinkedListSet<usize> = LinkedListSet::new();
    let list_index = set.new_list();

    assert_eq!(set.len(list_index), 0);
}

/// A non-empty linked list has at least one element.
#[test]
fn test_list_with_one_item_nonempty() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    set.push_back(list_index, 0);

    assert!(!set.list_is_empty(list_index));
}

/// A non-empty linked list has at least one element.
#[test]
fn test_list_with_one_item_length() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    set.push_back(list_index, 0);

    assert_eq!(set.len(list_index), 1);
}

/// Pushing an item to a linked list should correctly increment the 
/// length of the list.
#[test]
fn test_list_push_back_length() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    for item in 0..item_count {
        set.push_back(list_index, item);
    }

    assert_eq!(set.len(list_index), item_count);
}

/// A collection of items pushed to the back of the linked list should
/// appear in the order they we pushed to the linked list.
#[test]
fn test_list_push_back() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    for item in 0..item_count {
        set.push_back(list_index, item);
    }

    let expected: Vec<usize> = (0..item_count).collect();
    let result: Vec<usize> = set.iter(list_index).copied().collect();

    assert_eq!(result, expected);
}

/// With multiple linked lists in the same directory, pushing to the back
/// of one of them should not affect the other linked lists, only the list being
/// pushed to.
#[test]
fn test_list_push_back_length_multiple_lists() {
    let mut set = LinkedListSet::new();
    let list_indices = [
        set.new_list(),
        set.new_list(),
        set.new_list()   
    ];
    let list_lengths = [10, 8, 3];
    for (list_index, list_length) in list_indices.iter().copied()
        .zip(list_lengths.iter().copied())
    {
        for item in 0..list_length {
            set.push_back(list_index, item);
        }
    }

    for (list_index, list_length) in list_indices.iter().copied()
        .zip(list_lengths.iter().copied())
    {
        assert_eq!(set.len(list_index), list_length);
    }
}

/// With multiple linked lists in the same directory, pushing to the back
/// of one of them should not affect the other linked lists, only the list being
/// pushed to.
#[test]
fn test_list_push_back_multiple_lists() {
    let mut set = LinkedListSet::new();
    let list_indices = [
        set.new_list(),
        set.new_list(),
        set.new_list()   
    ];
    let list_lengths = [10, 8, 3];
    for (list_index, list_length) in list_indices.iter().copied()
        .zip(list_lengths.iter().copied())
    {
        for item in 0..list_length {
            set.push_back(list_index, item);
        }
    }

    let expected_vecs: Vec<Vec<usize>> = vec![
        (0..list_lengths[0]).collect(), 
        (0..list_lengths[1]).collect(), 
        (0..list_lengths[2]).collect()
    ];

    for (list_index, expected) in list_indices.iter().copied()
        .zip(expected_vecs.iter())
    {
        let result: Vec<usize> = set.iter(list_index).copied().collect();
        assert_eq!(result, *expected);
    }
}

/// Pushing an item to the front of a linked list should correctly increment 
/// the length of the list.
#[test]
fn test_list_push_front_length() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    for item in 0..item_count {
        set.push_front(list_index, item);
    }

    assert_eq!(set.len(list_index), item_count);
}

/// A collection of items pushed to the back of the linked list should
/// appear in the reverse order they we pushed to the linked list.
#[test]
fn test_list_push_front() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    for item in 0..item_count {
        set.push_front(list_index, item);
    }

    let expected: Vec<usize> = (0..item_count).rev().collect();
    let result: Vec<usize> = set.iter(list_index).copied().collect();

    assert_eq!(result, expected);
}

/// With multiple linked lists in the same directory, pushing to the front
/// of one of them should not affect the other linked lists, only the list being
/// pushed to.
#[test]
fn test_list_push_front_multiple_lists() {
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

    let expected_vecs: Vec<Vec<usize>> = vec![
        (0..list_lengths[0]).rev().collect(), 
        (0..list_lengths[1]).rev().collect(), 
        (0..list_lengths[2]).rev().collect()
    ];

    for (list_index, expected) in list_indices.iter().copied()
        .zip(expected_vecs.iter())
    {
        let result: Vec<usize> = set.iter(list_index).copied().collect();
        assert_eq!(result, *expected);
    }
}

/// A collection of items pushed to the back of the linked list should
/// appear in the order they we pushed to the linked list.
#[test]
fn test_list_push_back_elements_in_order() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    for (result, expected) in set.iter(list_index).zip(items.iter()) {
        assert_eq!(result, expected);
    }
}

/// With multiple linked lists in the same directory, pushing to the back
/// of one of them should not affect the other linked lists, only the list being
/// pushed to.
#[test]
fn test_multiple_list_push_back_lengths() {
    let mut set = LinkedListSet::new();
    let list_indices = [
        set.new_list(),
        set.new_list(),
        set.new_list()
    ];
    let list_lengths = [5, 80, 1];
    for (list_index, list_length) in list_indices.iter().copied()
        .zip(list_lengths.iter().copied())
    {
        for item in 0..list_length {
            set.push_back(list_index, item);
        }
    }

    for (list_index, expected_length) in list_indices.iter().copied()
        .zip(list_lengths.iter().copied())
    {
        assert_eq!(set.len(list_index), expected_length);
    }
}

/// A collection of items pushed to the back of the linked list should
/// appear in the order they we pushed to the linked list, even in the 
/// presence of other linked lists in the set.
#[test]
fn test_multiple_list_push_back_in_order() {
    let mut set = LinkedListSet::new();
    let list_indices = [
        set.new_list(),
        set.new_list(),
        set.new_list()
    ];
    let list_lengths = [5, 80, 1];
    let expected_lists: Vec<Vec<usize>> = list_lengths.iter()
        .copied()
        .map(|list_length| (0..list_length).collect())
        .collect();

    for (list_index, list_length) in list_indices.iter().copied()
        .zip(list_lengths.iter().copied())
    {
        for item in 0..list_length {
            set.push_back(list_index, item);
        }
    }

    let result_lists: Vec<Vec<usize>> = list_indices.iter()
        .copied()
        .map(|list_index| set.iter(list_index).copied().collect())
        .collect();

    for (result, expected) in result_lists.iter().zip(expected_lists.iter())
    {
        assert_eq!(result, expected);
    }
}

/// When removing an item at a specific position in a linked list, the 
/// returned item should be the correct one removed.
#[test]
fn test_list_remove_at_position() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let at = 5;
    let expected = 5;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    let (result, removed_at) = set.remove(list_index, at).unwrap();
    assert_eq!(removed_at, at);
    assert_eq!(result, expected);
}

/// When we successfully remove an item from a linked list, the node that held
/// that item no longer exists in the linked list. That is, the length of the linked
/// list is lowered by one.
#[test]
fn test_list_remove_at_len() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let at = 5;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    let expected = item_count - 1;
    let _ = set.remove(list_index, at);

    assert_eq!(set.len(list_index), expected);
}

/// When we successfully remove an item from a linked list, the node that held
/// that item no longer exists in the linked list.
#[test]
fn test_list_remove_at() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    let expected = vec![0, 1, 2, 3, 4, 5, 7, 8, 9];
    let _ = set.remove(list_index, 6);
    let result: Vec<usize> = set.iter(list_index).copied().collect();

    assert_eq!(result, expected);
}

/// Removing an item from an empty list should yield nothing.
#[test]
fn test_list_remove_at_empty() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let result: Option<(usize, usize)> = set.remove(list_index, 1);

    assert!(result.is_none());
}

/// When pushing an item to an empty list, and then removing an item
/// from the front of the list, the item removed from the list should be identical
/// the item added to the list.
#[test]
fn test_list_remove_at_one_element() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();

    set.push_back(list_index, 7);
    let item = set.remove(list_index, 0);
    
    assert_eq!(item, Some((7, 0)));

    let expected = vec![];
    let result: Vec<usize> = set.iter(list_index).copied().collect();

    assert_eq!(result, expected);
}

/// Removing an item from position 0 in a list should give the first item in 
/// the list.
#[test]
fn test_list_remove_at_head() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let _ = set.remove(list_index, 0);
    let result: Vec<usize> = set.iter(list_index).copied().collect();

    assert_eq!(result, expected);
}

/// Removing an item from the last position in a list should give the last item in
/// the list.
#[test]
fn test_list_remove_at_back() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let _ = set.remove(list_index, 9);
    let result: Vec<usize> = set.iter(list_index).copied().collect();

    assert_eq!(result, expected);
}

/// Removing a specific item from a list should remove the first item with
/// that value in the list. Any later values matching the removed value 
/// still lie in the list.
#[test]
fn test_list_remove_item() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let items: Vec<usize> = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    let expected = vec![1, 1, 2, 2, 3, 3, 4, 5, 5];
    let _ = set.remove_item(list_index, &4);
    let result: Vec<usize> = set.iter(list_index).copied().collect();

    assert_eq!(result, expected);
    assert!(set.contains(list_index, &4));
}

/// Removing every item from a list should yield an empty list.
#[test]
fn test_list_clear_list_with_remove() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    for _ in 0..item_count {
        let _ = set.remove(list_index, 0);
    }
    
    assert!(set.list_is_empty(list_index));
    assert_eq!(set.len(list_index), 0);
}

/// After removing a unique item from a linked list, the linked list
/// should not contain that item.
#[test]
fn test_list_contains() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }
    let removed = 4;

    assert!(set.contains(list_index, &removed));
    let _ = set.remove_item(list_index, &removed);
    assert!(!set.contains(list_index, &removed));
}

/// A linked list should be empty after clearing it.
#[test]
fn test_list_clear() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    assert_eq!(set.len(list_index), item_count);
    set.clear(list_index);
    assert_eq!(set.len(list_index), 0);
}

/// A linked list should contain no elements.
#[test]
fn test_list_clear_contains() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    set.clear(list_index);
    for item in items.iter() {
        assert!(!set.contains(list_index, item));
    }
}

/// An empty linked list does not have any items to remove.
#[test]
fn test_remove_from_empty_list() {
    let mut set: LinkedListSet<usize> = LinkedListSet::new();
    let list_index = set.new_list();

    assert!(set.remove(list_index, 1).is_none());
}

/// After calling remove on an empty list, the length should remain zero.
#[test]
fn test_remove_from_empty_list_len() {
    let mut set: LinkedListSet<usize> = LinkedListSet::new();
    let list_index = set.new_list();
    set.remove(list_index, 1);

    assert_eq!(set.len(list_index), 0);
}

/// Popping from the back of the list should yield the back item in the list.
#[test]
fn test_pop_back() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    let expected = items[items.len() - 1];
    let result = set.pop_back(list_index).unwrap();

    assert_eq!(result, expected);
}

/// Popping an item from the front of a list with at least one element
/// should lower the length of the list by one.
#[test]
fn test_pop_front_len() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    let expected = set.len(list_index) - 1;
    let _ = set.pop_front(list_index);
    let result = set.len(list_index);

    assert_eq!(result, expected);
}

/// Popping an item from the back of a list with at least one element
/// should lower the length of the list by one.
#[test]
fn test_pop_back_len() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }

    let expected = set.len(list_index) - 1;
    let _ = set.pop_back(list_index);
    let result = set.len(list_index);

    assert_eq!(result, expected);
}

/// Popping an item from the back of an empty list should yield no elements.
#[test]
fn test_empty_list_pop_back() {
    let mut set: LinkedListSet<usize> = LinkedListSet::new();
    let list_index = set.new_list();
    
    assert!(set.pop_back(list_index).is_none());
}

/// Popping an item from the front of a list with at least one element
/// should lower the length of the list by one.
#[test]
fn test_front_pop_front() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }
    let expected = set.front(list_index).copied();
    let result = set.pop_front(list_index);

    assert_eq!(result, expected);
}

/// Popping an item from the back of a list should yield the back item in 
/// the list.
#[test]
fn test_back_pop_back() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }
    let expected = set.back(list_index).copied();
    let result = set.pop_back(list_index);

    assert_eq!(result, expected);
}

/// Pushing an item to the back of the list, then popping an item from the back
/// of a list, should yield the original item pushed to the back of them list.
#[test]
fn test_push_back_pop_back() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }
    
    let expected = set.back(list_index).copied();
    set.push_back(list_index, 11);
    set.pop_back(list_index);
    let result = set.back(list_index).copied();

    assert_eq!(result, expected);
}

/// Pushing an item to the front of the list, then popping an item from the front
/// of a list, should yield the original item pushed to the front of them list.
#[test]
fn test_push_front_pop_front() {
    let mut set = LinkedListSet::new();
    let list_index = set.new_list();
    let item_count = 10;
    let items: Vec<usize> = (0..item_count).collect();
    for item in items.iter().copied() {
        set.push_back(list_index, item);
    }
    
    let expected = set.front(list_index).copied();
    set.push_front(list_index, 11);
    set.pop_front(list_index);
    let result = set.front(list_index).copied();

    assert_eq!(result, expected);
}

/// In a linked list set with multiple linked lists in it, removing an item from
/// one linked list should not affect the other linked lists.
#[test]
fn test_multiple_lists_remove_leaves_other_lists_unaffected() {
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
            set.push_back(list_index, item);
        }
    }

    let expected = [list_lengths[0], list_lengths[2]].iter()
        .map(|&len| (0..len).collect::<Vec<usize>>())
        .collect::<Vec<_>>();

    set.remove(list_indices[1], 5);
    set.remove(list_indices[1], 3);

    let result: Vec<Vec<usize>> = [list_indices[0], list_indices[2]].iter()
        .map(|list_index| set.iter(*list_index).copied().collect())
        .collect();

    assert_eq!(result, expected);
}

/// Given a linked list set with multiple linked lists in it, when one
/// of the lists is removed, the list no longer exists in the set. After removal,
/// the remaining lists are unaffected.
#[test]
fn test_removing_one_list_does_not_change_the_other_lists() {
    let mut set = LinkedListSet::new();
    let list_indices = [
        set.new_list(),
        set.new_list(),
        set.new_list(),
        set.new_list(),
        set.new_list(),
    ];
    let list_lengths = [10, 8, 30, 131, 47];
    for (list_index, list_length) in list_indices.iter().copied()
        .zip(list_lengths.iter().copied())
    {
        for item in 0..list_length {
            set.push_back(list_index, item);
        }
    }

    let removed_list_index = list_indices[3];
    let mut expected: HashMap<ListIndex, Vec<usize>> = HashMap::default();
    for (list_index, list_length) in 
        [list_indices[0], list_indices[1], list_indices[2], list_indices[4]]
        .iter()
        .copied()
        .zip([list_lengths[0], list_lengths[1], list_lengths[2], list_lengths[4]]
            .iter()
            .copied()
        )
    {
        let expected_list: Vec<usize> = (0..list_length).collect();
        expected.insert(list_index, expected_list);
    }

    let ok = set.remove_list(removed_list_index);
    assert!(ok);

    let mut result = HashMap::default();
    for list_index in set.list_indices() {
        let result_list = set.iter(list_index).copied().collect();
        result.insert(list_index, result_list);
    }

    assert_eq!(result, expected);
}

/// Given a linked list set with multiple linked lists in it, when one
/// of the lists is removed, that list no longer exists in the set, and the
/// number of lists in the collection is reduced by one.
#[test]
fn test_remove_list_list_count() {
    let mut set = LinkedListSet::new();
    let list_indices = [
        set.new_list(),
        set.new_list(),
        set.new_list(),
        set.new_list(),
        set.new_list(),
    ];
    let list_lengths = [10, 8, 30, 131, 47];
    for (list_index, list_length) in list_indices.iter().copied()
        .zip(list_lengths.iter().copied())
    {
        for item in 0..list_length {
            set.push_back(list_index, item);
        }
    }

    let expected = set.list_count() - 1;
    let removed_list_index = list_indices[3];
    let ok = set.remove_list(removed_list_index);
    assert!(ok);

    let result =  set.list_count();

    assert_eq!(result, expected);
}

/// Given a linked list set with multiple linked lists in it, when the list index
/// to be removed does not exist, nothing happens.
#[test]
fn test_remove_list_list_not_in_set() {
    let mut set = LinkedListSet::new();
    let list_indices = [
        set.new_list(),
        set.new_list(),
        set.new_list(),
        set.new_list(),
        set.new_list(),
    ];
    let list_lengths = [10, 8, 30, 131, 47];
    for (list_index, list_length) in list_indices.iter().copied()
        .zip(list_lengths.iter().copied())
    {
        for item in 0..list_length {
            set.push_back(list_index, item);
        }
    }

    let removed_list_index = ListIndex::end();
    let mut expected: HashMap<ListIndex, Vec<usize>> = HashMap::default();
    for list_index in set.list_indices() {
        expected.insert(list_index, set.iter(list_index).copied().collect());
    }

    let ok = set.remove_list(removed_list_index);
    assert!(!ok);

    let mut result: HashMap<ListIndex, Vec<usize>> = HashMap::default();
    for list_index in set.list_indices() {
        result.insert(list_index, set.iter(list_index).copied().collect());
    }

    assert_eq!(result, expected);
}

