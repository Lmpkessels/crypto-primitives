# Cryptographic Algorithms

![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-red.svg)

**Cryptographic Algorithms** is a Rust repository focused on implementing cryptographic algorithms from first principles.

Using minimal standard library methods to reveal the core logic and mathematics behind each function.

## Installation

Make sure [Rust](https://rust-lang.org/tools/install/) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) are installed.

```bash
git clone https://github.com/Lmpkessels/cryptographic-algos.git
cd cryptographic-algos
cargo test
```

**cargo test** will run all unit tests across **SHA-256, HMAC, RIPE-MD160**, and **future crypto Algorithms**.

## Current progress

- ✅ Big endian padding/parsing (complete)
- ✅ Little endian padding/parsing (complete)
- ✅ SHA-256 (complete)
- ✅ RIPEMD-160 (complete)
- ✅ HMAC (complete)
- 🚧 MD4/MD5 (next)
- 🧩 SHA-1 (planned)
- 🧩 SHA-512 (planned)

## Resources

- [Big/Little Endianness](https://en.wikipedia.org/wiki/Endianness)
- [SHA-256](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf)
- [HMAC](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.198-1.pdf)
- [RIPEMD-160](https://homes.esat.kuleuven.be/~bosselae/ripemd160/pdf/AB-9601/AB-9601.pdf)

## Contribution

Pull requests are welcome.
For major changes, please open an issue first to discuss what you’d like to improve or add.

## Status

This repository is an ongoing learning project to deepen my understanding of:

- Cryptographic algorithm design
- Low-level systems building
- Secure Rust programming

## License

Licensed under [MIT License](./LICENSE-MIT). <br/>
© 2025 Luuk Kessels

## Connect

- 📧 [l@lmpkessels.com](mailto:l@lmpkessels.com)
- 🐦 [X/Twitter](https://x.com/lmpkessels)
- 👨‍💻 [GitHub](https://github.com/Lmpkessels)
- 🛠️ [Open an issue](https://github.com/Lmpkessels/cryptographic-algos/issues)
