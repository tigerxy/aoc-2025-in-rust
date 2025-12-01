# 🎄 Advent of Code 2025 – Rust Solutions 🎁

Welcome to my little **winter wonderland of Rust code** ❄️🦀!  
This repository contains my solutions for **[Advent of Code 2025](https://adventofcode.com/2025)** — a festive collection of coding puzzles that appear each December like tiny gifts behind digital calendar doors.

Every day unlocks a new challenge with two parts.  
This repo gathers my solutions: **clean, safe, idiomatic Rust sprinkled with Christmas magic.**

---

## ⭐ Tech Stack (North-Pole approved)

- **Rust** – fast, reliable, and perfect for keeping icy bugs away  
- **Cargo** – the sleigh that carries everything  
- **`cargo fmt` & `cargo clippy`** – polishing gifts until they sparkle ✨  
- **Tests** – because even Santa double-checks his lists

---

## 🌟 Project Structure

```text
.
├── Cargo.toml
├── src
│   ├── lib.rs           # Shared helpers used by Santa's elves
│   └── bin
│       ├── day01.rs     # Each day is a little advent calendar door
│       ├── day02.rs
│       └── ...
└── inputs
    ├── day01.txt        # Your personal letter from Santa
    ├── day02.txt
    └── ...
````

Each `dayXX.rs` is a standalone binary solving one day’s puzzle.

---

## 🎅 Getting Started

### 1. Install Rust (if you aren’t already working in Santa’s workshop)

```bash
curl https://sh.rustup.rs -sSf | sh
```

### 2. Clone the repository

```bash
git clone <YOUR-REPO-URL> aoc-2025-rust
cd aoc-2025-rust
```

### 3. Add your puzzle input

1. Log in at **adventofcode.com/2025**
2. Open the daily puzzle
3. Copy your personal input
4. Save it to: `inputs/dayXX.txt`

---

## 🦌 Running Solutions

### Run a specific day:

```bash
cargo run --bin day01
```

Run with explicit input:

```bash
cargo run --bin day01 -- inputs/day01.txt
```

### Run all solutions (if supported):

```bash
cargo run --bin all
```

---

## 🔔 Running Tests

```bash
cargo test
```

Or test a specific day:

```bash
cargo test day01
```

---

## ⭐ Coding Philosophy (Festive Edition)

* **Parse first** — like unwrapping the puzzle gift
* **Pure functions** — no side-effects cluttering Santa’s workshop
* **Readable, idiomatic Rust** — even an elf can understand it
* **Performance where fun** — because reindeer love benchmarks 🦌💨

---

## 🎁 Using This as a Template

1. Fork or copy the repo
2. Remove the existing day files
3. Add your own solutions
4. Make tea, turn on Christmas music
5. Enjoy the journey 🎄

---

## ❄️ Notes & Christmas Disclaimer

* Respect the AoC rules — no early spoilers for others!
* Inputs are **personal** — don’t commit someone else’s gift.
* Hot CPUs during solving are normal; Rust is just excited.

---

## 📜 License

```text
MIT — as free as sharing cookies at a Christmas party.
```

---

✨ **Happy coding, happy puzzling, and may your Advent be full of joy and zero bugs!** 🎄🦀
