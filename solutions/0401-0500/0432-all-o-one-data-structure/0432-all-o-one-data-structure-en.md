---
title: "0432 All O`one Data Structure - EN"
problemUrl: "https://leetcode.com/problems/all-oone-data-structure/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["hash-table", "linked-list", "design", "doubly-linked-list"]
complexity:
  time: "O(1) per inc, dec, getMaxKey, and getMinKey operation"
  space: "O(N), where N is the number of distinct keys in the structure"
---

# The Ledger That Never Loses Track

## The Problem
Design an `AllOne` data structure that supports four operations, all in O(1) time: `inc(key)` which increments the count of `key` by 1 (or inserts it with count 1 if it does not exist), `dec(key)` which decrements the count of `key` by 1 (removing it if it reaches 0), `getMaxKey()` which returns any key with the maximum count, and `getMinKey()` which returns any key with the minimum count.

## Why a HashMap Alone Falls Short

My first instinct was to use a simple `HashMap<String, i32>` to track counts. Incrementing and decrementing would be O(1), perfect. But the problem demands that `getMaxKey` and `getMinKey` also run in O(1), and finding the maximum or minimum in a HashMap requires scanning all values. I need an additional structure that keeps counts ordered and gives me instant access to both extremes.

## The Doubly Linked List of Frequencies

The core idea is to combine a `HashMap` with a doubly linked list where each node represents a specific count and stores the set of keys that share that count. The nodes are ordered from smallest to largest count, and the list has sentinels at both ends: a `head` node with count 0 and a `tail` node with count `i32::MAX`. This eliminates edge cases when inserting or removing nodes.

With this structure:
- `getMinKey` simply looks at the first real node after `head`.
- `getMaxKey` looks at the last real node before `tail`.
- Both operations are O(1) because I just follow pointers.

## The Dance of Increment and Decrement

When I call `inc(key)`, there are two scenarios:

1. **The key is new**: I look for the node with count 1 right after `head`. If it does not exist, I create it. I add the key to that node and register it in the HashMap pointing to that node.
2. **The key already exists**: I find it in its current node with count `c`. I check whether the next node has count `c + 1`. If not, I create a new node between the current one and the next. I move the key to the count `c + 1` node. If the count `c` node becomes empty, I remove it.

`dec(key)` is the mirror image: if the count drops to 0, I simply remove the key from the HashMap. Otherwise, I find or create the node with count `c - 1` just before the current node. I move the key and clean up the old node if it becomes empty.

Each of these operations touches at most a couple of adjacent nodes in the list, so everything runs in O(1).

## Walking Through an Example

Consider the operations: `inc("a")`, `inc("b")`, `inc("a")`, `getMinKey()`, `getMaxKey()`, `dec("a")`.

- **inc("a")**: Create node with count 1, add "a". List: `head <-> [1: {a}] <-> tail`.
- **inc("b")**: Node with count 1 already exists, add "b". List: `head <-> [1: {a, b}] <-> tail`.
- **inc("a")**: Move "a" from node 1 to node 2 (which I create). List: `head <-> [1: {b}] <-> [2: {a}] <-> tail`.
- **getMinKey()**: The node after `head` has count 1, return "b".
- **getMaxKey()**: The node before `tail` has count 2, return "a".
- **dec("a")**: Move "a" from node 2 to node 1. Node 2 is now empty, remove it. List: `head <-> [1: {b, a}] <-> tail`.

## Rust Solution

```rust
use std::collections::{HashMap, HashSet};
use std::ptr;

struct Node {
    cnt: i32,
    keys: HashSet<String>,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(cnt: i32) -> *mut Node {
        Box::into_raw(Box::new(Node {
            cnt,
            keys: HashSet::new(),
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
}

pub struct AllOne {
    map: HashMap<String, *mut Node>,
    head: *mut Node,
    tail: *mut Node,
}

impl AllOne {
    fn new() -> Self {
        let head = Node::new(0);
        let tail = Node::new(i32::MAX);
        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }
        AllOne {
            map: HashMap::new(),
            head,
            tail,
        }
    }

    unsafe fn add_node(&self, prev: *mut Node, cnt: i32) -> *mut Node {
        let node = Node::new(cnt);
        let next = (*prev).next;
        (*node).prev = prev;
        (*node).next = next;
        (*prev).next = node;
        (*next).prev = node;
        node
    }

    unsafe fn remove_node(&self, node: *mut Node) {
        let prev = (*node).prev;
        let next = (*node).next;
        (*prev).next = next;
        (*next).prev = prev;
        let _ = Box::from_raw(node);
    }

    fn inc(&mut self, key: String) {
        unsafe {
            if let Some(&cur) = self.map.get(&key) {
                let cnt = (*cur).cnt;
                let next = (*cur).next;
                let target = if (*next).cnt == cnt + 1 {
                    next
                } else {
                    self.add_node(cur, cnt + 1)
                };
                (*target).keys.insert(key.clone());
                self.map.insert(key.clone(), target);
                (*cur).keys.remove(&key);
                if (*cur).keys.is_empty() {
                    self.remove_node(cur);
                }
            } else {
                let first = (*self.head).next;
                let target = if (*first).cnt == 1 {
                    first
                } else {
                    self.add_node(self.head, 1)
                };
                (*target).keys.insert(key.clone());
                self.map.insert(key, target);
            }
        }
    }

    fn dec(&mut self, key: String) {
        unsafe {
            if let Some(&cur) = self.map.get(&key) {
                let cnt = (*cur).cnt;
                if cnt > 1 {
                    let prev = (*cur).prev;
                    let target = if (*prev).cnt == cnt - 1 {
                        prev
                    } else {
                        self.add_node((*cur).prev, cnt - 1)
                    };
                    (*target).keys.insert(key.clone());
                    self.map.insert(key.clone(), target);
                } else {
                    self.map.remove(&key);
                }
                (*cur).keys.remove(&key);
                if (*cur).keys.is_empty() {
                    self.remove_node(cur);
                }
            }
        }
    }

    fn get_max_key(&self) -> String {
        unsafe {
            let last = (*self.tail).prev;
            if last == self.head {
                return "".to_string();
            }
            (*last).keys.iter().next().cloned().unwrap_or_default()
        }
    }

    fn get_min_key(&self) -> String {
        unsafe {
            let first = (*self.head).next;
            if first == self.tail {
                return "".to_string();
            }
            (*first).keys.iter().next().cloned().unwrap_or_default()
        }
    }
}
```

The implementation uses raw pointers (`*mut Node`) to build the doubly linked list, which in Rust requires `unsafe` blocks. Each node stores its count, a `HashSet` of keys, and pointers to the previous and next nodes. The `head` and `tail` sentinels simplify all node insertion and removal logic, eliminating the need to check whether we are at the edges of the list. The `HashMap` maps each key directly to the node where it lives, allowing any key to be located in O(1) and moved to the adjacent node in the same complexity. When a node runs out of keys, `remove_node` unlinks it from the list and frees its memory via `Box::from_raw`.

## Conclusion

All O`one Data Structure is a problem that tests the ability to design composite structures where every piece serves a precise purpose. The HashMap provides O(1) access to each key's location, the linked list keeps counts ordered with O(1) access to both extremes, and the sets inside each node group keys with the same count so that moving a key between adjacent frequencies is a constant-time operation. It is the perfect orchestration of three data structures that, working together, achieve what would be impossible individually: four operations in O(1) on a dynamic frequency dictionary.
