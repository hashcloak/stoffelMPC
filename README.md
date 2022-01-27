# StoffelMPC

StoffelMPC is a framework for multi-party computation (MPC).
It allows developers to write MPC software in a domain specific
language which abstracts away the inner working details of MPC protocols.

These MPC programs are compiled to bytecode for the StoffelVM, which
is a virtual machine that allows to execute them in a multi-party setting.
Our implementation focuses on robust MPC protocols, as these are
interesting in the MPC-as-a-sidechain context. This means that MPC
is used to provide the currently missing privacy layer for public
blockchains.

The project is still at the very beginning. To compile the workspace and
run the tests just run `cargo t`.

## StoffelVM

StoffelVM can be run as a standalone VM. Just pass some bytecode to it and start
the execution.

```rust
use vm::StoffelVM;

// Some fictious bytecode
let mut bytecode: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];

// Instantiate a new VM and run it
let mut vm = StoffelVM::new();
vm.load_byte_code(&byte_code);
vm.execute();
```
