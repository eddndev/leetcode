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
