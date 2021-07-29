[![Current Crates.io Version(https://img.shields.io/badge/Crates.io-Current%20Version-orange?style=for-the-badge&logo=rust)](https://crates.io/crates/machineid-rs)

## MachineID/HWID for Rust

Get the encrypted HWID/MachineID of a pc running Windows or Linux.

### How to use

Getting the encrypted id is as simple as doing this

```rust
    let id_result = machineid_rs::encrypted_id("Your Key").unwrap()
```
