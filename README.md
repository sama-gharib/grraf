# grraf

(Please find `grraf`'s license in the `LICENSE.md` file)

## What is grraf ?

`grraf` is an extendable graph library written in and for the Rust programming language.
It provides a modular interface for organizing your data in graphs without having to
deal with Rust's borrowing rules and annoying memory leaks.


The main goal of `grraf` is to be extendable. Several graphs backends are provided by
the library but the main adventage of using `grraf` is to be able to implement your own
graph system, as long as it implements the `Graph` trait.

## Why is grraf ?

This library is more of an experiment on my side, a learning project for my own use. However
I would be delighted if anyone was able to find a real life use case for `grraf` ! So feel free to clone,
fork and experiment with `grraf`.

## How is grraf ?

Here is a list of `grraf`'s main goals :

- [X] Provide the `Graph<Vertex, Edge>` trait, which exposes a safe interface for graph manipulation.
- [ ] Implement the `Graph<Vertex, Edge>` trait for several graph backends :
    - [ ] `AdjacencyMatrix`
    - [ ] `IndexList` <- ***Doing This Right Now !***
    - [ ] `PointerList`
    - [ ] `RefCountGraph`
- [ ] Add some basic algorithms
    - [X] Depth First Search
    - [ ] Breadth First Search
    - [ ] Cycle checking
    - [ ] Dijkstra's algorithm
