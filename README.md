# Linked List Set

A crate that provides a data structure for compactly storing multiple 
linked lists with O(1) allocations in adjacency list-based scene graphs
with high cache coherence.

## Introduction

**list_set** is a crate providing a set data structure for storing multiple 
linked lists in one contiguous container. The main use case for this crate is 
implementing fast adjacency-list based graph data structures. By storing the
linked lists in a linear container, we can implement the linked lists in a 
compact, cache-coherent, and memory-efficient way. The nodes of each list are 
stored in a disordered way in an array-based container to ensure maximum 
cache-coherence. Another factor that improves performance of linked lists stored 
in a linear container is that we can reduce the number of allocations 
from **O(n)** to **O(1)**, where **n** is the number of elements in the list set.

## Getting Started

Add **list_set** as a dependency in your project by adding the following line to
you `Cargo.toml` file

```
[dependencies]
list_set = "0.1.0"
```

or if using the **list_set** crate directly from the source tree

```
[dependencies.list_set]
path = "/path/to/source/list_set/crate"
version = "0.1.0"
```

After that, place the crate declaration in your `lib.rs` or `main.rs`
file

```
extern crate list_set;
```

## Usage
For examples of how to use the crate, there are ample examples in the linked 
list set module documentation.

