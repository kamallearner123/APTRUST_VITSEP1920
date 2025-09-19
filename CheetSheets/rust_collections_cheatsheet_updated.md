# Rust Collections — Practical Guide & Method Reference

> **Audience:** Rust learners moving beyond basics.  
> **Covers:** Core collections from `std` + `String`/`&str` + slices, with examples and commonly used methods (stable).  
> **Tip:** Prefer the method sets here as a *working* reference; the standard library docs are the source of truth.

---

## 0) Cheatsheet: Which collection should I pick?

| Need | Best bet | Why |
|---|---|---|
| Growable array, contiguous memory, fast random access | `Vec<T>` | Cache‑friendly, most common, great general‑purpose |
| Queue/deque (push/pop at both ends) | `VecDeque<T>` | O(1) amortized at both ends |
| Frequent fast ordered iteration by key | `BTreeMap<K,V>` / `BTreeSet<K>` | Sorted order, range queries |
| Fast average‑case lookup by key | `HashMap<K,V>` / `HashSet<K>` | O(1) average, order not guaranteed |
| Priority queue (max‑heap) | `BinaryHeap<T>` | Quickly get/remove largest item |
| Frequent insert/remove in the middle (rarely needed) | `LinkedList<T>` | True O(1) splices; poor cache locality |
| Text you own & can mutate | `String` | UTF‑8, growable |
| Borrowed text | `&str` | Slices UTF‑8 text without allocation |

---

## 1) Fundamentals (ownership, borrowing, iterators)

- **Ownership:** Collections own their elements (except for borrowed slices `&[T]` / `&str`).  
- **Borrowing:** Use `&T`/`&mut T` when you just need a view; this avoids moves/clones.  
- **Iteration:** `iter()` → `&T`, `iter_mut()` → `&mut T`, `into_iter()` → moves out `T` (consumes container).  
- **Capacity vs length:** `capacity()` is allocated space; `len()` is number of elements. `reserve()` grows capacity.
- **Traits that matter:** `Eq` + `Hash` for `Hash{Map,Set}`; `Ord` for `BTree{Map,Set}`, `BinaryHeap` (ordering).

---

## 2) `Vec<T>` — growable contiguous array

### Create
```rust
let mut v: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3];
let mut v3 = Vec::with_capacity(128);
```

### Access & modify
```rust
v.push(10);
let last = v.pop();
let x = v[0];                 // panics if out of bounds
let y = v.get(0);             // Option<&T>
v.insert(1, 99);
let removed = v.remove(0);
v[0] = 7;
```

### Capacity & housekeeping
```rust
v.len(); v.is_empty();
v.capacity(); v.reserve(100); v.shrink_to_fit();
v.clear(); v.truncate(5);
```

### Bulk ops
```rust
v.extend([10,11,12]);
let mut w = vec![100, 200];
v.append(&mut w);             // drains w into v
let tail = v.split_off(3);    // split vector
for x in v.drain(1..3) { /* ... */ }
v.splice(2..2, [8,9]);        // insert many at once
```

### Sorting & searching (when `T: Ord` unless using custom comparator)
```rust
v.sort(); v.sort_by(|a,b| a.cmp(b));
v.sort_unstable();
v.dedup();
match v.binary_search(&10) { Ok(i) => /* ... */, Err(pos) => /* ... */ }
```

**Common methods:** `new`, `with_capacity`, `len`, `is_empty`, `capacity`, `reserve`, `shrink_to_fit`,  
`push`, `pop`, `insert`, `remove`, `retain`, `clear`, `truncate`, `extend`, `append`, `split_off`, `drain`, `splice`,  
`get`, `get_mut`, `first`, `first_mut`, `last`, `last_mut`, `contains`, `sort`, `sort_by`, `sort_unstable`, `dedup`, `binary_search`.

---

## 3) `VecDeque<T>` — double‑ended queue

### Create & ends
```rust
use std::collections::VecDeque;
let mut dq: VecDeque<i32> = VecDeque::new();
dq.push_back(1); dq.push_front(0);
dq.pop_back(); dq.pop_front();
```

### Access & capacity
```rust
dq.front(); dq.front_mut();
dq.back(); dq.back_mut();
dq.len(); dq.is_empty();
dq.capacity(); dq.reserve(32); dq.shrink_to_fit();
dq.make_contiguous(); // returns &mut [T] slice of contiguous buffer
```

### Other ops
```rust
dq.insert(1, 42);
dq.remove(1);
dq.retain(|x| *x % 2 == 0);
let mut other = VecDeque::from([7,8]);
dq.append(&mut other);
let right = dq.split_off(2);
```

