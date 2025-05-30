# QUANTUM HSM PROTOCOL

## Overview

Quantum HSM Protocol defines the secure interface, data structures, and cryptographic expectations for interactions with the `quantum_hsm` engine. It provides a blueprint for quantum-safe wallet operations, including:

* Key generation
* Digital signing
* Signature verification

## Design Goals

* Post-quantum secure using Dilithium (CRYSTALS)
* Stateless API over HTTP (RESTful design)
* Safe key storage with in-memory encryption
* Designed for embedded HSM usage and remote calls

---

## Key Generation

**Endpoint:** `/generate`
**Method:** `POST`
**Request Body:**

```json
{
  "wallet_id": "string"
}
```

**Response:** Hex-encoded public key

## Message Signing

**Endpoint:** `/sign`
**Method:** `POST`
**Request Body:**

```json
{
  "wallet_id": "string",
  "message": "string (UTF-8 or hex)",
  "is_hex": true|false
}
```

**Response:** Hex-encoded signature

## Signature Verification

**Endpoint:** `/verify`
**Method:** `POST`
**Request Body:**

```json
{
  "message": "string",
  "signature": "hex-encoded",
  "public_key": "hex-encoded",
  "is_hex": true|false
}
```

**Response:**

```json
{
  "valid": true|false
}
```

---

## Key Security Model

* Keys are stored in-memory using `secrecy::Secret<Vec<u8>>`
* Access is gated via `parking_lot::Mutex`
* No persistence to disk by default
* Hardware support via pluggable interface (TBD)

---

## Future Extensions

* `/threshold/generate` – generate a threshold key
* `/multi-sign` – sign with multiple keys
* `/rotate` – rotate keypair

## Versioning

Protocol version: `v0.2.0`

See `RELEASE_NOTES.md` for change history.

## License

MIT License © 2025 Orly Trajano

