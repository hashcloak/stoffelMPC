#! /bin/sh
set -eux

rustup component add rustfmt

# Ensure all of our project's formatting maches our rustfmt configuration.
cargo fmt -- --check