**Common methods:** `new`, `with_capacity`, `len`, `is_empty`, `capacity`, `reserve`, `shrink_to_fit`,  
`push_back`, `push_front`, `pop_back`, `pop_front`, `front`, `front_mut`, `back`, `back_mut`,  
`insert`, `remove`, `retain`, `clear`, `append`, `split_off`, `make_contiguous`.

---

## 4) `LinkedList<T>` — doubly linked list (use sparingly)

```rust
use std::collections::LinkedList;
let mut ll: LinkedList<i32> = LinkedList::new();
ll.push_back(1); ll.push_front(0);
ll.pop_back(); ll.pop_front();
ll.len(); ll.is_empty(); ll.clear();
let mut other = LinkedList::new();
other.push_back(7);
ll.append(&mut other);   // moves nodes; O(1)
```

**Common methods:** `new`, `len`, `is_empty`, `clear`, `push_back`, `push_front`, `pop_back`, `pop_front`, `front`, `front_mut`, `back`, `back_mut`, `append`, `split_off`, `iter`, `iter_mut`.

---

## 5) `HashMap<K, V>` — hash‑table map (unordered)

```rust
use std::collections::HashMap;
let mut m: HashMap<&str, i32> = HashMap::new();
m.insert("apples", 3);
if let Some(x) = m.get("apples") { /* &i32 */ }
m.entry("oranges").or_insert(0);
m.entry("apples").and_modify(|x| *x += 1);
m.remove("apples");
```

### Keys & values
```rust
for (k, v) in m.iter() { /* ... */ }
for k in m.keys() { /* ... */ }
for v in m.values_mut() { *v *= 2; }
m.contains_key("k"); m.len(); m.is_empty();
```

### Capacity
```rust
m.capacity(); m.reserve(64); // pre‑size to avoid rehashing
```

**Common methods:** `new`, `with_capacity`, `len`, `is_empty`, `capacity`, `reserve`, `clear`, `shrink_to_fit`,  
`insert`, `get`, `get_mut`, `remove`, `contains_key`, `entry` (`or_insert`, `or_default`, `and_modify`),  
`iter`, `iter_mut`, `keys`, `values`, `values_mut`, `retain`, `extend`, `drain`.

> **Hashing:** Uses a randomized hash (SipHash or equivalent) to mitigate DoS. For custom hashers, use `HashMap<K,V, S>`.

---

## 6) `BTreeMap<K, V>` — ordered map (balanced B‑tree)

```rust
use std::collections::BTreeMap;
let mut bm = BTreeMap::new();
bm.insert("alice", 2);
bm.insert("bob", 1);
for (k,v) in bm.range("alice".."carol") { /* sorted by key */ }
bm.first_key_value();
bm.last_key_value();
```

**Strengths:** Sorted iteration, range queries, predictable order, no hashing cost.

**Common methods:** `new`, `len`, `is_empty`, `clear`, `insert`, `get`, `get_mut`, `remove`, `contains_key`,  
`iter`, `iter_mut`, `keys`, `values`, `values_mut`, `range`, `range_mut`, `first_key_value`, `last_key_value`, `split_off`, `append`, `retain`.

---

## 7) `HashSet<T>` — set backed by hash table

```rust
use std::collections::HashSet;
let mut s: HashSet<i32> = HashSet::new();
s.insert(1);
s.contains(&1);
s.remove(&1);
```

### Set algebra
```rust
let a: HashSet<_> = [1,2,3].into_iter().collect();
let b: HashSet<_> = [3,4].into_iter().collect();
let u: HashSet<_> = a.union(&b).copied().collect();
let i: HashSet<_> = a.intersection(&b).copied().collect();
let d: HashSet<_> = a.difference(&b).copied().collect();
let x: HashSet<_> = a.symmetric_difference(&b).copied().collect();
```

**Common methods:** `new`, `with_capacity`, `len`, `is_empty`, `capacity`, `reserve`, `clear`,  
`insert`, `remove`, `contains`, `get`, `take`, `replace`,  
`union`, `intersection`, `difference`, `symmetric_difference`, `is_subset`, `is_superset`, `drain`, `retain`, `extend`.

---

## 8) `BTreeSet<T>` — ordered set

```rust
use std::collections::BTreeSet;
let mut s = BTreeSet::from([1,2,3]);
s.insert(4);
s.contains(&2);
for x in s.range(2..) { /* 2,3,4... */ }
```

**Common methods:** `new`, `len`, `is_empty`, `clear`, `insert`, `remove`, `contains`, `iter`, `range`,  
`first`, `last`, `split_off`, `append`, `is_subset`, `is_superset`, `union`, `intersection`, `difference`, `symmetric_difference`, `retain`.

---

## 9) `BinaryHeap<T>` — max‑heap priority queue

