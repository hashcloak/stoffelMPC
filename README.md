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

