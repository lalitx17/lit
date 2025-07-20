cargo build
cargo run -- init
cargo run -- add .
cargo run -- commit -m "Initial commit"
touch src/utils/new_file.rs
cargo run -- add src/utils/new_file.rs
cargo run -- commit -m "Add new file"