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
