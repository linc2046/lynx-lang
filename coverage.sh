# https://marco-c.github.io/2020/11/24/rust-source-based-code-coverage.html
# cargo install grcov
# rustup component add llvm-tools-preview
# cargo test --package lynx --lib -- parser::unit_test::parse_fibonacci --exact --nocapture 

export RUSTFLAGS="-C instrument-coverage"
export LLVM_PROFILE_FILE="./coverage/lynx-%p-%m.profraw"
cargo test
grcov . --binary-path ./target/debug -s . -t html --branch --ignore-not-existing -o ./coverage/