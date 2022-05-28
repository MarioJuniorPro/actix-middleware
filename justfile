set dotenv-load
export RUST_BACKTRACE := "1"
alias d := dev

build:
    cargo build --release


test:
    # will print a stack trace if it crashes
    cargo test

default:
    echo 'Hello, world!'

dev:
    cargo watch --exec 'run'

install:
    cargo install cargo-edit
    cargo install cargo-watch

serve:
    @echo "Starting server with host $HOST on port $PORT…"


burst:
    wrk -t4 -c400 -d30s http://127.0.0.1:7008/

ping:
    watch -n 2 --color http ${HOST}:${PORT} -vv --pretty all

fix:
    cargo clippy --fix --allow-dirty
    cargo fmt

pr:
    gh pr create --fill -w