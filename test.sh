sudo cargo build
sudo cargo run -- init
sudo cargo run -- add .
sudo cargo run -- commit -m "Initial commit"
sudo touch src/utils/new_file.rs
sudo cargo run -- add src/utils/new_file.rs
sudo cargo run -- commit -m "Add new file"