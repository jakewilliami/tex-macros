set -xe

MODE="release"
cargo build --$MODE
D="$(basename "$PWD")"
F="${D%%.*}"
strip ./target/$MODE/$F
cp -f ./target/$MODE/$F ./
