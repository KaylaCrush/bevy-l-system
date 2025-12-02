# L-System Plant Generator

Procedural plant growth using deterministic and stochastic L-systems

![preview](./assets/img/bevy-lsys.gif)

---

## Overview

This project implements an L-system-based procedural plant generator.
L-systems (Lindenmayer systems) are parallel rewriting systems traditionally used to model plant growth, branching structures, and other self-similar natural forms.

The generator supports rule definitions, iterative string expansion, and geometric interpretation of the resulting symbol sequences. The output is rendered using Rust and Bevy, producing plant-like structures with tunable parameters.

---

## Features

* Deterministic or stochastic rewriting rules
* Adjustable iteration depth and rule sets
* Turtle-graphics interpretation for geometric rendering
* Parameterized branch length, angle, and scaling factors
* Support for multiple plant “species” using different rule definitions
* Real-time re-generation for interactive experimentation
* Modular rule parser and interpreter

---

## Technical Details

### L-System Structure

An L-system in this project is defined by:

* An alphabet of symbols (e.g., `F`, `+`, `-`, `[`, `]`)
* An axiom (initial string)
* A set of production rules, possibly stochastic
* An iteration count determining growth depth

At each iteration, rules are applied in parallel to all symbols in the current sequence.

### Rendering

A stack-based turtle interpreter converts the expanded symbol string into line segments representing branches.

Supported commands include:

* `F`: move forward and draw
* `f`: move forward without drawing
* `+` / `-`: rotate
* `[` : push current position/angle
* `]` : pop position/angle

Rendering is handled through (Bevy / another renderer), allowing interactive preview.

### Stochastic Rules

Optional probabilistic productions introduce structural variation.
Example:

```
F -> F[+F]F[-F]F (60%)
F -> F[+F]F (40%)
```

This produces more natural-looking diversity across iterations.

---

## Running the Project

Requirements:

* Rust
* Cargo

Run with:

```bash
cargo run --release
```

---

## Possible Extensions

* Integration with physics or wind simulation
* Growth influenced by environment
* Export as SVG or mesh geometry

---

## License

MIT (or your preferred license)