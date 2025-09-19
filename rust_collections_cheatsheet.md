
# 🦀 Rust Collections Cheat Sheet

This document explains Rust’s main collection types with **examples** and **key notes**.

---

## 1. String (`String` and `&str`)

- **`String`**: growable, owned UTF-8 text.
- **`&str`**: borrowed string slice.

```rust
fn main() {
    let mut s = String::from("Hello");
    s.push_str(" World");
    println!("{}", s);
    println!("Length = {}", s.len());
}
```

✅ Use `String` when you need ownership and mutation.

---

## 2. Vector (`Vec<T>`)

- Growable array, contiguous in memory.
- Fast indexing, but pushing in middle is costly.

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    for x in &v {
        println!("{}", x);
    }
}
```

✅ Most common collection.

---

## 3. HashMap (`HashMap<K, V>`)

- Stores key-value pairs.
- Requires keys to implement `Eq + Hash`.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);

    for (k, v) in &scores {
        println!("{} -> {}", k, v);
    }
}
```

✅ Use when you need fast lookup by key.

---

## 4. HashSet (`HashSet<T>`)

- Stores unique elements.
- Based on `HashMap`.

```rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2); // ignored
    println!("{:?}", set);
}
```

✅ Use for uniqueness checks, set operations.

---

## 5. VecDeque (`VecDeque<T>`)

- Double-ended queue.
- Efficient push/pop at both ends.

```rust
use std::collections::VecDeque;

fn main() {
    let mut dq = VecDeque::new();
    dq.push_back(1);
    dq.push_front(2);
    println!("{:?}", dq);
}
```

✅ Great for queues.

---

## 6. LinkedList (`LinkedList<T>`)

- Doubly linked list.
- Rarely needed, slower cache performance.

```rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_front(2);
    println!("{:?}", list);
}
```

✅ Use only if you need frequent middle insertions/removals.

---

## 7. BinaryHeap (`BinaryHeap<T>`)

- Priority queue (max-heap by default).

```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(5);
    heap.push(10);
    heap.push(1);
    println!("{:?}", heap.pop()); // Some(10)
}
```

✅ Use for priority-based tasks.

---

## 8. BTreeMap (`BTreeMap<K, V>`) and BTreeSet (`BTreeSet<T>`)

- Ordered maps/sets, keep elements sorted.

```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert(2, "b");
    map.insert(1, "a");
    map.insert(3, "c");
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }
}
```

✅ Use when you need sorted iteration.

---

# ✅ Summary

- **String** → text.
- **Vec** → growable list.
- **HashMap** → key-value store (unordered).
- **HashSet** → unique elements.
- **VecDeque** → queue/deque.
- **LinkedList** → linked list.
- **BinaryHeap** → priority queue.
- **BTreeMap / BTreeSet** → sorted map/set.

---

# 🔗 References

- [The Rust Book - Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [std::collections](https://doc.rust-lang.org/std/collections/)
