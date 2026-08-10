# Chapter 1 
Heyyy

Here, I will be documenting my learning progress and the challenges I take daily

## Day 3
Did more on learning about The Turing Machine, understanding and imlementing the turing machine model in rust, it was a bit challenging as I was trying to understand translating the mental model from maths or visual abstraction into a working algorithm in rust, it was fun, getting back to practicing so many basics I haven't done in a while, vector manipulations, traits, methods, enums, hashmaps, and testing e.t.c. (I encountered an overflow error 😅, causing a test to fail for something I couldn't immediately notice).

And earlier, I prioritized understanding code structuring and organization, utilizing rust builtins module system, and reorganized my code from yesterday into modules and writing now individual units tests within each unit of code.

Also, did some other deep dive into some resources learning about Unsafe Rust and Inner Mutability from a talk on [Advanced Rust Programming Techniques](https://www.youtube.com/watch?v=QQzAWxYKPSE) given by Florian Glitcher, as he also broke down essentially Rust has 2 main contsructs Data Structures (Which are just two, the Struct and Enum) and Functions in many forms and ways.

Pretty much all for the day. Check out my [turing-machine.rs](./turing_machine.rs) implementation.


## Day 4 — Data Structures, Graphs & GKR

## DSAR

Today I focused on understanding two data structures properly before implementing them in Rust: **Linked Lists and Graphs**.

The main goal was not to immediately start writing code, but to build a proper mental model of how these structures work, what problems they solve, and how their structure affects the algorithms that operate on them.

### Linked Lists

A Linked List is a data structure made up of nodes where each node stores a value and a reference/link to another node.

For a singly linked list, the basic structure is:

```text
Head → Node → Node → Node → None
```

Each node contains:

```text
value
next
```

Unlike arrays or `Vec`, the elements do not need to be stored next to each other in memory. Each node points to the next node.

I understood three common types:

* **Singly Linked List:** each node points in one direction to the next node.
* **Doubly Linked List:** each node can point both forward and backward.
* **Circular Linked List:** the final node points back to the beginning. This can also be singly or doubly linked.

### Linked List Operations

Some of the common operations are:

* Traversal
* Search
* Insertion
* Removal
* Appending
* Prepending

The main tradeoff I took away is that Linked Lists are useful when we need efficient insertion or removal at a known position, but they are not good for random access or searching because we generally have to traverse the list.

For example:

```text
A → B → C → D → E
```

To find `D`, we have to start from the head and move through the nodes.

This gives linear search a worst-case complexity of **O(n)**.

### Linked Lists in Rust

The interesting part was translating the abstract Linked List model into Rust.

A basic node can be represented conceptually as:

```rust
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
```

This introduced some important Rust concepts:

* `struct`
* `enum`
* `Option`
* `Box`
* ownership
* borrowing
* heap allocation
* recursive data structures

`Box` is important because a node contains another node recursively. Rust needs the size of a type to be known, so we cannot simply have a node directly contain another `Node`. `Box` gives us an owned pointer to the next node.

I also looked at how Rust handles dropping a linked list and why a custom `Drop` implementation can be used to destroy a long linked list iteratively instead of creating a deeply recursive destruction chain.

### Graphs

I also explored Graphs and the basic ways they can be represented.

A Graph consists fundamentally of:

```text
Vertices + Edges
```

For example:

```text
    A
   / \
  B   C
  |   |
  D---E
```

The vertices are the individual elements, while the edges represent the relationships between them.

### Types of Graphs

I focused on understanding the basic types rather than going deep into graph algorithms yet.

**Undirected Graph**

The relationship works both ways.

```text
A ─── B
```

If A is connected to B, B is also connected to A.

**Directed Graph**

Edges have a direction.

```text
A → B
```

A can point to B without B necessarily pointing back to A.

**Weighted Graph**

Edges have an associated value or cost.

```text
A ──5── B
```

Weighted graphs can also be directed or undirected.

For my implementation, I decided to start with an **undirected graph**.

### Adjacency List

The representation I want to implement is an adjacency list.

For:

```text
A ─── B
│     │
C ─── D
```

the adjacency list could look like:

```text
A → [B, C]
B → [A, D]
C → [A, D]
D → [B, C]
```

The important thing to remember is that because the graph is undirected:

```text
add_edge(A, B)
```

means both:

```text
A → B
B → A
```

### Graph Traversal

The two traversal algorithms I will implement are:

**DFS — Depth First Search**

Go as deep as possible before backtracking.

```text
DFS
 ↓
Stack / Recursion
 ↓
Go deep
 ↓
Backtrack
```

**BFS — Breadth First Search**

Explore the graph level by level.

```text
BFS
 ↓
Queue
 ↓
Current level
 ↓
Next level
```

Both can operate in **O(V + E)** time when using an adjacency-list representation, where `V` is the number of vertices and `E` is the number of edges.

I don't need to implement these today. The important thing for Day 4 was understanding the mental model so I can build them myself on Day 5.

## GKR Protocol Fix

Alongside the DSAR work, I also went back into my ZK repo and fixed my **GKR implementation**.

The goal for this day was to fix my GKR protocol. The tests were failing, and after debugging, I found two major bugs in my implementation.

The first was in `Circuit.rs`. Circuit creation was effectively hardcoded around the structure I was using in my tests rather than being dynamic enough to support arbitrary circuit structures. I was using the `layer_id` to determine the number of variables for the gates, instead of deriving it from the actual structure of each layer. I fixed this by using the length of each layer to determine the appropriate number of variables.

The second, and more serious, bug was in my `GKRVerifier`. I had not implemented thorough verification of the prover's claimed sums and proofs across each layer. This meant the verifier could accept invalid proofs because it was not checking that the claims remained consistent throughout the protocol.

I fixed this by evaluating the claims at each layer, generating the random challenges, and checking that the evaluations were consistent between the current layer and the previous layer's polynomials at every step. The verification then continues through the circuit until the final input evaluation, where the result is checked against the expected computation.

This made the verifier actually enforce the consistency of the prover's claims throughout the GKR protocol rather than simply accepting the proof without sufficiently validating each step. [MY GKR IMPL](https://github.com/michojekunle/zk/gkr)

