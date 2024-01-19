
<img width="1291" alt="Screenshot 2024-01-19 at 11 29 03 PM" src="https://github.com/Maniktherana/first-foray-into-rust-talk/assets/14011425/24b18040-9337-41a6-bd28-51426d9f4689">

# Slides and code examples from my talk

In this talk I covered my learnings when trying to solve Advent of Code and building a TCP server in Rust.

## Echo Server
To run the echo server:
```bash
cd echo_server
cargo run
```

Test it with telnet:
```bash
telnet localhost 8080
```

Optionally, host the server on a vm and test it against [Protohacker's tests](https://protohackers.com/problem/0)


## Advent of Code
This is just my attempt of [AoC Day 1](https://adventofcode.com/2023/day/1). You can view the rest of my attempts [here](https://github.com/Maniktherana/advent-of-code).

To run a test:
```bash
cd aoc_day1
cargo test --bin part1.rs
```

To run against the entire input:
```bash
cargo run --bin part1.rs
```
