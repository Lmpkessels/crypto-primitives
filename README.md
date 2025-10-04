# Crypto Primitives

![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-red.svg)

Crypto Primitives is a Rust project to explore cryptographic primitives from scratch.

It aims to build hash functions, message authenticators, Merkle trees, and more.

Implemented using only control flow with minimal method calls, to code from first principles.

## Installation

Make sure you have **Rust** and **Cargo** installed.  
Then clone and run tests:

## Usage

```bash
git clone https://github.com/Lmpkessels/crypto-primitives.git
cd crypto-primitives
cargo test
```

**cargo test** will run all unit tests across SHA-256, HMAC, and future primitives.

## Current progress

- ✅ SHA-256 (complete)
- ✅ HMAC (complete)
- ✅ Merkle Trees (complete)
- 🔄 UTXO (in progress)

## File Structure

```text
src/
├── hmac/
│ ├── hmac.rs        # HMAC implementation
│ ├── mod.rs         # Module declaration
│ └── size_util.rs   # Key normalization for HMAC
│
├── merkle_tree/
│ ├── branch.rs      # Combine left/right child nodes into parent
│ ├── loading.rs     # Hash leaves and handle odd counts
│ ├── merkle.rs      # Full Merkle tree construction
│ └── mod.rs         # Module declaration
│
├── sha256/
│ ├── compression.rs   # Compression function
│ ├── mod.rs           # Module declaration
│ ├── padding.rs       # Message padding
│ ├── parsing.rs       # Parse message into blocks
│ ├── schedule.rs      # Message schedule
│ ├── sha.rs           # Main SHA-256 pipeline
│ └── to_bytes.rs      # Convert digest words to bytes
│
├── lib.rs     # Library entry point
└── utils.rs   # Bitwise utilities (add, shift, rotate)
```

**Note:** All files include unit tests to validate correctness.

## Contribution

Pull requests are welcome.

For major changes, please open an issue first to discuss what you’d like to change.

## License

This project is licensed under the [MIT License](./LICENSE-MIT). <br/>
© 2025 Luuk Kessels

## Connect

- 📧 [l@lmpkessels.com](mailto:l@lmpkessels.com)
- 🐦 [@lmpkessels on X/Twitter](https://x.com/lmpkessels)
- 👨‍💻 [GitHub](https://github.com/Lmpkessels)
- 🛠️ [Open an issue](https://github.com/Lmpkessels/crypto-primitives/issues/new)
