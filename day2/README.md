# Day 2

<https://adventofcode.com/2023/day/2>

Two versions, one with regex (my initial version) and one using nom as I'd never used it and had been meaning to check it out for a while. Nom is 3 orders of magnitude faster and is much more readable to boot.

```text
Timer precision: 41 ns
bench                 fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_part1_nom    53.83 µs      │ 76.24 µs      │ 54.62 µs      │ 56.16 µs      │ 100     │ 100
├─ bench_part1_regex  99.4 ms       │ 108 ms        │ 101 ms        │ 102.3 ms      │ 100     │ 100
├─ bench_part2_nom    52.29 µs      │ 82.58 µs      │ 57.74 µs      │ 58.36 µs      │ 100     │ 100
╰─ bench_part2_regex  32.95 ms      │ 36.21 ms      │ 33.42 ms      │ 33.97 ms      │ 100     │ 100
```
