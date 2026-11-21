# cryptkatool

A lightweight command-line utility written in Rust that encrypts text and passwords using **AES-256-GCM**. It automatically generates a random 256-bit key and a unique 96-bit nonce, saving the hex-encoded ciphertext and decryption secrets into separate files.

## Features

- **AES-256-GCM Encryption:** Uses authenticated encryption via the [`aes-gcm`](https://crates.io/crates/aes-gcm)) crate.
- **Automatic Key & Nonce Generation:** Generates a fresh, cryptographically secure key and nonce for every run.
- **Separated Output:** Saves the encrypted ciphertext to your chosen file and stores the matching key and nonce in a separate `_secrets` file.
- **Smart Extension Handling:** Preserves your output file extension (e.g., `.txt`, `.csv`) for the secrets file, or defaults to `.txt` if none is provided.

## Prerequisites

- [Rust and Cargo](https://rust-lang.org) installed on your machine.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/kforkernel/cryptkatool
   cd cryptkatool
   ```

2. Build the release binary:
   ```bash
   cargo build --release
   ```

The compiled binary will be available at `target/release/cryptkatool`.

## Usage

Run the tool by passing the text you want to encrypt and the desired output file path:

```bash
cargo run -- <word_to_encrypt> <output_file>
```

Or, if using the compiled binary directly:

```bash
./target/release/cryptkatool <word_to_encrypt> <output_file>
```

### Example

```bash
cryptkatool "MySuperSecretPassword" password.txt
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

This project was built for learning and personal utility purposes. Keep your `_secrets` file secure and never commit keys, nonces, or sensitive passwords to public version control.