```rust
use std::collections::BinaryHeap;
let mut h = BinaryHeap::new();
h.push(5);
h.push(10);
assert_eq!(h.peek(), Some(&10));
assert_eq!(h.pop(), Some(10));
for x in h.clone().into_iter_sorted() { /* descending order */ }
```

**Common methods:** `new`, `with_capacity`, `len`, `is_empty`, `clear`, `peek`, `peek_mut`, `push`, `pop`, `append`, `drain`, `into_sorted_vec`, `into_iter_sorted`, `retain` (via manual rebuild pattern).

> **Min‑heap:** Wrap values with `std::cmp::Reverse(T)`.

---

## 10) Slices `&[T]` and Mutable slices `&mut [T]`

- Borrow views into arrays, `Vec`, `VecDeque::make_contiguous()`, or other slices—**zero‑cost**.
- No allocation; you cannot change length/capacity, only the elements (for `&mut [T]`).

**Common methods:** `len`, `is_empty`, `first`, `first_mut`, `last`, `last_mut`, `get`, `get_mut`,  
`iter`, `iter_mut`, `windows`, `chunks`, `chunks_mut`, `rchunks`, `split`, `split_mut`, `split_at`, `split_at_mut`,  
`rotate_left`, `rotate_right`, `fill`, `copy_from_slice`,  
`sort`, `sort_by`, `sort_unstable`, `binary_search`, `contains`.

---

## 11) Text: `String` and `&str`

### `String` (owned, mutable UTF‑8)
```rust
let mut s = String::with_capacity(32);
s.push('A');
s.push_str("lpha");
s.insert(1, 'l');
s.insert_str(2, "pha");
s.pop();                  // Option<char>
s.remove(0);              // remove char at byte index
s.replace_range(0..2, "Ω");
s.clear();
```

**Common methods:** `new`, `with_capacity`, `from`, `len`, `is_empty`, `capacity`, `reserve`, `shrink_to_fit`,  
`push`, `push_str`, `pop`, `insert`, `insert_str`, `remove`, `replace_range`, `clear`,  
`as_str`, `into_boxed_str`, `split_off`, `truncate`, `drain`, `extend` (of chars).

> Many useful text operations live on **`&str`** (thanks to `Deref<Target=str>`).

### `&str` (borrowed UTF‑8 slice)
```rust
let s = "hello\nworld";
for line in s.lines() { /* ... */ }
assert!(s.starts_with("he"));
let parts: Vec<&str> = s.split_whitespace().collect();
let trimmed = s.trim();
let idx = s.find("lo");  // Option<usize>, byte index
let replaced = s.replace("world", "Rust"); // -> String
```

**Common methods:** `len`, `is_empty`, `chars`, `bytes`, `lines`, `split`, `split_whitespace`, `trim`, `trim_start`, `trim_end`,  
`starts_with`, `ends_with`, `contains`, `find`, `rfind`, `get` (sub‑slicing by byte range), `to_string`, `parse`.

> **UTF‑8 gotcha:** Indexing by byte offsets can split a code point. Prefer iterating `chars()` or using grapheme crates for user‑visible units.

---

## 12) Iterators, `collect`, and `FromIterator`

```rust
let v: Vec<_> = (0..10).filter(|x| x % 2 == 0).collect();
let s: HashSet<_> = [1,2,2,3].into_iter().collect();
let m: HashMap<_,_> = [("a",1), ("b",2)].into_iter().collect();
```

- Many algorithms are **iterator adapters**: `map`, `filter`, `take`, `skip`, `enumerate`, `flat_map`, `rev`, etc.
- `collect::<T>()` builds the target collection when `T: FromIterator<_>`.

---

## 13) Example patterns

### Counting frequency (word histogram)
```rust
use std::collections::HashMap;
fn word_freq(input: &str) -> HashMap<&str, usize> {
    let mut freq = HashMap::new();
    for w in input.split_whitespace() {
        *freq.entry(w).or_insert(0) += 1;
    }
    freq
}
```

### Top‑k largest using `BinaryHeap`
```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn top_k<I>(iter: I, k: usize) -> Vec<i32>
where
    I: IntoIterator<Item = i32>,
{
    let mut heap = BinaryHeap::with_capacity(k);
    for x in iter {
        if heap.len() < k { heap.push(Reverse(x)); }
        else if let Some(&Reverse(min)) = heap.peek() {
            if x > min { heap.pop(); heap.push(Reverse(x)); }
        }
    }
    heap.into_iter().map(|Reverse(x)| x).collect()
}
```

### Stable order with `BTreeMap` range
```rust
use std::collections::BTreeMap;
let mut bm = BTreeMap::from([("alice",2), ("bob",1), ("carol",3)]);
for (k,v) in bm.range("b".."d") { /* bob, carol */ }
```

