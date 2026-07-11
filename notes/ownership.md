<!--markdownlint-disable-->



# Ownership

Ownership is a set of rules that govern how RUst program manages memory


1) Each value in Rust has a owner
2) There can only be one owner at a time
3) When the owner goes out of scope, the value will be dropped



# Bindings

A binding in rust is just a ownership

In rust ownership transfers when we use the =


```Rust
let a = String::from("hi");

let b = a;
// b now owns the string, a does not

```


if we wanted a to also stay valid we would need to clone it

```Rust
let b = a.clone()
```


or borrow the reference
```Rust
let b = &a

```

In function arguments, the arguments take ownership of whatever was passed in
**
**

# Rust Bindings: What's Happening in the Stack and Heap

## Stack vs Heap, Quick Refresher

- **Stack**: fast, fixed-size, LIFO. Every local variable's *binding metadata* lives here — size known at compile time.
- **Heap**: slower, dynamically sized, allocated/freed at runtime. Used for data whose size can grow or isn't known until runtime (`String`, `Vec`, `Box`, etc.).
## Case 1: Simple Types (i32, bool, etc.) — Stack Only

```rust
let x = 5;
let y = x;
```

Both `x` and `y` are just plain integers sitting on the stack. `let y = x` **copies** the bits. `x` is still valid afterward. No heap involved at all.

This works because `i32` implements the `Copy` trait — it's cheap to duplicate, so Rust just does that instead of "moving."

## Case 2: Heap-Backed Types (String, Vec) — This Is Where It Gets Interesting

```rust
let a = String::from("hi");
```

A `String` binding on the stack is actually a small fixed-size struct with **three fields**:

```
a: { ptr: 0x1234, len: 2, capacity: 2 }
```

- `ptr` — pointer to the actual bytes `"hi"` on the **heap**
- `len` — current length
- `capacity` — how much space is allocated
So the stack holds the *handle*, the heap holds the *actual data*.

```
Stack                Heap
┌─────────────┐      ┌──────┐
│ a: ptr ──────┼─────▶│ h i  │
│    len: 2    │      └──────┘
│    cap: 2    │
└─────────────┘
```

## Now the Move

```rust
let b = a;
```

Rust does **not** deep-copy the heap data. It copies the stack struct (`ptr`, `len`, `cap`) into `b` — cheap, just ~24 bytes — and then **invalidates `a`**.

```
Stack                Heap
┌─────────────┐      ┌──────┐
│ a: (moved,   │      │ h i  │
│  invalid)    │  ┌──▶└──────┘
│              │  │
│ b: ptr ──────┼──┘
│    len: 2    │
│    cap: 2    │
└─────────────┘
```

**Why invalidate `a` at all?** Because if both `a` and `b` pointed to the same heap data and both went out of scope, Rust would try to free that heap memory *twice* — a double-free, a classic memory-safety bug in C/C++. By making the move invalidate `a`, only `b` is responsible for freeing it. This is the whole point of ownership: **exactly one binding owns the heap data at any time, and it frees it when it goes out of scope** (this is the `Drop` mechanism).

## `.clone()` — the Deep Copy

```rust
let b = a.clone();
```

This actually allocates *new* heap memory and copies the bytes over:

```
Stack                    Heap
┌─────────────┐          ┌──────┐
│ a: ptr ──────┼─────────▶│ h i  │
│    len/cap   │          └──────┘
├─────────────┤
│ b: ptr ──────┼─────────▶┌──────┐
│    len/cap   │          │ h i  │
└─────────────┘          └──────┘
```

Now both are valid, independent, and both will eventually free their own heap memory.

## References (`&a`) — Borrowing, No Ownership Change

```rust
let a = String::from("hi");
let r = &a;
```

`r` is a binding on the stack too, but it just stores **a pointer to `a`'s stack data** (which itself points to the heap):

```
Stack                        Heap
┌─────────────┐              ┌──────┐
│ a: ptr ──────┼─────────────▶│ h i  │
│    len/cap   │              └──────┘
├─────────────┤
│ r: ────────► a (points at a's stack slot)
└─────────────┘
```

No ownership moves, no allocation. `a` is still the owner and still responsible for freeing the heap memory when it goes out of scope. `r` just "looks."

## Tying This Back to a Function Example

```rust
fn give_book_to_person(b: Book, p: &mut Person) {
    p.books.push(b);
}
```

- **`b: Book`** — the `Book` struct's *stack representation* (its fields, and pointers to any of its own heap data like `String` fields) gets **moved** into this function's stack frame. The caller's original variable is invalidated. When `push` runs, that ownership moves again — into the `Vec`'s heap buffer.
- **`p: &mut Person`** — no move at all. `p` is just a pointer on this function's stack frame pointing back at the *caller's* stack slot for `Person`. Because it's `&mut`, you're allowed to reach through that pointer and mutate — including calling `p.books.push(...)`, which reaches further through to `books`' heap-allocated buffer and writes into it (possibly reallocating if capacity is exceeded).
## The "Aha" Summary

