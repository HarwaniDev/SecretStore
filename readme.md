# 🔐 SecretStore - Simple Password Manager in Rust

SecretStore is a beginner-friendly, terminal-based password manager built entirely with Rust, designed to securely store your platform credentials, protected by a master password.

This project is a hands-on Rust learning experience, demonstrating how to:

✅ Build a CLI tool with argument parsing  
✅ Handle secure user input  
✅ Encrypt sensitive data  
✅ Structure cross-platform Rust applications  

## 🚀 Features

- **Secure Storage**: AES-256-GCM encryption for all stored credentials
- **Master Password Protection**: Single master password protects your entire vault
- **Terminal-Based**: Simple command-line interface
- **No Network Dependencies**: All data stored locally

## 🧩 Crates Used

Here are the main Rust crates powering SecretStore:

| Crate | Purpose | Link |
|-------|---------|------|
| `clap` | Command-line argument parsing | [clap on crates.io](https://crates.io/crates/clap) |
| `rpassword` | Read passwords without terminal echo | [rpassword on crates.io](https://crates.io/crates/rpassword) |
| `aes-gcm` | AES-256 authenticated encryption (GCM mode) | [aes-gcm on crates.io](https://crates.io/crates/aes-gcm) |
| `rand` | Random number generation for keys/salts | [rand on crates.io](https://crates.io/crates/rand) |
| `base64` | Encoding binary data to Base64 for storage | [base64 on crates.io](https://crates.io/crates/base64) |

## 🎯 Why Rust for This Project?

Rust offers:

- **Strong memory safety guarantees** 🦀
- **High performance** for low-overhead tools ⚡
- **Built-in support** for creating cross-platform binaries
- **A powerful ecosystem** for security and cryptography

SecretStore is a simple yet practical project to apply these benefits in real-world development.