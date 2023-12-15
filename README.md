# Slides and code examples from my talk


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

Optionally, host the server on a vm and test it against [this](https://protohackers.com/problem/0)


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
