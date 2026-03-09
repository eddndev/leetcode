---
title: "0460 LFU Cache - EN"
problemUrl: "https://leetcode.com/problems/lfu-cache/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["hash-table", "linked-list", "design"]
complexity:
  time: "O(1)"
  space: "O(N)"
---

# The Art of Forgetting What Matters Least

## The Problem
Design and implement a data structure for a Least Frequently Used (LFU) cache. It should support `get` and `put` operations, both in O(1) time. When the cache reaches capacity and a new key must be inserted, the least frequently used key is evicted. If there is a tie in frequency, the least recently used key among them is removed.

## The Initial Intuition

When I first approached this problem, I thought about the simpler cousin: the LRU cache. In LRU, you only care about recency, and a single doubly linked list paired with a hash map does the trick. But LFU introduces a second axis -- frequency -- and that changes everything. Now you need to know not just *when* something was used, but *how many times*.

My first instinct was to use a min-heap to always extract the element with the lowest frequency, but that pushes `get` and `put` to O(log N). The problem explicitly demands O(1). So I asked myself: how can we track the minimum frequency without a heap?

## The Double Hash Map Strategy

The key insight is to maintain **two hash maps** working in tandem with **doubly linked lists**:

1. **`key_map`**: Maps each key directly to its node in memory. This gives us O(1) lookup for any key.
2. **`freq_map`**: Maps each frequency to a doubly linked list containing all nodes with that frequency. Within each list, nodes are ordered by recency -- the most recently used is at the head.

We also maintain a single variable `min_freq` that always holds the current minimum frequency in the cache.

### How `get` Works

When we call `get(key)`:
- Look up the node in `key_map`. If it's not there, return -1.
- Remove the node from its current frequency list in `freq_map`.
- If that list is now empty *and* its frequency was `min_freq`, increment `min_freq`.
- Increment the node's frequency and add it to the head of the new frequency list.
- Return the value.

### How `put` Works

When we call `put(key, value)`:
- If the key already exists, update its value and perform the same frequency bump as `get`.
- If the key is new and we're at capacity, find the list at `min_freq` and remove the tail node (the least recently used among the least frequently used). Remove that key from `key_map`.
- Create a new node with frequency 1, add it to `key_map`, add it to the frequency-1 list, and set `min_freq = 1`.

The reason we can always set `min_freq = 1` when inserting a new key is simple: a brand new key has been used exactly once, and no key in the cache can have a frequency less than 1.

### Why This Is O(1)

Every individual operation -- hash map lookup, hash map insert, linked list insertion at head, linked list removal of a known node -- is O(1). The `min_freq` variable eliminates the need for any scanning or sorting. The doubly linked lists within each frequency bucket handle the tie-breaking by recency automatically: the tail is always the oldest, and new accesses go to the head.

### A Step-by-step Example

For a cache with capacity 2:
- `put(1, 1)`: Cache = {1:1 (freq=1)}. `min_freq = 1`
- `put(2, 2)`: Cache = {1:1 (freq=1), 2:2 (freq=1)}. `min_freq = 1`. Freq-1 list: [2, 1]
- `get(1)` -> 1: Key 1's frequency bumps to 2. Freq-1 list: [2]. Freq-2 list: [1]. `min_freq = 1`
- `put(3, 3)`: At capacity. `min_freq = 1`, so we evict the tail of freq-1 list: key 2. Cache = {1:1 (freq=2), 3:3 (freq=1)}. `min_freq = 1`
- `get(2)` -> -1: Key 2 was evicted.
- `get(3)` -> 3: Key 3's frequency bumps to 2. `min_freq = 2`
- `put(4, 4)`: At capacity. `min_freq = 2`, both keys have freq=2. Evict the tail of freq-2 list: key 1 (least recently used). Cache = {3:3 (freq=2), 4:4 (freq=1)}. `min_freq = 1`

## Rust Solution

