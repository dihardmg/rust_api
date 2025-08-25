#!/bin/sh
set -e

# Jalankan migrasi menggunakan `cargo run`
# Anda bisa mengganti `migration` dengan nama crate yang sesuai
echo "Running migrations..."
cargo run -p migration up

# Jalankan aplikasi utama Rust
# `exec` akan mengganti proses shell dengan aplikasi,
# yang merupakan praktik terbaik untuk kontainer
echo "Starting Rust API..."
exec cargo run --release
