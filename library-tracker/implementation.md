<!--markdownlint-disable-->


# Learning Rust Ownership: Book Lending Tracker

**Goal:** After building this, you'll understand ownership, borrowing (immutable and mutable), move semantics, and lifetimes well enough to reason about compiler errors instead of fighting them blindly.

**Time estimate:** A solid weekend, maybe stretched into a few evenings.

---

## Why This Project

Ownership is easiest to learn through a scenario that already matches its rules in real life. A **library lending system** is perfect: a book has exactly one owner at a time, it can be lent out (borrowed) to one person, multiple people can *read about* a book at once but only one can *hold* it, and if the owner "gives away" a book, they no longer have it. The domain logic and the Rust concept map onto each other almost one-to-one, which makes the abstract rules click.

## What You Build

A command-line program that manages a small personal library: a list of books, a list of friends, and operations to lend books out, return them, and permanently give books away. Nothing fancy — just structs, some collections, and functions that operate on them.

---

## Implementation Steps

### Step 1: Model the Data and Feel Ownership for the First Time

Create a structure representing a **Book** (title, author, condition) and a structure representing a **Person** (name, list of books they currently hold). Populate a small library — a handful of books, a couple of people.

Write a function that takes a Book and "gives" it to a Person, meaning the Book moves from wherever it was into that Person's collection. Try to use the same Book in two places afterward. The compiler will stop you.

Sit with that error message before fixing it — this is the whole point of the project. This is your first hands-on encounter with the rule that a value has exactly one owner at a time, and once it moves, the old owner can't use it anymore.

### Step 2: Add Borrowing Without Giving Anything Away

Now implement "lending" as distinct from "giving." Write a function that lets a Person look at the details of a Book — read its title and condition — without taking ownership of it.

This is where you introduce read-only borrowing: the function receives a reference to the book, peeks at it, and gives control back. Prove to yourself that you can have several people "look at" the same book description at the same time, because read-only access doesn't conflict with itself.

### Step 3: Add a Mutable Borrow and Hit the One-Writer Rule

Add a function that lets someone update a book's condition (say, marking it "damaged" after a rough trip in someone's bag). This requires a mutable reference.

Now try to let two different parts of your program mutate the same book's condition at the same time, or try to read a book's title while another part of the code is mid-update to that same book. You'll hit the rule that you can have many readers or exactly one writer, never both at once. Work through why the compiler enforces this — it's preventing the real-world equivalent of two people scribbling in the same book simultaneously and corrupting it.

### Step 4: Build the Actual Lending Mechanic with a Borrow-and-Return Cycle

This is the heart of the project. Implement a "lend book to friend" operation and a matching "return book" operation. When a book is lent, the friend gets temporary access to it — a reference — not ownership.

Track this lending relationship (who has what, and for how long) in a way that the data can't outlive the thing it refers to. This is where you'll start noticing lifetime concerns: what happens if you try to keep a reference to a book around after the book itself would logically be "returned" or removed from the library? Let the compiler guide you into explicitly reasoning about how long a borrowed reference is allowed to live.

### Step 5: Add Ownership Transfer as a Distinct Action from Lending

Implement a "give this book to a friend permanently" operation, separate from lending. This should genuinely move the Book out of the original owner's collection and into the friend's.

Compare this in your own head to Step 4: lending hands out a temporary reference and the original owner keeps ownership; giving is a full, permanent move and the original owner loses access entirely. Writing both side by side is what cements the difference between "borrowing" and "moving" as two fundamentally different operations.

### Step 6: Handle a Shared Multi-Owner Situation

Add a feature where a book can belong to a "book club" — jointly owned by several people, no single owner. This is where you'll want a shared-ownership tool (something that lets multiple parts of the program hold onto the same data and only cleans it up once nobody needs it anymore).

Implementing this after you've felt the pain of strict single-ownership makes it obvious *why* such a tool exists, instead of it feeling like a magic escape hatch.

### Step 7: Write a Small Report Function That Ties It Together

Finish with a function that prints the current state of the whole library: who owns what, who's currently borrowing what, and what's jointly owned. Writing this forces you to borrow from multiple structures at once without moving anything, which is a good final test of whether the rules have actually sunk in.

---

## Starter Challenge

Before writing any structures, just write down on paper (or in comments) the rules of your library in plain English: who can own a book, what happens when it's lent, what happens when it's given away, what happens when it's jointly owned. You'll notice you're describing ownership, borrowing, and moving without ever having heard the Rust terms — that's the mental model you're about to formalize.

## Stretch Goal

Add a "waiting list" feature: if a book is currently lent out and someone else wants it, they get queued. Implementing this cleanly usually requires you to think carefully about lifetimes again — the waiting list can't hold a usable reference to a book that might be returned, given away, or removed before their turn comes up. Wrestling with that is genuinely one of the best lifetime exercises there is.

---

## What Comes Next

Once this clicks, move on to a project involving concurrent access — for example, a chat room or task queue where multiple threads need to touch shared data. That's where ownership rules combine with thread-safety, and you'll meet tools built specifically for sharing data safely across threads. From there, real open-source Rust CLI tools are a good next step for reading idiomatic code.

## Key Resources

- The official **Rust Book**, chapter 4 (Understanding Ownership) — read it after Step 3 or 4, once you've already felt the pain it's explaining
- **Rustlings** — small compiler-error exercises that pair well with this project when you want isolated practice on a rule you're still shaky on
- The compiler's own error messages — genuinely one of the best teachers here; read them fully instead of skimming to the error code