```rust
use std::cell::RefCell;
use std::collections::HashMap;
use std::ptr;
use std::rc::Rc;

struct Node {
    key: i32,
    val: i32,
    freq: i32,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(key: i32, val: i32) -> *mut Node {
        Box::into_raw(Box::new(Node {
            key,
            val,
            freq: 1,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
}

struct DList {
    head: *mut Node,
    tail: *mut Node,
    size: i32,
}

impl DList {
    fn new() -> Self {
        let head = Node::new(0, 0);
        let tail = Node::new(0, 0);
        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }
        DList {
            head,
            tail,
            size: 0,
        }
    }

    fn add_to_head(&mut self, node: *mut Node) {
        unsafe {
            let next = (*self.head).next;
            (*node).prev = self.head;
            (*node).next = next;
            (*self.head).next = node;
            (*next).prev = node;
            self.size += 1;
        }
    }

    fn remove_node(&mut self, node: *mut Node) {
        unsafe {
            let prev = (*node).prev;
            let next = (*node).next;
            (*prev).next = next;
            (*next).prev = prev;
            self.size -= 1;
        }
    }

    fn remove_last(&mut self) -> *mut Node {
        if self.size == 0 {
            return ptr::null_mut();
        }
        unsafe {
            let last = (*self.tail).prev;
            self.remove_node(last);
            last
        }
    }
}

struct CacheCtx {
    cap: i32,
    min_freq: i32,
    key_map: HashMap<i32, *mut Node>,
    freq_map: HashMap<i32, DList>,
}

pub struct LFUCache {
    ctx: RefCell<CacheCtx>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            ctx: RefCell::new(CacheCtx {
                cap: capacity,
                min_freq: 0,
                key_map: HashMap::new(),
                freq_map: HashMap::new(),
            }),
        }
    }

    fn get(&self, key: i32) -> i32 {
        let mut ctx = self.ctx.borrow_mut();
        if !ctx.key_map.contains_key(&key) {
            return -1;
        }
        unsafe {
            let node = *ctx.key_map.get(&key).unwrap();
            Self::update(&mut ctx, node);
            (*node).val
        }
    }

    fn put(&self, key: i32, value: i32) {
        let mut ctx = self.ctx.borrow_mut();
        if ctx.cap == 0 {
            return;
        }

        unsafe {
            if let Some(&node) = ctx.key_map.get(&key) {
                (*node).val = value;
                Self::update(&mut ctx, node);
            } else {
                if ctx.key_map.len() as i32 == ctx.cap {
                    // CORRECCIÓN AQUÍ: Copiar min_freq antes de pedir el préstamo mutable
                    let min_freq = ctx.min_freq;
                    let min_list = ctx.freq_map.get_mut(&min_freq).unwrap();
                    let to_del = min_list.remove_last();

                    ctx.key_map.remove(&(*to_del).key);
                    // Reclamar memoria (opcional en CP pero correcto en Rust)
                    let _ = Box::from_raw(to_del);
                }
                let new_node = Node::new(key, value);
                ctx.key_map.insert(key, new_node);
                ctx.min_freq = 1;
                ctx.freq_map
                    .entry(1)
                    .or_insert(DList::new())
                    .add_to_head(new_node);
            }
        }
    }

    fn update(ctx: &mut CacheCtx, node: *mut Node) {
        unsafe {
            let freq = (*node).freq;
            let list = ctx.freq_map.get_mut(&freq).unwrap();
            list.remove_node(node);

            if list.size == 0 && freq == ctx.min_freq {
                ctx.min_freq += 1;
            }

            (*node).freq += 1;
            let new_freq = (*node).freq;
            ctx.freq_map
                .entry(new_freq)
                .or_insert(DList::new())
                .add_to_head(node);
        }
    }
}
```

The Rust implementation is where things get interesting. Rust's ownership model does not naturally accommodate doubly linked lists with shared mutable access, so this solution dives into `unsafe` territory with raw pointers (`*mut Node`). Nodes are heap-allocated via `Box::into_raw` and manually freed with `Box::from_raw` during eviction. The `DList` struct uses sentinel head and tail nodes to simplify insertion and removal at the boundaries -- a classic technique that eliminates edge cases. The entire cache state lives inside a `RefCell<CacheCtx>` to allow interior mutability, which is necessary because `get` logically modifies the cache (it bumps frequency) despite having a conceptually read-only interface. The `update` function is the heart of the design: it removes the node from its old frequency list, checks whether `min_freq` needs adjustment, and reinserts the node into the next frequency bucket -- all in constant time.

## Conclusion

The LFU cache is one of those problems where the naive approach seems almost impossible to optimize, but the right combination of data structures makes everything fall into place. The double hash map architecture -- one for direct key access, one for frequency buckets -- is the foundation. The doubly linked lists within each bucket handle tie-breaking by recency. And the `min_freq` variable is the elegant shortcut that avoids any kind of search for the eviction candidate. What I find most satisfying about this design is how each piece exists for exactly one reason, and removing any of them would break the O(1) guarantee. It's a reminder that sometimes the most complex requirements are met not by a single clever trick, but by the careful orchestration of simple, well-understood components.
