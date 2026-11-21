# cryptkatool

A lightweight Rust command-line utility for text and password encryption using **AES-256-GCM** [`aes-gcm`](https://crates.io), which automatically generates a 256-bit key and 96-bit nonce, saving ciphertext and decryption secrets separately.

## Features

- **AES-256-GCM Encryption:** Authenticated encryption via [`aes-gcm`](https://crates.io/crates/aes-gcm).
- **Automatic Key & Nonce Generation:** Generates a secure key and nonce per run.
- **Separated Output:** Saves ciphertext and secrets into distinct files.
- **Smart Extension Handling:** Retains original file extensions for secret files or defaults to `.txt`.

## Prerequisites

- [Rust and Cargo](https://rust-lang.org)

## Installation

```bash
git clone https://github.com/kforkernel/cryptkatool.com
cd cryptkatool
cargo build --release
```

## Usage

> **Note:** Tested on **Linux**. **macOS and Windows builds are untested.**

Run via Cargo:
```bash
cargo run -- <word_to_encrypt> <output_file>
```

Or using compiled binaries:

### Linux
```bash
./target/release/cryptkatool <word_to_encrypt> <output_file>
```

Move cryptkatool into the `/usr/local/bin` directory:

```bash
sudo mv ./target/release/cryptkatool /usr/local/bin
```

### macOS *(Untested)*
```bash
./target/release/cryptkatool <word_to_encrypt> <output_file>
```

### Windows *(Untested)*
```powershell
.\target\release\cryptkatool.exe <word_to_encrypt> <output_file>
```

### Example

```bash
./target/release/cryptkatool "MySuperSecretPassword" password.txt
```

**Terminal Output:**
```text
Printing encrypted password to: password.txt
Printing key and nonce to: password_secrets.txt

Encrypted password: 8f9a2b3c4d...
```

**Generated Files:**
- **`password.txt`** — Contains the hex-encoded ciphertext.
- **`password_secrets.txt`** — Contains the hex-encoded `Key` and `Nonce` required for decryption and the `original` and `encrypted` passwords.
## Security Disclaimer

Built for learning and personal utility. Keep your `_secrets` files safe and never commit sensitive keys or passwords to version control.

## License

This project is under the kforkernel Use License (KUL) v1.0.
For more details, read [LICENSE.md](LICENSE.md).
