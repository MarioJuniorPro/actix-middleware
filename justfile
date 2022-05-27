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
    watch -n 2 -c 'http ${HOST}:${PORT} -vv'