| Concept | What it means |
|---|---|
| **Stack** | Bindings' fixed-size handles (pointer + length + capacity, for heap types; or the actual value, for simple types) |
| **Heap** | The actual variable-sized data those handles point to |
| **Move** | Copy the stack handle, invalidate the old one, so only one owner can ever free the heap data |
| **Clone** | Actually duplicate the heap data too |
| **Reference (`&` / `&mut`)** | A stack value that just points at another binding's stack slot — no ownership transfer, no allocation |
| **`mut`** | Permission to write through a binding (or a reference) at all |

This is the entire reason Rust doesn't need a garbage collector: ownership is tracked at compile time via these move/borrow rules, and the compiler inserts the `drop` (dealloc) call automatically when the owning binding goes out of scope.





## What a Reference Actually Is Under the Hood

A reference is just a **pointer** stored on the stack. That's it. It holds the memory address of the original data — it does not hold a copy of the data, and it does not hold any of the heap-management responsibility (freeing memory).

```rust
let a = String::from("hi");
let r = &a;
```

```
Stack                        Heap
┌─────────────┐              ┌──────┐
│ a: ptr ──────┼─────────────▶│ h i  │
│    len/cap   │              └──────┘
├─────────────┤
│ r: ────────► (points at a's stack slot, address of `a`)
└─────────────┘
```

`a` still owns the heap data. `r` is just a stack value that says "go look over there."

## Passing a Reference into a Function

```rust
fn borrow_it(s: &String) {
    println!("{}", s);
} // s goes out of scope here — nothing is freed

fn main() {
    let a = String::from("hi");
    borrow_it(&a);
    println!("{}", a); // still valid! `a` was never moved
}
```

What happens on the stack when `borrow_it(&a)` runs:

```
main's stack frame          borrow_it's stack frame
┌─────────────┐             ┌─────────────┐
│ a: ptr ───┐  │             │ s: ────────┼──┐
│    len/cap│  │             └─────────────┘  │
└───────────┼──┘                              │
            │                                  │
            └──────────────────────────────────┘
                (s points to the same address a's data lives at)
```

- `s` is a **new stack binding**, local to `borrow_it`.
- It stores a copy of the *pointer value* (the address), not a copy of the `String`'s data.
- When `borrow_it` returns, `s` goes out of scope. Since `s` never owned anything, **nothing gets dropped/freed**.
- `a` in `main` is completely untouched and still owns its heap data. `main` can keep using `a` normally afterward.
## Mutable References (`&mut T`) — Same Ownership Rule, More Power

```rust
fn add_exclaim(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut a = String::from("hi");
    add_exclaim(&mut a);
    println!("{}", a); // "hi!"
}
```

```
main's stack frame          add_exclaim's stack frame
┌─────────────┐             ┌─────────────┐
│ a: ptr ───┐  │             │ s: ────────┼──┐
│    len/cap│  │             └─────────────┘  │
└───────────┼──┘                              │
            │                                  │
            └──────────────────────────────────┘
```

- `s` is still just a pointer — same as before.
- The difference is `s` is a **mutable** pointer, so the compiler allows you to write through it: `s.push('!')` reaches through the pointer, into `a`'s heap buffer, and modifies the actual bytes (possibly reallocating if capacity is exceeded).
- Ownership *still* never moves. `a` owns the data before, during, and after the call. `add_exclaim` never had the authority to free it — only to modify it.
## Why This Matters: No Double-Free, Ever

Because references never take ownership:

- A function can borrow data, use it or mutate it, and return — and **nothing gets deallocated** as a side effect of that function ending.
- Only the original owner's scope ending triggers `drop`.
- This is enforced entirely at **compile time** by the borrow checker — there's no runtime tracking, no reference counting (unless you explicitly opt into `Rc`/`Arc`).
## The Borrow Checker's Job

Since ownership doesn't move with a reference, Rust needs another way to prevent bugs like use-after-free or data races. That's what the borrow checker enforces at compile time:

| Rule | Why |
|---|---|
| You can have many `&T` (shared/immutable) references at once | Safe — nobody can mutate while others are reading |
| You can have only **one** `&mut T` (exclusive/mutable) reference at a time | Prevents two places from mutating simultaneously |
| You can't mix `&T` and `&mut T` at the same time | Prevents reading stale/inconsistent data while it's being mutated |
| A reference can't outlive the data it points to | Prevents dangling pointers |

All of this is checked statically — none of it costs anything at runtime. That's the payoff of the ownership/borrowing model: the compiler proves memory safety instead of needing a garbage collector or runtime checks.

## One-Line Summary

**Ownership only ever transfers on a move (passing by value). A reference — mutable or not — is just a pointer riding along on the stack; the original binding keeps ownership and keeps the responsibility of freeing the memory, no matter how many functions borrow it along the way.**