### Two‑ended queue
```rust
use std::collections::VecDeque;
let mut dq = VecDeque::from([1,2,3]);
dq.push_front(0);
dq.push_back(4);
assert_eq!(dq.pop_front(), Some(0));
assert_eq!(dq.pop_back(),  Some(4));
```

---

## 14) Performance tips

- Prefer `Vec` unless you have a clear reason; it’s the fastest general‑purpose container.
- Use `reserve`/`with_capacity` when the size is predictable.
- `HashMap`/`HashSet` need good keys: implement/derive `Hash` + `Eq`; avoid frequent rehashing by reserving.
- `BTreeMap`/`BTreeSet` shine with sorted iteration and range queries; slower constant factors than hash maps for random lookups.
- Avoid `LinkedList` unless you really need O(1) concatenation/split of lists already constructed.
- Avoid indexing `String` by bytes; iterate `chars()` for Unicode safety.

---

## 15) Quick API reference (by type)

- **Vec**: `new`, `with_capacity`, `len`, `is_empty`, `capacity`, `reserve`, `shrink_to_fit`, `push`, `pop`, `insert`, `remove`, `retain`, `clear`, `truncate`, `extend`, `append`, `split_off`, `drain`, `splice`, `get`, `get_mut`, `first(_mut)`, `last(_mut)`, `contains`, `sort(_by|_unstable)`, `dedup`, `binary_search`  
- **VecDeque**: `new`, `with_capacity`, `len`, `is_empty`, `capacity`, `reserve`, `shrink_to_fit`, `push_back/front`, `pop_back/front`, `front(_mut)`, `back(_mut)`, `insert`, `remove`, `retain`, `clear`, `append`, `split_off`, `make_contiguous`  
- **LinkedList**: `new`, `len`, `is_empty`, `clear`, `push_back/front`, `pop_back/front`, `front(_mut)`, `back(_mut)`, `append`, `split_off`, `iter(_mut)`  
- **HashMap**: `new`, `with_capacity`, `len`, `is_empty`, `capacity`, `reserve`, `clear`, `insert`, `get(_mut)`, `remove`, `contains_key`, `entry` (`or_insert`, `or_default`, `and_modify`), `iter(_mut)`, `keys`, `values(_mut)`, `retain`, `extend`, `drain`  
- **BTreeMap**: `new`, `len`, `is_empty`, `clear`, `insert`, `get(_mut)`, `remove`, `contains_key`, `iter(_mut)`, `keys`, `values(_mut)`, `range(_mut)`, `first_key_value`, `last_key_value`, `split_off`, `append`, `retain`  
- **HashSet**: `new`, `with_capacity`, `len`, `is_empty`, `capacity`, `reserve`, `clear`, `insert`, `remove`, `contains`, `get`, `take`, `replace`, `union`, `intersection`, `difference`, `symmetric_difference`, `is_subset`, `is_superset`, `drain`, `retain`, `extend`  
- **BTreeSet**: `new`, `len`, `is_empty`, `clear`, `insert`, `remove`, `contains`, `iter`, `range`, `first`, `last`, `split_off`, `append`, `is_subset`, `is_superset`, `union`, `intersection`, `difference`, `symmetric_difference`, `retain`  
- **BinaryHeap**: `new`, `with_capacity`, `len`, `is_empty`, `clear`, `peek`, `peek_mut`, `push`, `pop`, `append`, `into_sorted_vec`, `into_iter_sorted`  
- **String**: `new`, `with_capacity`, `from`, `len`, `is_empty`, `capacity`, `reserve`, `shrink_to_fit`, `push`, `push_str`, `pop`, `insert`, `insert_str`, `remove`, `replace_range`, `clear`, `as_str`, `into_boxed_str`, `split_off`, `truncate`, `drain`  
- **&str**: `len`, `is_empty`, `chars`, `bytes`, `lines`, `split(_whitespace)`, `trim(_start|_end)`, `starts_with`, `ends_with`, `contains`, `find`, `rfind`, `get`, `to_string`, `parse`  
- **Slices `&[T]`**: `len`, `is_empty`, `first(_mut)`, `last(_mut)`, `get(_mut)`, `iter(_mut)`, `windows`, `chunks(_mut)`, `split(_mut)`, `split_at(_mut)`, `rotate_left`, `rotate_right`, `fill`, `copy_from_slice`, `sort(_by|_unstable)`, `binary_search`, `contains`

---

## 16) Further reading

- Standard Library docs: `std::collections`, `std::vec::Vec`, `std::string::String`
- The Rust Book — Collections chapter
- Rust Reference & Nomicon (for ownership/borrowing subtleties)