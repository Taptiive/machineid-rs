## MachineID/HWID for Rust

Get the encrypted unique machine id of a pc running Windows or Linux.

### How to use

Getting the encrypted id is as simple as doing this

```rust
    let id_result = machineid_rs::encrypted_id("Your Key").unwrap()
```