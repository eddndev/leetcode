# LeetCode Solutions

A curated collection of algorithmic problems, data structures, and competitive programming challenges.

## Index

- [About](#about)
- [Repository Structure](#repository-structure)
- [Solutions](#solutions)
- [Languages & Tools](#languages--tools)

---

## About

This repository serves as a personal knowledge base for algorithmic problem solving. It maps the journey through LeetCode's extensive library, focusing on clean, efficient, and well-documented code.

The goal is not merely to solve, but to understand the underlying patterns—Graph Theory, Dynamic Programming, Greedy approaches, and more. Each solution is treated as a standalone study note.

## Repository Structure

To maintain organization and scalability across thousands of problems, the repository is structured by numerical ranges rather than arbitrary topic folders. This ensures a strictly ordered filesystem while allowing for topic-based tagging via internal file documentation.

### Organization Scheme

- **Source Code**: Grouped in ranges of 100 (e.g., `0001-0100`).
- **Naming Convention**: `ID-slug.ext` (e.g., `0001-two-sum.cpp`).
- **Documentation**: Self-contained within solution files or adjacent Markdown notes.

```
/
├── solutions/
│   ├── 0001-0100/
│   │   ├── 0001-two-sum.cpp
│   │   └── ...
│   ├── 0101-0200/
│   └── ...
├── scripts/
│   └── automation-tools
└── README.md
```

## Solutions

*An automated index of all solved problems will be generated here.*

<!-- SOLUTIONS_TABLE_START -->
| ID | Title | Difficulty | Time | Space | Languages |
| -- | ----- | ---------- | ---- | ----- | --------- |
| 0001 | [Two Sum - EN](solutions/0001-0100/0001-two-sum/0001-two-sum-en.md)<br>[Two Sum - ES](solutions/0001-0100/0001-two-sum/0001-two-sum-es.md) | Easy | O(n) | O(n) | [C](solutions/0001-0100/0001-two-sum/0001-two-sum.c), [Rust](solutions/0001-0100/0001-two-sum/0001-two-sum.rs) |
| 0006 | [Zig Zag Conversion - EN](solutions/0001-0100/0006-zig-zag-conversion/0006-zig-zag-conversion-en.md)<br>[Zig Zag Conversion - ES](solutions/0001-0100/0006-zig-zag-conversion/0006-zig-zag-conversion-es.md) | Medium | O(n) | O(1) | [C](solutions/0001-0100/0006-zig-zag-conversion/0006-zig-zag-conversion.c) |
| 3454 | [Separate Squares Ii - EN](solutions/3401-3500/3454-separate-squares-ii/3454-separate-squares-ii-en.md)<br>[Separate Squares Ii - ES](solutions/3401-3500/3454-separate-squares-ii/3454-separate-squares-ii-es.md) | Hard | O(N log N) | O(N) | [C](solutions/3401-3500/3454-separate-squares-ii/3454-separate-squares-ii.c) |
<!-- SOLUTIONS_TABLE_END -->

## Languages & Tools

Primary languages used for implementation:

- **C**: For raw performance and low-level control, honoring the roots of systems programming.
- **Rust**: For modern safety, concurrency, and expressive correctness. The primary vehicle for reliable solutions.

---

## License

This repository is dual-licensed to accommodate its different components:

- **Automation Scripts** (`/scripts`): Licensed under the **GNU General Public License v3.0 (GPLv3)**. This ensures that the tooling remains free and open source.
- **Solutions & Documentation** (`/solutions`): Provided as public domain / educational content. You are free to use, modify, and learn from them without restriction, though attributing the source is appreciated.

See the [LICENSE](LICENSE) file for the full GPLv3 text applicable to the codebase infrastructure.
