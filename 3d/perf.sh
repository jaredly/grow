set -e
cargo build --release
time cargo run --release -- make 1100 perf-mm.bin --start perf1.bin
cargo run --release -- shoot perf-mm.bin 0.0 10.5 30.0 0.0 10.0 0.0
