# Quantum HSM

![Quantum SDK](https://img.shields.io/badge/Quantum%20SDK-Dilithium-blueviolet?style=flat-square)
![Rust](https://img.shields.io/badge/Rust-cryptography-orange?style=flat-square)
![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)

> A high-assurance post-quantum Hardware Security Module (HSM) built in Rust.  
> Powered by [Dilithium](https://pq-crystals.org/dilithium/) signatures and designed for blockchain-grade digital signing.

---
## Use Case

-  Quantum Vault can be deployed by a crypto custody provider to replace traditional HSMs with quantum-safe signing APIs.
-  For example, a Solana-based wallet provider can use /sign to generate Dilithium-signed transactions stored securely with secrecy.
## Features

-  NIST-approved Dilithium digital signatures
-  Secure in-memory key storage using `secrecy`
-  Axum-powered API: `/generate`, `/sign`, `/verify`
-  Supports message signing for Solana, Ethereum, and Bitcoin
-  Ready for integration with validator or DeFi infrastructure

---

##  Installation

```bash
git clone https://github.com/0rlych1kk4/quantum_hsm.git
cd quantum_hsm
cargo build --release

##  Usage

Start the API server:

cargo run
 Example: generate a keypair
curl -X POST http://127.0.0.1:8080/generate \
  -H "Content-Type: application/json" \
  -d '{"wallet_id":"wallet1"}'
️ Sign a message
curl -X POST http://127.0.0.1:8080/sign \
  -H "Content-Type: application/json" \
  -d '{"wallet_id":"wallet1", "message":"48656c6c6f20776f726c64", "is_hex":true}'
 Verify a signature
curl -X POST http://127.0.0.1:8080/verify \
  -H "Content-Type: application/json" \
  -d '{"message":"48656c6c6f20776f726c64", "signature":"...", "public_key":"...", "is_hex":true}'
##  API Endpoints

Method	Endpoint	Description
POST	/generate	Generate a new keypair
POST	/sign	Sign a message using wallet ID
POST	/verify	Verify a message + signature

##  Release Notes

See RELEASE_NOTES.md

##  License

This project is licensed under the MIT License. See LICENSE for full terms.

##  Contributing

Contributions welcome! Open issues or submit PRs — especially for blockchain support, multichain signing, or API extensions.

##  Contact

GitHub: 0rlych1kk4

