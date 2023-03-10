<a href="https://crates.io/crates/machineid-rs"><img src="https://img.shields.io/crates/v/machineid-rs?style=for-the-badge&logo=rust&color=orange" /></a>
<a href="https://docs.rs/machineid-rs/latest/machineid_rs/">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=for-the-badge&logo=rust&color=blue"
      alt="docs.rs docs" />
</a>

## MachineID for Rust - Like .Net DeviceId

This Rust package is inspired by [DeviceId](https://github.com/MatthewKing/DeviceId), a .Net package to build a unique Machine ID.

### Features

- 3 Different types of hash (*MD5*, *SHA1*, *SHA256*)
- Different components to make the ID
- Support for Windows and Linux
- No Admin privileges are required

### How to use

First add this to your Cargo.toml file

```toml
[dependencies]
machineid-rs = "1.2.3"
```

Then, you need to define the builder variable with the encryption type you want.

For example, **SHA256**
```rust
use machineid_rs::{IdBuilder, Encryption};

// There are 3 different encryption types: MD5, SHA1 and SHA256.
let mut builder = IdBuilder::new(Encryption::SHA256);
```

After that, you just need to add the components you want the id to have.

The available components are:

- **System UUID**: Unique identifier of your machine
  
- **CPU Cores**: Number of physical cores from your computer
  
- **OS Name**: Operative System name, i.e., linux/windows
  
- **Username**: The username currently being used
  
- **Machine Name**: The name of the machine
  
- **CPU ID**: The serial number of the processor
  
- **Drive Serial** : The serial number of the disk storing the OS.
  
For example, i will add the System UUID and CPU Cores
```rust
use machineid_rs::HWIDComponent;

builder.add_component(HWIDComponent::SystemID).add_component(HWIDComponent::CPUCores);
```

Once you are ready, you just need to build the id with your key

```rust
let hwid = builder.build("mykey").unwrap();
```

### Todo

- Optimize the code
  
*Feel free to report any bug you find! ;)*
