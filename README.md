# Rust State Machine

A Rust-based state machine implementation following the [DotCodeSchool Rust State Machine course](https://dotcodeschool.com/courses/rust-state-machine). This project demonstrates the fundamental concepts of blockchain development by building a Polkadot-like state machine from scratch.

## Project Structure

The project is organized into several modules (pallets) that work together to form a complete state machine:

- `balances.rs`: Manages user balances and token transfers
- `system.rs`: Handles basic blockchain state management
- `main.rs`: The runtime that integrates all pallets and executes state transitions

## Learning Objectives

Through this project, you'll learn:

- Basic concepts of blockchain development
- Rust programming fundamentals
- State machine design patterns
- Safe math operations and error handling
- Generic and configurable types
- Block execution and transaction dispatch
- Rust macros for code generation

## Getting Started

1. Ensure you have Rust installed on your system
2. Clone this repository
3. Run the project:
   ```bash
   cargo run
   ```

## Testing

Run `cargo test` to test the functionality.

Use the following command to format the code:

```bash
cargo +nightly fmt
```

## Course Progress

The project follows the course structure:

1. Introduction to the Rust State Machine
2. The Balances Pallet
3. The System and Runtime
4. Generic and Configurable Types
5. Executing Blocks and Dispatching Calls
6. The Proof of Existence Pallet
7. Rust Macros

## License

This project is part of the educational material from DotCodeSchool's Rust State Machine course.
