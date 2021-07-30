<a href="https://crates.io/crates/machineid-rs"><img src="https://img.shields.io/crates/v/machineid-rs?style=for-the-badge&logo=rust&color=orange" /></a>

## MachineId for Rust - Like .Net DeviceId

This Rust package is inspired by [DeviceId](https://github.com/MatthewKing/DeviceId), a .Net package to build a unique Machine ID.

### Features

- 3 Different types of hash (MD5, SHA1, SHA256)
- Different components to make the id
- Support for Windows and Linux
- No Admin privileges are required

### How to use

First add this to your Cargo.toml file

```toml
[dependencies]
machineid-rs = "1.1.1"
```

Then, you need to define the builder variable with the encryption type you want.

For example, **SHA256**
```rust
use machineid_rs::{Encryption, IdBuilder};

let mut builder = IdBuilder::new(Encryption::SHA256);
```

After that, you just need to add the components you want the id to have.

The available components are:

- System UUID
- CPU Cores
- OS Name
- Username
- Machine Name
- CPU ID
- Drive Serial (Only in windows)
  
For example, i will add the System UUID and CPU Cores
```rust
    builder.add_system_id().add_cpu_cores();
```

Once you are ready, you just need to build the id with your key

```rust
    let unique_id = builder.build("your key");
```

### Todo

- Add MAC Address
- Add Motherboard Serial
- Add File Token
- Optimize the code
  
It is very possible for this code to have numerous bugs so if you find one, feel free to report it
