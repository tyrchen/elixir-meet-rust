# Elixir Meet Rust

This is my talk for Beijing elixir meetup 5/16. To run the slides, you need to have [marp-cli](https://github.com/marp-team/marp-cli) installed. Then just run:

```bash
make
```

Or you could directly visit it from github: [slides/elixir-meet-rust.md](slides/elixir-meet-rust.md).

## Demo 1: rmark

Markdown parser using [rustler](https://github.com/rusterlium/rustler) and [comrak](https://github.com/kivikakk/comrak). Compared with [earmark](https://github.com/pragdave/earmark), it is 20x-30x faster. This is a demo project to show how to use rustler to boost elixir/erlang projects.

See [rmark readme](rmark/README.md).

## Demo 2: BTreeMap (Sorted Map) for rust

In Rust, there're lots of beautiful, high-performance data structures. Elixir has built in support for Map, but it doesn't support ordered map (or sorted map). Don't worry, we could leverage [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) of rust!

see [rbtree readme](rbtree/README.md).
