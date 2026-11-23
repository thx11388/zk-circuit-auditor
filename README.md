# ZK Circuit Auditor

A Rust-based tool for analyzing basic Zero-Knowledge (ZK) Rank-1 Constraint System (R1CS) circuits using the [Arkworks](https://github.com/arkworks-rs) ecosystem.

## Overview

`zk-circuit-auditor` demonstrates how basic circuit checks can be analyzed by inspecting their generated constraints, counting public and private variables, and checking whether the supplied witness satisfies the circuit.

The project includes three example circuits:

- **Boolean Check:** Enforces that a witness value is either zero or one.
- **Range Check:** Uses a four-bit decomposition to constrain a value to the range 0–15.
- **Underconstrained Circuit:** Demonstrates how private witness variables can exist without any constraints restricting them.

## Features

- R1CS constraint generation using Arkworks.
- Constraint and variable counting.
- Witness satisfaction checking.
- Basic heuristic warnings for circuits with private variables but no constraints.
- Unit tests covering valid and invalid witnesses.

## Technologies

- Rust
- Arkworks
- `ark-ff`
- `ark-relations`
- `ark-bn254`

## Project Structure

```text
zk-circuit-auditor/
├── Cargo.toml
├── Cargo.lock
└── src/
    ├── lib.rs
    ├── main.rs
    ├── analyzer.rs
    └── circuits.rs
```

## Getting Started

### Prerequisites

Install Rust and Cargo:

https://www.rust-lang.org/tools/install

### Run the project

Clone the repository:

```bash
git clone https://github.com/thx11388/zk-circuit-auditor.git
cd zk-circuit-auditor
```

Run the application:

```bash
cargo run
```

Run the tests:

```bash
cargo test
```

Build an optimized release:

```bash
cargo build --release
```

## Example Output

```text
================================
       ZK CIRCUIT AUDITOR
================================

=== BOOLEAN CHECK ===
Constraints: 1
Public variables: 0
Private variables: 1
Satisfied: true

=== RANGE CHECK ===
Constraints: 5
Public variables: 0
Private variables: 5
Satisfied: true

=== UNDERCONSTRAINED CHECK ===
Constraints: 0
Public variables: 0
Private variables: 2
Satisfied: true
WARNING: Private variables exist, but no constraints were generated.
```

## Security Notes and Limitations

This project is an educational prototype, not a complete ZK circuit security scanner.

The analyzer currently reports basic R1CS statistics and checks whether the supplied witness satisfies the generated constraints. Its warning mechanism detects one simple suspicious pattern: private variables with zero generated constraints.

A circuit can still be underconstrained even when it contains constraints. Therefore, a successful satisfaction check does not prove that a circuit is secure or that its witness is uniquely determined.

The range-check example uses four Boolean-constrained bits and reconstructs the value in the BN254 scalar field, restricting the represented value to 0–15.

## Tests

The test suite covers:

1. Valid Boolean witness.
2. Invalid Boolean witness.
3. Underconstrained circuit demonstration.
4. Valid four-bit range value.
5. Value outside the four-bit range.

## Disclaimer

This project is intended for educational and research purposes. It does not replace a professional cryptographic audit or formal verification.

## Author

GitHub: [@thx11388](https://github.com/thx11388)